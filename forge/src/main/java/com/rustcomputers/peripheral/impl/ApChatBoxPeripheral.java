package com.rustcomputers.peripheral.impl;

import com.rustcomputers.peripheral.PeripheralException;
import com.rustcomputers.peripheral.PeripheralType;
import net.minecraft.core.BlockPos;
import net.minecraft.network.chat.Component;
import net.minecraft.network.chat.MutableComponent;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.server.level.ServerPlayer;
import net.minecraftforge.server.ServerLifecycleHooks;
import org.msgpack.core.MessageBufferPacker;
import org.msgpack.core.MessagePack;
import org.msgpack.core.MessageUnpacker;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.UUID;

/**
 * AdvancedPeripherals ChatBox ペリフェラル実装。
 * AdvancedPeripherals ChatBox peripheral implementation.
 *
 * <p>プレイヤーにチャットメッセージやトースト通知を送信する。
 * プレーンテキストとJSON形式のメッセージに対応。</p>
 *
 * <p>Sends chat messages and toast notifications to players.
 * Supports plain text and JSON-formatted messages.</p>
 *
 * <h3>Methods:</h3>
 * <ul>
 *   <li><b>sendMessage(message, prefix?, brackets?, color?)</b> - 全プレイヤーにメッセージ送信</li>
 *   <li><b>sendFormattedMessage(json, prefix?, brackets?, color?)</b> - 全プレイヤーにJSON形式メッセージ送信</li>
 *   <li><b>sendMessageToPlayer(message, player, prefix?, brackets?, color?)</b> - 特定プレイヤーにメッセージ送信</li>
 *   <li><b>sendFormattedMessageToPlayer(json, player, prefix?, brackets?, color?)</b> - 特定プレイヤーにJSON形式メッセージ送信</li>
 *   <li><b>sendToastToPlayer(message, title, player, prefix?, brackets?, color?)</b> - 特定プレイヤーにトースト送信</li>
 *   <li><b>sendFormattedToastToPlayer(json_message, json_title, player, prefix?, brackets?, color?)</b> - 特定プレイヤーにJSON形式トースト送信</li>
 * </ul>
 *
 * <h3>Three-Function Pair Pattern:</h3>
 * <p>各メソッドは Rust 側で以下の3つの形式で提供される:</p>
 * <ul>
 *   <li><b>book_next_*(args)</b> - リクエストを予約 / Book a request</li>
 *   <li><b>read_last_*()</b> - 前tickの結果を読み取り / Read result from previous tick</li>
 *   <li><b>async_*(args)</b> - .await で結果を取得 / Get result with .await</li>
 * </ul>
 *
 * <h3>Action Methods:</h3>
 * <p>全メソッドがワールド操作系（Action）。複数回実行可能（蓄積）。</p>
 */
public class ApChatBoxPeripheral implements PeripheralType {
    private static final Logger LOGGER = LoggerFactory.getLogger(ApChatBoxPeripheral.class);

    private static final String TYPE_NAME = "chat_box";
    
    /**
     * サポートされる全メソッド。
     * All supported methods.
     */
    private static final String[] METHODS = {
            "sendMessage",                      // Action: 全プレイヤーにメッセージ送信
            "sendFormattedMessage",             // Action: 全プレイヤーにJSON形式メッセージ送信
            "sendMessageToPlayer",              // Action: 特定プレイヤーにメッセージ送信
            "sendFormattedMessageToPlayer",     // Action: 特定プレイヤーにJSON形式メッセージ送信
            "sendToastToPlayer",                // Action: 特定プレイヤーにトースト送信
            "sendFormattedToastToPlayer"        // Action: 特定プレイヤーにJSON形式トースト送信
    };

    // デフォルト設定
    private static final String DEFAULT_PREFIX = "Computer";
    private static final String DEFAULT_BRACKETS = "[]";
    private static final int MAX_MESSAGE_SIZE = 256;

    @Override
    public String getTypeName() {
        return TYPE_NAME;
    }

    @Override
    public String[] getMethodNames() {
        return METHODS.clone();
    }

