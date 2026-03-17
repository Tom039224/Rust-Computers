package com.rustcomputers.peripheral.impl;

import com.rustcomputers.peripheral.PeripheralException;
import com.rustcomputers.peripheral.PeripheralType;
import net.minecraft.core.BlockPos;
import net.minecraft.server.level.ServerLevel;
import org.msgpack.core.MessageBufferPacker;
import org.msgpack.core.MessagePack;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.*;

/**
 * Generic peripheral wrapper for simple peripherals.
 * Provides a basic implementation for peripherals with simple method signatures.
 *
 * <p>This is a placeholder implementation that returns nil for all methods.
 * Actual implementations should override specific methods as needed.</p>
 */
public class CcGenericPeripheral implements PeripheralType {

    private static final Logger LOGGER = LoggerFactory.getLogger(CcGenericPeripheral.class);

    private final String typeName;
    private final String[] methods;
    private final Set<String> immediateMethods;

    /**
     * Create a generic peripheral with the given type name and methods.
     */
    public CcGenericPeripheral(String typeName, String[] methods) {
        this(typeName, methods, new HashSet<>());
    }

    /**
     * Create a generic peripheral with immediate method support.
     */
    public CcGenericPeripheral(String typeName, String[] methods, Set<String> immediateMethods) {
        this.typeName = typeName;
        this.methods = methods;
        this.immediateMethods = immediateMethods;
    }

    @Override
    public String getTypeName() {
        return typeName;
    }

    @Override
    public String[] getMethodNames() {
        return methods.clone();
    }

    @Override
    public byte[] callMethod(String methodName, byte[] args,
                             ServerLevel level, BlockPos peripheralPos)
            throws PeripheralException {
        try {
            // try_pull_* メソッドは CCEventReceiver に委譲する
            // Delegate try_pull_* methods to CCEventReceiver
            if (methodName.startsWith("try_pull_")) {
                return CCEventReceiver.tryPull(methodName, peripheralPos);
            }
            // Generic implementation: return nil for all methods
            // Specific peripherals should override this
            return encodeNil();
        } catch (IOException e) {
            LOGGER.error("Failed to encode result for method '{}'", methodName, e);
            throw new PeripheralException("Failed to encode result: " + e.getMessage());
        }
    }

    @Override
    public byte[] callImmediate(String methodName, byte[] args,
                                ServerLevel level, BlockPos peripheralPos)
            throws PeripheralException {
        if (immediateMethods.contains(methodName)) {
            return callMethod(methodName, args, level, peripheralPos);
        }
        return null;
    }

    /**
     * Encode nil result.
     */
    protected byte[] encodeNil() throws IOException {
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
        packer.packNil();
        packer.close();
        return packer.toByteArray();
    }

    /**
     * Encode boolean result.
     */
    protected byte[] encodeBoolean(boolean value) throws IOException {
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
        packer.packBoolean(value);
        packer.close();
        return packer.toByteArray();
    }

    /**
     * Encode integer result.
     */
    protected byte[] encodeInt(int value) throws IOException {
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
        packer.packInt(value);
        packer.close();
        return packer.toByteArray();
    }

    /**
     * Encode double result.
     */
    protected byte[] encodeDouble(double value) throws IOException {
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
        packer.packDouble(value);
        packer.close();
        return packer.toByteArray();
    }

    /**
     * Encode string result.
     */
    protected byte[] encodeString(String value) throws IOException {
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
        packer.packString(value);
        packer.close();
        return packer.toByteArray();
    }

    /**
     * Encode array result.
     */
    protected byte[] encodeArray(Object... values) throws IOException {
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
        packer.packArrayHeader(values.length);
        for (Object value : values) {
            packValue(packer, value);
        }
        packer.close();
        return packer.toByteArray();
    }

    /**
     * Encode map result.
     */
    protected byte[] encodeMap(Map<String, Object> map) throws IOException {
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
        packer.packMapHeader(map.size());
        for (Map.Entry<String, Object> entry : map.entrySet()) {
            packer.packString(entry.getKey());
            packValue(packer, entry.getValue());
        }
        packer.close();
        return packer.toByteArray();
    }

    /**
     * Pack a value to MessagePack.
     */
    protected void packValue(MessageBufferPacker packer, Object value) throws IOException {
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
        } else if (value instanceof Map) {
            Map<?, ?> map = (Map<?, ?>) value;
            packer.packMapHeader(map.size());
            for (Map.Entry<?, ?> entry : map.entrySet()) {
                packer.packString(entry.getKey().toString());
                packValue(packer, entry.getValue());
            }
        } else {
            packer.packString(value.toString());
        }
    }
}
