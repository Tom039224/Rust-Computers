package com.rustcomputers.computer;

import com.rustcomputers.Config;
import com.rustcomputers.RustComputers;
import com.rustcomputers.wasm.WasmEngine;
import net.minecraft.nbt.CompoundTag;
import net.minecraft.server.MinecraftServer;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.level.saveddata.SavedData;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.annotation.Nullable;
import java.nio.file.Path;
import java.util.HashMap;
import java.util.Map;

/**
 * 全コンピューターを一元管理するシングルトン（ワールド保存データ）。
 * Singleton manager for all computers (stored as world saved data).
 *
 * <p>コンピューター ID の発行・追跡、エンジンインスタンスの生成を担当する。
 * {@code saves/<world>/data/rustcomputers.dat} にシリアライズされる。</p>
 *
 * <p>Responsible for ID allocation, tracking, and engine instance creation.
 * Serialized to {@code saves/<world>/data/rustcomputers.dat}.</p>
 */
public class ComputerManager extends SavedData {

    private static final Logger LOGGER = LoggerFactory.getLogger(ComputerManager.class);
    private static final String DATA_NAME = RustComputers.MOD_ID;

    // ------------------------------------------------------------------
    // フィールド / Fields
    // ------------------------------------------------------------------

    /** 次に割り当てるコンピューター ID / Next computer ID to allocate */
    private int nextId;

    /** 生成済みのエンジンインスタンス（サーバーライフサイクル内のみ有効） */
    /** Active engine instances (valid only within the server lifecycle) */
    private final Map<Integer, WasmEngine> engines = new HashMap<>();

    /** サーバー参照 / Server reference */
    @Nullable
    private MinecraftServer server;

    // ------------------------------------------------------------------
    // コンストラクタ / Constructors
    // ------------------------------------------------------------------

    /** 新規作成用 / For new creation */
    private ComputerManager() {
        this.nextId = 0;
    }

    /** NBT 復元用 / For loading from NBT */
    private ComputerManager(CompoundTag tag) {
        this.nextId = tag.getInt("NextId");
    }

    // ------------------------------------------------------------------
    // SavedData シリアライゼーション / SavedData serialization
    // ------------------------------------------------------------------

    @Override
    public CompoundTag save(CompoundTag tag) {
        tag.putInt("NextId", nextId);
        return tag;
    }

    // ------------------------------------------------------------------
    // シングルトンアクセス / Singleton access
    // ------------------------------------------------------------------

    /**
     * サーバーに紐づいた ComputerManager を取得する（なければ作成）。
     * Get the ComputerManager associated with the server (create if absent).
     *
     * @param server Minecraft サーバーインスタンス / Minecraft server instance
     * @return ComputerManager インスタンス / ComputerManager instance
     */
    public static ComputerManager get(MinecraftServer server) {
        ServerLevel overworld = server.overworld();
        ComputerManager mgr = overworld.getDataStorage()
                .computeIfAbsent(ComputerManager::new, ComputerManager::new, DATA_NAME);
        mgr.server = server;
        return mgr;
    }

    // ------------------------------------------------------------------
    // ID 管理 / ID management
    // ------------------------------------------------------------------

    /**
     * 新しいコンピューター ID を発行する（ID をインクリメントして返す）。
     * Allocate a new unique computer ID (increments nextId and returns it).
     *
     * @return 新しい ID / new ID
     */
    public int allocateId() {
        int id = nextId++;
        setDirty();
        LOGGER.info("Allocated computer ID: {}", id);
        return id;
    }

    /**
     * 次に発行される ID（= 現在割り当て済み台数）を返す。
     * Returns the next ID to be issued (= number of allocated computers so far).
     * コマンド補完など読み取り専用用途向け。
     * For read-only use such as command tab-completion.
     *
     * @return 次の ID / next ID
     */
    public int getNextId() {
        return nextId;
    }

    // ------------------------------------------------------------------
    // エンジン管理 / Engine management
    // ------------------------------------------------------------------

    /**
     * 指定 ID のエンジンを生成（または既存を返す）。
     * Create (or return existing) engine for the given computer ID.
     *
     * @param computerId コンピューター ID / computer ID
     * @param level      サーバーレベル / server level
     * @return WasmEngine インスタンス / WasmEngine instance
     */
    public WasmEngine createEngine(int computerId, ServerLevel level) {
        return engines.computeIfAbsent(computerId, id -> {
            Path worldDir = level.getServer().getWorldPath(net.minecraft.world.level.storage.LevelResource.ROOT);
            Path computerDir = worldDir.resolve("rust computers").resolve("computer").resolve(String.valueOf(id));

            // ディレクトリが存在しなければ作成 / Create directory if it doesn't exist
            try {
                java.nio.file.Files.createDirectories(computerDir);
            } catch (java.io.IOException e) {
                LOGGER.error("Failed to create computer directory: {}", computerDir, e);
            }

            LOGGER.debug("Created WasmEngine for computer #{} at {}", id, computerDir);
            return new WasmEngine(id, computerDir);
        });
    }

    /**
     * 指定 ID のエンジンを取得する。
     * Get the engine for the given computer ID.
     *
     * @param computerId コンピューター ID / computer ID
     * @return エンジン、存在しなければ null / engine, or null if not found
     */
    @Nullable
    public WasmEngine getEngine(int computerId) {
        return engines.get(computerId);
    }

    /**
     * 指定 ID のエンジンを登録解除する（ブロック破壊時）。
     * Unregister the engine for the given computer ID (on block destruction).
     *
     * @param computerId コンピューター ID / computer ID
     */
    public void unregister(int computerId) {
        WasmEngine removed = engines.remove(computerId);
        if (removed != null) {
            removed.shutdown();
            LOGGER.info("Unregistered computer #{}", computerId);
        }
    }

    /**
     * 全コンピューターを安全に停止する（サーバー停止時）。
     * Gracefully shut down all computers (on server stop).
     */
    public static void shutdownAll() {
        // 静的メソッドのため、全アクティブサーバーのエンジンを停止する必要がある。
        // 実装上は onServerStopping で呼ばれるため、get(server) 経由でアクセスする。
        // This is intentionally a no-op here; actual shutdown is handled per-server.
        // See RustComputers.onServerStopping() which calls this.
        LOGGER.info("shutdownAll called — individual engines shut down via block entity lifecycle");
    }

    /**
     * 現在アクティブなエンジン数を返す。
     * Return the number of currently active engines.
     */
    public int activeCount() {
        return engines.size();
    }
}
