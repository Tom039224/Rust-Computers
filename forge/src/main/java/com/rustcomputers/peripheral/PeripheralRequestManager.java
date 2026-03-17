package com.rustcomputers.peripheral;

import com.rustcomputers.peripheral.buffer.*;
import com.rustcomputers.peripheral.event.EventMonitor;
import net.minecraft.core.BlockPos;
import net.minecraft.server.level.ServerLevel;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.annotation.Nullable;
import java.util.List;
import java.util.Map;

/**
 * ペリフェラルリクエスト管理。
 * Peripheral request manager.
 *
 * <p>RequestBuffer, ResultBuffer, EventMonitorを統合し、
 * tick()メソッドでリクエスト実行とイベントポーリングを行う。</p>
 *
 * <p>Integrates RequestBuffer, ResultBuffer, and EventMonitor.
 * Executes requests and polls events in tick() method.</p>
 */
public class PeripheralRequestManager {
    private static final Logger LOGGER = LoggerFactory.getLogger(PeripheralRequestManager.class);

    private final RequestBuffer requestBuffer = new RequestBuffer();
    private final ResultBuffer resultBuffer = new ResultBuffer();
    private final EventMonitor eventMonitor = new EventMonitor();

    /**
     * クエリリクエストを予約する。
     * Book a query request.
     *
     * @param periphId   ペリフェラルID / peripheral ID
     * @param methodName メソッド名 / method name
     * @param args       引数 / arguments
     */
    public void bookQuery(int periphId, String methodName, byte[] args) {
        requestBuffer.bookQuery(periphId, methodName, args);
    }

    /**
     * アクションリクエストを予約する。
     * Book an action request.
     *
     * @param periphId   ペリフェラルID / peripheral ID
     * @param methodName メソッド名 / method name
     * @param args       引数 / arguments
     */
    public void bookAction(int periphId, String methodName, byte[] args) {
        requestBuffer.bookAction(periphId, methodName, args);
    }

    /**
     * イベントリスナーを登録する。
     * Register an event listener.
     *
     * @param periphId  ペリフェラルID / peripheral ID
     * @param eventName イベント名 / event name
     */
    public void registerEventListener(int periphId, String eventName) {
        eventMonitor.registerListener(periphId, eventName);
    }

    /**
     * クエリ結果を取得する。
     * Get a query result.
     *
     * @param periphId   ペリフェラルID / peripheral ID
     * @param methodName メソッド名 / method name
     * @return 結果（byte[] or PeripheralError）、なければnull / result (byte[] or PeripheralError) or null
     */
    @Nullable
    public Object getQueryResult(int periphId, String methodName) {
        return resultBuffer.getQueryResult(periphId, methodName);
    }

    /**
     * アクション結果を取得する。
     * Get action results.
     *
     * @param periphId   ペリフェラルID / peripheral ID
     * @param methodName メソッド名 / method name
     * @return 結果のリスト（byte[] or PeripheralError） / list of results (byte[] or PeripheralError)
     */
    public List<Object> getActionResults(int periphId, String methodName) {
        return resultBuffer.getActionResults(periphId, methodName);
    }

    /**
     * イベントを取得する。
     * Get events.
     *
     * @param periphId  ペリフェラルID / peripheral ID
     * @param eventName イベント名 / event name
     * @return イベントのリスト / list of events
     */
    public List<Object> getEvents(int periphId, String eventName) {
        return eventMonitor.getEvents(periphId, eventName);
    }

    /**
     * tick処理を実行する。
     * Execute tick processing.
     *
     * <p>全リクエストを実行し、イベントをポーリングする。</p>
     * <p>Executes all requests and polls events.</p>
     *
     * @param peripherals 接続されているペリフェラルのマップ / map of attached peripherals
     * @param level       サーバーレベル / server level
     */
    public void tick(Map<Integer, AttachedPeripheral> peripherals, ServerLevel level) {
        // リクエストを実行
        // Execute requests
        executeAll(peripherals, level);

        // イベントをポーリング
        // Poll events
        eventMonitor.pollEvents(peripherals);
    }

