package com.rustcomputers.wasm;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.annotation.Nullable;
import java.util.HashMap;
import java.util.Iterator;
import java.util.Map;
import java.util.concurrent.atomic.AtomicLong;

/**
 * リクエスト ID の発行と保留結果の管理。
 * Manages request ID allocation and pending result tracking.
 *
 * <p>ホスト関数が呼ばれるたびに一意の request_id を発行し、
 * 結果が返るまで {@link PendingResult} として保持する。</p>
 *
 * <p>Issues unique request IDs for each host function call and
 * holds {@link PendingResult} instances until they are resolved.</p>
 */
public final class RequestManager {

    private static final Logger LOGGER = LoggerFactory.getLogger(RequestManager.class);

    /** モノトニック増加カウンター / Monotonically increasing counter */
    private final AtomicLong nextId = new AtomicLong(1L);

    /** 保留中のリクエスト / Pending requests: requestId → PendingResult */
    private final Map<Long, PendingResult> pending = new HashMap<>();

    /** 現在の tick 番号（WasmEngine が毎 tick 更新） / Current tick (updated by WasmEngine) */
    private long currentTick;

    // ------------------------------------------------------------------
    // ID 発行 / ID allocation
    // ------------------------------------------------------------------

    /**
     * 新しいリクエスト ID を発行する。
     * Allocate a new unique request ID.
     *
     * @return 正の request_id / positive request ID
     */
    public long nextRequestId() {
        return nextId.getAndIncrement();
    }

    // ------------------------------------------------------------------
    // 保留結果の登録・クエリ / Register & query pending results
    // ------------------------------------------------------------------

    /**
     * 保留結果を登録する。
     * Register a pending result.
     *
     * @param result 保留結果 / pending result to register
     */
    public void register(PendingResult result) {
        pending.put(result.getRequestId(), result);
    }

    /**
     * 指定 ID の保留結果を取得する。
     * Get the pending result for the given ID.
     *
     * @param requestId リクエスト ID / request ID
     * @return 保留結果、存在しなければ null / pending result, or null if not found
     */
    @Nullable
    public PendingResult get(long requestId) {
        return pending.get(requestId);
    }

    /**
     * 完了済みの保留結果を除去する。
     * Remove a completed pending result.
     *
     * @param requestId リクエスト ID / request ID
     * @return 除去された結果、存在しなければ null / removed result, or null if not found
     */
    @Nullable
    public PendingResult remove(long requestId) {
        return pending.remove(requestId);
    }

    // ------------------------------------------------------------------
    // Tick 管理 / Tick management
    // ------------------------------------------------------------------

    /**
     * 現在の tick 番号を更新し、タイムアウトした保留結果を処理する。
     * Update the current tick and handle timed-out pending results.
     *
     * @param tick       現在の tick 番号 / current tick number
     * @param timeoutTicks タイムアウト tick 数 / timeout duration in ticks
     */
    public void tick(long tick, int timeoutTicks) {
        this.currentTick = tick;

        // タイムアウト処理 / Handle timeouts
        Iterator<Map.Entry<Long, PendingResult>> it = pending.entrySet().iterator();
        while (it.hasNext()) {
            Map.Entry<Long, PendingResult> entry = it.next();
            PendingResult pr = entry.getValue();
            if (!pr.isCompleted() && (tick - pr.getIssuedAtTick()) > timeoutTicks) {
                LOGGER.warn("Request {} timed out after {} ticks", pr.getRequestId(), timeoutTicks);
                pr.completeWithError(ErrorCodes.ERR_TIMEOUT);
            }
        }
    }

    /**
     * 現在の tick 番号を返す。
     * Return the current tick number.
     */
    public long getCurrentTick() {
        return currentTick;
    }

    /**
     * 保留中のリクエスト数を返す。
     * Return the number of pending requests.
     */
    public int pendingCount() {
        return pending.size();
    }

    /**
     * 全保留結果をクリアする（シャットダウン用）。
     * Clear all pending results (for shutdown).
     */
    public void clear() {
        pending.clear();
    }
}
