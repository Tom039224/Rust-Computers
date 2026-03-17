package com.rustcomputers.peripheral.buffer;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.annotation.Nullable;
import java.util.*;
import java.util.concurrent.ConcurrentHashMap;

/**
 * 結果バッファ管理。
 * Result buffer management.
 *
 * <p>クエリ結果（単一）とアクション結果（複数）を管理する。
 * エラーも保存する。</p>
 *
 * <p>Manages query results (single) and action results (multiple).
 * Also stores errors.</p>
 */
public class ResultBuffer {
    private static final Logger LOGGER = LoggerFactory.getLogger(ResultBuffer.class);

    /**
     * クエリ結果: periph_id → method_name → result (byte[] or PeripheralError)
     * Query results: periph_id → method_name → result (byte[] or PeripheralError)
     */
    private final Map<Integer, Map<String, Object>> queryResults = new ConcurrentHashMap<>();

    /**
     * アクション結果: periph_id → method_name → list of results (byte[] or PeripheralError)
     * Action results: periph_id → method_name → list of results (byte[] or PeripheralError)
     */
    private final Map<Integer, Map<String, List<Object>>> actionResults = new ConcurrentHashMap<>();

    /**
     * クエリ結果を保存する。
     * Store a query result.
     *
     * @param periphId   ペリフェラルID / peripheral ID
     * @param methodName メソッド名 / method name
     * @param result     結果（byte[]） / result (byte[])
     */
    public void storeQueryResult(int periphId, String methodName, byte[] result) {
        queryResults.computeIfAbsent(periphId, k -> new ConcurrentHashMap<>())
                .put(methodName, result);
        LOGGER.debug("Stored query result: periphId={}, method={}, size={}", 
                periphId, methodName, result.length);
    }

    /**
     * クエリエラーを保存する。
     * Store a query error.
     *
     * @param periphId   ペリフェラルID / peripheral ID
     * @param methodName メソッド名 / method name
     * @param error      エラー / error
     */
    public void storeQueryError(int periphId, String methodName, PeripheralError error) {
        queryResults.computeIfAbsent(periphId, k -> new ConcurrentHashMap<>())
                .put(methodName, error);
        LOGGER.debug("Stored query error: periphId={}, method={}, error={}", 
                periphId, methodName, error.type());
    }

    /**
     * アクション結果を保存する。
     * Store an action result.
     *
     * @param periphId   ペリフェラルID / peripheral ID
     * @param methodName メソッド名 / method name
     * @param result     結果（byte[]） / result (byte[])
     */
    public void storeActionResult(int periphId, String methodName, byte[] result) {
        actionResults.computeIfAbsent(periphId, k -> new ConcurrentHashMap<>())
                .computeIfAbsent(methodName, k -> new ArrayList<>())
                .add(result);
        LOGGER.debug("Stored action result: periphId={}, method={}, size={}", 
                periphId, methodName, result.length);
    }

    /**
     * アクションエラーを保存する。
     * Store an action error.
     *
     * @param periphId   ペリフェラルID / peripheral ID
     * @param methodName メソッド名 / method name
     * @param error      エラー / error
     */
    public void storeActionError(int periphId, String methodName, PeripheralError error) {
        actionResults.computeIfAbsent(periphId, k -> new ConcurrentHashMap<>())
                .computeIfAbsent(methodName, k -> new ArrayList<>())
                .add(error);
        LOGGER.debug("Stored action error: periphId={}, method={}, error={}", 
                periphId, methodName, error.type());
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
        return queryResults.getOrDefault(periphId, new HashMap<>()).get(methodName);
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
        return actionResults.getOrDefault(periphId, new HashMap<>())
                .getOrDefault(methodName, new ArrayList<>());
    }

    /**
     * 特定のペリフェラルの結果をクリアする。
     * Clear results for a specific peripheral.
     *
     * @param periphId ペリフェラルID / peripheral ID
     */
    public void clear(int periphId) {
        queryResults.remove(periphId);
        actionResults.remove(periphId);
        LOGGER.debug("Cleared results for periphId={}", periphId);
    }

    /**
     * 全結果をクリアする。
     * Clear all results.
     */
    public void clearAll() {
        queryResults.clear();
        actionResults.clear();
        LOGGER.debug("Cleared all results");
    }
}
