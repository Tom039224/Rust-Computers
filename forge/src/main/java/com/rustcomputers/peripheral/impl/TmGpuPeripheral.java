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
 * Tom's Peripherals GPU peripheral implementation.
 *
 * <p>Delegates to {@code GPUBlockEntity.getPeripheral()} (IPeripheral) via reflection
 * since Toms-Peripherals is not a compile-time dependency.</p>
 *
 * <p>Uses CcBlockEntityBridge pattern for most methods, with custom handling for
 * getSize, getTextLength, createWindow, newImage which need special encoding.</p>
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
            // Custom encoding for specific methods
            return switch (methodName) {
                case "getSize"       -> encodeGetSize(level, peripheralPos);
                case "getTextLength" -> encodeGetTextLength(args);
                case "createWindow"  -> encodeCreateWindow(args);
                case "newImage"      -> encodeNewImage(args);
                case "decodeImage"   -> encodeDecodeImage();
                default              -> delegateToPeripheral(methodName, args, level, peripheralPos);
            };
        } catch (IOException e) {
            LOGGER.error("TmGpuPeripheral: failed to encode result for '{}'", methodName, e);
            throw new PeripheralException("Failed to encode result: " + e.getMessage());
        }
    }

    @Override
    public byte[] callImmediate(String methodName, byte[] args,
                                ServerLevel level, BlockPos peripheralPos)
            throws PeripheralException {
        if (IMM_METHODS.contains(methodName)) {
            return callMethod(methodName, args, level, peripheralPos);
        }
        return null;
    }

    // ------------------------------------------------------------------
    // Delegate to actual GPU peripheral via reflection
    // ------------------------------------------------------------------

    private byte[] delegateToPeripheral(String methodName, byte[] args,
                                       ServerLevel level, BlockPos pos) throws IOException {
        IPeripheral peripheral = getPeripheral(level, pos);
        if (peripheral == null) {
            LOGGER.warn("TmGpuPeripheral: no peripheral at {}", pos);
            return encodeNil();
        }

        try {
            Method method = findLuaMethod(peripheral, methodName);
            if (method == null) {
                LOGGER.debug("TmGpuPeripheral: @LuaFunction '{}' not found", methodName);
                return encodeNil();
            }

            Object[] javaArgs = decodeMsgpackArgs(args, method);
            Object result = method.invoke(peripheral, javaArgs);
            return encodeResult(result);
        } catch (Exception e) {
            LOGGER.warn("TmGpuPeripheral: callMethod '{}' failed: {}", methodName, e.getMessage());
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
            LOGGER.debug("TmGpuPeripheral: getPeripheral() failed: {}", e.getMessage());
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
            LOGGER.debug("TmGpuPeripheral: failed to decode args: {}", e.getMessage());
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
    // getSize → [pixel_width, pixel_height, monitor_cols, monitor_rows, pixel_size]
    // ------------------------------------------------------------------

    private byte[] encodeGetSize(ServerLevel level, BlockPos pos) throws IOException, PeripheralException {
        int[] vals = readSizeFromBlockEntity(level, pos);
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
        packer.packArrayHeader(5);
        for (int v : vals) packer.packInt(v);
        packer.close();
        return packer.toByteArray();
    }

    /**
     * Reads size data from GPUBlockEntity via reflection.
     * Returns {pixel_width, pixel_height, maxX, maxY, size}.
     */
    private int[] readSizeFromBlockEntity(ServerLevel level, BlockPos pos) {
        BlockEntity be = level.getBlockEntity(pos);
        if (be == null) {
            LOGGER.warn("TmGpuPeripheral: no BlockEntity at {}", pos);
            return new int[]{0, 0, 0, 0, 16};
        }
        try {
            // GPUBlockEntity.getPeripheral() → ITMPeripheral (actually GPUPeripheral)
            Method getPeripheral = be.getClass().getMethod("getPeripheral");
            Object peripheral = getPeripheral.invoke(be);

            // GPUExt (extends GPUImpl) has getSize() → Object[]
            // GPUPeripheral delegates to GPUExt via impl.callInt()
            // Easier: call getWidth() and getHeight() directly on GPUPeripheral (GPUContext)
            // and read maxX, maxY, size via getDeclaredField on the inner class.
            Method getWidth  = peripheral.getClass().getMethod("getWidth");
            Method getHeight = peripheral.getClass().getMethod("getHeight");
            int width  = ((Number) getWidth.invoke(peripheral)).intValue();
            int height = ((Number) getHeight.invoke(peripheral)).intValue();

            // maxX, maxY, size are private fields of the inner GPUPeripheral class
            int maxX = getPrivateInt(peripheral, "maxX");
            int maxY = getPrivateInt(peripheral, "maxY");
            int size = getPrivateInt(peripheral, "size");

            return new int[]{width, height, maxX, maxY, size};
        } catch (Exception e) {
            LOGGER.warn("TmGpuPeripheral: reflection failed for getSize at {}: {}", pos, e.getMessage());
            return new int[]{0, 0, 0, 0, 16};
        }
    }

    private int getPrivateInt(Object obj, String fieldName) throws Exception {
        // Try declared fields on the object's class and its superclasses
        Class<?> cls = obj.getClass();
        while (cls != null) {
            try {
                var field = cls.getDeclaredField(fieldName);
                field.setAccessible(true);
                return ((Number) field.get(obj)).intValue();
            } catch (NoSuchFieldException ignored) {
                cls = cls.getSuperclass();
            }
        }
        throw new NoSuchFieldException(fieldName + " not found in " + obj.getClass().getName());
    }

    // ------------------------------------------------------------------
    // getTextLength → u32 (pixel length estimate, 8px per char)
    // ------------------------------------------------------------------

    private byte[] encodeGetTextLength(byte[] args) throws IOException, PeripheralException {
        String text = unpackFirstString(args);
        int len = text.length() * 8;
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
        packer.packInt(len);
        packer.close();
        return packer.toByteArray();
    }

    // ------------------------------------------------------------------
    // createWindow → {x, y, width, height}
    // ------------------------------------------------------------------

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

    // ------------------------------------------------------------------
    // newImage → {width, height, data:[0...]}
    // ------------------------------------------------------------------

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

    // ------------------------------------------------------------------
    // decodeImage → stub {width:0, height:0, data:[]}
    // ------------------------------------------------------------------

    private byte[] encodeDecodeImage() throws IOException {
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

    /**
     * Unpacks the first string from a msgpack array arg.
     * Handles both bare string and array-wrapped string.
     */
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

    /**
     * Unpacks {@code count} ints from a msgpack array arg.
     */
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
