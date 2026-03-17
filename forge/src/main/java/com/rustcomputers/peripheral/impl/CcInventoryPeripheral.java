package com.rustcomputers.peripheral.impl;

import com.rustcomputers.peripheral.PeripheralException;
import com.rustcomputers.peripheral.PeripheralType;
import net.minecraft.core.BlockPos;
import net.minecraft.server.level.ServerLevel;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

/**
 * CC:Tweaked Inventory ペリフェラル実装。
 * CC:Tweaked Inventory peripheral implementation.
 *
 * <p>pushItems/pullItems の複数結果対応を含む。</p>
 * <p>Includes support for multiple results from pushItems/pullItems.</p>
 */
public class CcInventoryPeripheral implements PeripheralType {
    private static final Logger LOGGER = LoggerFactory.getLogger(CcInventoryPeripheral.class);

    private static final String TYPE_NAME = "inventory";
    private static final String[] METHODS = {
            "size", "list", "getItemDetail", "getItemLimit",
            "pushItems", "pullItems"
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
        // Inventory は CC:Tweaked の GenericPeripheral として実装されているため、
        // CcGenericPeripheral に委譲する
        // Inventory is implemented as CC:Tweaked's GenericPeripheral,
        // so delegate to CcGenericPeripheral
        CcGenericPeripheral delegate = new CcGenericPeripheral(TYPE_NAME, METHODS);
        return delegate.callMethod(methodName, args, level, peripheralPos);
    }

    @Override
    public byte[] callImmediate(String methodName, byte[] args,
                                ServerLevel level, BlockPos peripheralPos)
            throws PeripheralException {
        // pushItems/pullItems はアクション系なので immediate 非対応
        // pushItems/pullItems are action methods, not supported for immediate
        if ("pushItems".equals(methodName) || "pullItems".equals(methodName)) {
            return null;
        }

        // その他のメソッドは immediate 対応
        // Other methods support immediate
        CcGenericPeripheral delegate = new CcGenericPeripheral(TYPE_NAME, METHODS);
        return delegate.callImmediate(methodName, args, level, peripheralPos);
    }
}
