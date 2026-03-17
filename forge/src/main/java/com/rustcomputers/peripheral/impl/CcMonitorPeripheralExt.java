package com.rustcomputers.peripheral.impl;

import com.rustcomputers.peripheral.PeripheralException;
import com.rustcomputers.peripheral.PeripheralType;
import net.minecraft.core.BlockPos;
import net.minecraft.server.level.ServerLevel;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

/**
 * CC:Tweaked Monitor ペリフェラル実装（拡張版）。
 * CC:Tweaked Monitor peripheral implementation (extended).
 *
 * <p>monitor_touch イベント対応を含む。</p>
 * <p>Includes support for monitor_touch events.</p>
 */
public class CcMonitorPeripheralExt implements PeripheralType {
    private static final Logger LOGGER = LoggerFactory.getLogger(CcMonitorPeripheralExt.class);

    private static final String TYPE_NAME = "monitor";
    private static final String[] METHODS = {
            // TermMethods
            "write", "blit", "clear", "clearLine", "scroll",
            "getCursorPos", "setCursorPos", "getCursorBlink", "setCursorBlink",
            "isColor", "isColour",
            "setTextColor", "setTextColour", "getTextColor", "getTextColour",
            "setBackgroundColor", "setBackgroundColour", "getBackgroundColor", "getBackgroundColour",
            "getPaletteColor", "getPaletteColour", "setPaletteColor", "setPaletteColour",
            "getSize",
            // Monitor specific
            "setTextScale", "getTextScale",
            // Event
            "try_pull_monitor_touch"  // イベント受信メソッド
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
        // Monitor は CC:Tweaked の IPeripheral として実装されているため、
        // CcGenericPeripheral に委譲する
        // Monitor is implemented as CC:Tweaked's IPeripheral,
        // so delegate to CcGenericPeripheral
        CcGenericPeripheral delegate = new CcGenericPeripheral(TYPE_NAME, METHODS);
        return delegate.callMethod(methodName, args, level, peripheralPos);
    }

    @Override
    public byte[] callImmediate(String methodName, byte[] args,
                                ServerLevel level, BlockPos peripheralPos)
            throws PeripheralException {
        // 描画系メソッドはアクション系なので immediate 非対応
        // Drawing methods are action methods, not supported for immediate
        if ("write".equals(methodName) || "blit".equals(methodName) ||
            "clear".equals(methodName) || "clearLine".equals(methodName) ||
            "scroll".equals(methodName) || "setCursorPos".equals(methodName) ||
            "setCursorBlink".equals(methodName) || "setTextColor".equals(methodName) ||
            "setTextColour".equals(methodName) || "setBackgroundColor".equals(methodName) ||
            "setBackgroundColour".equals(methodName) || "setPaletteColor".equals(methodName) ||
            "setPaletteColour".equals(methodName) || "setTextScale".equals(methodName)) {
            return null;
        }

        // try_pull_monitor_touch はイベント系なので immediate 非対応
        // try_pull_monitor_touch is an event method, not supported for immediate
        if ("try_pull_monitor_touch".equals(methodName)) {
            return null;
        }

        // その他のメソッド（読み取り系）は immediate 対応
        // Other methods (read-only) support immediate
        CcGenericPeripheral delegate = new CcGenericPeripheral(TYPE_NAME, METHODS);
        return delegate.callImmediate(methodName, args, level, peripheralPos);
    }
}
