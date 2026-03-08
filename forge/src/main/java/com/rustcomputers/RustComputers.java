package com.rustcomputers;

import com.rustcomputers.command.RustComputersCommand;
import com.rustcomputers.computer.ComputerManager;
import com.rustcomputers.network.NetworkHandler;
import com.rustcomputers.peripheral.PeripheralProvider;
import com.rustcomputers.peripheral.impl.CcMonitorPeripheral;
import com.rustcomputers.peripheral.impl.PeripheralRegistrations;
import com.rustcomputers.peripheral.impl.SomePeripheralsBallisticAcceleratorPeripheral;
import com.rustcomputers.peripheral.impl.SomePeripheralsDigitizerPeripheral;
import com.rustcomputers.peripheral.impl.SomePeripheralsRadarPeripheral;
import com.rustcomputers.peripheral.impl.SomePeripheralsRaycasterPeripheral;
import com.rustcomputers.peripheral.impl.SomePeripheralsWorldScannerPeripheral;
import com.rustcomputers.peripheral.impl.VanillaInventoryPeripheral;
import com.rustcomputers.peripheral.impl.VanillaRedstonePeripheral;
import net.minecraft.core.Direction;
import net.minecraft.resources.ResourceLocation;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.level.block.entity.BlockEntity;
import net.minecraft.world.phys.BlockHitResult;
import net.minecraft.world.phys.Vec3;
import net.minecraftforge.fml.ModList;
import net.minecraftforge.registries.ForgeRegistries;
import net.minecraftforge.api.distmarker.Dist;
import net.minecraftforge.common.MinecraftForge;
import net.minecraftforge.event.RegisterCommandsEvent;
import net.minecraftforge.event.entity.player.PlayerEvent;
import net.minecraftforge.event.entity.player.PlayerInteractEvent;
import net.minecraftforge.fml.LogicalSide;
import net.minecraftforge.event.server.ServerStartingEvent;
import net.minecraftforge.event.server.ServerStoppingEvent;
import net.minecraftforge.eventbus.api.IEventBus;
import net.minecraftforge.eventbus.api.SubscribeEvent;
import net.minecraftforge.fml.common.Mod;
import net.minecraftforge.fml.event.lifecycle.FMLClientSetupEvent;
import net.minecraftforge.fml.event.lifecycle.FMLCommonSetupEvent;
import net.minecraftforge.fml.javafmlmod.FMLJavaModLoadingContext;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

/**
 * RustComputers — Forge 1.20.1 Mod メインクラス。
 * Main mod entry point for RustComputers.
 *
 * <p>Rust (WASM) プログラムを Minecraft 内のコンピューターブロックで実行する。
 * Chicory (pure-Java WASM runtime) を使用し、JNI やネイティブバイナリは不要。</p>
 *
 * <p>Runs Rust (WASM) programs on in-game computer blocks.
 * Uses Chicory (pure-Java WASM runtime) — no JNI or native binaries required.</p>
 */
@Mod(RustComputers.MOD_ID)
public class RustComputers {

    /** Mod ID — リソースパス・レジストリ名に使用 / Used in resource paths and registry names */
    public static final String MOD_ID = "rustcomputers";

    private static final Logger LOGGER = LoggerFactory.getLogger(RustComputers.class);

    public RustComputers() {
        LOGGER.info("RustComputers initializing...");

        IEventBus modBus = FMLJavaModLoadingContext.get().getModEventBus();

        // レジストリ登録 / Register all registry objects
        ModRegistries.register(modBus);

        // サーバー設定の登録 / Register server config
        Config.register();

        // ライフサイクルイベント / Lifecycle events
        modBus.addListener(this::commonSetup);

        // Forge イベントバス / Forge event bus
        MinecraftForge.EVENT_BUS.register(this);
    }

