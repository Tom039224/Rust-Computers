package com.rustcomputers.peripheral.impl;

import com.rustcomputers.peripheral.PeripheralException;
import com.rustcomputers.peripheral.PeripheralType;
import net.minecraft.core.BlockPos;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.Vec3;
import net.minecraftforge.server.ServerLifecycleHooks;
import org.msgpack.core.MessageBufferPacker;
import org.msgpack.core.MessagePack;
import org.msgpack.core.MessageUnpacker;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.UUID;

/**
 * AdvancedPeripherals PlayerDetector ペリフェラル実装。
 * AdvancedPeripherals PlayerDetector peripheral implementation.
 *
 * <p>プレイヤーの検出、位置取得、範囲内判定を行う。
 * 球形範囲、座標ボックス、立方体範囲の3種類の検出方法に対応。</p>
 *
 * <p>Detects players, gets their positions, and checks if they are within range.
 * Supports three detection methods: spherical radius, coordinate bounding box, and cubic area.</p>
 *
 * <h3>Methods:</h3>
 * <ul>
 *   <li><b>getOnlinePlayers()</b> - オンラインプレイヤー一覧取得</li>
 *   <li><b>getPlayersInRange(radius)</b> - 球形範囲内のプレイヤー取得</li>
 *   <li><b>getPlayersInCoords(x1,y1,z1,x2,y2,z2)</b> - 座標ボックス内のプレイヤー取得</li>
 *   <li><b>getPlayersInCubic(dx,dy,dz)</b> - 立方体範囲内のプレイヤー取得</li>
 *   <li><b>isPlayersInRange(radius)</b> - 球形範囲内にプレイヤーがいるか判定</li>
 *   <li><b>isPlayersInCoords(x1,y1,z1,x2,y2,z2)</b> - 座標ボックス内にプレイヤーがいるか判定</li>
 *   <li><b>isPlayersInCubic(dx,dy,dz)</b> - 立方体範囲内にプレイヤーがいるか判定</li>
 *   <li><b>isPlayerInRange(player,radius)</b> - 特定プレイヤーが球形範囲内にいるか判定</li>
 *   <li><b>isPlayerInCoords(player,x1,y1,z1,x2,y2,z2)</b> - 特定プレイヤーが座標ボックス内にいるか判定</li>
 *   <li><b>isPlayerInCubic(player,dx,dy,dz)</b> - 特定プレイヤーが立方体範囲内にいるか判定</li>
 *   <li><b>getPlayerPos(player,decimals?)</b> - プレイヤーの位置取得</li>
 *   <li><b>getPlayer(player,decimals?)</b> - プレイヤーの詳細情報取得</li>
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
 * <h3>Query Methods:</h3>
 * <p>全メソッドが情報取得系（Query）。最後のリクエストのみ有効（上書き）。
 * callImmediate 対応。</p>
 */
public class ApPlayerDetectorPeripheral implements PeripheralType {
    private static final Logger LOGGER = LoggerFactory.getLogger(ApPlayerDetectorPeripheral.class);

    private static final String TYPE_NAME = "player_detector";
    
