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
 * Tom's Peripherals GPU peripheral implementation.
 *
 * <p>Tom's Peripherals uses its own {@code ITMPeripheral} API (not CC:Tweaked's IPeripheral),
 * with {@code @LuaMethod} annotations and {@code call(IComputer, String, Object[])} dispatch.
 * We obtain the peripheral via {@code GPUBlockEntity.getPeripheral()} and call it directly.</p>
 *
 * <p>Special cases:
 * <ul>
 *   <li>{@code getSize} — decoded from Object[] return value</li>
 *   <li>{@code getTextLength} — estimated locally (8px/char)</li>
 *   <li>{@code createWindow} — echoes args back as a map</li>
 *   <li>{@code newImage} — creates a blank image locally</li>
 *   <li>{@code decodeImage} — not yet implemented (returns empty image)</li>
 * </ul>
 * </p>
 */
public class TmGpuPeripheral implements PeripheralType {

    private static final Logger LOGGER = LoggerFactory.getLogger(TmGpuPeripheral.class);
    private static final String TYPE_NAME = "tm_gpu";

    private static final String[] METHODS = {
        "setSize", "refreshSize", "getSize", "fill", "sync",
        "filledRectangle", "drawImage", "drawText", "drawChar",
        "getTextLength", "setFont", "clearChars", "addNewChar",
        "createWindow", "decodeImage", "newImage"
    };

    private static final Set<String> IMM_METHODS = Set.of(
        "getSize", "getTextLength", "createWindow", "newImage"
    );

    @Override
    public String getTypeName() { return TYPE_NAME; }

    @Override
    public String[] getMethodNames() { return METHODS.clone(); }