    @Override
    public byte[] callMethod(String methodName, byte[] args,
                             ServerLevel level, BlockPos peripheralPos)
            throws PeripheralException {
        
        try {
            return switch (methodName) {
                case "sendMessage" -> executeSendMessage(args, level);
                case "sendFormattedMessage" -> executeSendFormattedMessage(args, level);
                case "sendMessageToPlayer" -> executeSendMessageToPlayer(args, level);
                case "sendFormattedMessageToPlayer" -> executeSendFormattedMessageToPlayer(args, level);
                case "sendToastToPlayer" -> executeSendToastToPlayer(args, level);
                case "sendFormattedToastToPlayer" -> executeSendFormattedToastToPlayer(args, level);
                default -> throw new PeripheralException("Unknown method: " + methodName);
            };
        } catch (IOException e) {
            LOGGER.error("Failed to process method '{}'", methodName, e);
            throw new PeripheralException("Failed to process method: " + e.getMessage());
        }
    }

    @Override
    public byte[] callImmediate(String methodName, byte[] args,
                                ServerLevel level, BlockPos peripheralPos)
            throws PeripheralException {
        // 全メソッドが Action なので immediate 非対応
        // All methods are actions, so immediate is not supported
        throw new PeripheralException("Method '" + methodName + "' is an action and cannot be called immediately");
    }

    /**
     * sendMessage 実装。
     * 全プレイヤーにプレーンテキストメッセージを送信。
     */
    private byte[] executeSendMessage(byte[] args, ServerLevel level) throws IOException, PeripheralException {
        MessageUnpacker unpacker = MessagePack.newDefaultUnpacker(args);
        
        // 引数: message, prefix?, brackets?, color?
        String message = unpacker.unpackString();
        String prefix = unpacker.hasNext() ? unpacker.unpackString() : DEFAULT_PREFIX;
        String brackets = unpacker.hasNext() ? unpacker.unpackString() : DEFAULT_BRACKETS;
        String color = unpacker.hasNext() ? unpacker.unpackString() : "";
        unpacker.close();
        
        // メッセージサイズチェック
        if (message.length() > MAX_MESSAGE_SIZE) {
            return encodeResult(false, "Message is too long");
        }
        
        // ブラケット検証
        if (brackets.length() != 2) {
            return encodeResult(false, "incorrect bracket string (e.g. [], {}, <>, ...)");
        }
        
        // メッセージ作成
        MutableComponent preparedMessage = appendPrefix(prefix, brackets, color).append(message);
        
        // 全プレイヤーに送信
        for (ServerPlayer player : ServerLifecycleHooks.getCurrentServer().getPlayerList().getPlayers()) {
            player.sendSystemMessage(preparedMessage);
        }
        
        return encodeResult(true, null);
    }

    /**
     * sendFormattedMessage 実装。
     * 全プレイヤーにJSON形式メッセージを送信。
     */
    private byte[] executeSendFormattedMessage(byte[] args, ServerLevel level) throws IOException, PeripheralException {
        MessageUnpacker unpacker = MessagePack.newDefaultUnpacker(args);
        
        // 引数: json, prefix?, brackets?, color?
        String json = unpacker.unpackString();
        String prefix = unpacker.hasNext() ? unpacker.unpackString() : DEFAULT_PREFIX;
        String brackets = unpacker.hasNext() ? unpacker.unpackString() : DEFAULT_BRACKETS;
        String color = unpacker.hasNext() ? unpacker.unpackString() : "";
        unpacker.close();
        
        // メッセージサイズチェック
        if (json.length() > MAX_MESSAGE_SIZE) {
            return encodeResult(false, "Message is too long");
        }
        
        // ブラケット検証
        if (brackets.length() != 2) {
            return encodeResult(false, "incorrect bracket string (e.g. [], {}, <>, ...)");
        }
        
        // JSON パース
        MutableComponent component;
        try {
            component = Component.Serializer.fromJson(json);
            if (component == null) {
                return encodeResult(false, "incorrect json");
            }
        } catch (Exception e) {
            return encodeResult(false, "incorrect json");
        }
        
        // メッセージ作成
        MutableComponent preparedMessage = appendPrefix(prefix, brackets, color).append(component);
        
        // 全プレイヤーに送信
        for (ServerPlayer player : ServerLifecycleHooks.getCurrentServer().getPlayerList().getPlayers()) {
            player.sendSystemMessage(preparedMessage);
        }
        
        return encodeResult(true, null);
    }