    /**
     * サポートされる全メソッド。
     * All supported methods.
     */
    private static final String[] METHODS = {
            "getOnlinePlayers",       // Query: オンラインプレイヤー一覧
            "getPlayersInRange",      // Query: 球形範囲内のプレイヤー
            "getPlayersInCoords",     // Query: 座標ボックス内のプレイヤー
            "getPlayersInCubic",      // Query: 立方体範囲内のプレイヤー
            "isPlayersInRange",       // Query: 球形範囲内にプレイヤーがいるか
            "isPlayersInCoords",      // Query: 座標ボックス内にプレイヤーがいるか
            "isPlayersInCubic",       // Query: 立方体範囲内にプレイヤーがいるか
            "isPlayerInRange",        // Query: 特定プレイヤーが球形範囲内にいるか
            "isPlayerInCoords",       // Query: 特定プレイヤーが座標ボックス内にいるか
            "isPlayerInCubic",        // Query: 特定プレイヤーが立方体範囲内にいるか
            "getPlayerPos",           // Query: プレイヤーの位置
            "getPlayer"               // Query: プレイヤーの詳細情報
    };

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
                case "getOnlinePlayers" -> executeGetOnlinePlayers();
                case "getPlayersInRange" -> executeGetPlayersInRange(args, level, peripheralPos);
                case "getPlayersInCoords" -> executeGetPlayersInCoords(args, level);
                case "getPlayersInCubic" -> executeGetPlayersInCubic(args, level, peripheralPos);
                case "isPlayersInRange" -> executeIsPlayersInRange(args, level, peripheralPos);
                case "isPlayersInCoords" -> executeIsPlayersInCoords(args, level);
                case "isPlayersInCubic" -> executeIsPlayersInCubic(args, level, peripheralPos);
                case "isPlayerInRange" -> executeIsPlayerInRange(args, level, peripheralPos);
                case "isPlayerInCoords" -> executeIsPlayerInCoords(args, level);
                case "isPlayerInCubic" -> executeIsPlayerInCubic(args, level, peripheralPos);
                case "getPlayerPos" -> executeGetPlayerPos(args, level);
                case "getPlayer" -> executeGetPlayer(args, level);
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
        // 全メソッドが Query なので immediate 対応
        // All methods are queries, so support immediate
        return callMethod(methodName, args, level, peripheralPos);
    }

    /**
     * getOnlinePlayers 実装。
     * オンラインプレイヤーの名前一覧を取得。
     */
    private byte[] executeGetOnlinePlayers() throws IOException {
        List<ServerPlayer> players = ServerLifecycleHooks.getCurrentServer().getPlayerList().getPlayers();
        
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
        packer.packArrayHeader(players.size());
        for (ServerPlayer player : players) {
            packer.packString(player.getName().getString());
        }
        packer.close();
        return packer.toByteArray();
    }

    /**
     * getPlayersInRange 実装。
     * 球形範囲内のプレイヤーを取得。
     */
    private byte[] executeGetPlayersInRange(byte[] args, ServerLevel level, BlockPos peripheralPos) throws IOException {
        MessageUnpacker unpacker = MessagePack.newDefaultUnpacker(args);
        double radius = unpacker.unpackDouble();
        unpacker.close();
        
        Vec3 center = Vec3.atCenterOf(peripheralPos);
        List<ServerPlayer> playersInRange = new ArrayList<>();
        
        for (ServerPlayer player : level.players()) {
            if (player.position().distanceTo(center) <= radius) {
                playersInRange.add(player);
            }
        }
        
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
        packer.packArrayHeader(playersInRange.size());
        for (ServerPlayer player : playersInRange) {
            packer.packString(player.getName().getString());
        }
        packer.close();
        return packer.toByteArray();
    }

    /**
     * getPlayersInCoords 実装。
     * 座標ボックス内のプレイヤーを取得。
     */
    private byte[] executeGetPlayersInCoords(byte[] args, ServerLevel level) throws IOException {
        MessageUnpacker unpacker = MessagePack.newDefaultUnpacker(args);
        double x1 = unpacker.unpackDouble();
        double y1 = unpacker.unpackDouble();
        double z1 = unpacker.unpackDouble();
        double x2 = unpacker.unpackDouble();
        double y2 = unpacker.unpackDouble();
        double z2 = unpacker.unpackDouble();
        unpacker.close();
        
        AABB box = new AABB(
            Math.min(x1, x2), Math.min(y1, y2), Math.min(z1, z2),
            Math.max(x1, x2), Math.max(y1, y2), Math.max(z1, z2)
        );
        
        List<ServerPlayer> playersInBox = new ArrayList<>();
        for (ServerPlayer player : level.players()) {
            if (box.contains(player.position())) {
                playersInBox.add(player);
            }
        }
        
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
        packer.packArrayHeader(playersInBox.size());
        for (ServerPlayer player : playersInBox) {
            packer.packString(player.getName().getString());
        }
        packer.close();
        return packer.toByteArray();
    }

    /**
     * getPlayersInCubic 実装。
     * 立方体範囲内のプレイヤーを取得。
     */
    private byte[] executeGetPlayersInCubic(byte[] args, ServerLevel level, BlockPos peripheralPos) throws IOException {
        MessageUnpacker unpacker = MessagePack.newDefaultUnpacker(args);
        double dx = unpacker.unpackDouble();
        double dy = unpacker.unpackDouble();
        double dz = unpacker.unpackDouble();
        unpacker.close();
        
        Vec3 center = Vec3.atCenterOf(peripheralPos);
        AABB box = new AABB(
            center.x - dx, center.y - dy, center.z - dz,
            center.x + dx, center.y + dy, center.z + dz
        );
        
        List<ServerPlayer> playersInBox = new ArrayList<>();
        for (ServerPlayer player : level.players()) {
            if (box.contains(player.position())) {
                playersInBox.add(player);
            }
        }
        
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
        packer.packArrayHeader(playersInBox.size());
        for (ServerPlayer player : playersInBox) {
            packer.packString(player.getName().getString());
        }
        packer.close();
        return packer.toByteArray();
    }

    /**
     * isPlayersInRange 実装。
     * 球形範囲内にプレイヤーがいるか判定。
     */
    private byte[] executeIsPlayersInRange(byte[] args, ServerLevel level, BlockPos peripheralPos) throws IOException {
        MessageUnpacker unpacker = MessagePack.newDefaultUnpacker(args);
        double radius = unpacker.unpackDouble();
        unpacker.close();
        
        Vec3 center = Vec3.atCenterOf(peripheralPos);
        boolean found = false;
        
        for (ServerPlayer player : level.players()) {
            if (player.position().distanceTo(center) <= radius) {
                found = true;
                break;
            }
        }
        
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
        packer.packBoolean(found);
        packer.close();
        return packer.toByteArray();
    }

    /**
     * isPlayersInCoords 実装。
     * 座標ボックス内にプレイヤーがいるか判定。
     */
    private byte[] executeIsPlayersInCoords(byte[] args, ServerLevel level) throws IOException {
        MessageUnpacker unpacker = MessagePack.newDefaultUnpacker(args);
        double x1 = unpacker.unpackDouble();
        double y1 = unpacker.unpackDouble();
        double z1 = unpacker.unpackDouble();
        double x2 = unpacker.unpackDouble();
        double y2 = unpacker.unpackDouble();
        double z2 = unpacker.unpackDouble();
        unpacker.close();
        
        AABB box = new AABB(
            Math.min(x1, x2), Math.min(y1, y2), Math.min(z1, z2),
            Math.max(x1, x2), Math.max(y1, y2), Math.max(z1, z2)
        );
        
        boolean found = false;
        for (ServerPlayer player : level.players()) {
            if (box.contains(player.position())) {
                found = true;
                break;
            }
        }
        
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
        packer.packBoolean(found);
        packer.close();
        return packer.toByteArray();
    }

    /**
     * isPlayersInCubic 実装。
     * 立方体範囲内にプレイヤーがいるか判定。
     */
    private byte[] executeIsPlayersInCubic(byte[] args, ServerLevel level, BlockPos peripheralPos) throws IOException {
        MessageUnpacker unpacker = MessagePack.newDefaultUnpacker(args);
        double dx = unpacker.unpackDouble();
        double dy = unpacker.unpackDouble();
        double dz = unpacker.unpackDouble();
        unpacker.close();
        
        Vec3 center = Vec3.atCenterOf(peripheralPos);
        AABB box = new AABB(
            center.x - dx, center.y - dy, center.z - dz,
            center.x + dx, center.y + dy, center.z + dz
        );
        
        boolean found = false;
        for (ServerPlayer player : level.players()) {
            if (box.contains(player.position())) {
                found = true;
                break;
            }
        }
        
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
        packer.packBoolean(found);
        packer.close();
        return packer.toByteArray();
    }

    /**
     * isPlayerInRange 実装。
     * 特定プレイヤーが球形範囲内にいるか判定。
     */
    private byte[] executeIsPlayerInRange(byte[] args, ServerLevel level, BlockPos peripheralPos) throws IOException {
        MessageUnpacker unpacker = MessagePack.newDefaultUnpacker(args);
        String playerName = unpacker.unpackString();
        double radius = unpacker.unpackDouble();
        unpacker.close();
        
        ServerPlayer player = getPlayer(playerName);
        if (player == null) {
            MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
            packer.packBoolean(false);
            packer.close();
            return packer.toByteArray();
        }
        
        Vec3 center = Vec3.atCenterOf(peripheralPos);
        boolean inRange = player.position().distanceTo(center) <= radius;
        
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
        packer.packBoolean(inRange);
        packer.close();
        return packer.toByteArray();
    }

    /**
     * isPlayerInCoords 実装。
     * 特定プレイヤーが座標ボックス内にいるか判定。
     */
    private byte[] executeIsPlayerInCoords(byte[] args, ServerLevel level) throws IOException {
        MessageUnpacker unpacker = MessagePack.newDefaultUnpacker(args);
        String playerName = unpacker.unpackString();
        double x1 = unpacker.unpackDouble();
        double y1 = unpacker.unpackDouble();
        double z1 = unpacker.unpackDouble();
        double x2 = unpacker.unpackDouble();
        double y2 = unpacker.unpackDouble();
        double z2 = unpacker.unpackDouble();
        unpacker.close();
        
        ServerPlayer player = getPlayer(playerName);
        if (player == null) {
            MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
            packer.packBoolean(false);
            packer.close();
            return packer.toByteArray();
        }
        
        AABB box = new AABB(
            Math.min(x1, x2), Math.min(y1, y2), Math.min(z1, z2),
            Math.max(x1, x2), Math.max(y1, y2), Math.max(z1, z2)
        );
        
        boolean inBox = box.contains(player.position());
        
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
        packer.packBoolean(inBox);
        packer.close();
        return packer.toByteArray();
    }

    /**
     * isPlayerInCubic 実装。
     * 特定プレイヤーが立方体範囲内にいるか判定。
     */
    private byte[] executeIsPlayerInCubic(byte[] args, ServerLevel level, BlockPos peripheralPos) throws IOException {
        MessageUnpacker unpacker = MessagePack.newDefaultUnpacker(args);
        String playerName = unpacker.unpackString();
        double dx = unpacker.unpackDouble();
        double dy = unpacker.unpackDouble();
        double dz = unpacker.unpackDouble();
        unpacker.close();
        
        ServerPlayer player = getPlayer(playerName);
        if (player == null) {
            MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
            packer.packBoolean(false);
            packer.close();
            return packer.toByteArray();
        }
        
        Vec3 center = Vec3.atCenterOf(peripheralPos);
        AABB box = new AABB(
            center.x - dx, center.y - dy, center.z - dz,
            center.x + dx, center.y + dy, center.z + dz
        );
        
        boolean inBox = box.contains(player.position());
        
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
        packer.packBoolean(inBox);
        packer.close();
        return packer.toByteArray();
    }

    /**
     * getPlayerPos 実装。
     * プレイヤーの位置を取得。
     */
    private byte[] executeGetPlayerPos(byte[] args, ServerLevel level) throws IOException, PeripheralException {
        MessageUnpacker unpacker = MessagePack.newDefaultUnpacker(args);
        String playerName = unpacker.unpackString();
        
        // decimals パラメータ（オプション）
        int decimals = 2; // デフォルト
        if (unpacker.hasNext()) {
            decimals = unpacker.unpackInt();
        }
        unpacker.close();
        
        ServerPlayer player = getPlayer(playerName);
        if (player == null) {
            throw new PeripheralException("Player not found: " + playerName);
        }
        
        Vec3 pos = player.position();
        
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
        packer.packMapHeader(3);
        
        packer.packString("x");
        packer.packDouble(round(pos.x, decimals));
        
        packer.packString("y");
        packer.packDouble(round(pos.y, decimals));
        
        packer.packString("z");
        packer.packDouble(round(pos.z, decimals));
        
        packer.close();
        return packer.toByteArray();
    }

    /**
     * getPlayer 実装。
     * プレイヤーの詳細情報を取得。
     */
    private byte[] executeGetPlayer(byte[] args, ServerLevel level) throws IOException, PeripheralException {
        MessageUnpacker unpacker = MessagePack.newDefaultUnpacker(args);
        String playerName = unpacker.unpackString();
        
        // decimals パラメータ（オプション）
        int decimals = 2; // デフォルト
        if (unpacker.hasNext()) {
            decimals = unpacker.unpackInt();
        }
        unpacker.close();
        
        ServerPlayer player = getPlayer(playerName);
        if (player == null) {
            throw new PeripheralException("Player not found: " + playerName);
        }
        
        Vec3 pos = player.position();
        
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
        packer.packMapHeader(13);
        
        packer.packString("x");
        packer.packDouble(round(pos.x, decimals));
        
        packer.packString("y");
        packer.packDouble(round(pos.y, decimals));
        
        packer.packString("z");
        packer.packDouble(round(pos.z, decimals));
        
        packer.packString("name");
        packer.packString(player.getName().getString());
        
        packer.packString("uuid");
        packer.packString(player.getUUID().toString());
        
        packer.packString("health");
        packer.packFloat(player.getHealth());
        
        packer.packString("max_health");
        packer.packFloat(player.getMaxHealth());
        
        packer.packString("is_flying");
        packer.packBoolean(player.getAbilities().flying);
        
        packer.packString("is_sprinting");
        packer.packBoolean(player.isSprinting());
        
        packer.packString("is_sneaking");
        packer.packBoolean(player.isCrouching());
        
        packer.packString("game_mode");
        packer.packString(player.gameMode.getGameModeForPlayer().getName());
        
        packer.packString("experience");
        packer.packInt(player.totalExperience);
        
        packer.packString("level");
        packer.packInt(player.experienceLevel);
        
        packer.close();
        return packer.toByteArray();
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
     * 数値を指定桁数で丸める。
     * Round a number to specified decimal places.
     */
    private double round(double value, int decimals) {
        double scale = Math.pow(10, decimals);
        return Math.round(value * scale) / scale;
    }
}
