package com.rustcomputers.computer;

import com.rustcomputers.ModRegistries;
import net.minecraft.core.BlockPos;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.world.InteractionHand;
import net.minecraft.world.InteractionResult;
import net.minecraft.world.entity.player.Player;
import net.minecraft.world.level.Level;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.EntityBlock;
import net.minecraft.world.level.block.SoundType;
import net.minecraft.world.level.block.entity.BlockEntity;
import net.minecraft.world.level.block.entity.BlockEntityTicker;
import net.minecraft.world.level.block.entity.BlockEntityType;
import net.minecraft.world.level.block.state.BlockBehaviour;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.phys.BlockHitResult;
import net.minecraftforge.network.NetworkHooks;

import javax.annotation.Nullable;

/**
 * コンピューターブロック。
 * The computer block — right-click to open the GUI.
 *
 * <p>設置時にブロックエンティティが生成され、コンピューター ID が割り振られる。
 * サーバー側でのみ tick 処理が実行される。</p>
 *
 * <p>A block entity is created when placed, and a computer ID is assigned.
 * Tick processing runs server-side only.</p>
 */
public class ComputerBlock extends Block implements EntityBlock {

    public ComputerBlock() {
        super(BlockBehaviour.Properties.of()
                .strength(2.0f, 6.0f)
                .sound(SoundType.METAL)
                .requiresCorrectToolForDrops());
    }

    // ------------------------------------------------------------------
    // ブロックエンティティ / Block entity
    // ------------------------------------------------------------------

    @Nullable
    @Override
    public BlockEntity newBlockEntity(BlockPos pos, BlockState state) {
        return new ComputerBlockEntity(pos, state);
    }

    @Nullable
    @Override
    public <T extends BlockEntity> BlockEntityTicker<T> getTicker(
            Level level, BlockState state, BlockEntityType<T> type) {
        // サーバー側でのみ tick する / Only tick on the server side
        if (!level.isClientSide() && type == ModRegistries.COMPUTER_BE.get()) {
            return (lvl, pos, st, be) -> ((ComputerBlockEntity) be).serverTick();
        }
        return null;
    }

    // ------------------------------------------------------------------
    // インタラクション / Interaction
    // ------------------------------------------------------------------

    @Override
    @SuppressWarnings("deprecation")
    public InteractionResult use(BlockState state, Level level, BlockPos pos,
                                 Player player, InteractionHand hand, BlockHitResult hit) {
        if (level.isClientSide()) {
            return InteractionResult.SUCCESS;
        }

        BlockEntity be = level.getBlockEntity(pos);
        if (be instanceof ComputerBlockEntity computerBe && player instanceof ServerPlayer serverPlayer) {
            // GUI を開く / Open the GUI
            String[] programs = computerBe.getEngine() != null
                    ? computerBe.getEngine().listPrograms()
                    : new String[0];
            String selectedProgram = computerBe.getProgramName();
            NetworkHooks.openScreen(serverPlayer, computerBe, buf -> {
                buf.writeBlockPos(pos);
                buf.writeInt(computerBe.getComputerId());
                // プログラム一覧 / Program list
                buf.writeInt(programs.length);
                for (String p : programs) buf.writeUtf(p, 256);
                // 選択中のプログラム / Selected program
                buf.writeBoolean(selectedProgram != null);
                if (selectedProgram != null) buf.writeUtf(selectedProgram, 256);
            });
        }

        return InteractionResult.CONSUME;
    }

    // ------------------------------------------------------------------
    // 破壊時 / On destroy
    // ------------------------------------------------------------------

    @Override
    @SuppressWarnings("deprecation")
    public void onRemove(BlockState state, Level level, BlockPos pos,
                         BlockState newState, boolean movedByPiston) {
        if (!state.is(newState.getBlock())) {
            BlockEntity be = level.getBlockEntity(pos);
            if (be instanceof ComputerBlockEntity computerBe) {
                computerBe.onBlockDestroyed();
            }
        }
        super.onRemove(state, level, pos, newState, movedByPiston);
    }
}