    @Override
    public byte[] callMethod(String methodName, byte[] args,
                             ServerLevel level, BlockPos pos) throws PeripheralException {
        try {
            return switch (methodName) {
                case "getTextLength" -> encodeGetTextLength(args);
                case "createWindow"  -> encodeCreateWindow(args);
                case "newImage"      -> encodeNewImage(args);
                case "decodeImage"   -> encodeDecodeImage();
                default              -> delegateToTmPeripheral(methodName, args, level, pos);
            };
        } catch (IOException e) {
            LOGGER.error("TmGpuPeripheral: encode failed for '{}'", methodName, e);
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
    // Delegate to ITMPeripheral.call() via reflection
    // ------------------------------------------------------------------

    private byte[] delegateToTmPeripheral(String methodName, byte[] args,
                                          ServerLevel level, BlockPos pos) throws IOException {
        Object peripheral = getTmPeripheral(level, pos);
        if (peripheral == null) {
            LOGGER.warn("TmGpuPeripheral: no peripheral at {}", pos);
            return encodeNil();
        }

        try {
            // ITMPeripheral.call(IComputer computer, String method, Object[] args)
            Method callMethod = findMethodByArity(peripheral.getClass(), "call", 3);
            if (callMethod == null) {
                LOGGER.warn("TmGpuPeripheral: ITMPeripheral.call() not found on {}", peripheral.getClass());
                return encodeNil();
            }

            Object[] javaArgs = decodeMsgpackToObjectArray(args);
            Object result = callMethod.invoke(peripheral, null, methodName, javaArgs);
            return encodeResult(result);
        } catch (Exception e) {
            Throwable cause = e.getCause() != null ? e.getCause() : e;
            LOGGER.warn("TmGpuPeripheral: '{}' failed: {}", methodName, cause.getMessage());
            return encodeNil();
        }
    }

    @Nullable
    private Object getTmPeripheral(ServerLevel level, BlockPos pos) {
        BlockEntity be = level.getBlockEntity(pos);
        if (be == null) {
            LOGGER.debug("TmGpuPeripheral: no BlockEntity at {}", pos);
            return null;
        }
        try {
            // getPeripheral() is public on AbstractPeripheralBlockEntity, but returns a private inner class.
            // Use getDeclaredMethod to ensure we find it even if overridden with a covariant return type.
            Method getPeripheral = findMethodByArity(be.getClass(), "getPeripheral", 0);
            if (getPeripheral == null) {
                LOGGER.warn("TmGpuPeripheral: getPeripheral() not found on {}", be.getClass());
                return null;
            }
            return getPeripheral.invoke(be);
        } catch (Exception e) {
            LOGGER.warn("TmGpuPeripheral: getPeripheral() failed at {}: {}", pos, e.getMessage());
            return null;
        }
    }

    /**
     * Searches for a method by name and parameter count up the class hierarchy.
     * Uses parameter count only to avoid classloader issues with cross-mod types (IComputer etc.).
     */
    @Nullable
    private Method findMethodByArity(Class<?> cls, String name, int paramCount) {
        // Search interfaces first (ITMPeripheral.call is defined there)
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
            LOGGER.debug("TmGpuPeripheral: failed to decode args: {}", e.getMessage());
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
    // Result encoding: Object/Object[] → msgpack
    // ------------------------------------------------------------------

    private byte[] encodeResult(@Nullable Object value) {
        try {
            MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
            // ITMPeripheral.call() returns Object[] — unwrap single-element arrays
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
            LOGGER.warn("TmGpuPeripheral: encode failed: {}", e.getMessage());
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

    // ------------------------------------------------------------------
    // Local implementations for non-delegatable methods
    // ------------------------------------------------------------------

    private byte[] encodeGetTextLength(byte[] args) throws IOException, PeripheralException {
        String text = unpackFirstString(args);
        int len = text.length() * 8;
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
        packer.packInt(len);
        packer.close();
        return packer.toByteArray();
    }

    private byte[] encodeCreateWindow(byte[] args) throws IOException, PeripheralException {
        int[] p = unpackInts(args, 4);
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
        packer.packMapHeader(4);
        packer.packString("x");      packer.packDouble(p[0]);
        packer.packString("y");      packer.packDouble(p[1]);
        packer.packString("width");  packer.packInt(p[2]);
        packer.packString("height"); packer.packInt(p[3]);
        packer.close();
        return packer.toByteArray();
    }

    private byte[] encodeNewImage(byte[] args) throws IOException, PeripheralException {
        int[] p = unpackInts(args, 2);
        int w = p[0], h = p[1];
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
        packer.packMapHeader(3);
        packer.packString("width");  packer.packInt(w);
        packer.packString("height"); packer.packInt(h);
        packer.packString("data");
        packer.packArrayHeader(w * h);
        for (int i = 0; i < w * h; i++) packer.packInt(0);
        packer.close();
        return packer.toByteArray();
    }

    private byte[] encodeDecodeImage() throws IOException {
        // TODO: implement Base64 image decoding
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
        packer.packMapHeader(3);
        packer.packString("width");  packer.packInt(0);
        packer.packString("height"); packer.packInt(0);
        packer.packString("data");   packer.packArrayHeader(0);
        packer.close();
        return packer.toByteArray();
    }

    private byte[] encodeNil() {
        return new byte[]{(byte) 0xC0};
    }

    private String unpackFirstString(byte[] args) throws PeripheralException {
        if (args == null || args.length == 0) return "";
        try (MessageUnpacker u = MessagePack.newDefaultUnpacker(args)) {
            if (!u.hasNext()) return "";
            var fmt = u.getNextFormat();
            if (fmt.getValueType().isArrayType()) u.unpackArrayHeader();
            return u.unpackString();
        } catch (IOException e) {
            throw new PeripheralException("Failed to unpack string arg: " + e.getMessage());
        }
    }

    private int[] unpackInts(byte[] args, int count) throws PeripheralException {
        int[] result = new int[count];
        if (args == null || args.length == 0) return result;
        try (MessageUnpacker u = MessagePack.newDefaultUnpacker(args)) {
            if (u.hasNext()) {
                var fmt = u.getNextFormat();
                if (fmt.getValueType().isArrayType()) u.unpackArrayHeader();
            }
            for (int i = 0; i < count && u.hasNext(); i++) {
                result[i] = u.unpackInt();
            }
            return result;
        } catch (IOException e) {
            throw new PeripheralException("Failed to unpack int args: " + e.getMessage());
        }
    }
}
