package com.rustcomputers.peripheral.impl;

import com.rustcomputers.peripheral.PeripheralException;
import com.rustcomputers.peripheral.PeripheralType;
import net.minecraft.core.BlockPos;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.level.block.entity.BlockEntity;
import org.msgpack.core.MessageBufferPacker;
import org.msgpack.core.MessagePack;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.lang.reflect.Method;
import java.util.Set;

/**
 * Tom's Peripherals WatchdogTimer peripheral implementation.
 *
 * <p>Uses reflection to read from {@code WatchDogTimerBlockEntity} since
 * Toms-Peripherals is not a compile-time dependency.</p>
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
                default           -> encodeNil();
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

    private byte[] encodeNil() throws IOException {
        MessageBufferPacker p = MessagePack.newDefaultBufferPacker();
        p.packNil();
        p.close();
        return p.toByteArray();
    }
}
