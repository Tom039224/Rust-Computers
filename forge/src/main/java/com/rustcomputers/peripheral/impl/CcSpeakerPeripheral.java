package com.rustcomputers.peripheral.impl;

import com.rustcomputers.peripheral.PeripheralException;
import com.rustcomputers.peripheral.PeripheralType;
import net.minecraft.core.BlockPos;
import net.minecraft.server.level.ServerLevel;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

/**
 * CC:Tweaked Speaker ペリフェラル実装。
 * CC:Tweaked Speaker peripheral implementation.
 *
 * <p>playNote, playSound, playAudio, stop メソッドをサポート。</p>
 * <p>Supports playNote, playSound, playAudio, and stop methods.</p>
 */
public class CcSpeakerPeripheral implements PeripheralType {
    private static final Logger LOGGER = LoggerFactory.getLogger(CcSpeakerPeripheral.class);

    private static final String TYPE_NAME = "speaker";
    private static final String[] METHODS = {
            "playNote", "playSound", "playAudio", "stop"
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
        // Speaker は CC:Tweaked の IPeripheral として実装されているため、
        // CcGenericPeripheral に委譲する
        // Speaker is implemented as CC:Tweaked's IPeripheral,
        // so delegate to CcGenericPeripheral
        CcGenericPeripheral delegate = new CcGenericPeripheral(TYPE_NAME, METHODS);
        return delegate.callMethod(methodName, args, level, peripheralPos);
    }

    @Override
    public byte[] callImmediate(String methodName, byte[] args,
                                ServerLevel level, BlockPos peripheralPos)
            throws PeripheralException {
        // 全メソッドがアクション系なので immediate 非対応
        // All methods are action methods, not supported for immediate
        return null;
    }
}
