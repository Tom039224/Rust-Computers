package com.rustcomputers.network;

import com.rustcomputers.gui.ComputerScreen;
import net.minecraft.network.FriendlyByteBuf;
import net.minecraftforge.network.NetworkEvent;

import java.util.ArrayList;
import java.util.List;
import java.util.function.Supplier;

/**
 * S2C パケット: コンピューターのプログラム一覧をクライアントに通知する。
 * Server-to-Client packet: notifies the client of the program list for a computer.
 *
 * <p>送信タイミング:
 * - GUI を開いたとき (ComputerBlock.use でのみ送信 — openScreen buffer では文字列非対応)
 * - WASM ファイルをアップロードしたとき（一覧更新）</p>
 */
public record ProgramListPacket(List<String> programs) {

    // ------------------------------------------------------------------
    // エンコード・デコード / Encode & decode
    // ------------------------------------------------------------------

    public void encode(FriendlyByteBuf buf) {
        buf.writeInt(programs.size());
        for (String p : programs) {
            buf.writeUtf(p, 256);
        }
    }

    public static ProgramListPacket decode(FriendlyByteBuf buf) {
        int count = buf.readInt();
        List<String> programs = new ArrayList<>(count);
        for (int i = 0; i < count; i++) {
            programs.add(buf.readUtf(256));
        }
        return new ProgramListPacket(programs);
    }

    // ------------------------------------------------------------------
    // ハンドラ / Handler
    // ------------------------------------------------------------------

    public void handle(Supplier<NetworkEvent.Context> ctx) {
        ctx.get().enqueueWork(() -> ComputerScreen.handleProgramListUpdate(programs));
        ctx.get().setPacketHandled(true);
    }
}
