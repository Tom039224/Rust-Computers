package com.rustcomputers.peripheral;

import net.minecraft.core.BlockPos;
import net.minecraft.core.Direction;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.level.block.Block;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.annotation.Nullable;
import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;
import java.util.function.Supplier;

/**
 * ペリフェラルのレジストリとスキャナー。
 * Peripheral registry and adjacent-block scanner.
 *
 * <p>各 Block → PeripheralType のマッピングを管理し、
 * コンピュータ周囲6方向のペリフェラルを検出する。</p>
 *
 * <p>Manages Block → PeripheralType mappings and scans the 6 adjacent
 * directions around a computer for peripherals.</p>
 *
 * <h3>方向マッピング / Direction mapping</h3>
 * <pre>
 * periph_id | Direction | 説明
 * 0         | DOWN      | 下
 * 1         | UP        | 上
 * 2         | NORTH     | 北
 * 3         | SOUTH     | 南
 * 4         | WEST      | 西
 * 5         | EAST      | 東
 * </pre>
 */
public final class PeripheralProvider {

    private static final Logger LOGGER = LoggerFactory.getLogger(PeripheralProvider.class);

    /**
     * Block → PeripheralType のレジストリ。
     * Registry mapping Block → PeripheralType factory.
     *
     * <p>スレッドセーフ: Mod 初期化時とサーバーティックの両方からアクセスされるため。</p>
     * <p>Thread-safe: accessed from both mod init and server tick threads.</p>
     */
    private static final Map<Block, Supplier<PeripheralType>> REGISTRY = new ConcurrentHashMap<>();

    // インスタンス化禁止 / No instantiation
    private PeripheralProvider() {}

    // ==================================================================
    // 登録 / Registration
    // ==================================================================

    /**
     * ブロックをペリフェラルとして登録する。
     * Register a block as a peripheral.
     *
     * <p>Mod の共通セットアップ（commonSetup）で呼び出すこと。</p>
     * <p>Should be called during mod common setup.</p>
     *
     * @param block    対象ブロック / target block
     * @param supplier PeripheralType のファクトリ / factory for PeripheralType
     */
    public static void register(Block block, Supplier<PeripheralType> supplier) {
        LOGGER.info("Registered peripheral for block: {}", block);
        REGISTRY.put(block, supplier);
    }

    /**
     * ブロックに対応する PeripheralType を取得する。
     * Get the PeripheralType for a given block.
     *
     * @param block 対象ブロック / target block
     * @return PeripheralType、未登録なら null / PeripheralType or null if not registered
     */
    @Nullable
    public static PeripheralType getForBlock(Block block) {
        var supplier = REGISTRY.get(block);
        return supplier != null ? supplier.get() : null;
    }

    /**
     * 登録済みブロック数を返す（デバッグ用）。
     * Return the number of registered blocks (for debugging).
     */
    public static int registeredCount() {
        return REGISTRY.size();
    }

    // ==================================================================
    // スキャン / Scanning
    // ==================================================================

    /**
     * コンピュータ周囲6方向をスキャンし、検出されたペリフェラルを返す。
     * Scan 6 adjacent directions around a computer and return detected peripherals.
     *
     * @param level      サーバーレベル / server level
     * @param computerPos コンピュータの座標 / computer block position
     * @return periph_id → AttachedPeripheral のマップ（0〜5） / map of periph_id → AttachedPeripheral (0–5)
     */
    public static Map<Integer, AttachedPeripheral> scanAdjacent(ServerLevel level, BlockPos computerPos) {
        Map<Integer, AttachedPeripheral> result = new HashMap<>();

        for (Direction dir : Direction.values()) {
            BlockPos adjacentPos = computerPos.relative(dir);

            // チャンクがロードされていない場合はスキップ
            // Skip if chunk is not loaded
            if (!level.isLoaded(adjacentPos)) {
                continue;
            }

            Block block = level.getBlockState(adjacentPos).getBlock();
            PeripheralType type = getForBlock(block);

            if (type != null) {
                int periphId = dir.ordinal(); // Direction enum の序数 = periph_id
                result.put(periphId, new AttachedPeripheral(type, dir, adjacentPos));
                LOGGER.debug("Found peripheral '{}' at {} (periph_id={})",
                        type.getTypeName(), adjacentPos, periphId);
            }
        }

        return result;
    }
}
