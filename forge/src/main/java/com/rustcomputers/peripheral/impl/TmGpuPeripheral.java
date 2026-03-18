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
 * <p>Argument conversion (Rust → Java):
 * <ul>
 *   <li>Coordinates: Rust is 0-indexed, Java BaseGPU is 1-indexed → pass (x+1, y+1)</li>
 *   <li>Colors for fill/filledRectangle: Rust 0.0-1.0 floats → Java 0-255 Doubles (ParamCheck.toColor)</li>
 *   <li>Colors for drawText/drawChar: Rust r,g,b,a 0.0-1.0 → packed ARGB int</li>
 *   <li>Image refs: newImage/decodeImage return a String reference; pass to drawImage as-is</li>
 * </ul>
 * </p>
 */
public class TmGpuPeripheral implements PeripheralType {

    private static final Logger LOGGER = LoggerFactory.getLogger(TmGpuPeripheral.class);
    private static final String TYPE_NAME = "tm_gpu";

    private static final String[] METHODS = {
        // setSize / size management
        "setSize", "refreshSize", "getSize",
        // 2D drawing
        "fill", "filledRectangle", "rectangle", "line", "lineS",
        "drawText", "drawTextSmart", "drawChar", "drawBuffer", "drawImage",
        // sync
        "sync",
        // font
        "getFont", "setFont", "getTextLength",
        "addNewChar", "delChar", "freeChars", "clearChars",
        "setFontDefaultCharID", "getFontDefaultCharID",
        // image / buffer
        "decodeImage", "imageFromBuffer", "newImage", "newBuffer",
        // memory
        "getUsedMemory", "getMaxMemory",
        // window / misc
        "createWindow", "createWindow3D", "getBounds",
    };

    private static final Set<String> IMM_METHODS = Set.of(
        "getSize", "getTextLength", "getFont", "getFontDefaultCharID",
        "freeChars", "getUsedMemory", "getMaxMemory", "getBounds",
        "createWindow", "createWindow3D", "newImage", "getConstants"
    );    @Override
    public String getTypeName() { return TYPE_NAME; }

    @Override
    public String[] getMethodNames() { return METHODS.clone(); }

    @Override
    public byte[] callMethod(String methodName, byte[] args,
                             ServerLevel level, BlockPos pos) throws PeripheralException {
        try {
            return switch (methodName) {
                case "drawText", "drawChar", "drawTextSmart" -> delegateWithTextArgs(methodName, args, level, pos);
                case "fill"                      -> delegateWithFillArgs(args, level, pos);
                case "filledRectangle",
                     "rectangle"                 -> delegateWithRectArgs(methodName, args, level, pos);
                case "line", "lineS"             -> delegateWithLineArgs(methodName, args, level, pos);
                case "drawImage"                 -> delegateWithDrawImageArgs(args, level, pos);
                case "createWindow",
                     "createWindow3D"            -> delegateWithCreateWindowArgs(methodName, args, level, pos);
                default                          -> delegateToTmPeripheral(methodName, args, level, pos);
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
    }    // ------------------------------------------------------------------
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
            // Tom's Peripherals ParamCheck.getInt() does checkcast Double, so integers must be Double.
            case INTEGER -> (double) u.unpackLong();
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
            // Tom's Peripherals LuaImage / ReferenceableLuaObject: encode as reference string
            // so Rust can pass it back to drawImage(x, y, ref_string).
            String refStr = tryGetRef(value);
            if (refStr != null) {
                p.packString(refStr);
            } else {
                p.packString(value.toString());
            }
        }
    }

    /** Try to call .ref() on a Tom's Peripherals ReferenceableLuaObject. Returns null if not applicable. */
    @Nullable
    private String tryGetRef(Object obj) {
        try {
            Method refMethod = findMethodByArity(obj.getClass(), "ref", 0);
            if (refMethod == null) return null;
            Object result = refMethod.invoke(obj);
            return result instanceof String s ? s : null;
        } catch (Exception e) {
            return null;
        }
    }

    // ------------------------------------------------------------------
    // Argument transformation helpers
    //
    // Coordinate convention: Java BaseGPU does getInt(args, i) - 1 internally,
    // so it expects 1-indexed coords from Lua. Rust uses 0-indexed coords,
    // so we pass (x+1, y+1) to Java.
    //
    // Wait — actually Java subtracts 1 itself, so passing x as-is gives x-1 internally.
    // Rust x=0 → Java receives 0 → internal = 0-1 = -1 → Out of boundary.
    // Therefore we must pass (x+1) so Java computes (x+1)-1 = x (correct 0-indexed).
    //
    // Color convention:
    //   - fill/filledRectangle: ParamCheck.toColor() expects 3 or 4 Doubles in 0-255 range.
    //     Rust sends 0.0-1.0 floats → multiply by 255.
    //   - drawText/drawChar: text_color is a single ARGB int (packed).
    //     Rust sends r,g,b,a as 0.0-1.0 → pack into ARGB int.
    // ------------------------------------------------------------------

