package com.rustcomputers.peripheral.impl;

import com.rustcomputers.peripheral.PeripheralException;
import com.rustcomputers.peripheral.PeripheralType;
import net.minecraft.core.BlockPos;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.level.block.state.BlockState;
import org.msgpack.core.MessageBufferPacker;
import org.msgpack.core.MessagePack;
import org.msgpack.core.MessageUnpacker;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.*;

/**
 * AdvancedPeripherals GeoScanner peripheral implementation.
 * Provides block scanning and chunk ore analysis functionality.
 *
 * <p>Methods:
 * <ul>
 *   <li><b>cost(radius)</b> - Get fuel cost for scan (IMM)</li>
 *   <li><b>scan(radius)</b> - Scan blocks within radius</li>
 *   <li><b>chunkAnalyze()</b> - Analyze ore distribution in current chunk</li>
 * </ul>
 */
public class ApGeoScannerPeripheral implements PeripheralType {

    private static final Logger LOGGER = LoggerFactory.getLogger(ApGeoScannerPeripheral.class);
    private static final String TYPE_NAME = "geo_scanner";

    private static final String[] METHODS = {
            "cost",          // Query (IMM): Get fuel cost
            "scan",          // Query: Scan blocks
            "chunkAnalyze"   // Query: Analyze chunk ores
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
                case "cost" -> {
                    double radius = unpackRadius(args);
                    yield encodeCost(calculateCost(radius));
                }
                case "scan" -> {
                    double radius = unpackRadius(args);
                    yield encodeScanResults(scanBlocks(level, peripheralPos, radius));
                }
                case "chunkAnalyze" -> encodeChunkAnalysis(analyzeChunk(level, peripheralPos));
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
        // Only 'cost' is immediate
        if ("cost".equals(methodName)) {
            return callMethod(methodName, args, level, peripheralPos);
        }
        return null;
    }

    /**
     * Unpack radius from MessagePack args.
     */
    private double unpackRadius(byte[] args) throws PeripheralException {
        if (args == null || args.length == 0) {
            throw new PeripheralException("Missing radius argument");
        }

        try (MessageUnpacker unpacker = MessagePack.newDefaultUnpacker(args)) {
            if (unpacker.hasNext()) {
                return unpacker.unpackDouble();
            }
            throw new PeripheralException("Missing radius argument");
        } catch (IOException e) {
            throw new PeripheralException("Failed to unpack radius: " + e.getMessage());
        }
    }

    /**
     * Calculate fuel cost for a scan at the given radius.
     */
    private double calculateCost(double radius) {
        // Cost formula: base cost + radius-based cost
        return 10.0 + (radius * radius * 0.5);
    }

    /**
     * Encode cost result.
     */
    private byte[] encodeCost(double cost) throws IOException {
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
        packer.packDouble(cost);
        packer.close();
        return packer.toByteArray();
    }

    /**
     * Scan blocks within the given radius.
     */
    private List<Map<String, Object>> scanBlocks(ServerLevel level, BlockPos center, double radius) {
        List<Map<String, Object>> results = new ArrayList<>();
        int r = (int) Math.ceil(radius);

        for (int dx = -r; dx <= r; dx++) {
            for (int dy = -r; dy <= r; dy++) {
                for (int dz = -r; dz <= r; dz++) {
                    double distance = Math.sqrt(dx * dx + dy * dy + dz * dz);
                    if (distance > radius) continue;

                    BlockPos scanPos = center.offset(dx, dy, dz);
                    BlockState scanState = level.getBlockState(scanPos);

                    if (scanState.isAir()) continue;

                    Map<String, Object> entry = new HashMap<>();
                    entry.put("x", (double) dx);
                    entry.put("y", (double) dy);
                    entry.put("z", (double) dz);
                    entry.put("name", BuiltInRegistries.BLOCK.getKey(scanState.getBlock()).toString());

                    // Get block tags
                    List<String> tags = new ArrayList<>();
                    scanState.getTags().forEach(tag -> tags.add(tag.location().toString()));
                    entry.put("tags", tags);

                    results.add(entry);
                }
            }
        }

        return results;
    }

    /**
     * Encode scan results.
     */
    private byte[] encodeScanResults(List<Map<String, Object>> results) throws IOException {
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();

        packer.packArrayHeader(results.size());
        for (Map<String, Object> entry : results) {
            packer.packMapHeader(entry.size());
            for (Map.Entry<String, Object> kv : entry.entrySet()) {
                packer.packString(kv.getKey());
                packValue(packer, kv.getValue());
            }
        }

        packer.close();
        return packer.toByteArray();
    }

    /**
     * Analyze ore distribution in the current chunk.
     */
    private Map<String, Integer> analyzeChunk(ServerLevel level, BlockPos center) {
        Map<String, Integer> oreCount = new HashMap<>();

        // Get chunk coordinates
        int chunkX = center.getX() >> 4;
        int chunkZ = center.getZ() >> 4;

        // Scan entire chunk (16x16 horizontal, full height)
        int startX = chunkX << 4;
        int startZ = chunkZ << 4;

        for (int x = startX; x < startX + 16; x++) {
            for (int z = startZ; z < startZ + 16; z++) {
                for (int y = level.getMinBuildHeight(); y < level.getMaxBuildHeight(); y++) {
                    BlockPos scanPos = new BlockPos(x, y, z);
                    BlockState scanState = level.getBlockState(scanPos);

                    if (scanState.isAir()) continue;

                    String blockName = BuiltInRegistries.BLOCK.getKey(scanState.getBlock()).toString();

                    // Check if block is an ore
                    boolean isOre = blockName.toLowerCase().contains("ore")
                            || scanState.getTags().anyMatch(tag -> tag.location().toString().contains("ore"));

                    if (isOre) {
                        oreCount.merge(blockName, 1, Integer::sum);
                    }
                }
            }
        }

        return oreCount;
    }

    /**
     * Encode chunk analysis results.
     */
    private byte[] encodeChunkAnalysis(Map<String, Integer> oreCount) throws IOException {
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();

        packer.packMapHeader(oreCount.size());
        for (Map.Entry<String, Integer> entry : oreCount.entrySet()) {
            packer.packString(entry.getKey());
            packer.packInt(entry.getValue());
        }

        packer.close();
        return packer.toByteArray();
    }

    /**
     * Pack a value to MessagePack.
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
        } else if (value instanceof List) {
            List<?> list = (List<?>) value;
            packer.packArrayHeader(list.size());
            for (Object item : list) {
                packValue(packer, item);
            }
        } else {
            packer.packString(value.toString());
        }
    }
}
