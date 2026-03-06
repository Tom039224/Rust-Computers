package com.rustcomputers.peripheral.impl;

import com.rustcomputers.peripheral.MsgPack;
import com.rustcomputers.peripheral.PeripheralException;
import com.rustcomputers.peripheral.PeripheralType;
import net.minecraft.core.BlockPos;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.level.block.Block;
import net.minecraftforge.registries.ForgeRegistries;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

/**
 * Some Peripherals の WorldScanner ブロック向けペリフェラル。
 * Peripheral for Some Peripherals' WorldScanner block (some_peripherals:world_scanner).
 *
 * <p>ブロックエンティティなし。Level から直接ブロックを取得して返す。</p>
 * <p>No BlockEntity required. Reads block state directly from the Level.</p>
 *
 * <h3>メソッド一覧 / Methods</h3>
 * <pre>
 * getBlockAt(x, y, z, in_shipyard) → map
 *   返値キー: block_type (str), ship_id (f64, オプション)
 *   Return keys: block_type (str), ship_id (f64, optional)
 *   ※ in_shipyard は VS なしの場合は無視 / in_shipyard ignored when VS is not installed
 * </pre>
 */
public class SomePeripheralsWorldScannerPeripheral implements PeripheralType {

    private static final Logger LOGGER = LoggerFactory.getLogger(SomePeripheralsWorldScannerPeripheral.class);

    private static final String[] METHODS = {"getBlockAt"};

    // ------------------------------------------------------------------
    // PeripheralType 実装 / PeripheralType implementation
    // ------------------------------------------------------------------

    @Override
    public String getTypeName() {
        return "world_scanner";
    }

    @Override
    public String[] getMethodNames() {
        return METHODS.clone();
    }

    @Override
    public byte[] callMethod(String methodName, byte[] args,
                             ServerLevel level, BlockPos peripheralPos)
            throws PeripheralException {

        switch (methodName) {
            case "getBlockAt": {
                int off0 = MsgPack.argOffset(args, 0);
                int off1 = MsgPack.argOffset(args, 1);
                int off2 = MsgPack.argOffset(args, 2);
                if (off0 < 0 || off1 < 0 || off2 < 0) {
                    throw new PeripheralException("getBlockAt requires 3 arguments: x, y, z");
                }
                double x = MsgPack.decodeF64(args, off0);
                double y = MsgPack.decodeF64(args, off1);
                double z = MsgPack.decodeF64(args, off2);
                // in_shipyard (arg 3) は VS 統合なしでは無視する
                // in_shipyard (arg 3) is ignored without VS integration

                BlockPos targetPos = new BlockPos(
                        (int) Math.floor(x + peripheralPos.getX()),
                        (int) Math.floor(y + peripheralPos.getY()),
                        (int) Math.floor(z + peripheralPos.getZ())
                );

                if (!level.isLoaded(targetPos)) {
                    return MsgPack.map("error", MsgPack.str("Chunk is not loaded"));
                }

                Block block = level.getBlockState(targetPos).getBlock();
                // SP と同じく getDescriptionId() (翻訳キー形式) で返す
                // Use getDescriptionId() (translation-key format) to match SP's output
                String blockType = block.getDescriptionId();

                return MsgPack.map("block_type", MsgPack.str(blockType));
            }

            default:
                throw new PeripheralException("Unknown method: " + methodName);
        }
    }
}
