package com.rustcomputers;

import com.rustcomputers.command.RustComputersCommand;
import com.rustcomputers.computer.ComputerManager;
import com.rustcomputers.network.NetworkHandler;
import net.minecraftforge.api.distmarker.Dist;
import net.minecraftforge.common.MinecraftForge;
import net.minecraftforge.event.RegisterCommandsEvent;
import net.minecraftforge.event.entity.player.PlayerEvent;
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
    }

    /**
     * コマンド登録 — /rc コマンドツリーを Brigadier に登録。
     * Command registration — register the /rc command tree with Brigadier.
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
     * クライアント専用イベント。
     * Client-side event handlers.
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
}
