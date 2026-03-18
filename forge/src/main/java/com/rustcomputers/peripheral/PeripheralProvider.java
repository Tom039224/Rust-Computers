package com.rustcomputers.peripheral;

import com.rustcomputers.peripheral.impl.CcRustComputerPeripheralProvider;
import dan200.computercraft.api.peripheral.IComputerAccess;
import dan200.computercraft.api.peripheral.IPeripheral;
import net.minecraft.core.BlockPos;
import net.minecraft.core.Direction;
import net.minecraft.resources.ResourceLocation;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.entity.BlockEntity;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.annotation.Nullable;
import java.lang.reflect.Field;
import java.lang.reflect.Proxy;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.IdentityHashMap;
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

    private record RemoteCandidate(IPeripheral peripheral, @Nullable BlockPos hintedPos) {}

    @Nullable
    private static Object ccCapabilityPeripheral;
    private static boolean ccCapabilityInitTried = false;

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
            // attach bridge 失敗等
            LOGGER.warn("Wired modem scan failed: {}", e.getMessage());
        }
    }

    /**
     * CC が管理する attach(IComputerAccess) 情報を使って有線ネットワーク上の
     * 到達可能ペリフェラルを追加する。
     *
     * Uses CC-managed attach(IComputerAccess) visibility data to add reachable
     * wired-network peripherals.
     */
    private static void scanWiredNetworkImpl(
            ServerLevel level, BlockPos computerPos, Map<Integer, AttachedPeripheral> result) {

        // 既にマップに含まれている BlockPos（直接接続）を記録
        Set<BlockPos> knownPositions = new HashSet<>();
        knownPositions.add(computerPos);
        for (AttachedPeripheral ap : result.values()) {
            knownPositions.add(ap.peripheralPos());
        }

        int nextPeriphId = 6;

        // CC の公開機能（attach bridge）と、隣接有線モデム capability の
        // 両経路を併用して remote peripherals を取得する。
        // WiredElement 直接操作は行わない。
        Map<String, IPeripheral> attachedVisible = CcRustComputerPeripheralProvider.getAvailablePeripherals(computerPos);
        List<RemoteCandidate> remotes = new ArrayList<>(attachedVisible.size() + 8);
        for (IPeripheral p : attachedVisible.values()) {
            remotes.add(new RemoteCandidate(p, null));
        }

        // attach bridge が部分集合しか返さないケースに備えて、常に capability 経路も併用する
        List<RemoteCandidate> fallbackRemotes = scanRemotesViaAdjacentWiredModems(level, computerPos);
        if (!fallbackRemotes.isEmpty()) {
            remotes.addAll(fallbackRemotes);
        }

        if (remotes.isEmpty()) return;

        appendRemotePeripherals(level, remotes, knownPositions, result, nextPeriphId);
    }

    private static List<RemoteCandidate> scanRemotesViaAdjacentWiredModems(ServerLevel level, BlockPos computerPos) {
        List<RemoteCandidate> result = new ArrayList<>();

        for (Direction dir : Direction.values()) {
            BlockPos pos = computerPos.relative(dir);
            if (!level.isLoaded(pos)) continue;

            Block block = level.getBlockState(pos).getBlock();
            ResourceLocation key = net.minecraftforge.registries.ForgeRegistries.BLOCKS.getKey(block);
            if (key == null) continue;

                // 有線モデム系ブロックのみ対象
                // CC:Tweaked では "cable" ブロックにも side modem を装着でき、
                // peripheral capability が有効になるため対象に含める。
            if (!"computercraft".equals(key.getNamespace())) {
                continue;
            }
            String path = key.getPath();
                if (!"wired_modem".equals(path)
                    && !"wired_modem_full".equals(path)
                    && !"cable".equals(path)) {
                continue;
            }

            BlockEntity be = level.getBlockEntity(pos);
            if (be == null) continue;

            Object modemPeripheral = getCcPeripheralFromBlockEntity(be);
            if (!(modemPeripheral instanceof IPeripheral)) continue;
            extractRemotesFromModemPeripheral(modemPeripheral, result);
        }

        return result;
    }

    @Nullable
    private static Object getCcPeripheralFromBlockEntity(BlockEntity be) {
        try {
            if (!ccCapabilityInitTried) {
                ccCapabilityInitTried = true;
                Class<?> capClass = Class.forName("dan200.computercraft.shared.Capabilities");
                Field f = capClass.getDeclaredField("CAPABILITY_PERIPHERAL");
                f.setAccessible(true);
                ccCapabilityPeripheral = f.get(null);
            }
            if (ccCapabilityPeripheral == null) return null;

            var getCap = be.getClass().getMethod(
                    "getCapability",
                    Class.forName("net.minecraftforge.common.capabilities.Capability"),
                    net.minecraft.core.Direction.class
            );

            // 全方向を試す
            for (Direction dir : Direction.values()) {
                try {
                    Object p = unwrapLazyOptional(getCap.invoke(be, ccCapabilityPeripheral, dir));
                    if (p != null) return p;
                } catch (Throwable ignored) {
                }
            }

            // null side
            try {
                Object p = unwrapLazyOptional(getCap.invoke(be, ccCapabilityPeripheral, (Object) null));
                if (p != null) return p;
            } catch (Throwable ignored) {
            }

            LOGGER.debug("getCcPeripheralFromBlockEntity: no capability found on BE {}", be);
            return null;
        } catch (Throwable e) {
            LOGGER.error("getCcPeripheralFromBlockEntity failed on {}, {}", be, e);
            return null;
        }
    }

    @Nullable
    private static Object unwrapLazyOptional(@Nullable Object lazyOptional) {
        if (lazyOptional == null) return null;
        try {
            var orElse = lazyOptional.getClass().getMethod("orElse", Object.class);
            return orElse.invoke(lazyOptional, (Object) null);
        } catch (Throwable ignored) {
            return null;
        }
    }

    private static void extractRemotesFromModemPeripheral(Object modemPeripheral, List<RemoteCandidate> out) {
        IComputerAccess fake = (IComputerAccess) Proxy.newProxyInstance(
                PeripheralProvider.class.getClassLoader(),
                new Class<?>[]{ IComputerAccess.class },
                (proxy, method, args) -> {
                    String name = method.getName();
                    if ("equals".equals(name)) {
                        return proxy == args[0];
                    }
                    if ("hashCode".equals(name)) {
                        return System.identityHashCode(proxy);
                    }
                    if ("toString".equals(name)) {
                        return "RustComputersFakeComputerAccess";
                    }
                    switch (name) {
                        case "getID": return 0;
                        case "getAttachmentName": return "rust_scan";
                        case "getAvailablePeripherals": return Map.of();
                        case "getAvailablePeripheral": return null;
                        case "mount": return null;
                        case "mountWritable": return null;
                        default: return null;
                    }
                }
        );

        try {
            var attach = modemPeripheral.getClass().getMethod("attach", IComputerAccess.class);
            var detach = modemPeripheral.getClass().getMethod("detach", IComputerAccess.class);

            attach.invoke(modemPeripheral, fake);
            try {
                Field wrappersField = findField(modemPeripheral.getClass(), "peripheralWrappers");
                if (wrappersField == null) {
                    LOGGER.warn("extractRemotesFromModemPeripheral: no peripheralWrappers field found on {}", modemPeripheral.getClass());
                    return;
                }
                wrappersField.setAccessible(true);
                Object wrappersMapObj = wrappersField.get(modemPeripheral);
                if (!(wrappersMapObj instanceof Map<?, ?> wrappersMap)) {
                    LOGGER.warn("extractRemotesFromModemPeripheral: peripheralWrappers is not a Map: {}", wrappersMapObj);
                    return;
                }

                Object wrappersObj = wrappersMap.get(fake);
                if (!(wrappersObj instanceof Map<?, ?> wrappers)) {
                    LOGGER.warn("extractRemotesFromModemPeripheral: wrapper map for fake is not a Map: {}", wrappersObj);
                    return;
                }

                for (Object wrapper : wrappers.values()) {
                    if (wrapper == null) continue;
                    Field peripheralField = findField(wrapper.getClass(), "peripheral");
                    if (peripheralField == null) continue;
                    peripheralField.setAccessible(true);
                    Object p = peripheralField.get(wrapper);
                    if (p instanceof IPeripheral ip) {
                        BlockPos hintedPos = extractBlockPos(wrapper);
                        if (hintedPos == null) {
                            Field posField = findField(wrapper.getClass(), "position");
                            if (posField != null) {
                                posField.setAccessible(true);
                                Object pv = posField.get(wrapper);
                                if (pv instanceof BlockPos bp) hintedPos = bp;
                            }
                        }
                        // element フィールド (WiredModemElement サブクラス) から BlockPos を取得
                        if (hintedPos == null) {
                            Field elementField = findField(wrapper.getClass(), "element");
                            if (elementField != null) {
                                elementField.setAccessible(true);
                                Object elem = elementField.get(wrapper);
                                if (elem != null) {
                                    hintedPos = extractBlockPosDeep(elem, 3, new IdentityHashMap<>());
                                }
                            }
                        }
                        // IPeripheral.getTarget() から BlockPos を取得
                        if (hintedPos == null) {
                            try {
                                Object target = ip.getTarget();
                                if (target instanceof BlockEntity be) {
                                    hintedPos = be.getBlockPos();
                                } else if (target != null) {
                                    hintedPos = extractBlockPosDeep(target, 3, new IdentityHashMap<>());
                                }
                            } catch (Throwable ignored) {}
                        }
                        // Tom's Peripherals: CCPeripheral$PeripheralWrapper.p -> ITMPeripheral -> this$0 (BlockEntity)
                        if (hintedPos == null) {
                            hintedPos = extractPosFromTmPeripheralWrapper(ip);
                        }
                        out.add(new RemoteCandidate(ip, hintedPos));
                    }
                }
            } finally {
                detach.invoke(modemPeripheral, fake);
            }
        } catch (Throwable e) {
            LOGGER.error("extractRemotesFromModemPeripheral failed", e);
        }
    }

    @Nullable
    private static Field findField(Class<?> cls, String name) {
        Class<?> c = cls;
        while (c != null && c != Object.class) {
            try {
                return c.getDeclaredField(name);
            } catch (NoSuchFieldException ignored) {
                c = c.getSuperclass();
            }
        }
        return null;
    }

    /**
     * Tom's Peripherals の CCPeripheral$PeripheralWrapper から BlockPos を取得する。
     * 経路: PeripheralWrapper.p (ITMPeripheral) -> GPUBlockEntity$GPUPeripheral.this$0 (BlockEntity)
     */
    @Nullable
    private static BlockPos extractPosFromTmPeripheralWrapper(IPeripheral peripheral) {
        try {
            // PeripheralWrapper.p フィールドを取得
            Field pField = findField(peripheral.getClass(), "p");
            if (pField == null) return null;
            pField.setAccessible(true);
            Object tmPeripheral = pField.get(peripheral);
            if (tmPeripheral == null) return null;

            // ITMPeripheral の実装クラス (例: GPUBlockEntity$GPUPeripheral) から this$0 を取得
            Field outerField = findField(tmPeripheral.getClass(), "this$0");
            if (outerField == null) return null;
            outerField.setAccessible(true);
            Object outer = outerField.get(tmPeripheral);
            if (outer instanceof BlockEntity be) {
                return be.getBlockPos();
            }
        } catch (Throwable e) {
            LOGGER.debug("extractPosFromTmPeripheralWrapper failed: {}", e.toString());
        }
        return null;
    }

    /**
     * remote IPeripheral 群を走査して result に追記する共通処理。
     */
    private static int appendRemotePeripherals(
            ServerLevel level,
            java.util.Collection<RemoteCandidate> remotePeripherals,
            Set<BlockPos> knownPositions,
            Map<Integer, AttachedPeripheral> result,
            int nextPeriphId
    ) {
        int id = nextPeriphId;

        for (RemoteCandidate candidate : remotePeripherals) {
            IPeripheral peripheral = candidate.peripheral();

            Object target = peripheral.getTarget();
            BlockPos bp = candidate.hintedPos() != null
                    ? candidate.hintedPos()
                    : resolvePeripheralPos(peripheral, target);
            if (bp == null) {
                LOGGER.debug("Remote peripheral skipped (no position): type='{}', class='{}', target='{}'",
                    safePeripheralType(peripheral), peripheral.getClass().getName(),
                    peripheral.getTarget() != null ? peripheral.getTarget().getClass().getName() : "null");
                continue;
            }

            if (knownPositions.contains(bp)) continue;
            knownPositions.add(bp);

            if (!level.isLoaded(bp)) continue;

            Block block = level.getBlockState(bp).getBlock();
            ResourceLocation blockKey = net.minecraftforge.registries.ForgeRegistries.BLOCKS.getKey(block);
            PeripheralType pt = getForBlock(block);
            if (pt != null) {
                result.put(id, new AttachedPeripheral(pt, null, bp));
                LOGGER.debug("Found wired peripheral '{}' at {} (periph_id={})",
                        pt.getTypeName(), bp, id);
                id++;
            } else {
                LOGGER.debug("Remote peripheral skipped (unregistered block): type='{}', class='{}', pos={}, block={}",
                        safePeripheralType(peripheral), peripheral.getClass().getName(), bp,
                        blockKey != null ? blockKey : block.toString());
            }
        }

        return id;
    }

    /**
     * IPeripheral から対象 BlockPos を推定する。
     *
     * <p>優先順:
     * 1) getTarget() が BlockEntity ならその座標
     * 2) target/peripheral の getBlockPos()/getPos() メソッド
     * 3) target/peripheral の pos/blockPos フィールド
     * </p>
     */
    @Nullable
    private static BlockPos resolvePeripheralPos(IPeripheral peripheral, @Nullable Object target) {
        if (target instanceof BlockEntity be) {
            return be.getBlockPos();
        }

        BlockPos posFromTarget = extractBlockPosDeep(target, 4, new IdentityHashMap<>());
        if (posFromTarget != null) return posFromTarget;

        return extractBlockPosDeep(peripheral, 5, new IdentityHashMap<>());
    }

    @Nullable
    private static BlockPos extractBlockPos(@Nullable Object obj) {
        if (obj == null) return null;

        // 1) public/protected methods
        try {
            var m = obj.getClass().getMethod("getBlockPos");
            Object v = m.invoke(obj);
            if (v instanceof BlockPos bp) return bp;
        } catch (Throwable ignored) {
        }

        try {
            var m = obj.getClass().getMethod("getPos");
            Object v = m.invoke(obj);
            if (v instanceof BlockPos bp) return bp;
        } catch (Throwable ignored) {
        }

        // 2) fields (including private)
        Field[] fields = obj.getClass().getDeclaredFields();
        for (Field f : fields) {
            String n = f.getName();
            if (!"pos".equals(n) && !"blockPos".equals(n)) continue;
            try {
                f.setAccessible(true);
                Object v = f.get(obj);
                if (v instanceof BlockPos bp) return bp;
            } catch (Throwable ignored) {
            }
        }

        return null;
    }

    @Nullable
    private static BlockPos extractBlockPosDeep(
            @Nullable Object obj,
            int depth,
            IdentityHashMap<Object, Boolean> visited
    ) {
        if (obj == null || depth < 0) return null;
        if (visited.put(obj, Boolean.TRUE) != null) return null;

        BlockPos direct = extractBlockPos(obj);
        if (direct != null) return direct;

        Field[] fields = obj.getClass().getDeclaredFields();
        for (Field f : fields) {
            try {
                f.setAccessible(true);
                Object v = f.get(obj);
                if (v == null) continue;

                if (v instanceof BlockPos bp) {
                    return bp;
                }

                boolean likelyPeripheralWrapper =
                        v instanceof IPeripheral
                                || v instanceof net.minecraft.world.level.block.entity.BlockEntity
                                || f.getName().equals("peripheral")
                                || f.getName().equals("delegate")
                                || f.getName().equals("wrapped")
                                || f.getName().equals("inner")
                                || f.getName().startsWith("this$")
                                || v.getClass().getName().toLowerCase().contains("peripheral");

                if (likelyPeripheralWrapper) {
                    LOGGER.debug("extractBlockPosDeep: depth={} obj={} field={} -> recurse into {}",
                            depth, obj.getClass().getSimpleName(), f.getName(), v.getClass().getSimpleName());
                    BlockPos nested = extractBlockPosDeep(v, depth - 1, visited);
                    if (nested != null) return nested;
                }
            } catch (Throwable e) {
                LOGGER.debug("extractBlockPosDeep: field access failed on {}.{}: {}", obj.getClass().getSimpleName(), f.getName(), e.toString());
            }
        }

        return null;
    }

    private static String safePeripheralType(IPeripheral peripheral) {
        try {
            return peripheral.getType();
        } catch (Throwable ignored) {
            return "<error>";
        }
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
