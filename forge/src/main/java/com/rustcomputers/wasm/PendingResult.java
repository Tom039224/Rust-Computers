package com.rustcomputers.wasm;

import javax.annotation.Nullable;

/**
 * ホスト関数リクエストの保留結果を保持する。
 * Holds a pending result for a host function request.
 *
 * <p>Java がペリフェラル応答を受け取った時点で {@link #complete(byte[])} を呼び、
 * WASM 側 {@code host_poll_result()} で取得する。</p>
 *
 * <p>Call {@link #complete(byte[])} when the peripheral response arrives.
 * WASM polls the result via {@code host_poll_result()}.</p>
 */
public final class PendingResult {

    /** リクエスト種別 / Request type */
    public enum Type {
        /** ペリフェラル情報取得 / Peripheral info request */
        INFO,
        /** ペリフェラルアクション / Peripheral action */
        ACTION,
        /** 標準入力 / Standard input (stdin) */
        STDIN
    }

    private final long requestId;
    private final Type type;

    /** 発行された tick 番号（タイムアウト計算用） / Tick number when issued (for timeout) */
    private final long issuedAtTick;

    /** 完了済みか / Whether this result has been completed */
    private volatile boolean completed;

    /** 完了時のペイロード（null = 未完了） / Completed payload (null = not yet) */
    @Nullable
    private volatile byte[] payload;

    /** エラーコード（0 = 正常） / Error code (0 = no error) */
    private volatile int errorCode;

    /**
     * @param requestId    リクエスト ID / request ID
     * @param type         リクエスト種別 / request type
     * @param currentTick  現在の tick 番号 / current tick number
     */
    public PendingResult(long requestId, Type type, long currentTick) {
        this.requestId = requestId;
        this.type = type;
        this.issuedAtTick = currentTick;
    }

    // ------------------------------------------------------------------
    // 完了操作 / Completion
    // ------------------------------------------------------------------

    /**
     * 正常完了させる。
     * Mark as completed with the given payload.
     *
     * @param data 結果データ（MessagePack バイト列等） / result data (MessagePack bytes, etc.)
     */
    public void complete(byte[] data) {
        this.payload = data;
        this.completed = true;
    }

    /**
     * エラーで完了させる。
     * Mark as completed with an error.
     *
     * @param errorCode エラーコード（{@link ErrorCodes} 参照） / error code (see {@link ErrorCodes})
     */
    public void completeWithError(int errorCode) {
        this.errorCode = errorCode;
        this.payload = new byte[0];
        this.completed = true;
    }

    // ------------------------------------------------------------------
    // アクセサ / Accessors
    // ------------------------------------------------------------------

    public long getRequestId()    { return requestId; }
    public Type getType()         { return type; }
    public long getIssuedAtTick() { return issuedAtTick; }
    public boolean isCompleted()  { return completed; }
    public int  getErrorCode()    { return errorCode; }

    /**
     * 完了済みペイロードを返す（未完了なら null）。
     * Return the completed payload, or null if not yet completed.
     */
    @Nullable
    public byte[] getPayload() {
        return payload;
    }
}
