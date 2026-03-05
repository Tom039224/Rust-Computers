package com.rustcomputers.wasm;

/**
 * ホスト関数が返すエラーコード定数。
 * Error code constants returned by host functions.
 *
 * <p>負の値がエラーを示す。Rust 側の {@code BridgeError} と対応する。</p>
 * <p>Negative values indicate errors. Corresponds to Rust-side {@code BridgeError}.</p>
 */
public final class ErrorCodes {

    /** 無効な request_id / Invalid request ID */
    public static final int ERR_INVALID_REQUEST_ID = -1;

    /** 無効なペリフェラル ID / Invalid peripheral ID */
    public static final int ERR_INVALID_PERIPHERAL = -2;

    /** メソッドが見つからない / Method not found */
    public static final int ERR_METHOD_NOT_FOUND = -3;

    /** Java 側で例外が発生 / Java-side exception occurred */
    public static final int ERR_JAVA_EXCEPTION = -4;

    /** リクエストがタイムアウト / Request timed out */
    public static final int ERR_TIMEOUT = -5;

    /** Fuel 切れ / Fuel exhausted */
    public static final int ERR_FUEL_EXHAUSTED = -6;

    /** result バッファが小さすぎる / Result buffer too small */
    public static final int ERR_RESULT_BUF_TOO_SMALL = -7;

    /** 要求された Mod が利用不可 / Requested mod is not available */
    public static final int ERR_MOD_NOT_AVAILABLE = -8;

    /** 結果が失われた（タイムアウト等で破棄された） / Result lost (discarded due to timeout, etc.) */
    public static final int ERR_RESULT_LOST = -9;

    /**
     * エラーコードを人間が読める文字列に変換する。
     * Convert an error code to a human-readable string.
     *
     * @param code エラーコード / error code
     * @return 説明文字列 / description string
     */
    public static String describe(int code) {
        return switch (code) {
            case ERR_INVALID_REQUEST_ID  -> "INVALID_REQUEST_ID";
            case ERR_INVALID_PERIPHERAL  -> "INVALID_PERIPHERAL";
            case ERR_METHOD_NOT_FOUND    -> "METHOD_NOT_FOUND";
            case ERR_JAVA_EXCEPTION      -> "JAVA_EXCEPTION";
            case ERR_TIMEOUT             -> "TIMEOUT";
            case ERR_FUEL_EXHAUSTED      -> "FUEL_EXHAUSTED";
            case ERR_RESULT_BUF_TOO_SMALL -> "RESULT_BUF_TOO_SMALL";
            case ERR_MOD_NOT_AVAILABLE   -> "MOD_NOT_AVAILABLE";
            case ERR_RESULT_LOST         -> "RESULT_LOST";
            default -> "UNKNOWN_ERROR(" + code + ")";
        };
    }

    private ErrorCodes() { /* ユーティリティクラス / Utility class */ }
}