    /**
     * sendMessageToPlayer 実装。
     * 特定プレイヤーにプレーンテキストメッセージを送信。
     */
    private byte[] executeSendMessageToPlayer(byte[] args, ServerLevel level) throws IOException, PeripheralException {
        MessageUnpacker unpacker = MessagePack.newDefaultUnpacker(args);
        
        // 引数: message, player, prefix?, brackets?, color?
        String message = unpacker.unpackString();
        String playerName = unpacker.unpackString();
        String prefix = unpacker.hasNext() ? unpacker.unpackString() : DEFAULT_PREFIX;
        String brackets = unpacker.hasNext() ? unpacker.unpackString() : DEFAULT_BRACKETS;
        String color = unpacker.hasNext() ? unpacker.unpackString() : "";
        unpacker.close();
        
        // メッセージサイズチェック
        if (message.length() > MAX_MESSAGE_SIZE) {
            return encodeResult(false, "Message is too long");
        }
        
        // ブラケット検証
        if (brackets.length() != 2) {
            return encodeResult(false, "incorrect bracket string (e.g. [], {}, <>, ...)");
        }
        
        // プレイヤー取得
        ServerPlayer player = getPlayer(playerName);
        if (player == null) {
            return encodeResult(false, "incorrect player name/uuid");
        }
        
        // メッセージ作成
        MutableComponent preparedMessage = appendPrefix(prefix, brackets, color).append(message);
        
        // プレイヤーに送信
        player.sendSystemMessage(preparedMessage);
        
        return encodeResult(true, null);
    }

    /**
     * sendFormattedMessageToPlayer 実装。
     * 特定プレイヤーにJSON形式メッセージを送信。
     */
    private byte[] executeSendFormattedMessageToPlayer(byte[] args, ServerLevel level) throws IOException, PeripheralException {
        MessageUnpacker unpacker = MessagePack.newDefaultUnpacker(args);
        
        // 引数: json, player, prefix?, brackets?, color?
        String json = unpacker.unpackString();
        String playerName = unpacker.unpackString();
        String prefix = unpacker.hasNext() ? unpacker.unpackString() : DEFAULT_PREFIX;
        String brackets = unpacker.hasNext() ? unpacker.unpackString() : DEFAULT_BRACKETS;
        String color = unpacker.hasNext() ? unpacker.unpackString() : "";
        unpacker.close();
        
        // メッセージサイズチェック
        if (json.length() > MAX_MESSAGE_SIZE) {
            return encodeResult(false, "Message is too long");
        }
        
        // ブラケット検証
        if (brackets.length() != 2) {
            return encodeResult(false, "incorrect bracket string (e.g. [], {}, <>, ...)");
        }
        
        // プレイヤー取得
        ServerPlayer player = getPlayer(playerName);
        if (player == null) {
            return encodeResult(false, "incorrect player name/uuid");
        }
        
        // JSON パース
        MutableComponent component;
        try {
            component = Component.Serializer.fromJson(json);
            if (component == null) {
                return encodeResult(false, "incorrect json");
            }
        } catch (Exception e) {
            return encodeResult(false, "incorrect json");
        }
        
        // メッセージ作成
        MutableComponent preparedMessage = appendPrefix(prefix, brackets, color).append(component);
        
        // プレイヤーに送信
        player.sendSystemMessage(preparedMessage);
        
        return encodeResult(true, null);
    }

    /**
     * sendToastToPlayer 実装。
     * 特定プレイヤーにプレーンテキストトーストを送信。
     * 
     * Note: Toast notifications require client-side packet handling.
     * This is a simplified implementation that sends a chat message instead.
     */
    private byte[] executeSendToastToPlayer(byte[] args, ServerLevel level) throws IOException, PeripheralException {
        MessageUnpacker unpacker = MessagePack.newDefaultUnpacker(args);
        
        // 引数: title, subtitle, player, prefix?, brackets?, color?
        String title = unpacker.unpackString();
        String subtitle = unpacker.unpackString();
        String playerName = unpacker.unpackString();
        String prefix = unpacker.hasNext() ? unpacker.unpackString() : DEFAULT_PREFIX;
        String brackets = unpacker.hasNext() ? unpacker.unpackString() : DEFAULT_BRACKETS;
        String color = unpacker.hasNext() ? unpacker.unpackString() : "";
        unpacker.close();
        
        // メッセージサイズチェック
        if (subtitle.length() > MAX_MESSAGE_SIZE) {
            return encodeResult(false, "Message is too long");
        }
        
        // ブラケット検証
        if (brackets.length() != 2) {
            return encodeResult(false, "incorrect bracket string (e.g. [], {}, <>, ...)");
        }
        
        // プレイヤー取得
        ServerPlayer player = getPlayer(playerName);
        if (player == null) {
            return encodeResult(false, "incorrect player name/uuid");
        }
        
        // トーストメッセージ作成（タイトル + サブタイトル）
        MutableComponent preparedMessage = appendPrefix(prefix, brackets, color)
                .append(Component.literal("[" + title + "] "))
                .append(subtitle);
        
        // プレイヤーに送信（チャットメッセージとして）
        player.sendSystemMessage(preparedMessage);
        
        return encodeResult(true, null);
    }

