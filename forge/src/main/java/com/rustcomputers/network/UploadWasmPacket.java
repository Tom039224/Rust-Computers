package com.rustcomputers.network;

import com.rustcomputers.Config;
import com.rustcomputers.computer.ComputerBlockEntity;
import com.rustcomputers.wasm.WasmEngine;
import net.minecraft.core.BlockPos;
import net.minecraft.network.FriendlyByteBuf;
import net.minecraft.network.chat.Component;
import net.minecraft.server.level.ServerPlayer;
import net.minecraftforge.network.NetworkEvent;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.function.Supplier;

/**
 * C2S パケット: .wasm ファイルをサーバーにアップロードする。
 * Client-to-Server packet: upload a .wasm file to the server.
 *
 * <p>クライアントの Drag & Drop でドロップされたファイルを
 * サーバー側の {@code computer/<id>/} に保存する。</p>
 *
 * <p>Saves files dropped via client-side Drag & Drop to the
 * server's {@code computer/<id>/} directory.</p>
 */
public record UploadWasmPacket(BlockPos pos, String fileName, byte[] data) {

    private static final Logger LOGGER = LoggerFactory.getLogger(UploadWasmPacket.class);

    // ------------------------------------------------------------------
    // エンコード・デコード / Encode & decode
    // ------------------------------------------------------------------

    public void encode(FriendlyByteBuf buf) {
        buf.writeBlockPos(pos);
        buf.writeUtf(fileName, 256);
        buf.writeByteArray(data);
    }

    public static UploadWasmPacket decode(FriendlyByteBuf buf) {
        BlockPos blockPos = buf.readBlockPos();
        String name = buf.readUtf(256);
        byte[] bytes = buf.readByteArray();
        return new UploadWasmPacket(blockPos, name, bytes);
    }

    // ------------------------------------------------------------------
    // ハンドラ / Handler
    // ------------------------------------------------------------------

    public void handle(Supplier<NetworkEvent.Context> ctx) {
        NetworkEvent.Context context = ctx.get();
        ServerPlayer player = context.getSender();
        if (player == null) return;

        // ファイル名サニタイズ / Sanitize file name
        String safeName = WasmEngine.sanitizeFileName(fileName);
        if (safeName == null) {
            player.sendSystemMessage(Component.translatable("message.rustcomputers.invalid_wasm"));
            context.setPacketHandled(true);
            return;
        }

        // サイズ検証 / Size validation
        long maxSize = Config.MAX_WASM_SIZE.get();
        if (data.length > maxSize) {
            player.sendSystemMessage(Component.translatable(
                    "message.rustcomputers.file_too_large",
                    formatBytes(maxSize)));
            context.setPacketHandled(true);
            return;
        }

        // WASM マジックナンバー検証 / WASM magic number validation
        if (!WasmEngine.isValidWasm(data)) {
            player.sendSystemMessage(Component.translatable("message.rustcomputers.invalid_wasm"));
            context.setPacketHandled(true);
            return;
        }

        // ブロックエンティティ取得 / Get block entity
        if (!(player.level().getBlockEntity(pos) instanceof ComputerBlockEntity be)) {
            context.setPacketHandled(true);
            return;
        }

        // エンジン取得・ファイル保存 / Get engine and save file
        WasmEngine engine = be.getEngine();
        if (engine == null) {
            context.setPacketHandled(true);
            return;
        }

        try {
            Path target = engine.getComputerDir().resolve(safeName);
            Files.createDirectories(target.getParent());
            Files.write(target, data);

            LOGGER.info("Player {} uploaded {} ({} bytes) to computer #{}",
                    player.getName().getString(), safeName, data.length, be.getComputerId());
            player.sendSystemMessage(Component.translatable(
                    "message.rustcomputers.upload_success", safeName));

        } catch (IOException e) {
            LOGGER.error("Failed to save uploaded WASM", e);
            player.sendSystemMessage(Component.translatable(
                    "message.rustcomputers.upload_failed", e.getMessage()));
        }

        context.setPacketHandled(true);
    }

    // ------------------------------------------------------------------
    // ユーティリティ / Utility
    // ------------------------------------------------------------------

    private static String formatBytes(long bytes) {
        if (bytes >= 1_048_576) return (bytes / 1_048_576) + " MB";
        if (bytes >= 1024) return (bytes / 1024) + " KB";
        return bytes + " B";
    }
}
