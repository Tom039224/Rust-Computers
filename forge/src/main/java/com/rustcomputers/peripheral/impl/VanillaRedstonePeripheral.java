package com.rustcomputers.peripheral.impl;

import com.rustcomputers.peripheral.MsgPack;
import com.rustcomputers.peripheral.PeripheralException;
import com.rustcomputers.peripheral.PeripheralType;
import net.minecraft.core.BlockPos;
import net.minecraft.core.Direction;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.level.block.LeverBlock;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.level.block.state.properties.BlockStateProperties;

/**
 * バニラ Minecraft の Redstone 系ブロック向けペリフェラル。
 * Peripheral for vanilla Minecraft Redstone blocks.
 *
 * <h3>サポートするブロック / Supported blocks</h3>
 * <ul>
 *   <li>Redstone Block — 常に power=15</li>
 *   <li>Lever — 状態読み取り・切り替え</li>
 *   <li>Redstone Torch — 通常は active</li>
 *   <li>Redstone Wire — wire の power レベルを読み取る</li>
 *   <li>Redstone Lamp — powered 状態を読み取る</li>
 * </ul>
 *
 * <h3>メソッド一覧 / Methods</h3>
 * <pre>
 * getPower()          → int   6方向から受け取る最大レッドストーン信号強度 (0–15)
 * getDirectPower()    → int   直接信号の最大強度 (0–15)
 * isActive()          → bool  信号が 1 以上かどうか
 * toggle()            → bool  Lever の場合、状態を反転して新しい状態を返す
 *
 * getPower()          → int   max redstone signal from 6 directions (0–15)
 * getDirectPower()    → int   max direct signal (0–15)
 * isActive()          → bool  whether signal >= 1
 * toggle()            → bool  for Lever: flip state and return new state
 * </pre>
 */
public class VanillaRedstonePeripheral implements PeripheralType {

    private static final String[] METHODS = {
            "getPower", "getDirectPower", "isActive", "toggle",
    };

    @Override
    public String getTypeName() {
        return "redstone";
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
            case "getPower": {
                int power = 0;
                for (Direction dir : Direction.values()) {
                    power = Math.max(power, level.getSignal(peripheralPos, dir));
                }
                return MsgPack.int32(power);
            }

            case "getDirectPower": {
                int power = 0;
                for (Direction dir : Direction.values()) {
                    power = Math.max(power, level.getDirectSignal(peripheralPos, dir));
                }
                return MsgPack.int32(power);
            }

            case "isActive": {
                for (Direction dir : Direction.values()) {
                    if (level.getSignal(peripheralPos, dir) > 0) {
                        return MsgPack.bool(true);
                    }
                }
                return MsgPack.bool(false);
            }

            case "toggle": {
                if (state.getBlock() instanceof LeverBlock) {
                    if (!state.hasProperty(BlockStateProperties.POWERED)) {
                        return MsgPack.bool(false);
                    }
                    boolean current = state.getValue(BlockStateProperties.POWERED);
                    boolean next = !current;
                    level.setBlock(peripheralPos,
                            state.setValue(BlockStateProperties.POWERED, next), 3);
                    return MsgPack.bool(next);
                }
                throw new PeripheralException("toggle() is only supported on Lever blocks");
            }

            default:
                throw new PeripheralException("Unknown method: " + methodName);
        }
    }

    // getPower / getDirectPower / isActive はスレッドセーフな読み取り専用操作なので
    // callImmediate (host_request_info_imm) でも利用可能。
    // getPower / getDirectPower / isActive are read-only, safe for immediate calls.
    @Override
    public byte[] callImmediate(String methodName, byte[] args,
                                ServerLevel level, BlockPos peripheralPos)
            throws PeripheralException {

        if (methodName.equals("toggle")) {
            // 書き込み操作 — immediate では実行不可
            return null;
        }
        return callMethod(methodName, args, level, peripheralPos);
    }
}
