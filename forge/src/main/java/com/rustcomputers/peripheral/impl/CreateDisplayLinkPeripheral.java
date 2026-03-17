package com.rustcomputers.peripheral.impl;

import com.rustcomputers.peripheral.PeripheralException;
import com.rustcomputers.peripheral.PeripheralType;
import net.minecraft.core.BlockPos;
import net.minecraft.server.level.ServerLevel;
import org.msgpack.core.MessageBufferPacker;
import org.msgpack.core.MessagePack;
import org.msgpack.core.MessageUnpacker;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;

/**
 * Create DisplayLink peripheral implementation.
 * Controls a display linked to Create's display boards.
 *
 * <p>Methods:
 * <ul>
 *   <li><b>setCursorPos(x, y)</b> - Set cursor position</li>
 *   <li><b>getCursorPos()</b> - Get cursor position (IMM)</li>
 *   <li><b>getSize()</b> - Get display size</li>
 *   <li><b>isColor()</b> - Check if display supports color (IMM)</li>
 *   <li><b>write(text)</b> - Write text at cursor</li>
 *   <li><b>writeBytes(data)</b> - Write raw bytes</li>
 *   <li><b>clearLine()</b> - Clear current line</li>
 *   <li><b>clear()</b> - Clear entire display</li>
 *   <li><b>update()</b> - Flush and update display</li>
 * </ul>
 */
public class CreateDisplayLinkPeripheral implements PeripheralType {

    private static final Logger LOGGER = LoggerFactory.getLogger(CreateDisplayLinkPeripheral.class);
    private static final String TYPE_NAME = "Create_DisplayLink";

    private static final String[] METHODS = {
            "setCursorPos",
            "getCursorPos",
            "getSize",
            "isColor",
            "write",
            "writeBytes",
            "clearLine",
            "clear",
            "update"
    };

    // Display state (simplified simulation)
    private int cursorX = 1;
    private int cursorY = 1;
    private final int width = 80;
    private final int height = 25;
    private final boolean supportsColor = true;

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
                case "setCursorPos" -> {
                    int[] pos = unpackCursorPos(args);
                    setCursorPos(pos[0], pos[1]);
                    yield encodeVoid();
                }
                case "getCursorPos" -> encodeCursorPos();
                case "getSize" -> encodeSize();
                case "isColor" -> encodeBoolean(supportsColor);
                case "write" -> {
                    String text = unpackString(args);
                    write(text);
                    yield encodeVoid();
                }
                case "writeBytes" -> {
                    byte[] data = unpackBytes(args);
                    writeBytes(data);
                    yield encodeVoid();
                }
                case "clearLine" -> {
                    clearLine();
                    yield encodeVoid();
                }
                case "clear" -> {
                    clear();
                    yield encodeVoid();
                }
                case "update" -> {
                    update();
                    yield encodeVoid();
                }
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
        // getCursorPos and isColor are immediate
        if ("getCursorPos".equals(methodName) || "isColor".equals(methodName)) {
            return callMethod(methodName, args, level, peripheralPos);
        }
        return null;
    }

    /**
     * Unpack cursor position from args.
     */
    private int[] unpackCursorPos(byte[] args) throws PeripheralException {
        if (args == null || args.length == 0) {
            throw new PeripheralException("Missing cursor position arguments");
        }

        try (MessageUnpacker unpacker = MessagePack.newDefaultUnpacker(args)) {
            int x = unpacker.unpackInt();
            int y = unpacker.unpackInt();
            return new int[]{x, y};
        } catch (IOException e) {
            throw new PeripheralException("Failed to unpack cursor position: " + e.getMessage());
        }
    }

    /**
     * Unpack string from args.
     */
    private String unpackString(byte[] args) throws PeripheralException {
        if (args == null || args.length == 0) {
            throw new PeripheralException("Missing string argument");
        }

        try (MessageUnpacker unpacker = MessagePack.newDefaultUnpacker(args)) {
            return unpacker.unpackString();
        } catch (IOException e) {
            throw new PeripheralException("Failed to unpack string: " + e.getMessage());
        }
    }

    /**
     * Unpack bytes from args.
     */
    private byte[] unpackBytes(byte[] args) throws PeripheralException {
        if (args == null || args.length == 0) {
            throw new PeripheralException("Missing bytes argument");
        }

        try (MessageUnpacker unpacker = MessagePack.newDefaultUnpacker(args)) {
            int length = unpacker.unpackBinaryHeader();
            return unpacker.readPayload(length);
        } catch (IOException e) {
            throw new PeripheralException("Failed to unpack bytes: " + e.getMessage());
        }
    }

    /**
     * Set cursor position.
     */
    private void setCursorPos(int x, int y) {
        this.cursorX = Math.max(1, Math.min(x, width));
        this.cursorY = Math.max(1, Math.min(y, height));
    }

    /**
     * Write text at cursor position.
     */
    private void write(String text) {
        // Simplified: just advance cursor
        cursorX += text.length();
        if (cursorX > width) {
            cursorX = 1;
            cursorY++;
        }
    }

    /**
     * Write raw bytes.
     */
    private void writeBytes(byte[] data) {
        // Simplified: treat as text
        cursorX += data.length;
        if (cursorX > width) {
            cursorX = 1;
            cursorY++;
        }
    }

    /**
     * Clear current line.
     */
    private void clearLine() {
        cursorX = 1;
    }

    /**
     * Clear entire display.
     */
    private void clear() {
        cursorX = 1;
        cursorY = 1;
    }

    /**
     * Update display.
     */
    private void update() {
        // Simplified: no-op
    }

    /**
     * Encode void result.
     */
    private byte[] encodeVoid() throws IOException {
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
        packer.packNil();
        packer.close();
        return packer.toByteArray();
    }

    /**
     * Encode cursor position.
     */
    private byte[] encodeCursorPos() throws IOException {
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
        packer.packArrayHeader(2);
        packer.packInt(cursorX);
        packer.packInt(cursorY);
        packer.close();
        return packer.toByteArray();
    }

    /**
     * Encode display size.
     */
    private byte[] encodeSize() throws IOException {
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
        packer.packArrayHeader(2);
        packer.packInt(width);
        packer.packInt(height);
        packer.close();
        return packer.toByteArray();
    }

    /**
     * Encode boolean result.
     */
    private byte[] encodeBoolean(boolean value) throws IOException {
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
        packer.packBoolean(value);
        packer.close();
        return packer.toByteArray();
    }
}
