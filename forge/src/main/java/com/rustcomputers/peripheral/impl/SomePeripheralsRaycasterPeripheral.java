package com.rustcomputers.peripheral.impl;

import com.rustcomputers.peripheral.MsgPack;
import com.rustcomputers.peripheral.PeripheralException;
import com.rustcomputers.peripheral.PeripheralType;
import net.minecraft.core.BlockPos;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.level.block.state.properties.BlockStateProperties;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.Optional;

/**
 * Some Peripherals の Raycaster ブロック向けペリフェラル。
 * Peripheral for Some Peripherals' Raycaster block (some_peripherals:raycaster).
 *
 * <p>ブロックエンティティ不要。ブロックステートの操作のみを行う。</p>
 * <p>No BlockEntity access required. Only operates on block state properties.</p>
 *
 * <h3>メソッド一覧 / Methods</h3>
 * <pre>
 * getFacingDirection() → str
 *   "north" / "south" / "east" / "west" / "up" / "down"
 *
 * addStickers(state: bool) → nil   [action]
 *   VS スティッカーを有効／無効にする（POWERED ブロックステートを設定）。
 *   Enable/disable VS stickers (sets the POWERED block state).
 * </pre>
 */
public class SomePeripheralsRaycasterPeripheral implements PeripheralType {

    private static final Logger LOGGER =
            LoggerFactory.getLogger(SomePeripheralsRaycasterPeripheral.class);

    private static final String[] METHODS = {"getFacingDirection", "addStickers"};

    // ------------------------------------------------------------------
    // PeripheralType 実装 / PeripheralType implementation
    // ------------------------------------------------------------------

    @Override
    public String getTypeName() {
        return "raycaster";
    }

    @Override
    public String[] getMethodNames() {
        return METHODS.clone();
    }

    @Override
    public byte[] callMethod(String methodName, byte[] args,
                             ServerLevel level, BlockPos peripheralPos)
            throws PeripheralException {

        BlockState state = level.getBlockState(peripheralPos);

        switch (methodName) {
            case "getFacingDirection": {
                // BlockStateProperties.FACING が存在しない場合は "unknown" を返す
                // Return "unknown" if FACING property is absent
                if (!state.hasProperty(BlockStateProperties.FACING)) {
                    return MsgPack.str("unknown");
                }
                return MsgPack.str(state.getValue(BlockStateProperties.FACING).getName());
            }

            case "addStickers": {
                int off0 = MsgPack.argOffset(args, 0);
                if (off0 < 0) {
                    throw new PeripheralException("addStickers requires 1 argument: state (bool)");
                }
                // decodeInt: 0xC2=false, 0xC3=true, also accept fixint
                boolean powered;
                int b = off0 < args.length ? (args[off0] & 0xFF) : 0;
                if (b == 0xC2) {
                    powered = false;
                } else if (b == 0xC3) {
                    powered = true;
                } else {
                    // fall back: non-zero int = true
                    powered = MsgPack.decodeInt(args, off0) != 0;
                }

                if (!state.hasProperty(BlockStateProperties.POWERED)) {
                    throw new PeripheralException(
                            "Raycaster block does not have POWERED property");
                }
                level.setBlockAndUpdate(peripheralPos,
                        state.setValue(BlockStateProperties.POWERED, powered));
                return MsgPack.nil();
            }

            default:
                throw new PeripheralException("Unknown method: " + methodName);
        }
    }
}
