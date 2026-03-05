package com.rustcomputers.gui;

import com.rustcomputers.ModRegistries;
import com.rustcomputers.computer.ComputerBlockEntity;
import com.rustcomputers.wasm.ComputerState;
import net.minecraft.core.BlockPos;
import net.minecraft.network.FriendlyByteBuf;
import net.minecraft.world.entity.player.Inventory;
import net.minecraft.world.entity.player.Player;
import net.minecraft.world.inventory.AbstractContainerMenu;
import net.minecraft.world.inventory.ContainerData;
import net.minecraft.world.inventory.SimpleContainerData;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.level.block.entity.BlockEntity;

import javax.annotation.Nullable;
import java.util.ArrayList;
import java.util.List;

/**
 * コンピューター GUI のメニュー（サーバー/クライアント間の同期）。
 * Computer GUI menu (server/client synchronization).
 *
 * <p>インベントリスロットは持たない。ContainerData で基本状態を同期する。</p>
 * <p>No inventory slots. Uses ContainerData to sync basic state.</p>
 */
public class ComputerMenu extends AbstractContainerMenu {

    // ContainerData インデックス / ContainerData indices
    private static final int DATA_COMPUTER_ID = 0;
    private static final int DATA_STATE = 1;
    private static final int DATA_SIZE = 2;

    /** 同期データ / Synced data */
    private final ContainerData data;

    /** サーバー側でのみ非 null / Non-null only on server side */
    @Nullable
    private final ComputerBlockEntity blockEntity;

    /** ブロック位置（クライアント側の参照用） / Block position (for client-side reference) */
    private final BlockPos blockPos;

    /** コンピューターディレクトリ内の .wasm ファイル一覧（クライアント側） / .wasm file list (client-side) */
    private final List<String> programs;

    /** 選択中のプログラムインデックス（クライアント側のみ） / Selected program index (client-side only) */
    private int selectedProgramIndex = 0;

    // ------------------------------------------------------------------
    // コンストラクタ / Constructors
    // ------------------------------------------------------------------

    /**
     * サーバー側コンストラクタ。
     * Server-side constructor.
     */
    public ComputerMenu(int containerId, Inventory playerInv, ComputerBlockEntity be) {
        super(ModRegistries.COMPUTER_MENU.get(), containerId);
        this.blockEntity = be;
        this.blockPos = be.getBlockPos();
        this.programs = new ArrayList<>(); // サーバー側では不使用 / Unused on server side

        // ContainerData でサーバー→クライアントの基本状態を同期
        // Sync basic state from server to client via ContainerData
        this.data = new ContainerData() {
            @Override
            public int get(int index) {
                return switch (index) {
                    case DATA_COMPUTER_ID -> be.getComputerId();
                    case DATA_STATE -> be.getComputerState().ordinal();
                    default -> 0;
                };
            }

            @Override
            public void set(int index, int value) {
                // サーバー側は読み取り専用 / Server side is read-only
            }

            @Override
            public int getCount() { return DATA_SIZE; }
        };
        addDataSlots(data);
    }

    /**
     * クライアント側コンストラクタ（ネットワークから生成）。
     * Client-side constructor (created from network).
     */
    private ComputerMenu(int containerId, Inventory playerInv, BlockPos pos, List<String> programs,
                         @Nullable String selectedProgram) {
        super(ModRegistries.COMPUTER_MENU.get(), containerId);
        this.blockEntity = null;
        this.blockPos = pos;
        this.data = new SimpleContainerData(DATA_SIZE);
        addDataSlots(data);
        this.programs = new ArrayList<>(programs);
        // アップロード済みなら最初に一致するインデックスを選択 / Pre-select if already present
        if (selectedProgram != null) {
            int idx = this.programs.indexOf(selectedProgram);
            this.selectedProgramIndex = idx >= 0 ? idx : 0;
        }
    }

    /**
     * ネットワークデコード用ファクトリメソッド。
     * Factory method for network decoding.
     */
    public static ComputerMenu fromNetwork(int containerId, Inventory playerInv, FriendlyByteBuf buf) {
        BlockPos pos = buf.readBlockPos();
        buf.readInt(); // computerId — ContainerData で同期するため不要 / Synced via ContainerData
        int count = buf.readInt();
        List<String> programs = new ArrayList<>(count);
        for (int i = 0; i < count; i++) programs.add(buf.readUtf(256));
        String selectedProgram = buf.readBoolean() ? buf.readUtf(256) : null;
        return new ComputerMenu(containerId, playerInv, pos, programs, selectedProgram);
    }

    // ------------------------------------------------------------------
    // AbstractContainerMenu 実装 / AbstractContainerMenu implementation
    // ------------------------------------------------------------------

    @Override
    public ItemStack quickMoveStack(Player player, int index) {
        return ItemStack.EMPTY; // スロットなし / No slots
    }

    @Override
    public boolean stillValid(Player player) {
        if (blockEntity != null) {
            return player.distanceToSqr(
                    blockPos.getX() + 0.5,
                    blockPos.getY() + 0.5,
                    blockPos.getZ() + 0.5) <= 64.0;
        }
        return true;
    }

    // ------------------------------------------------------------------
    // 同期データアクセサ / Synced data accessors
    // ------------------------------------------------------------------

    /** コンピューター ID を返す / Return the computer ID */
    public int getComputerId() {
        return data.get(DATA_COMPUTER_ID);
    }

    /** コンピューター状態を返す / Return the computer state */
    public ComputerState getComputerState() {
        return ComputerState.fromOrdinal(data.get(DATA_STATE));
    }

    /** ブロック位置を返す / Return the block position */
    public BlockPos getBlockPos() {
        return blockPos;
    }

    /** サーバー側のブロックエンティティを返す / Return the server-side block entity */
    @Nullable
    public ComputerBlockEntity getBlockEntity() {
        return blockEntity;
    }

    // ------------------------------------------------------------------
    // プログラムリスト管理 / Program list management (client-side)
    // ------------------------------------------------------------------

    /** プログラム一覧を返す / Return the program list */
    public List<String> getPrograms() {
        return programs;
    }

    /** 選択中のプログラムインデックスを返す / Return the selected program index */
    public int getSelectedProgramIndex() {
        return selectedProgramIndex;
    }

    /** 選択中のプログラム名を返す（なければ null） / Return the selected program name (null if none) */
    @Nullable
    public String getSelectedProgram() {
        if (programs.isEmpty()) return null;
        int idx = Math.max(0, Math.min(selectedProgramIndex, programs.size() - 1));
        return programs.get(idx);
    }

    /** 選択インデックスを設定する / Set the selected program index */
    public void setSelectedProgramIndex(int index) {
        if (!programs.isEmpty()) {
            this.selectedProgramIndex = Math.max(0, Math.min(index, programs.size() - 1));
        }
    }

    /**
     * プログラム一覧を更新する（ProgramListPacket 受信時に呼ばれる）。
     * Update the program list (called when ProgramListPacket is received).
     */
    public void updatePrograms(List<String> newPrograms) {
        String currentSelection = getSelectedProgram();
        programs.clear();
        programs.addAll(newPrograms);
        // 可能なら以前の選択を維持 / Try to keep the previous selection
        if (currentSelection != null) {
            int idx = programs.indexOf(currentSelection);
            selectedProgramIndex = idx >= 0 ? idx : 0;
        } else {
            selectedProgramIndex = 0;
        }
    }
}
