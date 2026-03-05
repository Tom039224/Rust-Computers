package com.rustcomputers.network;

import net.minecraft.network.FriendlyByteBuf;
import net.minecraftforge.network.NetworkEvent;

import java.util.ArrayList;
import java.util.List;
import java.util.function.Supplier;

/**
 * S2C パケット: ログ更新をクライアントに送信する。
 * Server-to-Client packet: send log updates to the client.
 *
 * <p>コンピューター GUI を開いているプレイヤーへ、
 * 新しいログ行をリアルタイムで配信する。</p>
 *
 * <p>Delivers new log lines in real-time to players
 * who have the computer GUI open.</p>
 */
public record LogUpdatePacket(int computerId, List<String> lines) {

    /** 1パケットあたりの最大行数 / Max lines per packet */
    private static final int MAX_LINES = 50;

    /** 1行あたりの最大文字数 / Max characters per line */
    private static final int MAX_LINE_LENGTH = 1024;

    // ------------------------------------------------------------------
    // エンコード・デコード / Encode & decode
    // ------------------------------------------------------------------

    public void encode(FriendlyByteBuf buf) {
        buf.writeInt(computerId);
        int count = Math.min(lines.size(), MAX_LINES);
        buf.writeInt(count);
        for (int i = 0; i < count; i++) {
            buf.writeUtf(lines.get(i), MAX_LINE_LENGTH);
        }
    }

    public static LogUpdatePacket decode(FriendlyByteBuf buf) {
        int id = buf.readInt();
        int count = buf.readInt();
        List<String> logLines = new ArrayList<>(count);
        for (int i = 0; i < count; i++) {
            logLines.add(buf.readUtf(MAX_LINE_LENGTH));
        }
        return new LogUpdatePacket(id, logLines);
    }

    // ------------------------------------------------------------------
    // ハンドラ / Handler
    // ------------------------------------------------------------------

    public void handle(Supplier<NetworkEvent.Context> ctx) {
        NetworkEvent.Context context = ctx.get();
        // クライアント側で処理 / Process on client side
        // GUI がこのコンピューター ID を表示中であればログを追加する
        // If the GUI is displaying this computer ID, append the log lines
        context.enqueueWork(() -> {
            // クライアント側のログ受信処理は ComputerScreen が担当する
            // Client-side log reception is handled by ComputerScreen
            com.rustcomputers.gui.ComputerScreen.handleLogUpdate(computerId, lines);
        });
        context.setPacketHandled(true);
    }
}