    /**
     * 共通セットアップ（サーバー・クライアント両方で実行）。
     * Common setup — runs on both server and client.
     */
    private void commonSetup(final FMLCommonSetupEvent event) {
        LOGGER.info("RustComputers common setup");
        NetworkHandler.register();

        // ペリフェラル登録（enqueueWork でレジストリアクセスをメインスレッドに
        // 安全に委譲する / Register peripherals safely on the main thread)
        event.enqueueWork(() -> {
            // --- Redstone ペリフェラル / Redstone peripherals ---
            PeripheralProvider.register(Blocks.LEVER,               VanillaRedstonePeripheral::new);
            PeripheralProvider.register(Blocks.REDSTONE_BLOCK,      VanillaRedstonePeripheral::new);
            PeripheralProvider.register(Blocks.REDSTONE_TORCH,      VanillaRedstonePeripheral::new);
            PeripheralProvider.register(Blocks.REDSTONE_WALL_TORCH, VanillaRedstonePeripheral::new);
            PeripheralProvider.register(Blocks.REDSTONE_WIRE,       VanillaRedstonePeripheral::new);
            PeripheralProvider.register(Blocks.REDSTONE_LAMP,       VanillaRedstonePeripheral::new);
            PeripheralProvider.register(Blocks.COMPARATOR,          VanillaRedstonePeripheral::new);
            PeripheralProvider.register(Blocks.REPEATER,            VanillaRedstonePeripheral::new);
            PeripheralProvider.register(Blocks.OBSERVER,            VanillaRedstonePeripheral::new);
            PeripheralProvider.register(Blocks.STONE_BUTTON,        VanillaRedstonePeripheral::new);
            PeripheralProvider.register(Blocks.OAK_BUTTON,          VanillaRedstonePeripheral::new);
            PeripheralProvider.register(Blocks.SPRUCE_BUTTON,       VanillaRedstonePeripheral::new);
            PeripheralProvider.register(Blocks.BIRCH_BUTTON,        VanillaRedstonePeripheral::new);
            PeripheralProvider.register(Blocks.JUNGLE_BUTTON,       VanillaRedstonePeripheral::new);
            PeripheralProvider.register(Blocks.ACACIA_BUTTON,       VanillaRedstonePeripheral::new);
            PeripheralProvider.register(Blocks.DARK_OAK_BUTTON,     VanillaRedstonePeripheral::new);
            PeripheralProvider.register(Blocks.CRIMSON_BUTTON,      VanillaRedstonePeripheral::new);
            PeripheralProvider.register(Blocks.WARPED_BUTTON,       VanillaRedstonePeripheral::new);
            PeripheralProvider.register(Blocks.MANGROVE_BUTTON,     VanillaRedstonePeripheral::new);
            PeripheralProvider.register(Blocks.BAMBOO_BUTTON,       VanillaRedstonePeripheral::new);
            PeripheralProvider.register(Blocks.CHERRY_BUTTON,       VanillaRedstonePeripheral::new);
            PeripheralProvider.register(Blocks.POLISHED_BLACKSTONE_BUTTON, VanillaRedstonePeripheral::new);
            PeripheralProvider.register(Blocks.STONE_PRESSURE_PLATE, VanillaRedstonePeripheral::new);
            PeripheralProvider.register(Blocks.OAK_PRESSURE_PLATE,  VanillaRedstonePeripheral::new);
            PeripheralProvider.register(Blocks.SPRUCE_PRESSURE_PLATE, VanillaRedstonePeripheral::new);
            PeripheralProvider.register(Blocks.BIRCH_PRESSURE_PLATE, VanillaRedstonePeripheral::new);
            PeripheralProvider.register(Blocks.JUNGLE_PRESSURE_PLATE, VanillaRedstonePeripheral::new);
            PeripheralProvider.register(Blocks.ACACIA_PRESSURE_PLATE, VanillaRedstonePeripheral::new);
            PeripheralProvider.register(Blocks.DARK_OAK_PRESSURE_PLATE, VanillaRedstonePeripheral::new);
            PeripheralProvider.register(Blocks.CRIMSON_PRESSURE_PLATE, VanillaRedstonePeripheral::new);
            PeripheralProvider.register(Blocks.WARPED_PRESSURE_PLATE, VanillaRedstonePeripheral::new);
            PeripheralProvider.register(Blocks.MANGROVE_PRESSURE_PLATE, VanillaRedstonePeripheral::new);
            PeripheralProvider.register(Blocks.BAMBOO_PRESSURE_PLATE, VanillaRedstonePeripheral::new);
            PeripheralProvider.register(Blocks.CHERRY_PRESSURE_PLATE, VanillaRedstonePeripheral::new);
            PeripheralProvider.register(Blocks.HEAVY_WEIGHTED_PRESSURE_PLATE, VanillaRedstonePeripheral::new);
            PeripheralProvider.register(Blocks.LIGHT_WEIGHTED_PRESSURE_PLATE, VanillaRedstonePeripheral::new);
            PeripheralProvider.register(Blocks.POLISHED_BLACKSTONE_PRESSURE_PLATE, VanillaRedstonePeripheral::new);
            PeripheralProvider.register(Blocks.SCULK_SENSOR,        VanillaRedstonePeripheral::new);
            PeripheralProvider.register(Blocks.CALIBRATED_SCULK_SENSOR, VanillaRedstonePeripheral::new);
            PeripheralProvider.register(Blocks.DAYLIGHT_DETECTOR,   VanillaRedstonePeripheral::new);
            PeripheralProvider.register(Blocks.LIGHTNING_ROD,       VanillaRedstonePeripheral::new);
            PeripheralProvider.register(Blocks.TRIPWIRE_HOOK,       VanillaRedstonePeripheral::new);

            // --- インベントリ ペリフェラル / Inventory peripherals ---
            PeripheralProvider.register(Blocks.CHEST,               VanillaInventoryPeripheral::new);
            PeripheralProvider.register(Blocks.TRAPPED_CHEST,       VanillaInventoryPeripheral::new);
            PeripheralProvider.register(Blocks.BARREL,              VanillaInventoryPeripheral::new);
            PeripheralProvider.register(Blocks.HOPPER,              VanillaInventoryPeripheral::new);
            PeripheralProvider.register(Blocks.FURNACE,             VanillaInventoryPeripheral::new);
            PeripheralProvider.register(Blocks.BLAST_FURNACE,       VanillaInventoryPeripheral::new);
            PeripheralProvider.register(Blocks.SMOKER,              VanillaInventoryPeripheral::new);
            PeripheralProvider.register(Blocks.DROPPER,             VanillaInventoryPeripheral::new);
            PeripheralProvider.register(Blocks.DISPENSER,           VanillaInventoryPeripheral::new);
            PeripheralProvider.register(Blocks.BREWING_STAND,       VanillaInventoryPeripheral::new);
            PeripheralProvider.register(Blocks.WHITE_SHULKER_BOX,        VanillaInventoryPeripheral::new);
            PeripheralProvider.register(Blocks.ORANGE_SHULKER_BOX,       VanillaInventoryPeripheral::new);
            PeripheralProvider.register(Blocks.MAGENTA_SHULKER_BOX,      VanillaInventoryPeripheral::new);
            PeripheralProvider.register(Blocks.LIGHT_BLUE_SHULKER_BOX,   VanillaInventoryPeripheral::new);
            PeripheralProvider.register(Blocks.YELLOW_SHULKER_BOX,       VanillaInventoryPeripheral::new);
            PeripheralProvider.register(Blocks.LIME_SHULKER_BOX,         VanillaInventoryPeripheral::new);
            PeripheralProvider.register(Blocks.PINK_SHULKER_BOX,         VanillaInventoryPeripheral::new);
            PeripheralProvider.register(Blocks.GRAY_SHULKER_BOX,         VanillaInventoryPeripheral::new);
            PeripheralProvider.register(Blocks.LIGHT_GRAY_SHULKER_BOX,   VanillaInventoryPeripheral::new);
            PeripheralProvider.register(Blocks.CYAN_SHULKER_BOX,         VanillaInventoryPeripheral::new);
            PeripheralProvider.register(Blocks.PURPLE_SHULKER_BOX,       VanillaInventoryPeripheral::new);
            PeripheralProvider.register(Blocks.BLUE_SHULKER_BOX,         VanillaInventoryPeripheral::new);
            PeripheralProvider.register(Blocks.BROWN_SHULKER_BOX,        VanillaInventoryPeripheral::new);
            PeripheralProvider.register(Blocks.GREEN_SHULKER_BOX,        VanillaInventoryPeripheral::new);
            PeripheralProvider.register(Blocks.RED_SHULKER_BOX,          VanillaInventoryPeripheral::new);
            PeripheralProvider.register(Blocks.BLACK_SHULKER_BOX,        VanillaInventoryPeripheral::new);
            PeripheralProvider.register(Blocks.SHULKER_BOX,              VanillaInventoryPeripheral::new);

            // --- CC:Tweaked Monitor (オプション / optional) ---
            // CC:Tweaked がロードされている場合のみ登録する。
            // Register only when CC:Tweaked is loaded.
            if (ModList.get().isLoaded("computercraft")) {
                registerCcMonitors();
            }

            // --- Some Peripherals (オプション / optional) ---
            // Some Peripherals がロードされている場合のみ登録する。
            // Register only when Some Peripherals is loaded.
            if (ModList.get().isLoaded("some_peripherals")) {
                registerSomePeripherals();
            }

            // --- CcGenericPeripheral を使った汎用ペリフェラル登録 ---
            // Generic peripheral registrations using CcGenericPeripheral bridge.
            // 各 Mod がロードされている場合のみ登録する。
            // Only register when each mod is loaded.
            if (ModList.get().isLoaded("computercraft")) {
                PeripheralRegistrations.registerCcTweaked();
            }
            if (ModList.get().isLoaded("some_peripherals")) {
                PeripheralRegistrations.registerSomePeripheralsExtras();
            }
            if (ModList.get().isLoaded("cc_vs")) {
                PeripheralRegistrations.registerCcVs();
            }
            if (ModList.get().isLoaded("toms_peripherals")) {
                PeripheralRegistrations.registerTomsPeripherals();
            }
            if (ModList.get().isLoaded("clockwork_cc_compat")) {
                PeripheralRegistrations.registerClockworkCcCompat();
            }
            if (ModList.get().isLoaded("create")) {
                PeripheralRegistrations.registerCreate();
            }
            if (ModList.get().isLoaded("createaddition")) {
                PeripheralRegistrations.registerCreateAddition();
            }
            if (ModList.get().isLoaded("controlcraft")) {
                PeripheralRegistrations.registerControlCraft();
            }
            if (ModList.get().isLoaded("advancedperipherals")) {
                PeripheralRegistrations.registerAdvancedPeripherals();
            }
            if (ModList.get().isLoaded("cbc_cc_control")) {
                PeripheralRegistrations.registerCbcCcControl();
            }

            LOGGER.info("RustComputers: registered {} peripherals", PeripheralProvider.registeredCount());
        });
    }

