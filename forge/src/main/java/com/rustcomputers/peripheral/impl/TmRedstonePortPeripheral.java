package com.rustcomputers.peripheral.impl;

import com.rustcomputers.peripheral.PeripheralException;
import com.rustcomputers.peripheral.PeripheralType;
import dan200.computercraft.api.lua.LuaFunction;
import dan200.computercraft.api.peripheral.IPeripheral;
import net.minecraft.core.BlockPos;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.level.block.entity.BlockEntity;
import org.msgpack.core.MessageBufferPacker;
import org.msgpack.core.MessagePack;
import org.msgpack.core.MessageUnpacker;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.annotation.Nullable;
import java.io.IOException;
import java.lang.reflect.Method;
import java.util.*;

/**
 * Tom's Peripherals RedstonePort peripheral implementation.
 *
 * <p>Delegates to {@code RedstonePortBlockEntity.getPeripheral()} (IPeripheral) via reflection
 * since Toms-Peripherals is not a compile-time dependency.</p>
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
                // Delegate setters to actual peripheral
                default -> delegateToPeripheral(methodName, args, level, pos);
            };
        } catch (IOException e) {
            LOGGER.error("TmRedstonePortPeripheral: encode failed for '{}'", methodName, e);
            throw new PeripheralException("Failed to encode result: " + e.getMessage());
        }
    }

    // ------------------------------------------------------------------
    // Delegate to actual RedstonePort peripheral via reflection
    // ------------------------------------------------------------------

    private byte[] delegateToPeripheral(String methodName, byte[] args,
                                       ServerLevel level, BlockPos pos) throws IOException {
        IPeripheral peripheral = getPeripheral(level, pos);
        if (peripheral == null) {
            LOGGER.warn("TmRedstonePortPeripheral: no peripheral at {}", pos);
            return encodeNil();
        }

        try {
            Method method = findLuaMethod(peripheral, methodName);
            if (method == null) {
                LOGGER.debug("TmRedstonePortPeripheral: @LuaFunction '{}' not found", methodName);
                return encodeNil();
            }

            Object[] javaArgs = decodeMsgpackArgs(args, method);
            Object result = method.invoke(peripheral, javaArgs);
            return encodeResult(result);
        } catch (Exception e) {
            LOGGER.warn("TmRedstonePortPeripheral: callMethod '{}' failed: {}", methodName, e.getMessage());
            return encodeNil();
        }
    }

    @Nullable
    private IPeripheral getPeripheral(ServerLevel level, BlockPos pos) {
        BlockEntity be = level.getBlockEntity(pos);
        if (be == null) return null;
        try {
            Method getPeripheral = be.getClass().getMethod("getPeripheral");
            return (IPeripheral) getPeripheral.invoke(be);
        } catch (Exception e) {
            LOGGER.debug("TmRedstonePortPeripheral: getPeripheral() failed: {}", e.getMessage());
            return null;
        }
    }

    @Nullable
    private Method findLuaMethod(IPeripheral peripheral, String methodName) {
        Class<?> cls = peripheral.getClass();
        while (cls != null && cls != Object.class) {
            for (Method m : cls.getDeclaredMethods()) {
                LuaFunction ann = m.getAnnotation(LuaFunction.class);
                if (ann == null) continue;
                String[] names = ann.value();
                if (names.length > 0) {
                    for (String name : names) {
                        if (name.equals(methodName)) {
                            m.setAccessible(true);
                            return m;
                        }
                    }
                } else if (m.getName().equals(methodName)) {
                    m.setAccessible(true);
                    return m;
                }
            }
            cls = cls.getSuperclass();
        }
        return null;
    }

    private Object[] decodeMsgpackArgs(byte[] data, Method method) {
        Class<?>[] paramTypes = method.getParameterTypes();
        if (paramTypes.length == 0) return new Object[0];

        List<Object> rawArgs = decodeMsgpackToList(data);
        Object[] result = new Object[paramTypes.length];
        for (int i = 0; i < paramTypes.length && i < rawArgs.size(); i++) {
            result[i] = coerce(rawArgs.get(i), paramTypes[i]);
        }
        return result;
    }

    private List<Object> decodeMsgpackToList(byte[] data) {
        List<Object> list = new ArrayList<>();
        if (data == null || data.length == 0) return list;
        try (MessageUnpacker u = MessagePack.newDefaultUnpacker(data)) {
            if (!u.hasNext()) return list;
            var fmt = u.getNextFormat();
            if (fmt.getValueType().isArrayType()) {
                int count = u.unpackArrayHeader();
                for (int i = 0; i < count && u.hasNext(); i++) {
                    list.add(unpackValue(u));
                }
            } else {
                list.add(unpackValue(u, fmt));
            }
        } catch (IOException e) {
            LOGGER.debug("TmRedstonePortPeripheral: failed to decode args: {}", e.getMessage());
        }
        return list;
    }

    private Object unpackValue(MessageUnpacker u) throws IOException {
        return unpackValue(u, u.getNextFormat());
    }

    private Object unpackValue(MessageUnpacker u, org.msgpack.core.MessageFormat fmt) throws IOException {
        return switch (fmt.getValueType()) {
            case NIL -> { u.unpackNil(); yield null; }
            case BOOLEAN -> u.unpackBoolean();
            case INTEGER -> u.unpackLong();
            case FLOAT -> u.unpackDouble();
            case STRING -> u.unpackString();
            case ARRAY -> {
                int n = u.unpackArrayHeader();
                List<Object> arr = new ArrayList<>(n);
                for (int i = 0; i < n; i++) arr.add(unpackValue(u));
                yield arr;
            }
            case MAP -> {
                int n = u.unpackMapHeader();
                Map<Object, Object> map = new LinkedHashMap<>(n);
                for (int i = 0; i < n; i++) {
                    Object k = unpackValue(u);
                    Object v = unpackValue(u);
                    map.put(k, v);
                }
                yield map;
            }
            default -> { u.skipValue(); yield null; }
        };
    }

    @Nullable
    private Object coerce(@Nullable Object value, Class<?> targetType) {
        if (value == null) return null;
        if (targetType.isInstance(value)) return value;
        if (targetType == int.class || targetType == Integer.class) {
            if (value instanceof Number n) return n.intValue();
        } else if (targetType == long.class || targetType == Long.class) {
            if (value instanceof Number n) return n.longValue();
        } else if (targetType == double.class || targetType == Double.class) {
            if (value instanceof Number n) return n.doubleValue();
        } else if (targetType == float.class || targetType == Float.class) {
            if (value instanceof Number n) return n.floatValue();
        } else if (targetType == boolean.class || targetType == Boolean.class) {
            if (value instanceof Boolean b) return b;
        } else if (targetType == String.class) {
            return value.toString();
        }
        return value;
    }

    private byte[] encodeResult(@Nullable Object value) {
        try {
            MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
            packValue(packer, value);
            packer.close();
            return packer.toByteArray();
        } catch (IOException e) {
            LOGGER.warn("TmRedstonePortPeripheral: encode failed: {}", e.getMessage());
            return encodeNil();
        }
    }

    private void packValue(MessageBufferPacker p, Object value) throws IOException {
        if (value == null) {
            p.packNil();
        } else if (value instanceof Boolean b) {
            p.packBoolean(b);
        } else if (value instanceof Byte b) {
            p.packInt(b);
        } else if (value instanceof Short s) {
            p.packInt(s);
        } else if (value instanceof Integer i) {
            p.packInt(i);
        } else if (value instanceof Long l) {
            p.packLong(l);
        } else if (value instanceof Float f) {
            p.packFloat(f);
        } else if (value instanceof Double d) {
            p.packDouble(d);
        } else if (value instanceof Number n) {
            p.packDouble(n.doubleValue());
        } else if (value instanceof String s) {
            p.packString(s);
        } else if (value instanceof Map<?, ?> map) {
            p.packMapHeader(map.size());
            for (var entry : map.entrySet()) {
                packValue(p, entry.getKey());
                packValue(p, entry.getValue());
            }
        } else if (value instanceof List<?> list) {
            p.packArrayHeader(list.size());
            for (Object item : list) packValue(p, item);
        } else if (value instanceof Object[] arr) {
            p.packArrayHeader(arr.length);
            for (Object item : arr) packValue(p, item);
        } else {
            p.packString(value.toString());
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

    private byte[] encodeNil() {
        return new byte[]{(byte) 0xC0};
    }
}
