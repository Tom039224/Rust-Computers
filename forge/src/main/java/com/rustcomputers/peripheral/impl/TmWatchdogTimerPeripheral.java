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
 * Tom's Peripherals WatchdogTimer peripheral implementation.
 *
 * <p>Delegates to {@code WatchDogTimerBlockEntity.getPeripheral()} (IPeripheral) via reflection
 * since Toms-Peripherals is not a compile-time dependency.</p>
 */
public class TmWatchdogTimerPeripheral implements PeripheralType {

    private static final Logger LOGGER = LoggerFactory.getLogger(TmWatchdogTimerPeripheral.class);
    private static final String TYPE_NAME = "tm_wdt";

    private static final String[] METHODS = {
        "isEnabled", "getTimeout", "setEnabled", "setTimeout", "reset"
    };

    private static final Set<String> IMM_METHODS = Set.of("isEnabled", "getTimeout");

    @Override
    public String getTypeName() { return TYPE_NAME; }

    @Override
    public String[] getMethodNames() { return METHODS.clone(); }

    @Override
    public byte[] callMethod(String methodName, byte[] args,
                             ServerLevel level, BlockPos pos) throws PeripheralException {
        try {
            return switch (methodName) {
                case "isEnabled"  -> encodeBool(readIsEnabled(level, pos));
                case "getTimeout" -> encodeInt(readGetTimeout(level, pos));
                // Delegate setters to actual peripheral
                default           -> delegateToPeripheral(methodName, args, level, pos);
            };
        } catch (IOException e) {
            LOGGER.error("TmWatchdogTimerPeripheral: encode failed for '{}'", methodName, e);
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
    // Delegate to actual WatchdogTimer peripheral via reflection
    // ------------------------------------------------------------------

    private byte[] delegateToPeripheral(String methodName, byte[] args,
                                       ServerLevel level, BlockPos pos) throws IOException {
        IPeripheral peripheral = getPeripheral(level, pos);
        if (peripheral == null) {
            LOGGER.warn("TmWatchdogTimerPeripheral: no peripheral at {}", pos);
            return encodeNil();
        }

        try {
            Method method = findLuaMethod(peripheral, methodName);
            if (method == null) {
                LOGGER.debug("TmWatchdogTimerPeripheral: @LuaFunction '{}' not found", methodName);
                return encodeNil();
            }

            Object[] javaArgs = decodeMsgpackArgs(args, method);
            Object result = method.invoke(peripheral, javaArgs);
            return encodeResult(result);
        } catch (Exception e) {
            LOGGER.warn("TmWatchdogTimerPeripheral: callMethod '{}' failed: {}", methodName, e.getMessage());
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
            LOGGER.debug("TmWatchdogTimerPeripheral: getPeripheral() failed: {}", e.getMessage());
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
            LOGGER.debug("TmWatchdogTimerPeripheral: failed to decode args: {}", e.getMessage());
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
            LOGGER.warn("TmWatchdogTimerPeripheral: encode failed: {}", e.getMessage());
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

    // ------------------------------------------------------------------
    // Reflection helpers
    // ------------------------------------------------------------------

    private boolean readIsEnabled(ServerLevel level, BlockPos pos) {
        BlockEntity be = level.getBlockEntity(pos);
        if (be == null) return false;
        try {
            Method m = be.getClass().getMethod("isEnabled");
            return (boolean) m.invoke(be);
        } catch (Exception e) {
            LOGGER.warn("TmWatchdogTimerPeripheral: isEnabled reflection failed: {}", e.getMessage());
            return false;
        }
    }

    private int readGetTimeout(ServerLevel level, BlockPos pos) {
        BlockEntity be = level.getBlockEntity(pos);
        if (be == null) return 0;
        try {
            Method m = be.getClass().getMethod("getTimeLimit");
            return ((Number) m.invoke(be)).intValue();
        } catch (Exception e) {
            LOGGER.warn("TmWatchdogTimerPeripheral: getTimeLimit reflection failed: {}", e.getMessage());
            return 0;
        }
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
