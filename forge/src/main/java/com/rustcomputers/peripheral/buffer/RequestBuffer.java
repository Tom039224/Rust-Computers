package com.rustcomputers.peripheral.buffer;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.*;
import java.util.concurrent.ConcurrentHashMap;

/**
 * リクエストバッファ管理。
 * Request buffer management.
 *
 * <p>クエリリクエスト（情報取得系）とアクションリクエスト（ワールド干渉系）を管理する。
 * クエリは最新のリクエストのみ保持（上書き）、アクションは全て保持（追記）。</p>
 *
 * <p>Manages query requests (information retrieval) and action requests (world interaction).
 * Query requests overwrite previous ones, action requests accumulate.</p>
 */
public class RequestBuffer {
    private static final Logger LOGGER = LoggerFactory.getLogger(RequestBuffer.class);

    /**
     * クエリリクエスト: periph_id → method_name → latest request
     * Query requests: periph_id → method_name → latest request
     */
    private final Map<Integer, Map<String, PendingRequest>> queryRequests = new ConcurrentHashMap<>();

    /**
     * アクションリクエスト: periph_id → method_name → list of requests
     * Action requests: periph_id → method_name → list of requests
     */
    private final Map<Integer, Map<String, List<PendingRequest>>> actionRequests = new ConcurrentHashMap<>();

    /**
     * クエリリクエストを予約する（上書き）。
     * Book a query request (overwrites previous).
     *
     * @param periphId   ペリフェラルID / peripheral ID
     * @param methodName メソッド名 / method name
     * @param args       引数 / arguments
     */
    public void bookQuery(int periphId, String methodName, byte[] args) {
        queryRequests.computeIfAbsent(periphId, k -> new ConcurrentHashMap<>())
                .put(methodName, PendingRequest.query(methodName, args));
        LOGGER.debug("Booked query: periphId={}, method={}", periphId, methodName);
    }

    /**
     * アクションリクエストを予約する（追記）。
     * Book an action request (accumulates).
     *
     * @param periphId   ペリフェラルID / peripheral ID
     * @param methodName メソッド名 / method name
     * @param args       引数 / arguments
     */
    public void bookAction(int periphId, String methodName, byte[] args) {
        actionRequests.computeIfAbsent(periphId, k -> new ConcurrentHashMap<>())
                .computeIfAbsent(methodName, k -> new ArrayList<>())
                .add(PendingRequest.action(methodName, args));
        LOGGER.debug("Booked action: periphId={}, method={}", periphId, methodName);
    }

    /**
     * クエリリクエストを取得する。
     * Get query requests.
     */
    public Map<Integer, Map<String, PendingRequest>> getQueryRequests() {
        return queryRequests;
    }

    /**
     * アクションリクエストを取得する。
     * Get action requests.
     */
    public Map<Integer, Map<String, List<PendingRequest>>> getActionRequests() {
        return actionRequests;
    }

    /**
     * 特定のペリフェラルのリクエストをクリアする。
     * Clear requests for a specific peripheral.
     *
     * @param periphId ペリフェラルID / peripheral ID
     */
    public void clear(int periphId) {
        queryRequests.remove(periphId);
        actionRequests.remove(periphId);
        LOGGER.debug("Cleared requests for periphId={}", periphId);
    }

    /**
     * 全リクエストをクリアする。
     * Clear all requests.
     */
    public void clearAll() {
        queryRequests.clear();
        actionRequests.clear();
        LOGGER.debug("Cleared all requests");
    }
}