    /**
     * CC:Tweaked モニターブロックをペリフェラルとして登録する。
     * Register CC:Tweaked monitor blocks as peripherals.
     *
     * <p>{@code computercraft} Mod がロードされている場合にのみ呼び出す。</p>
     * <p>Only called when the {@code computercraft} mod is loaded.</p>
     */
    private void registerCcMonitors() {
        registerCcBlock("computercraft", "monitor_normal");
        registerCcBlock("computercraft", "monitor_advanced");
        LOGGER.info("RustComputers: CC:Tweaked monitor peripherals registered");
    }

    @SuppressWarnings("deprecation")
    private void registerCcBlock(String namespace, String path) {
        Block block = ForgeRegistries.BLOCKS.getValue(new ResourceLocation(namespace, path));
        if (block != null && block != Blocks.AIR) {
            PeripheralProvider.register(block, CcMonitorPeripheral::new);
            LOGGER.info("  Registered CC peripheral: {}:{}", namespace, path);
        } else {
            LOGGER.warn("  CC block not found: {}:{}", namespace, path);
        }
    }

    /**
     * Some Peripherals ブロックをペリフェラルとして登録する。
     * Register Some Peripherals blocks as peripherals.
     *
     * <p>{@code some_peripherals} Mod がロードされている場合にのみ呼び出す。</p>
     * <p>Only called when the {@code some_peripherals} mod is loaded.</p>
     */
    private void registerSomePeripherals() {
        registerSpBlock("some_peripherals", "radar",                   SomePeripheralsRadarPeripheral::new);
        registerSpBlock("some_peripherals", "world_scanner",           SomePeripheralsWorldScannerPeripheral::new);
        registerSpBlock("some_peripherals", "ballistic_accelerator",   SomePeripheralsBallisticAcceleratorPeripheral::new);
        registerSpBlock("some_peripherals", "raycaster",               SomePeripheralsRaycasterPeripheral::new);
        registerSpBlock("some_peripherals", "digitizer",               SomePeripheralsDigitizerPeripheral::new);
        LOGGER.info("RustComputers: Some Peripherals peripherals registered");
    }

