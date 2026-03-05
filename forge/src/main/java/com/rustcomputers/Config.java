package com.rustcomputers;

import net.minecraftforge.common.ForgeConfigSpec;
import net.minecraftforge.fml.ModLoadingContext;
import net.minecraftforge.fml.config.ModConfig;

/**
 * サーバー設定ファイル（config/rustcomputers-server.toml）。
 * Server-side configuration file (config/rustcomputers-server.toml).
 *
 * <p>WASM バイナリサイズ上限、Fuel 上限、タイムアウト、同時実行数を制御する。</p>
 * <p>Controls WASM binary size limit, fuel cap, timeout, and max concurrent computers.</p>
 */
public final class Config {

    private static final ForgeConfigSpec.Builder BUILDER = new ForgeConfigSpec.Builder();

    // ------------------------------------------------------------------
    // 制限値 / Limits
    // ------------------------------------------------------------------

    /** WASM バイナリの最大サイズ（バイト） / Max WASM binary size in bytes */
    public static final ForgeConfigSpec.LongValue MAX_WASM_SIZE;

    /** 1 tick あたりの Fuel 上限 / Fuel budget per tick */
    public static final ForgeConfigSpec.LongValue FUEL_PER_TICK;

    /** リクエストタイムアウト（tick 数） / Request timeout in ticks */
    public static final ForgeConfigSpec.IntValue REQUEST_TIMEOUT_TICKS;

    /** 同時に実行できるコンピューター数の上限 / Max concurrent running computers */
    public static final ForgeConfigSpec.IntValue MAX_COMPUTERS;

    /** ビジー状態タイムアウト（tick 数） / Busy-state timeout in ticks */
    public static final ForgeConfigSpec.IntValue BUSY_TIMEOUT_TICKS;

    /** ログバッファの最大行数 / Max lines in the log buffer */
    public static final ForgeConfigSpec.IntValue LOG_BUFFER_SIZE;

    static {
        BUILDER.comment(
                "RustComputers サーバー設定 / Server Configuration",
                "WASM 実行環境の制限値を設定します。",
                "Configure limits for the WASM execution environment."
        ).push("limits");

        MAX_WASM_SIZE = BUILDER
                .comment("WASM バイナリの最大サイズ（バイト）。デフォルト: 4 MB",
                         "Maximum WASM binary size in bytes. Default: 4 MB")
                .defineInRange("maxWasmSize", 4_194_304L, 1024L, 64_000_000L);

        FUEL_PER_TICK = BUILDER
                .comment("1 tick あたりの Fuel 上限。デフォルト: 10,000,000",
                         "Fuel budget per tick. Default: 10,000,000")
                .defineInRange("fuelPerTick", 10_000_000L, 1000L, 1_000_000_000L);

        REQUEST_TIMEOUT_TICKS = BUILDER
                .comment("ペリフェラルリクエストのタイムアウト（tick 数）。デフォルト: 100（約 5 秒）",
                         "Peripheral request timeout in ticks. Default: 100 (~5 sec)")
                .defineInRange("requestTimeoutTicks", 100, 1, 6000);

        MAX_COMPUTERS = BUILDER
                .comment("同時に実行できるコンピューター数の上限。デフォルト: 16",
                         "Max concurrent running computers. Default: 16")
                .defineInRange("maxComputers", 16, 1, 256);

        BUSY_TIMEOUT_TICKS = BUILDER
                .comment("ビジー状態（レスポンス待ち中）のタイムアウト tick 数。デフォルト: 20（約 1 秒）",
                         "Busy-state timeout in ticks. Default: 20 (~1 sec)")
                .defineInRange("busyTimeoutTicks", 20, 1, 200);

        LOG_BUFFER_SIZE = BUILDER
                .comment("ログバッファの最大行数。デフォルト: 200",
                         "Max lines in the log buffer. Default: 200")
                .defineInRange("logBufferSize", 200, 50, 10000);

        BUILDER.pop();
    }

    /** ビルド済み設定スペック / Built config spec */
    public static final ForgeConfigSpec SPEC = BUILDER.build();

    /**
     * Mod ロード時に呼ばれ、サーバー設定を登録する。
     * Called during mod loading to register the server config.
     */
    public static void register() {
        ModLoadingContext.get().registerConfig(ModConfig.Type.SERVER, SPEC, "rustcomputers-server.toml");
    }

    private Config() { /* ユーティリティクラス / Utility class */ }
}