    /**
     * sendFormattedToastToPlayer 実装。
     * 特定プレイヤーにJSON形式トーストを送信。
     * 
     * Note: Toast notifications require client-side packet handling.
     * This is a simplified implementation that sends a chat message instead.
     */
    private byte[] executeSendFormattedToastToPlayer(byte[] args, ServerLevel level) throws IOException, PeripheralException {
        MessageUnpacker unpacker = MessagePack.newDefaultUnpacker(args);
        
        // 引数: json_title, json_subtitle, player, prefix?, brackets?, color?
        String jsonTitle = unpacker.unpackString();
        String jsonSubtitle = unpacker.unpackString();
        String playerName = unpacker.unpackString();
        String prefix = unpacker.hasNext() ? unpacker.unpackString() : DEFAULT_PREFIX;
        String brackets = unpacker.hasNext() ? unpacker.unpackString() : DEFAULT_BRACKETS;
        String color = unpacker.hasNext() ? unpacker.unpackString() : "";
        unpacker.close();
        
        // メッセージサイズチェック
        if (jsonSubtitle.length() > MAX_MESSAGE_SIZE) {
            return encodeResult(false, "Message is too long");
        }
        
        // ブラケット検証
        if (brackets.length() != 2) {
            return encodeResult(false, "incorrect bracket string (e.g. [], {}, <>, ...)");
        }
        
        // プレイヤー取得
        ServerPlayer player = getPlayer(playerName);
        if (player == null) {
            return encodeResult(false, "incorrect player name/uuid");
        }
        
        // JSON パース
        MutableComponent titleComponent;
        MutableComponent subtitleComponent;
        try {
            titleComponent = Component.Serializer.fromJson(jsonTitle);
            if (titleComponent == null) {
                return encodeResult(false, "incorrect json for title");
            }
            subtitleComponent = Component.Serializer.fromJson(jsonSubtitle);
            if (subtitleComponent == null) {
                return encodeResult(false, "incorrect json for message");
            }
        } catch (Exception e) {
            return encodeResult(false, "incorrect json");
        }
        
        // トーストメッセージ作成
        MutableComponent preparedMessage = appendPrefix(prefix, brackets, color)
                .append(Component.literal("["))
                .append(titleComponent)
                .append(Component.literal("] "))
                .append(subtitleComponent);
        
        // プレイヤーに送信（チャットメッセージとして）
        player.sendSystemMessage(preparedMessage);
        
        return encodeResult(true, null);
    }

    /**
     * プレフィックスを追加。
     * Append prefix with brackets and color.
     */
    private MutableComponent appendPrefix(String prefix, String brackets, String color) {
        // カラーコード変換（& → §）
        String colorCode = color.replace("&", "§");
        
        // プレフィックスコンポーネント作成
        MutableComponent prefixComponent;
        try {
            prefixComponent = Component.Serializer.fromJson(prefix);
            if (prefixComponent == null) {
                prefixComponent = Component.literal(prefix);
            }
        } catch (Exception e) {
            prefixComponent = Component.literal(prefix);
        }
        
        // ブラケット付きプレフィックス作成
        return Component.literal(colorCode + brackets.charAt(0) + "§r")
                .append(prefixComponent)
                .append(colorCode + brackets.charAt(1) + "§r ");
    }

    /**
     * プレイヤー取得（名前またはUUID）。
     * Get player by name or UUID.
     */
    private ServerPlayer getPlayer(String argument) {
        // UUID形式チェック
        if (argument.matches("\\b[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}\\b")) {
            return ServerLifecycleHooks.getCurrentServer().getPlayerList().getPlayer(UUID.fromString(argument));
        }
        // 名前で検索
        return ServerLifecycleHooks.getCurrentServer().getPlayerList().getPlayerByName(argument);
    }

    /**
     * 結果をエンコード。
     * Encode result (success boolean).
     */
    private byte[] encodeResult(boolean success, String errorMessage) throws IOException {
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
        
        if (success) {
            packer.packBoolean(true);
        } else {
            packer.packBoolean(false);
            if (errorMessage != null) {
                packer.packString(errorMessage);
            }
        }
        
        packer.close();
        return packer.toByteArray();
    }
}
