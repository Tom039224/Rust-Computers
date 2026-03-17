package com.rustcomputers.peripheral.impl;

import com.rustcomputers.peripheral.PeripheralException;
import com.rustcomputers.peripheral.PeripheralType;
import net.minecraft.core.BlockPos;
import net.minecraft.core.Direction;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.level.block.entity.BlockEntity;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.level.block.state.properties.Property;
import org.msgpack.core.MessageBufferPacker;
import org.msgpack.core.MessagePack;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.HashMap;
import java.util.Map;

/**
 * AdvancedPeripherals BlockReader ペリフェラル実装。
 * AdvancedPeripherals BlockReader peripheral implementation.
 *
 * <p>ペリフェラルの前方にあるブロックの情報を読み取る。
 * ブロック名、NBTデータ、ブロックステート、タイルエンティティ検出に対応。</p>
 *
 * <p>Reads information about the block in front of the peripheral.
 * Supports block name, NBT data, block states, and tile entity detection.</p>
 *
 * <h3>Methods:</h3>
 * <ul>
 *   <li><b>getBlockName()</b> - ブロックのリソースID取得 / Get block resource ID</li>
 *   <li><b>getBlockData()</b> - ブロックのNBTデータ取得 / Get block NBT data</li>
 *   <li><b>getBlockStates()</b> - ブロックステートプロパティ取得 / Get block state properties</li>
 *   <li><b>isTileEntity()</b> - タイルエンティティ判定 / Check if block is a tile entity</li>
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
public class ApBlockReaderPeripheral implements PeripheralType {
    private static final Logger LOGGER = LoggerFactory.getLogger(ApBlockReaderPeripheral.class);

    private static final String TYPE_NAME = "block_reader";
    
    /**
     * サポートされる全メソッド。
     * All supported methods.
     */
    private static final String[] METHODS = {
            "getBlockName",    // Query: ブロック名取得 / Get block name
            "getBlockData",    // Query: NBTデータ取得 / Get NBT data
            "getBlockStates",  // Query: ブロックステート取得 / Get block states
            "isTileEntity"     // Query: タイルエンティティ判定 / Check if tile entity
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
        
        // BlockReader は方向を持つブロックなので、facing を取得
        // BlockReader is a directional block, so get the facing direction
        BlockState peripheralState = level.getBlockState(peripheralPos);
        Direction facing = getFacing(peripheralState);
        
        // 前方のブロック位置を計算
        // Calculate the position of the block in front
        BlockPos targetPos = peripheralPos.relative(facing);
        BlockState targetState = level.getBlockState(targetPos);
        
        try {
            return switch (methodName) {
                case "getBlockName" -> encodeBlockName(targetState);
                case "getBlockData" -> encodeBlockData(level, targetPos, targetState);
                case "getBlockStates" -> encodeBlockStates(targetState);
                case "isTileEntity" -> encodeTileEntity(level, targetPos, targetState);
                default -> throw new PeripheralException("Unknown method: " + methodName);
            };
        } catch (IOException e) {
            LOGGER.error("Failed to encode result for method '{}'", methodName, e);
            throw new PeripheralException("Failed to encode result: " + e.getMessage());
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
     * ペリフェラルブロックの向きを取得。
     * Get the facing direction of the peripheral block.
     */
    private Direction getFacing(BlockState state) {
        // AdvancedPeripherals の BlockReader は FACING プロパティを持つ
        // AdvancedPeripherals BlockReader has a FACING property
        try {
            Property<?> facingProp = state.getBlock().getStateDefinition()
                    .getProperty("facing");
            if (facingProp != null) {
                Object value = state.getValue(facingProp);
                if (value instanceof Direction) {
                    return (Direction) value;
                }
            }
        } catch (Exception e) {
            LOGGER.debug("Failed to get facing direction, defaulting to NORTH", e);
        }
        
        // デフォルトは北向き
        // Default to north
        return Direction.NORTH;
    }

    /**
     * ブロック名をエンコード。
     * Encode block name.
     */
    private byte[] encodeBlockName(BlockState state) throws IOException {
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
        
        if (state.is(Blocks.AIR)) {
            packer.packString("none");
        } else {
            String blockName = BuiltInRegistries.BLOCK.getKey(state.getBlock()).toString();
            packer.packString(blockName);
        }
        
        packer.close();
        return packer.toByteArray();
    }

    /**
     * ブロックのNBTデータをエンコード。
     * Encode block NBT data.
     * 
     * Note: NBT data extraction is complex and version-dependent.
     * For now, we return a minimal representation.
     */
    private byte[] encodeBlockData(ServerLevel level, BlockPos pos, BlockState state) throws IOException {
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
        
        if (state.is(Blocks.AIR)) {
            packer.packNil();
        } else {
            BlockEntity blockEntity = level.getBlockEntity(pos);
            if (blockEntity == null) {
                packer.packNil();
            } else {
                // BlockEntity の基本情報を返す
                // Return basic BlockEntity information
                packer.packMapHeader(3);
                
                packer.packString("id");
                String id = BuiltInRegistries.BLOCK_ENTITY_TYPE.getKey(blockEntity.getType()).toString();
                packer.packString(id);
                
                packer.packString("x");
                packer.packInt(pos.getX());
                
                packer.packString("z");
                packer.packInt(pos.getZ());
            }
        }
        
        packer.close();
        return packer.toByteArray();
    }

    /**
     * ブロックステートをエンコード。
     * Encode block states.
     */
    private byte[] encodeBlockStates(BlockState state) throws IOException {
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
        
        if (state.is(Blocks.AIR)) {
            packer.packNil();
        } else {
            Map<String, Object> states = new HashMap<>();
            for (Property<?> property : state.getProperties()) {
                String key = property.getName();
                Object value = stateValueToObject(state.getValue(property));
                states.put(key, value);
            }
            
            packer.packMapHeader(states.size());
            for (Map.Entry<String, Object> entry : states.entrySet()) {
                packer.packString(entry.getKey());
                packValue(packer, entry.getValue());
            }
        }
        
        packer.close();
        return packer.toByteArray();
    }

    /**
     * タイルエンティティ判定をエンコード。
     * Encode tile entity check.
     */
    private byte[] encodeTileEntity(ServerLevel level, BlockPos pos, BlockState state) throws IOException {
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
        
        if (state.is(Blocks.AIR)) {
            packer.packBoolean(false);
        } else {
            BlockEntity blockEntity = level.getBlockEntity(pos);
            packer.packBoolean(blockEntity != null);
        }
        
        packer.close();
        return packer.toByteArray();
    }

    /**
     * ブロックステートの値をオブジェクトに変換。
     * Convert block state value to object.
     */
    private Object stateValueToObject(Comparable<?> value) {
        if (value instanceof Boolean) {
            return value;
        } else if (value instanceof Integer) {
            return value;
        } else if (value instanceof Enum<?>) {
            return ((Enum<?>) value).name().toLowerCase();
        } else {
            return value.toString();
        }
    }

    /**
     * 値をMessagePackにエンコード。
     * Encode value to MessagePack.
     */
    private void packValue(MessageBufferPacker packer, Object value) throws IOException {
        if (value == null) {
            packer.packNil();
        } else if (value instanceof Boolean) {
            packer.packBoolean((Boolean) value);
        } else if (value instanceof Integer) {
            packer.packInt((Integer) value);
        } else if (value instanceof Long) {
            packer.packLong((Long) value);
        } else if (value instanceof Float) {
            packer.packFloat((Float) value);
        } else if (value instanceof Double) {
            packer.packDouble((Double) value);
        } else if (value instanceof String) {
            packer.packString((String) value);
        } else {
            packer.packString(value.toString());
        }
    }
}
