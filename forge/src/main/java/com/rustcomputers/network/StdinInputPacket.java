package com.rustcomputers.network;

import com.rustcomputers.computer.ComputerBlockEntity;
import net.minecraft.core.BlockPos;
import net.minecraft.network.FriendlyByteBuf;
import net.minecraft.server.level.ServerPlayer;
import net.minecraftforge.network.NetworkEvent;

import java.util.function.Supplier;

/**
 * C2S パケット: GUI の入力欄から stdin 行を送信する。
 * Client-to-Server packet: send a stdin line from the GUI input field.
 */
public record StdinInputPacket(BlockPos pos, String line) {

    /** 最大入力長（バイト） / Maximum input length in bytes */
    private static final int MAX_LINE_LENGTH = 4096;

    // ------------------------------------------------------------------
    // エンコード・デコード / Encode & decode
    // ------------------------------------------------------------------

    public void encode(FriendlyByteBuf buf) {
        buf.writeBlockPos(pos);
        buf.writeUtf(line, MAX_LINE_LENGTH);
    }

    public static StdinInputPacket decode(FriendlyByteBuf buf) {
        BlockPos pos = buf.readBlockPos();
        String line = buf.readUtf(MAX_LINE_LENGTH);
        return new StdinInputPacket(pos, line);
    }

    // ------------------------------------------------------------------
    // ハンドラ / Handler
    // ------------------------------------------------------------------

    public void handle(Supplier<NetworkEvent.Context> ctx) {
        NetworkEvent.Context context = ctx.get();
        ServerPlayer player = context.getSender();
        if (player == null) return;

        // ブロックエンティティを取得して stdin 送信 / Get block entity and submit stdin
        if (player.level().getBlockEntity(pos) instanceof ComputerBlockEntity be) {
            be.submitStdin(line);
        }
        context.setPacketHandled(true);
    }
}
