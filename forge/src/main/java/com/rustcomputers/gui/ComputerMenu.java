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
    private ComputerMenu(int containerId, Inventory playerInv, BlockPos pos, int computerId) {
        super(ModRegistries.COMPUTER_MENU.get(), containerId);
        this.blockEntity = null;
        this.blockPos = pos;
        this.data = new SimpleContainerData(DATA_SIZE);
        addDataSlots(data);
    }

    /**
     * ネットワークデコード用ファクトリメソッド。
     * Factory method for network decoding.
     */
    public static ComputerMenu fromNetwork(int containerId, Inventory playerInv, FriendlyByteBuf buf) {
        BlockPos pos = buf.readBlockPos();
        int computerId = buf.readInt();
        return new ComputerMenu(containerId, playerInv, pos, computerId);
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
}