    /** rgba floats (0.0-1.0) → packed ARGB int (as Double for Java) */
    private static double rgbaToArgbInt(double r, double g, double b, double a) {
        int ai = (int) Math.round(a * 255) & 0xFF;
        int ri = (int) Math.round(r * 255) & 0xFF;
        int gi = (int) Math.round(g * 255) & 0xFF;
        int bi = (int) Math.round(b * 255) & 0xFF;
        return (double) ((ai << 24) | (ri << 16) | (gi << 8) | bi);
    }

    /** float 0.0-1.0 → Double 0-255 (for ParamCheck.toColor) */
    private static double toColor255(double v) {
        return Math.round(v * 255) & 0xFF;
    }

    /**
     * drawText / drawChar:
     *   Rust:  [text, x, y, r, g, b, a]   (x,y are 0-indexed, rgba 0.0-1.0)
     *   Java:  [x+1, y+1, text, argb_int]  (1-indexed, color as packed ARGB Double)
     */
    private byte[] delegateWithTextArgs(String methodName, byte[] args,
                                        ServerLevel level, BlockPos pos) throws IOException {
        Object[] raw = decodeMsgpackToObjectArray(args);
        String text = raw.length > 0 ? String.valueOf(raw[0]) : "";
        double x    = raw.length > 1 ? toDouble(raw[1]) : 0;
        double y    = raw.length > 2 ? toDouble(raw[2]) : 0;
        double r    = raw.length > 3 ? toDouble(raw[3]) : 1;
        double g    = raw.length > 4 ? toDouble(raw[4]) : 1;
        double b    = raw.length > 5 ? toDouble(raw[5]) : 1;
        double a    = raw.length > 6 ? toDouble(raw[6]) : 1;
        double argb = rgbaToArgbInt(r, g, b, a);
        // Java: (x, y, text, text_color) — x,y are 1-indexed
        Object[] javaArgs = new Object[]{x + 1, y + 1, text, argb};
        return delegateWithArgs(methodName, javaArgs, level, pos);
    }

    /**
     * fill:
     *   Rust:  [r, g, b, a]  (0.0-1.0)
     *   Java:  [r, g, b, a]  (0-255 as Double, via ParamCheck.toColor(args, 0))
     */
    private byte[] delegateWithFillArgs(byte[] args, ServerLevel level, BlockPos pos) throws IOException {
        Object[] raw = decodeMsgpackToObjectArray(args);
        double r = raw.length > 0 ? toDouble(raw[0]) : 0;
        double g = raw.length > 1 ? toDouble(raw[1]) : 0;
        double b = raw.length > 2 ? toDouble(raw[2]) : 0;
        double a = raw.length > 3 ? toDouble(raw[3]) : 1;
        Object[] javaArgs = new Object[]{toColor255(r), toColor255(g), toColor255(b), toColor255(a)};
        return delegateWithArgs("fill", javaArgs, level, pos);
    }

    /**
     * filledRectangle / rectangle:
     *   Rust:  [x, y, w, h, r, g, b, a]  (x,y 0-indexed, rgba 0.0-1.0)
     *   Java:  [x+1, y+1, w, h, r, g, b, a]  (1-indexed, color 0-255 Double)
     */
    private byte[] delegateWithRectArgs(String methodName, byte[] args, ServerLevel level, BlockPos pos) throws IOException {
        Object[] raw = decodeMsgpackToObjectArray(args);
        double x = raw.length > 0 ? toDouble(raw[0]) : 0;
        double y = raw.length > 1 ? toDouble(raw[1]) : 0;
        double w = raw.length > 2 ? toDouble(raw[2]) : 0;
        double h = raw.length > 3 ? toDouble(raw[3]) : 0;
        double r = raw.length > 4 ? toDouble(raw[4]) : 0;
        double g = raw.length > 5 ? toDouble(raw[5]) : 0;
        double b = raw.length > 6 ? toDouble(raw[6]) : 0;
        double a = raw.length > 7 ? toDouble(raw[7]) : 1;
        Object[] javaArgs = new Object[]{x + 1, y + 1, w, h, toColor255(r), toColor255(g), toColor255(b), toColor255(a)};
        return delegateWithArgs(methodName, javaArgs, level, pos);
    }