    @SuppressWarnings("deprecation")
    private void registerSpBlock(String namespace, String path,
                                 java.util.function.Supplier<com.rustcomputers.peripheral.PeripheralType> supplier) {
        Block block = ForgeRegistries.BLOCKS.getValue(new ResourceLocation(namespace, path));
        if (block != null && block != Blocks.AIR) {
            PeripheralProvider.register(block, supplier);
            LOGGER.info("  Registered SP peripheral: {}:{}", namespace, path);
        } else {
            LOGGER.warn("  SP block not found: {}:{}", namespace, path);
        }
    }

    /**
     * コマンド登録 — /rustcomputers コマンドツリーを Brigadier に登録。
     * Command registration — register the /rustcomputers command tree with Brigadier.
     */
    @SubscribeEvent
    public void onRegisterCommands(RegisterCommandsEvent event) {
        RustComputersCommand.register(event.getDispatcher());
    }

    /**
     * プレイヤーログアウト時 — ログストリーミング設定をリセット。
     * Player logout — reset log streaming settings.
     */
    @SubscribeEvent
    public void onPlayerLoggedOut(PlayerEvent.PlayerLoggedOutEvent event) {
        RustComputersCommand.clearPlayerStreams(event.getEntity().getUUID());
    }

    /**
     * CC:Tweaked 高度モニター右クリック → タッチイベントをキューに積む。
     * CC:Tweaked advanced monitor right-click → enqueue a touch event.
     *
     * <p>右クリック位置をモニター面上の正規化 UV 座標 (0.0–1.0) に変換し、
     * {@link CcMonitorPeripheral#queueTouchEvent} へ渡す。</p>
     *
     * <p>Converts the right-click hit position to normalized UV (0.0–1.0)
     * on the monitor face, then calls {@link CcMonitorPeripheral#queueTouchEvent}.</p>
     */
    @SubscribeEvent
    public void onRightClickBlock(PlayerInteractEvent.RightClickBlock event) {
        // サーバー側のみ処理 / Process on server side only
        if (event.getSide() == LogicalSide.CLIENT) return;

        net.minecraft.core.BlockPos pos = event.getPos();
        net.minecraft.world.level.Level level = event.getLevel();

        // CC:Tweaked advanced monitor かチェック / Check for CC:Tweaked advanced monitor
        ResourceLocation rl = ForgeRegistries.BLOCKS.getKey(level.getBlockState(pos).getBlock());
        if (rl == null || !"computercraft:monitor_advanced".equals(rl.toString())) return;

        // ヒット位置からモニター面の UV を計算 / Compute face UV from hit position
        BlockHitResult hit = event.getHitVec();
        if (hit == null) return;

        Vec3 loc = hit.getLocation();
        Direction face = hit.getDirection();

        // ブロック内相対座標 (0.0–1.0) / Relative coords within block (0.0–1.0)
        double bx = loc.x - Math.floor(loc.x);
        double by = loc.y - Math.floor(loc.y);
        double bz = loc.z - Math.floor(loc.z);

        // 面に応じて U/V を決定 / Determine U/V based on face
        float u, v;
        switch (face) {
            case NORTH -> { u = (float)(1.0 - bx); v = (float)(1.0 - by); }
            case SOUTH -> { u = (float)(bx);       v = (float)(1.0 - by); }
            case EAST  -> { u = (float)(1.0 - bz); v = (float)(1.0 - by); }
            case WEST  -> { u = (float)(bz);        v = (float)(1.0 - by); }
            case UP    -> { u = (float)(bx);        v = (float)(bz);       }
            case DOWN  -> { u = (float)(bx);        v = (float)(1.0 - bz); }
            default    -> { u = 0.5f; v = 0.5f; }
        }

        BlockEntity monitorBe = level.getBlockEntity(pos);
        net.minecraft.core.BlockPos originPos = CcMonitorPeripheral.resolveMonitorOrigin(monitorBe, pos);
        CcMonitorPeripheral.queueTouchEvent(originPos, u, v);
        LOGGER.debug("Monitor touch queued at {} (origin={}) face={} u={} v={}", pos, originPos, face, u, v);
    }

