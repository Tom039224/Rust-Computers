package com.rustcomputers.peripheral;

import com.rustcomputers.peripheral.impl.CcRustComputerPeripheralProvider;
import dan200.computercraft.api.peripheral.IPeripheral;
import net.minecraft.core.BlockPos;
import net.minecraft.core.Direction;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.entity.BlockEntity;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.annotation.Nullable;
import java.lang.reflect.Method;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;
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

        // 有線モデム経由のペリフェラルも追加 (CC:Tweaked がインストールされている場合)
        // Also add peripherals connected via wired modem (when CC:Tweaked is installed)
        scanWiredNetwork(level, computerPos, result);

        return result;
    }

    // ==================================================================
    // 有線モデムネットワーク走査 / Wired modem network scanning
    // ==================================================================

    /**
     * 隣接する CC:Tweaked 有線モデムを検出し、ネットワーク上のペリフェラルを追加する。
     * Detect adjacent CC:Tweaked wired modems and add network peripherals.
     *
     * <p>CC:Tweaked が存在しない環境では何もせず安全にスキップする。</p>
     * <p>Silently skips if CC:Tweaked is not installed.</p>
     *
     * @param level       サーバーレベル / server level
     * @param computerPos コンピュータの座標 / computer block position
     * @param result      既存のペリフェラルマップ（直接接続 periph_id 0–5）に追記する
     *                    / existing peripheral map (direct periph_id 0–5) to append to
     */
    private static void scanWiredNetwork(
            ServerLevel level, BlockPos computerPos, Map<Integer, AttachedPeripheral> result) {
        try {
            scanWiredNetworkImpl(level, computerPos, result);
        } catch (LinkageError e) {
            // CC:Tweaked が未インストール — 正常動作
            LOGGER.debug("CC:Tweaked not available, skipping wired modem scan");
        } catch (Throwable e) {
            // リフレクション失敗等
            LOGGER.warn("Wired modem scan failed: {}", e.getMessage());
        }
    }

    /**
     * CC:Tweaked API を使用した有線モデムネットワーク走査の内部実装。
     * Internal implementation of wired modem network scanning using CC:Tweaked API.
     *
     * <p>隣接有線モデムの WiredElement から remote peripheral 一覧を取得し、
     * 各 IPeripheral の getTarget() が指す BlockEntity を RustComputers の
     * PeripheralType レジストリに照合して periph_id 6+ として追加する。</p>
     *
     * <p>Get remote peripherals from adjacent wired modem elements and resolve each
     * peripheral target BlockEntity. Matching blocks in RustComputers registry are
     * added as periph_id 6+.</p>
     */
    private static void scanWiredNetworkImpl(
            ServerLevel level, BlockPos computerPos, Map<Integer, AttachedPeripheral> result) throws Exception {

        // 既にマップに含まれている BlockPos（直接接続）を記録
        Set<BlockPos> knownPositions = new HashSet<>();
        knownPositions.add(computerPos);
        for (AttachedPeripheral ap : result.values()) {
            knownPositions.add(ap.peripheralPos());
        }

        int nextPeriphId = 6;

        // 1) まず CC の attach(IComputerAccess) 経由で取得できる到達可能ペリフェラルを使う。
        //    RustComputers コンピューターが有線モデムに右クリック接続されていれば、
        //    こちらが最も正確。
        Map<String, IPeripheral> attachedVisible = CcRustComputerPeripheralProvider.getAvailablePeripherals(computerPos);
        if (!attachedVisible.isEmpty()) {
            LOGGER.debug("Attach bridge found {} available peripheral(s) for computer at {}",
                attachedVisible.size(), computerPos);
            nextPeriphId = appendRemotePeripherals(level, attachedVisible.values(), knownPositions, result, nextPeriphId);
        }
        // 探索候補: 自ブロック + 隣接6面
        // Probe candidates: this block + 6 adjacent blocks.
        List<BlockPos> probePositions = new ArrayList<>(7);
        probePositions.add(computerPos);
        for (Direction dir : Direction.values()) {
            probePositions.add(computerPos.relative(dir));
        }

        // 重複除去しつつ WiredElement を収集（同一要素を複数面で拾うため）
        // Collect unique wired elements (the same element may appear on multiple faces).
        java.util.Set<Object> wiredElements = java.util.Collections.newSetFromMap(new java.util.IdentityHashMap<>());

        for (BlockPos probePos : probePositions) {
            if (!level.isLoaded(probePos)) continue;

            for (Direction probeSide : Direction.values()) {
                var optElement = dan200.computercraft.api.ForgeComputerCraftAPI
                        .getWiredElementAt(level, probePos, probeSide);
                if (!optElement.isPresent()) continue;

                var element = optElement.orElse(null);
                if (element != null) {
                    wiredElements.add(element);
                }
            }
        }

        if (wiredElements.isEmpty()) {
            LOGGER.debug("No wired element found around computer at {}", computerPos);
            return;
        }

        LOGGER.debug("Found {} wired element(s) around computer at {}", wiredElements.size(), computerPos);

        for (Object element : wiredElements) {
            // WiredElement (interface) には getRemotePeripherals がないため、
            // 実装クラス (WiredModemElement) のメソッドをリフレクションで呼ぶ。
            Method getRemotePeripherals = element.getClass().getMethod("getRemotePeripherals");
            Object remoteObj = getRemotePeripherals.invoke(element);
            if (!(remoteObj instanceof Map<?, ?> remotePeripherals)) {
                continue;
            }

            LOGGER.debug("Wired element {} has {} remote peripheral(s)",
                    element.getClass().getSimpleName(), remotePeripherals.size());

            nextPeriphId = appendRemotePeripherals(level, remotePeripherals.values(), knownPositions, result, nextPeriphId);
        }
    }

    /**
     * remote IPeripheral 群を走査して result に追記する共通処理。
     */
    private static int appendRemotePeripherals(
            ServerLevel level,
            java.util.Collection<?> remotePeripherals,
            Set<BlockPos> knownPositions,
            Map<Integer, AttachedPeripheral> result,
            int nextPeriphId
    ) {
        int id = nextPeriphId;

        for (Object remotePeripheral : remotePeripherals) {
            if (!(remotePeripheral instanceof IPeripheral peripheral)) {
                continue;
            }

            Object target = peripheral.getTarget();
            if (!(target instanceof BlockEntity be)) {
                continue;
            }

            BlockPos bp = be.getBlockPos();
            if (knownPositions.contains(bp)) continue;
            knownPositions.add(bp);

            if (!level.isLoaded(bp)) continue;

            Block block = level.getBlockState(bp).getBlock();
            PeripheralType pt = getForBlock(block);
            if (pt != null) {
                result.put(id, new AttachedPeripheral(pt, null, bp));
                LOGGER.debug("Found wired peripheral '{}' at {} (periph_id={})",
                        pt.getTypeName(), bp, id);
                id++;
            } else {
                LOGGER.debug("Remote peripheral target at {} is not registered in RustComputers registry", bp);
            }
        }

        return id;
    }

    // ==================================================================
    // 検索 / Search
    // ==================================================================

    /**
     * 接続済みペリフェラルの中から型名が一致するものの periph_id リストを返す。
     * Return the list of periph_ids whose type name matches {@code typeName}.
     *
     * <p>{@code find_imm&lt;T&gt;()} の Java 側実装。periph_id 0–5（直接接続）のほか、
     * 将来追加される有線モデム接続（periph_id 6+）も対象となる。</p>
     *
     * <p>Java-side implementation for {@code find_imm<T>()}. Covers periph_ids 0–5
     * (direct connections) as well as future wired-modem connections (periph_id 6+).</p>
     *
     * @param allPeripherals  WasmEngine が保持するペリフェラルマップ / peripheral map held by WasmEngine
     * @param typeName        検索する型名（Rust 側の {@code NAME} 定数と一致する値）
     *                        / type name to search (matches Rust-side {@code NAME} constant)
     * @return マッチした periph_id のリスト（なければ空） / list of matching periph_ids, empty if none
     */
    public static List<Integer> findByTypeName(
            Map<Integer, AttachedPeripheral> allPeripherals, String typeName) {
        List<Integer> result = new ArrayList<>();
        for (Map.Entry<Integer, AttachedPeripheral> entry : allPeripherals.entrySet()) {
            if (typeName.equals(entry.getValue().typeName())) {
                result.add(entry.getKey());
            }
        }
        return result;
    }
}