    /**
     * line / lineS:
     *   Rust:  [x1, y1, x2, y2, r, g, b, a]  (0-indexed, rgba 0.0-1.0)
     *   Java:  [x1+1, y1+1, x2+1, y2+1, color]  (1-indexed, color 0-255 Double x4)
     */
    private byte[] delegateWithLineArgs(String methodName, byte[] args, ServerLevel level, BlockPos pos) throws IOException {
        Object[] raw = decodeMsgpackToObjectArray(args);
        double x1 = raw.length > 0 ? toDouble(raw[0]) : 0;
        double y1 = raw.length > 1 ? toDouble(raw[1]) : 0;
        double x2 = raw.length > 2 ? toDouble(raw[2]) : 0;
        double y2 = raw.length > 3 ? toDouble(raw[3]) : 0;
        double r  = raw.length > 4 ? toDouble(raw[4]) : 0;
        double g  = raw.length > 5 ? toDouble(raw[5]) : 0;
        double b  = raw.length > 6 ? toDouble(raw[6]) : 0;
        double a  = raw.length > 7 ? toDouble(raw[7]) : 1;
        Object[] javaArgs = new Object[]{x1 + 1, y1 + 1, x2 + 1, y2 + 1, toColor255(r), toColor255(g), toColor255(b), toColor255(a)};
        return delegateWithArgs(methodName, javaArgs, level, pos);
    }

    /**
     * createWindow / createWindow3D:
     *   Rust:  [x, y, w, h]  (0-indexed)
     *   Java:  [x+1, y+1, w, h]  (1-indexed)
     *   Returns: TMLuaObject ref string
     */
    private byte[] delegateWithCreateWindowArgs(String methodName, byte[] args, ServerLevel level, BlockPos pos) throws IOException {
        Object[] raw = decodeMsgpackToObjectArray(args);
        double x = raw.length > 0 ? toDouble(raw[0]) : 0;
        double y = raw.length > 1 ? toDouble(raw[1]) : 0;
        double w = raw.length > 2 ? toDouble(raw[2]) : 0;
        double h = raw.length > 3 ? toDouble(raw[3]) : 0;
        Object[] javaArgs = new Object[]{x + 1, y + 1, w, h};
        return delegateWithArgs(methodName, javaArgs, level, pos);
    }

    /**
     * drawImage:
     *   Rust:  [image_ref(String), x, y]  (x,y 0-indexed, image is a ref string from newImage/decodeImage)
     *   Java:  [x+1, y+1, image_ref]      (1-indexed, ref string at index 2)
     */
    private byte[] delegateWithDrawImageArgs(byte[] args, ServerLevel level, BlockPos pos) throws IOException {
        Object[] raw = decodeMsgpackToObjectArray(args);
        Object imageRef = raw.length > 0 ? raw[0] : null;
        double x        = raw.length > 1 ? toDouble(raw[1]) : 0;
        double y        = raw.length > 2 ? toDouble(raw[2]) : 0;
        Object[] javaArgs = new Object[]{x + 1, y + 1, imageRef};
        return delegateWithArgs("drawImage", javaArgs, level, pos);
    }

    private byte[] delegateWithArgs(String methodName, Object[] javaArgs,
                                    ServerLevel level, BlockPos pos) throws IOException {
        Object peripheral = getTmPeripheral(level, pos);
        if (peripheral == null) {
            LOGGER.warn("TmGpuPeripheral: no peripheral at {}", pos);
            return encodeNil();
        }
        try {
            Method callMethod = findMethodByArity(peripheral.getClass(), "call", 3);
            if (callMethod == null) {
                LOGGER.warn("TmGpuPeripheral: ITMPeripheral.call() not found");
                return encodeNil();
            }
            Object result = callMethod.invoke(peripheral, null, methodName, javaArgs);
            return encodeResult(result);
        } catch (Exception e) {
            Throwable cause = e.getCause() != null ? e.getCause() : e;
            LOGGER.warn("TmGpuPeripheral: '{}' failed: {}", methodName, cause.getMessage());
            return encodeNil();
        }
    }

    private static double toDouble(Object v) {
        if (v instanceof Number n) return n.doubleValue();
        return 0;
    }

    // ------------------------------------------------------------------
    // Encode helpers
    // ------------------------------------------------------------------

    private byte[] encodeNil() {
        return new byte[]{(byte) 0xC0};
    }
}
