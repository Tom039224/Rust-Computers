package com.rustcomputers.peripheral.impl;

import com.rustcomputers.peripheral.PeripheralException;
import com.rustcomputers.peripheral.PeripheralType;
import net.minecraft.core.BlockPos;
import net.minecraft.server.level.ServerLevel;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

/**
 * CC:Tweaked Modem ペリフェラル実装。
 * CC:Tweaked Modem peripheral implementation.
 *
 * <p>modem_message イベント対応を含む。</p>
 * <p>Includes support for modem_message events.</p>
 */
public class CcModemPeripheral implements PeripheralType {
    private static final Logger LOGGER = LoggerFactory.getLogger(CcModemPeripheral.class);

    private static final String TYPE_NAME = "modem";
    private static final String[] METHODS = {
            "open", "isOpen", "close", "closeAll", "transmit", "isWireless",
            "try_pull_modem_message"  // イベント受信メソッド
    };

    @Override
    public String getTypeName() {
        return TYPE_NAME;
    }

    @Override
    public String[] getMethodNames() {
        return METHODS.clone();
    }

    @Override
    public byte[] callMethod(String methodName, byte[] args,
                             ServerLevel level, BlockPos peripheralPos)
            throws PeripheralException {
        // Modem は CC:Tweaked の IPeripheral として実装されているため、
        // CcGenericPeripheral に委譲する
        // Modem is implemented as CC:Tweaked's IPeripheral,
        // so delegate to CcGenericPeripheral
        CcGenericPeripheral delegate = new CcGenericPeripheral(TYPE_NAME, METHODS);
        return delegate.callMethod(methodName, args, level, peripheralPos);
    }

    @Override
    public byte[] callImmediate(String methodName, byte[] args,
                                ServerLevel level, BlockPos peripheralPos)
            throws PeripheralException {
        // transmit はアクション系なので immediate 非対応
        // transmit is an action method, not supported for immediate
        if ("transmit".equals(methodName)) {
            return null;
        }

        // try_pull_modem_message はイベント系なので immediate 非対応
        // try_pull_modem_message is an event method, not supported for immediate
        if ("try_pull_modem_message".equals(methodName)) {
            return null;
        }

        // その他のメソッドは immediate 対応
        // Other methods support immediate
        CcGenericPeripheral delegate = new CcGenericPeripheral(TYPE_NAME, METHODS);
        return delegate.callImmediate(methodName, args, level, peripheralPos);
    }
}