    /**
     * 全リクエストを実行する。
     * Execute all requests.
     *
     * @param peripherals 接続されているペリフェラルのマップ / map of attached peripherals
     * @param level       サーバーレベル / server level
     */
    private void executeAll(Map<Integer, AttachedPeripheral> peripherals, ServerLevel level) {
        // クエリリクエストを実行
        // Execute query requests
        for (Map.Entry<Integer, Map<String, PendingRequest>> entry : 
                requestBuffer.getQueryRequests().entrySet()) {
            int periphId = entry.getKey();
            AttachedPeripheral periph = peripherals.get(periphId);

            if (periph == null) {
                // ペリフェラルが見つからない
                // Peripheral not found
                for (String methodName : entry.getValue().keySet()) {
                    resultBuffer.storeQueryError(periphId, methodName, 
                            PeripheralError.notFound("Peripheral not found: " + periphId));
                }
                continue;
            }

            for (Map.Entry<String, PendingRequest> reqEntry : entry.getValue().entrySet()) {
                String methodName = reqEntry.getKey();
                PendingRequest req = reqEntry.getValue();

                try {
                    byte[] result = periph.type().callMethod(
                            methodName, 
                            req.args(), 
                            level, 
                            periph.peripheralPos()
                    );
                    resultBuffer.storeQueryResult(periphId, methodName, result);
                } catch (PeripheralException e) {
                    resultBuffer.storeQueryError(periphId, methodName, 
                            PeripheralError.executionFailed(e.getMessage()));
                } catch (Exception e) {
                    LOGGER.error("Query execution failed: periphId={}, method={}", 
                            periphId, methodName, e);
                    resultBuffer.storeQueryError(periphId, methodName, 
                            PeripheralError.executionFailed("Unexpected error: " + e.getMessage()));
                }
            }
        }

        // アクションリクエストを実行
        // Execute action requests
        for (Map.Entry<Integer, Map<String, List<PendingRequest>>> entry : 
                requestBuffer.getActionRequests().entrySet()) {
            int periphId = entry.getKey();
            AttachedPeripheral periph = peripherals.get(periphId);

            if (periph == null) {
                // ペリフェラルが見つからない
                // Peripheral not found
                for (Map.Entry<String, List<PendingRequest>> reqEntry : entry.getValue().entrySet()) {
                    String methodName = reqEntry.getKey();
                    for (int i = 0; i < reqEntry.getValue().size(); i++) {
                        resultBuffer.storeActionError(periphId, methodName, 
                                PeripheralError.notFound("Peripheral not found: " + periphId));
                    }
                }
                continue;
            }

            for (Map.Entry<String, List<PendingRequest>> reqEntry : entry.getValue().entrySet()) {
                String methodName = reqEntry.getKey();
                List<PendingRequest> requests = reqEntry.getValue();

                for (PendingRequest req : requests) {
                    try {
                        byte[] result = periph.type().callMethod(
                                methodName, 
                                req.args(), 
                                level, 
                                periph.peripheralPos()
                        );
                        resultBuffer.storeActionResult(periphId, methodName, result);
                    } catch (PeripheralException e) {
                        resultBuffer.storeActionError(periphId, methodName, 
                                PeripheralError.executionFailed(e.getMessage()));
                    } catch (Exception e) {
                        LOGGER.error("Action execution failed: periphId={}, method={}", 
                                periphId, methodName, e);
                        resultBuffer.storeActionError(periphId, methodName, 
                                PeripheralError.executionFailed("Unexpected error: " + e.getMessage()));
                    }
                }
            }
        }

        // リクエストバッファをクリア
        // Clear request buffer
        requestBuffer.clearAll();
    }

    /**
     * 特定のペリフェラルのデータをクリアする。
     * Clear data for a specific peripheral.
     *
     * @param periphId ペリフェラルID / peripheral ID
     */
    public void clear(int periphId) {
        requestBuffer.clear(periphId);
        resultBuffer.clear(periphId);
        eventMonitor.clearEvents(periphId);
    }

    /**
     * 全データをクリアする。
     * Clear all data.
     */
    public void clearAll() {
        requestBuffer.clearAll();
        resultBuffer.clearAll();
        eventMonitor.clearAllEvents();
    }
}
