package com.rustcomputers.peripheral.impl;

import com.rustcomputers.peripheral.PeripheralException;
import com.rustcomputers.peripheral.PeripheralType;
import net.minecraft.core.BlockPos;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.level.block.entity.BlockEntity;
import org.msgpack.core.MessageBufferPacker;
import org.msgpack.core.MessagePack;
import org.msgpack.core.MessageUnpacker;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.lang.reflect.Method;
import java.util.Set;

/**
 * Tom's Peripherals RedstonePort peripheral implementation.
 *
 * <p>Uses reflection to read from {@code RedstonePortBlockEntity} since
 * Toms-Peripherals is not a compile-time dependency.</p>
 */
public class TmRedstonePortPeripheral implements PeripheralType {

    private static final Logger LOGGER = LoggerFactory.getLogger(TmRedstonePortPeripheral.class);
    private static final String TYPE_NAME = "tm_rsPort";

    private static final String[] METHODS = {
        "getInput", "getAnalogInput", "getBundledInput",
        "getOutput", "getAnalogOutput", "getBundledOutput",
        "setOutput", "setAnalogOutput", "setBundledOutput", "testBundledInput"
    };

    private static final Set<String> IMM_METHODS = Set.of(
        "getInput", "getOutput", "getAnalogOutput", "getBundledOutput"
    );

    // Direction ordinals matching Minecraft's Direction enum
    private static final String[] SIDE_NAMES = {"down", "up", "north", "south", "west", "east"};

    @Override
    public String getTypeName() { return TYPE_NAME; }

    @Override
    public String[] getMethodNames() { return METHODS.clone(); }

    @Override
    public byte[] callMethod(String methodName, byte[] args,
                             ServerLevel level, BlockPos pos) throws PeripheralException {
        try {
            return switch (methodName) {
                case "getInput"         -> encodeBool(readInput(level, pos, args) > 0);
                case "getAnalogInput"   -> encodeInt(readInput(level, pos, args));
                case "getBundledInput"  -> encodeInt(readBundledInput(level, pos, args));
                case "getOutput"        -> encodeBool(readOutput(level, pos, args) > 0);
                case "getAnalogOutput"  -> encodeInt(readOutput(level, pos, args));
                case "getBundledOutput" -> encodeInt(readBundledOutput(level, pos, args));
                case "testBundledInput" -> encodeBool(testBundledInput(level, pos, args));
                // setters: no return value
                default -> encodeNil();
            };
        } catch (IOException e) {
            LOGGER.error("TmRedstonePortPeripheral: encode failed for '{}'", methodName, e);
            throw new PeripheralException("Failed to encode result: " + e.getMessage());
        }
    }

    @Override
    public byte[] callImmediate(String methodName, byte[] args,
                                ServerLevel level, BlockPos pos) throws PeripheralException {
        if (IMM_METHODS.contains(methodName)) {
            return callMethod(methodName, args, level, pos);
        }
        return null;
    }

    // ------------------------------------------------------------------
    // Reflection helpers
    // ------------------------------------------------------------------

    private int readInput(ServerLevel level, BlockPos pos, byte[] args) {
        int dir = parseSideArg(args);
        BlockEntity be = level.getBlockEntity(pos);
        if (be == null) return 0;
        try {
            Method m = be.getClass().getMethod("getInput", net.minecraft.core.Direction.class);
            return ((Number) m.invoke(be, directionFromOrdinal(dir))).intValue();
        } catch (Exception e) {
            LOGGER.warn("TmRedstonePortPeripheral: getInput reflection failed: {}", e.getMessage());
            return 0;
        }
    }

    private int readBundledInput(ServerLevel level, BlockPos pos, byte[] args) {
        int dir = parseSideArg(args);
        BlockEntity be = level.getBlockEntity(pos);
        if (be == null) return 0;
        try {
            Method m = be.getClass().getMethod("getBundledInput", net.minecraft.core.Direction.class);
            return ((Number) m.invoke(be, directionFromOrdinal(dir))).intValue();
        } catch (Exception e) {
            LOGGER.warn("TmRedstonePortPeripheral: getBundledInput reflection failed: {}", e.getMessage());
            return 0;
        }
    }

