package com.rustcomputers.computer;

import com.rustcomputers.ModRegistries;
import com.rustcomputers.gui.ComputerMenu;
import com.rustcomputers.network.LogUpdatePacket;
import com.rustcomputers.network.NetworkHandler;
import com.rustcomputers.wasm.ComputerState;
import com.rustcomputers.wasm.WasmEngine;
import net.minecraft.core.BlockPos;
import net.minecraft.nbt.CompoundTag;
import net.minecraft.network.chat.Component;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.world.MenuProvider;
import net.minecraft.world.entity.player.Inventory;
import net.minecraft.world.entity.player.Player;
import net.minecraft.world.inventory.AbstractContainerMenu;
import net.minecraft.world.level.block.entity.BlockEntity;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraftforge.network.PacketDistributor;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.annotation.Nullable;
import java.util.List;

/**
 * コンピューターブロックエンティティ。
 * Block entity for the computer block, managing WASM execution state.
 *
 * <p>コンピューター ID を保持し、{@link WasmEngine} を通じて WASM プログラムを実行する。
 * チャンクのロード/アンロードに応じてエンジンの起動/停止を行う。</p>
 *
 * <p>Holds the computer ID and runs WASM programs via {@link WasmEngine}.
 * Starts and stops the engine based on chunk load/unload events.</p>
 */
public class ComputerBlockEntity extends BlockEntity implements MenuProvider {

    private static final Logger LOGGER = LoggerFactory.getLogger(ComputerBlockEntity.class);

    // NBT キー / NBT keys
    private static final String TAG_COMPUTER_ID = "ComputerId";
    private static final String TAG_PROGRAM_NAME = "ProgramName";
    private static final String TAG_STATE = "State";

    // ------------------------------------------------------------------
    // フィールド / Fields
    // ------------------------------------------------------------------

    /** コンピューター ID（-1 = 未割り当て） / Computer ID (-1 = unassigned) */
    private int computerId = -1;

    /** 選択中のプログラム名 / Currently selected program name */
    @Nullable
    private String programName;

    /** シャットダウン前の状態（再ロード時に復元） / State before shutdown (restored on reload) */
    private ComputerState savedState = ComputerState.STOPPED;

    /** WASM 実行エンジン（サーバー側のみ） / WASM execution engine (server-side only) */
    @Nullable
    private WasmEngine engine;

    // ------------------------------------------------------------------
    // コンストラクタ / Constructor
    // ------------------------------------------------------------------

    public ComputerBlockEntity(BlockPos pos, BlockState state) {
        super(ModRegistries.COMPUTER_BE.get(), pos, state);
    }

    // ------------------------------------------------------------------
    // ライフサイクル / Lifecycle
    // ------------------------------------------------------------------

    /**
     * チャンクロード時にエンジンを初期化する。
     * Initialize the engine when the chunk is loaded.
     */
    @Override
    public void onLoad() {
        super.onLoad();

        if (level instanceof ServerLevel serverLevel) {
            // ID が未割り当てなら新規発行 / Assign a new ID if not yet assigned
            if (computerId < 0) {
                computerId = ComputerManager.get(serverLevel.getServer()).allocateId();
                setChanged();
            }

            // エンジン初期化 / Initialize engine
            engine = ComputerManager.get(serverLevel.getServer())
                    .createEngine(computerId, serverLevel);

            // 前回実行中だったプログラムを再起動 / Restart previously running program
            if (savedState == ComputerState.RUNNING && programName != null) {
                engine.start(programName, serverLevel, getBlockPos());
            }

            LOGGER.debug("Computer #{} loaded at {}", computerId, getBlockPos());
        }
    }

    /**
     * チャンクアンロード時にエンジンを停止する。
     * Stop the engine when the chunk is unloaded.
     */
    @Override
    public void setRemoved() {
        if (engine != null) {
            savedState = engine.getState();
            engine.shutdown();
            engine = null;
        }
        super.setRemoved();
    }

    /**
     * ブロック破壊時のクリーンアップ。
     * Cleanup when the block is destroyed.
     */
    public void onBlockDestroyed() {
        if (engine != null) {
            engine.shutdown();
            engine = null;
        }
        if (level instanceof ServerLevel serverLevel) {
            ComputerManager.get(serverLevel.getServer()).unregister(computerId);
        }
    }

