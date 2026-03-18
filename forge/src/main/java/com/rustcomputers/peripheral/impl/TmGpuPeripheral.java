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

/**
 * Tom's Peripherals GPU peripheral implementation.
 *
 * <p>Uses reflection to call {@code GPUBlockEntity.getPeripheral().getSize()}
 * since Toms-Peripherals is not a compile-time dependency.</p>
 *
 * <p>getSize() returns Object[]{pixel_width, pixel_height, monitor_cols, monitor_rows, pixel_size}
 * which maps to the Rust tuple (u32, u32, u32, u32, u32).</p>
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
                case "getSize"       -> encodeGetSize(level, peripheralPos);
                case "getTextLength" -> encodeGetTextLength(args);
                case "createWindow"  -> encodeCreateWindow(args);
                case "newImage"      -> encodeNewImage(args);
                case "decodeImage"   -> encodeDecodeImage();
                default              -> encodeNil();
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
        return switch (methodName) {
            case "getSize", "getTextLength", "createWindow", "newImage" ->
                callMethod(methodName, args, level, peripheralPos);
            default -> null;
        };
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

    // ------------------------------------------------------------------
    // Helpers
    // ------------------------------------------------------------------

    private byte[] encodeNil() throws IOException {
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
        packer.packNil();
        packer.close();
        return packer.toByteArray();
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
