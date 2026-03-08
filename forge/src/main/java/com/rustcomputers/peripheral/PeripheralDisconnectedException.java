package com.rustcomputers.peripheral;

/**
 * ペリフェラルが切断または破壊された場合にスローされる例外。
 * Exception thrown when a peripheral has been disconnected or its block entity is gone.
 *
 * <p>WASM 側には {@code ErrorCodes.ERR_PERIPHERAL_DISCONNECTED} として伝搬する。</p>
 * <p>Propagated to the WASM side as {@code ErrorCodes.ERR_PERIPHERAL_DISCONNECTED}.</p>
 */
public class PeripheralDisconnectedException extends PeripheralException {

    public PeripheralDisconnectedException(String message) {
        super(message);
    }
}
