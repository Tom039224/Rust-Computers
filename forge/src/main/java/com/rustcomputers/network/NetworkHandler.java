package com.rustcomputers.network;

import com.rustcomputers.RustComputers;
import net.minecraft.resources.ResourceLocation;
import net.minecraftforge.network.NetworkDirection;
import net.minecraftforge.network.NetworkRegistry;
import net.minecraftforge.network.simple.SimpleChannel;

/**
 * ネットワークパケットの登録と送信チャネル。
 * Network packet registration and communication channel.
 *
 * <p>クライアント→サーバー: stdin 入力、WASM アップロード、操作コマンド
 * サーバー→クライアント: ログ更新、状態同期</p>
 *
 * <p>Client→Server: stdin input, WASM upload, action commands
 * Server→Client: log updates, state sync</p>
 */
public final class NetworkHandler {

    private static final String PROTOCOL_VERSION = "1";

    /** ネットワークチャネル / Network channel */
    public static final SimpleChannel CHANNEL = NetworkRegistry.newSimpleChannel(
            new ResourceLocation(RustComputers.MOD_ID, "main"),
            () -> PROTOCOL_VERSION,
            PROTOCOL_VERSION::equals,
            PROTOCOL_VERSION::equals
    );

    private static int nextPacketId = 0;

    /**
     * 全パケットタイプを登録する。
     * Register all packet types.
     */
    public static void register() {
        // C2S: stdin 入力行 / stdin input line
        CHANNEL.messageBuilder(StdinInputPacket.class, nextPacketId++, NetworkDirection.PLAY_TO_SERVER)
                .encoder(StdinInputPacket::encode)
                .decoder(StdinInputPacket::decode)
                .consumerMainThread(StdinInputPacket::handle)
                .add();

        // C2S: WASM ファイルアップロード / WASM file upload
        CHANNEL.messageBuilder(UploadWasmPacket.class, nextPacketId++, NetworkDirection.PLAY_TO_SERVER)
                .encoder(UploadWasmPacket::encode)
                .decoder(UploadWasmPacket::decode)
                .consumerMainThread(UploadWasmPacket::handle)
                .add();

        // C2S: コンピューター操作（開始/停止/プログラム選択） / Computer actions
        CHANNEL.messageBuilder(ComputerActionPacket.class, nextPacketId++, NetworkDirection.PLAY_TO_SERVER)
                .encoder(ComputerActionPacket::encode)
                .decoder(ComputerActionPacket::decode)
                .consumerMainThread(ComputerActionPacket::handle)
                .add();

        // S2C: ログ更新 / Log update
        CHANNEL.messageBuilder(LogUpdatePacket.class, nextPacketId++, NetworkDirection.PLAY_TO_CLIENT)
                .encoder(LogUpdatePacket::encode)
                .decoder(LogUpdatePacket::decode)
                .consumerMainThread(LogUpdatePacket::handle)
                .add();
    }

    private NetworkHandler() { /* ユーティリティクラス / Utility class */ }
}