    private int readOutput(ServerLevel level, BlockPos pos, byte[] args) {
        int dir = parseSideArg(args);
        BlockEntity be = level.getBlockEntity(pos);
        if (be == null) return 0;
        try {
            Method m = be.getClass().getMethod("getOutput", net.minecraft.core.Direction.class);
            return ((Number) m.invoke(be, directionFromOrdinal(dir))).intValue();
        } catch (Exception e) {
            LOGGER.warn("TmRedstonePortPeripheral: getOutput reflection failed: {}", e.getMessage());
            return 0;
        }
    }

    private int readBundledOutput(ServerLevel level, BlockPos pos, byte[] args) {
        int dir = parseSideArg(args);
        BlockEntity be = level.getBlockEntity(pos);
        if (be == null) return 0;
        try {
            Method m = be.getClass().getMethod("getBundledOutput", net.minecraft.core.Direction.class);
            return ((Number) m.invoke(be, directionFromOrdinal(dir))).intValue();
        } catch (Exception e) {
            LOGGER.warn("TmRedstonePortPeripheral: getBundledOutput reflection failed: {}", e.getMessage());
            return 0;
        }
    }

    private boolean testBundledInput(ServerLevel level, BlockPos pos, byte[] args) {
        if (args == null || args.length == 0) return false;
        try (MessageUnpacker u = MessagePack.newDefaultUnpacker(args)) {
            if (u.hasNext()) {
                var fmt = u.getNextFormat();
                if (fmt.getValueType().isArrayType()) u.unpackArrayHeader();
            }
            String side = u.unpackString();
            int mask = u.unpackInt();
            int dir = sideNameToOrdinal(side);
            int bundled = readBundledInput(level, pos, encodeSideArg(dir));
            return (bundled & mask) == mask;
        } catch (IOException e) {
            return false;
        }
    }

    // ------------------------------------------------------------------
    // Arg parsing
    // ------------------------------------------------------------------

    private int parseSideArg(byte[] args) {
        if (args == null || args.length == 0) return 0;
        try (MessageUnpacker u = MessagePack.newDefaultUnpacker(args)) {
            if (u.hasNext()) {
                var fmt = u.getNextFormat();
                if (fmt.getValueType().isArrayType()) u.unpackArrayHeader();
            }
            if (!u.hasNext()) return 0;
            String side = u.unpackString();
            return sideNameToOrdinal(side);
        } catch (IOException e) {
            return 0;
        }
    }

    private byte[] encodeSideArg(int ordinal) {
        try {
            MessageBufferPacker p = MessagePack.newDefaultBufferPacker();
            p.packArrayHeader(1);
            p.packString(SIDE_NAMES[Math.max(0, Math.min(ordinal, 5))]);
            p.close();
            return p.toByteArray();
        } catch (IOException e) {
            return new byte[0];
        }
    }

    private int sideNameToOrdinal(String name) {
        for (int i = 0; i < SIDE_NAMES.length; i++) {
            if (SIDE_NAMES[i].equalsIgnoreCase(name)) return i;
        }
        return 0;
    }

    private net.minecraft.core.Direction directionFromOrdinal(int ordinal) {
        return net.minecraft.core.Direction.values()[Math.max(0, Math.min(ordinal, 5))];
    }

    // ------------------------------------------------------------------
    // Encoders
    // ------------------------------------------------------------------

    private byte[] encodeBool(boolean v) throws IOException {
        MessageBufferPacker p = MessagePack.newDefaultBufferPacker();
        p.packBoolean(v);
        p.close();
        return p.toByteArray();
    }

    private byte[] encodeInt(int v) throws IOException {
        MessageBufferPacker p = MessagePack.newDefaultBufferPacker();
        p.packInt(v);
        p.close();
        return p.toByteArray();
    }

    private byte[] encodeNil() throws IOException {
        MessageBufferPacker p = MessagePack.newDefaultBufferPacker();
        p.packNil();
        p.close();
        return p.toByteArray();
    }
}
