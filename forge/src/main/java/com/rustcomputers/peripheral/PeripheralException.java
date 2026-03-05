package com.rustcomputers.peripheral;

/**
 * ペリフェラルメソッド実行時の例外。
 * Exception thrown during peripheral method execution.
 *
 * <p>WASM 側には {@code ErrorCodes.ERR_JAVA_EXCEPTION} として伝搬する。</p>
 * <p>Propagated to the WASM side as {@code ErrorCodes.ERR_JAVA_EXCEPTION}.</p>
 */
public class PeripheralException extends Exception {

    public PeripheralException(String message) {
        super(message);
    }

    public PeripheralException(String message, Throwable cause) {
        super(message, cause);
    }
}
