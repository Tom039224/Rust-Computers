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

import javax.annotation.Nullable;
import java.io.IOException;
import java.lang.reflect.Method;
import java.util.*;

/**
 * Tom's Peripherals RedstonePort peripheral implementation.
 *
 * <p>Tom's Peripherals uses its own {@code ITMPeripheral} / {@code ObjectWrapper} API.
 * We obtain the peripheral via {@code RedstonePortBlockEntity.getPeripheral()} and
 * call {@code ITMPeripheral.call(null, methodName, args)} directly.</p>
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

    @Override
    public String getTypeName() { return TYPE_NAME; }

    @Override
    public String[] getMethodNames() { return METHODS.clone(); }

    @Override
    public byte[] callMethod(String methodName, byte[] args,
                             ServerLevel level, BlockPos pos) throws PeripheralException {
        return delegateToTmPeripheral(methodName, args, level, pos);
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
    // Delegate to ITMPeripheral.call() via reflection
    // ------------------------------------------------------------------

    private byte[] delegateToTmPeripheral(String methodName, byte[] args,
                                          ServerLevel level, BlockPos pos) {
        Object peripheral = getTmPeripheral(level, pos);
        if (peripheral == null) {
            LOGGER.warn("TmRedstonePortPeripheral: no peripheral at {}", pos);
            return encodeNil();
        }

        try {
            Method callMethod = findMethodByArity(peripheral.getClass(), "call", 3);
            if (callMethod == null) {
                LOGGER.warn("TmRedstonePortPeripheral: ITMPeripheral.call() not found on {}", peripheral.getClass());
                return encodeNil();
            }

            Object[] javaArgs = decodeMsgpackToObjectArray(args);
            Object result = callMethod.invoke(peripheral, null, methodName, javaArgs);
            return encodeResult(result);
        } catch (Exception e) {
            Throwable cause = e.getCause() != null ? e.getCause() : e;
            LOGGER.warn("TmRedstonePortPeripheral: '{}' failed: {}", methodName, cause.getMessage());
            return encodeNil();
        }
    }

    @Nullable
    private Object getTmPeripheral(ServerLevel level, BlockPos pos) {
        BlockEntity be = level.getBlockEntity(pos);
        if (be == null) return null;
        try {
            Method getPeripheral = findMethodByArity(be.getClass(), "getPeripheral", 0);
            if (getPeripheral == null) {
                LOGGER.warn("TmRedstonePortPeripheral: getPeripheral() not found on {}", be.getClass());
                return null;
            }
            return getPeripheral.invoke(be);
        } catch (Exception e) {
            LOGGER.warn("TmRedstonePortPeripheral: getPeripheral() failed at {}: {}", pos, e.getMessage());
            return null;
        }
    }

    @Nullable
    private Method findMethodByArity(Class<?> cls, String name, int paramCount) {
        for (Class<?> iface : cls.getInterfaces()) {
            Method m = findMethodByArity(iface, name, paramCount);
            if (m != null) return m;
        }
        while (cls != null && cls != Object.class) {
            for (Method m : cls.getDeclaredMethods()) {
                if (m.getName().equals(name) && m.getParameterCount() == paramCount) {
                    m.setAccessible(true);
                    return m;
                }
            }
            cls = cls.getSuperclass();
        }
        return null;
    }

    // ------------------------------------------------------------------
    // Args decoding: msgpack → Object[]
    // ------------------------------------------------------------------

    private Object[] decodeMsgpackToObjectArray(byte[] data) {
        if (data == null || data.length == 0) return new Object[0];
        List<Object> list = new ArrayList<>();
        try (MessageUnpacker u = MessagePack.newDefaultUnpacker(data)) {
            if (!u.hasNext()) return new Object[0];
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
        return list.toArray();
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
                yield arr.toArray();
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

    // ------------------------------------------------------------------
    // Result encoding
    // ------------------------------------------------------------------

    private byte[] encodeResult(@Nullable Object value) {
        try {
            MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
            if (value instanceof Object[] arr) {
                if (arr.length == 0) {
                    packer.packNil();
                } else if (arr.length == 1) {
                    packValue(packer, arr[0]);
                } else {
                    packer.packArrayHeader(arr.length);
                    for (Object item : arr) packValue(packer, item);
                }
            } else {
                packValue(packer, value);
            }
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
        } else if (value instanceof Object[] arr) {
            p.packArrayHeader(arr.length);
            for (Object item : arr) packValue(p, item);
        } else if (value instanceof List<?> list) {
            p.packArrayHeader(list.size());
            for (Object item : list) packValue(p, item);
        } else {
            p.packString(value.toString());
        }
    }

    private byte[] encodeNil() {
        return new byte[]{(byte) 0xC0};
    }
}
