package com.rustcomputers.network;

import com.rustcomputers.computer.ComputerBlockEntity;
import net.minecraft.core.BlockPos;
import net.minecraft.network.FriendlyByteBuf;
import net.minecraft.server.level.ServerPlayer;
import net.minecraftforge.network.NetworkEvent;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.function.Supplier;

/**
 * C2S パケット: コンピューター操作コマンド（開始/停止/プログラム選択）。
 * Client-to-Server packet: computer action commands (start/stop/select program).
 */
public record ComputerActionPacket(BlockPos pos, Action action, String argument) {

    private static final Logger LOGGER = LoggerFactory.getLogger(ComputerActionPacket.class);

    /**
     * 操作種別 / Action type
     */
    public enum Action {
        /** プログラム開始 / Start program (argument = file name) */
        START,
        /** プログラム停止 / Stop program */
        STOP,
        /** プログラム選択（実行はしない） / Select program (without starting) */
        SELECT
    }

    // ------------------------------------------------------------------
    // エンコード・デコード / Encode & decode
    // ------------------------------------------------------------------

    public void encode(FriendlyByteBuf buf) {
        buf.writeBlockPos(pos);
        buf.writeEnum(action);
        buf.writeUtf(argument, 256);
    }

    public static ComputerActionPacket decode(FriendlyByteBuf buf) {
        BlockPos blockPos = buf.readBlockPos();
        Action act = buf.readEnum(Action.class);
        String arg = buf.readUtf(256);
        return new ComputerActionPacket(blockPos, act, arg);
    }

    // ------------------------------------------------------------------
    // ハンドラ / Handler
    // ------------------------------------------------------------------

    public void handle(Supplier<NetworkEvent.Context> ctx) {
        NetworkEvent.Context context = ctx.get();
        ServerPlayer player = context.getSender();
        if (player == null) return;

        if (!(player.level().getBlockEntity(pos) instanceof ComputerBlockEntity be)) {
            context.setPacketHandled(true);
            return;
        }

        switch (action) {
            case START -> {
                LOGGER.info("Player {} started program '{}' on computer #{}",
                        player.getName().getString(), argument, be.getComputerId());
                be.startProgram(argument);
            }
            case STOP -> {
                LOGGER.info("Player {} stopped computer #{}",
                        player.getName().getString(), be.getComputerId());
                be.stopProgram();
            }
            case SELECT -> {
                // プログラム選択のみ（実行はしない） / Select only (don't start)
                LOGGER.debug("Player {} selected program '{}' on computer #{}",
                        player.getName().getString(), argument, be.getComputerId());
                // Note: 実際のプログラム選択は GUI 側で管理
                // Note: Actual program selection is managed GUI-side
            }
        }

        context.setPacketHandled(true);
    }
}