    /**
     * サーバー起動時 — ComputerManager を初期化。
     * Server starting — initialize the ComputerManager singleton.
     */
    @SubscribeEvent
    public void onServerStarting(ServerStartingEvent event) {
        LOGGER.info("RustComputers: server starting, initializing ComputerManager");
        ComputerManager.get(event.getServer());
    }

    /**
     * サーバー停止時 — 全コンピューターを安全に停止。
     * Server stopping — gracefully shut down all running computers.
     */
    @SubscribeEvent
    public void onServerStopping(ServerStoppingEvent event) {
        LOGGER.info("RustComputers: server stopping, shutting down all computers");
        ComputerManager.shutdownAll();
    }

    /**
     * クライアント専用イベント（MOD バス）。
     * Client-side event handlers (MOD bus).
     */
    @Mod.EventBusSubscriber(modid = MOD_ID, bus = Mod.EventBusSubscriber.Bus.MOD, value = Dist.CLIENT)
    public static class ClientEvents {
        @SubscribeEvent
        public static void onClientSetup(FMLClientSetupEvent event) {
            LOGGER.info("RustComputers client setup");
            com.rustcomputers.gui.ComputerScreen.register();
        }

        private static final Logger LOGGER = LoggerFactory.getLogger(ClientEvents.class);
    }

    /**
     * クライアント FORGE バスイベント — ワールド切替時のクリーンアップ。
     * Client FORGE bus events — cleanup on world disconnect.
     */
    @Mod.EventBusSubscriber(modid = MOD_ID, bus = Mod.EventBusSubscriber.Bus.FORGE, value = Dist.CLIENT)
    public static class ClientForgeEvents {
        /**
         * ワールドからのログアウト時にクライアント側ログキャッシュをクリアする。
         * これにより前ワールドのログが新ワールドの UI に表示される問題を防ぐ。
         *
         * Clear client-side log cache when logging out of a world.
         * This prevents logs from a previous world leaking into the new world's UI.
         */
        @SubscribeEvent
        public static void onClientDisconnect(net.minecraftforge.client.event.ClientPlayerNetworkEvent.LoggingOut event) {
            com.rustcomputers.gui.ComputerScreen.clearLogCache();
            LOGGER.info("RustComputers: cleared client log cache (world disconnect)");
        }

        private static final Logger LOGGER = LoggerFactory.getLogger(ClientForgeEvents.class);
    }
}