    // ------------------------------------------------------------------
    // Tick 処理 / Tick processing
    // ------------------------------------------------------------------

    /**
     * サーバー tick — {@link ComputerBlock} の BlockEntityTicker から呼ばれる。
     * Server tick — called by the BlockEntityTicker in {@link ComputerBlock}.
     */
    public void serverTick() {
        if (engine != null) {
            engine.tick();
            sendPendingLog();
        }
    }

    // ------------------------------------------------------------------
    // プログラム管理 / Program management
    // ------------------------------------------------------------------

    /**
     * プログラムを開始する。
     * Start a program.
     *
     * @param wasmFileName .wasm ファイル名 / .wasm file name
     * @return 成功したら true / true if successful
     */
    public boolean startProgram(String wasmFileName) {
        this.programName = wasmFileName;
        setChanged();
        if (engine != null) {
            ServerLevel serverLevel = (level instanceof ServerLevel sl) ? sl : null;
            boolean ok = engine.start(wasmFileName, serverLevel, getBlockPos());
            // 実行成否に関わらず、ログバッファを即座フラッシュ (GUIにエラー表示するため)
            // Immediately flush log regardless of success/failure (so GUI shows errors)
            sendPendingLog();
            return ok;
        }
        return false;
    }

    /**
     * 未送信のログ行を近くの全プレイヤーに送信する。
     * Send pending log lines to all players in the server level.
     */
    private void sendPendingLog() {
        if (engine == null || !(level instanceof ServerLevel serverLevel)) return;
        List<String> lines = engine.drainPendingLog();
        if (lines.isEmpty()) return;
        LogUpdatePacket packet = new LogUpdatePacket(computerId, lines);
        for (ServerPlayer player : serverLevel.players()) {
            NetworkHandler.CHANNEL.send(PacketDistributor.PLAYER.with(() -> player), packet);
        }
    }

    /**
     * プログラムを停止する。
     * Stop the running program.
     */
    public void stopProgram() {
        if (engine != null) {
            engine.shutdown();
        }
        setChanged();
    }

    /**
     * stdin 入力行を送信する。
     * Submit a stdin input line.
     *
     * @param line 入力行 / input line
     */
    public void submitStdin(String line) {
        if (engine != null) {
            engine.submitStdinLine(line);
        }
    }

    // ------------------------------------------------------------------
    // NBT 永続化 / NBT persistence
    // ------------------------------------------------------------------

    @Override
    protected void saveAdditional(CompoundTag tag) {
        super.saveAdditional(tag);
        tag.putInt(TAG_COMPUTER_ID, computerId);
        if (programName != null) {
            tag.putString(TAG_PROGRAM_NAME, programName);
        }
        // 現在の実行状態を保存（エンジンがあればその状態、なければ保存済み状態）
        // Save current execution state (from engine if available, otherwise saved state)
        ComputerState currentState = engine != null ? engine.getState() : savedState;
        tag.putInt(TAG_STATE, currentState.ordinal());
    }

    @Override
    public void load(CompoundTag tag) {
        super.load(tag);
        computerId = tag.getInt(TAG_COMPUTER_ID);
        programName = tag.contains(TAG_PROGRAM_NAME) ? tag.getString(TAG_PROGRAM_NAME) : null;
        savedState = ComputerState.fromOrdinal(tag.getInt(TAG_STATE));
    }

    // ------------------------------------------------------------------
    // MenuProvider 実装 / MenuProvider implementation
    // ------------------------------------------------------------------

    @Override
    public Component getDisplayName() {
        return Component.translatable("gui.rustcomputers.computer.title", computerId);
    }

    @Nullable
    @Override
    public AbstractContainerMenu createMenu(int containerId, Inventory playerInv, Player player) {
        return new ComputerMenu(containerId, playerInv, this);
    }

    // ------------------------------------------------------------------
    // アクセサ / Accessors
    // ------------------------------------------------------------------

    public int getComputerId() { return computerId; }

    @Nullable
    public String getProgramName() { return programName; }

    @Nullable
    public WasmEngine getEngine() { return engine; }

    /**
     * 現在のコンピューター状態を返す。
     * Return the current computer state.
     */
    public ComputerState getComputerState() {
        if (engine != null) return engine.getState();
        return savedState;
    }
}
