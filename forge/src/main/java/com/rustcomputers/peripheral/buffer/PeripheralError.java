package com.rustcomputers.peripheral.buffer;

/**
 * ペリフェラルエラー。
 * Peripheral error record.
 *
 * @param type    エラータイプ / error type
 * @param message エラーメッセージ / error message
 */
public record PeripheralError(ErrorType type, String message) {

    /**
     * エラータイプ。
     * Error type enum.
     */
    public enum ErrorType {
        NOT_FOUND,              // ペリフェラルが見つからない
        METHOD_NOT_FOUND,       // メソッドが見つからない
        INVALID_ARGUMENT,       // 無効な引数
        EXECUTION_FAILED,       // 実行失敗
        DISCONNECTED,           // 接続切断
        TIMEOUT,                // タイムアウト
        SERIALIZATION_ERROR     // シリアライゼーションエラー
    }

    /**
     * エラーコードを取得する（FFI通信用）。
     * Get error code (for FFI communication).
     *
     * <p>負の値を返す: -1 = NOT_FOUND, -2 = METHOD_NOT_FOUND, ...</p>
     * <p>Returns negative value: -1 = NOT_FOUND, -2 = METHOD_NOT_FOUND, ...</p>
     */
    public int getErrorCode() {
        return -(type.ordinal() + 1);
    }

    /**
     * エラーコードからErrorTypeを取得する。
     * Get ErrorType from error code.
     *
     * @param code エラーコード（負の値） / error code (negative value)
     * @return ErrorType、無効なコードの場合はEXECUTION_FAILED / ErrorType or EXECUTION_FAILED for invalid code
     */
    public static ErrorType fromErrorCode(int code) {
        if (code >= 0) {
            return ErrorType.EXECUTION_FAILED;
        }
        int index = -(code + 1);
        ErrorType[] types = ErrorType.values();
        if (index >= 0 && index < types.length) {
            return types[index];
        }
        return ErrorType.EXECUTION_FAILED;
    }

    /**
     * NOT_FOUNDエラーを作成する。
     * Create a NOT_FOUND error.
     */
    public static PeripheralError notFound(String message) {
        return new PeripheralError(ErrorType.NOT_FOUND, message);
    }

    /**
     * METHOD_NOT_FOUNDエラーを作成する。
     * Create a METHOD_NOT_FOUND error.
     */
    public static PeripheralError methodNotFound(String methodName) {
        return new PeripheralError(ErrorType.METHOD_NOT_FOUND, "Method not found: " + methodName);
    }

    /**
     * INVALID_ARGUMENTエラーを作成する。
     * Create an INVALID_ARGUMENT error.
     */
    public static PeripheralError invalidArgument(String message) {
        return new PeripheralError(ErrorType.INVALID_ARGUMENT, message);
    }

    /**
     * EXECUTION_FAILEDエラーを作成する。
     * Create an EXECUTION_FAILED error.
     */
    public static PeripheralError executionFailed(String message) {
        return new PeripheralError(ErrorType.EXECUTION_FAILED, message);
    }

    /**
     * DISCONNECTEDエラーを作成する。
     * Create a DISCONNECTED error.
     */
    public static PeripheralError disconnected(String message) {
        return new PeripheralError(ErrorType.DISCONNECTED, message);
    }

    /**
     * TIMEOUTエラーを作成する。
     * Create a TIMEOUT error.
     */
    public static PeripheralError timeout(String message) {
        return new PeripheralError(ErrorType.TIMEOUT, message);
    }

    /**
     * SERIALIZATION_ERRORエラーを作成する。
     * Create a SERIALIZATION_ERROR error.
     */
    public static PeripheralError serializationError(String message) {
        return new PeripheralError(ErrorType.SERIALIZATION_ERROR, message);
    }
}
