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
import java.util.*;

/**
 * Create Station peripheral implementation.
 * Manages train station operations including assembly, scheduling, and train tracking.
 *
 * <p>Methods:
 * <ul>
 *   <li><b>assemble()</b> - Assemble a train</li>
 *   <li><b>disassemble()</b> - Disassemble the train</li>
 *   <li><b>setAssemblyMode(mode)</b> - Set assembly mode</li>
 *   <li><b>isInAssemblyMode()</b> - Check assembly mode (IMM)</li>
 *   <li><b>getStationName()</b> - Get station name (IMM)</li>
 *   <li><b>setStationName(name)</b> - Set station name</li>
 *   <li><b>isTrainPresent()</b> - Check if train present (IMM)</li>
 *   <li><b>isTrainImminent()</b> - Check if train imminent (IMM)</li>
 *   <li><b>isTrainEnroute()</b> - Check if train en route (IMM)</li>
 *   <li><b>getTrainName()</b> - Get train name (IMM)</li>
 *   <li><b>setTrainName(name)</b> - Set train name</li>
 *   <li><b>hasSchedule()</b> - Check if schedule exists (IMM)</li>
 *   <li><b>getSchedule()</b> - Get train schedule (IMM)</li>
 *   <li><b>setSchedule(schedule)</b> - Set train schedule</li>
 *   <li><b>canTrainReach(dest)</b> - Check if train can reach destination (IMM)</li>
 *   <li><b>distanceTo(dest)</b> - Get distance to destination (IMM)</li>
 *   <li><b>try_pull_train_arrive()</b> - Wait for train arrival event</li>
 *   <li><b>try_pull_train_depart()</b> - Wait for train departure event</li>
 * </ul>
 */
public class CreateStationPeripheral implements PeripheralType {

    private static final Logger LOGGER = LoggerFactory.getLogger(CreateStationPeripheral.class);
    private static final String TYPE_NAME = "create:station";

    private static final String[] METHODS = {
            "assemble",
            "disassemble",
            "setAssemblyMode",
            "isInAssemblyMode",
            "getStationName",
            "setStationName",
            "isTrainPresent",
            "isTrainImminent",
            "isTrainEnroute",
            "getTrainName",
            "setTrainName",
            "hasSchedule",
            "getSchedule",
            "setSchedule",
            "canTrainReach",
            "distanceTo",
            "try_pull_train_arrive",
            "try_pull_train_depart"
    };

    // Station state (simplified simulation)
    private boolean assemblyMode = false;
    private String stationName = "Station";
    private boolean trainPresent = false;
    private boolean trainImminent = false;
    private boolean trainEnroute = false;
    private String trainName = "Train";
    private boolean hasSchedule = false;
    private Map<String, Object> schedule = new HashMap<>();

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
                case "assemble" -> {
                    assemble();
                    yield encodeVoid();
                }
                case "disassemble" -> {
                    disassemble();
                    yield encodeVoid();
                }
                case "setAssemblyMode" -> {
                    boolean mode = unpackBoolean(args);
                    setAssemblyMode(mode);
                    yield encodeVoid();
                }
                case "isInAssemblyMode" -> encodeBoolean(assemblyMode);
                case "getStationName" -> encodeString(stationName);
                case "setStationName" -> {
                    String name = unpackString(args);
                    setStationName(name);
                    yield encodeVoid();
                }
                case "isTrainPresent" -> encodeBoolean(trainPresent);
                case "isTrainImminent" -> encodeBoolean(trainImminent);
                case "isTrainEnroute" -> encodeBoolean(trainEnroute);
                case "getTrainName" -> encodeString(trainName);
                case "setTrainName" -> {
                    String name = unpackString(args);
                    setTrainName(name);
                    yield encodeVoid();
                }
                case "hasSchedule" -> encodeBoolean(hasSchedule);
                case "getSchedule" -> encodeSchedule();
                case "setSchedule" -> {
                    Map<String, Object> sched = unpackSchedule(args);
                    setSchedule(sched);
                    yield encodeVoid();
                }
                case "canTrainReach" -> {
                    String dest = unpackString(args);
                    yield encodeCanTrainReach(canTrainReach(dest));
                }
                case "distanceTo" -> {
                    String dest = unpackString(args);
                    yield encodeDistanceTo(distanceTo(dest));
                }
                case "try_pull_train_arrive" -> encodeTryPullEvent(false);
                case "try_pull_train_depart" -> encodeTryPullEvent(false);
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
        // Immediate methods: all getters and checks
        if (methodName.startsWith("is") || methodName.startsWith("get") || methodName.startsWith("has")
                || "canTrainReach".equals(methodName) || "distanceTo".equals(methodName)) {
            return callMethod(methodName, args, level, peripheralPos);
        }
        return null;
    }

    /**
     * Unpack boolean from args.
     */
    private boolean unpackBoolean(byte[] args) throws PeripheralException {
        if (args == null || args.length == 0) {
            throw new PeripheralException("Missing boolean argument");
        }

        try (MessageUnpacker unpacker = MessagePack.newDefaultUnpacker(args)) {
            return unpacker.unpackBoolean();
        } catch (IOException e) {
            throw new PeripheralException("Failed to unpack boolean: " + e.getMessage());
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
     * Unpack schedule from args.
     */
    private Map<String, Object> unpackSchedule(byte[] args) throws PeripheralException {
        if (args == null || args.length == 0) {
            throw new PeripheralException("Missing schedule argument");
        }

        try (MessageUnpacker unpacker = MessagePack.newDefaultUnpacker(args)) {
            int size = unpacker.unpackMapHeader();
            Map<String, Object> result = new HashMap<>();
            for (int i = 0; i < size; i++) {
                String key = unpacker.unpackString();
                // Simplified: just store as string
                Object value = unpacker.unpackValue();
                result.put(key, value);
            }
            return result;
        } catch (IOException e) {
            throw new PeripheralException("Failed to unpack schedule: " + e.getMessage());
        }
    }

    /**
     * Assemble a train.
     */
    private void assemble() {
        trainPresent = true;
        LOGGER.debug("Train assembled at station");
    }

    /**
     * Disassemble the train.
     */
    private void disassemble() {
        trainPresent = false;
        LOGGER.debug("Train disassembled at station");
    }

    /**
     * Set assembly mode.
     */
    private void setAssemblyMode(boolean mode) {
        this.assemblyMode = mode;
    }

    /**
     * Set station name.
     */
    private void setStationName(String name) {
        this.stationName = name;
    }

    /**
     * Set train name.
     */
    private void setTrainName(String name) {
        this.trainName = name;
    }

    /**
     * Set train schedule.
     */
    private void setSchedule(Map<String, Object> sched) {
        this.schedule = sched;
        this.hasSchedule = !sched.isEmpty();
    }

    /**
     * Check if train can reach destination.
     */
    private boolean canTrainReach(String dest) {
        // Simplified: always reachable
        return true;
    }

    /**
     * Get distance to destination.
     */
    private double distanceTo(String dest) {
        // Simplified: random distance
        return Math.random() * 1000.0;
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
     * Encode boolean result.
     */
    private byte[] encodeBoolean(boolean value) throws IOException {
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
        packer.packBoolean(value);
        packer.close();
        return packer.toByteArray();
    }

    /**
     * Encode string result.
     */
    private byte[] encodeString(String value) throws IOException {
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
        packer.packString(value);
        packer.close();
        return packer.toByteArray();
    }

    /**
     * Encode schedule result.
     */
    private byte[] encodeSchedule() throws IOException {
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
        packer.packMapHeader(schedule.size());
        for (Map.Entry<String, Object> entry : schedule.entrySet()) {
            packer.packString(entry.getKey());
            packValue(packer, entry.getValue());
        }
        packer.close();
        return packer.toByteArray();
    }

    /**
     * Encode canTrainReach result: [reachable, reason].
     */
    private byte[] encodeCanTrainReach(boolean reachable) throws IOException {
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
        packer.packArrayHeader(2);
        packer.packBoolean(reachable);
        if (reachable) {
            packer.packNil();
        } else {
            packer.packString("Unreachable");
        }
        packer.close();
        return packer.toByteArray();
    }

    /**
     * Encode distanceTo result: [distance, reason].
     */
    private byte[] encodeDistanceTo(double distance) throws IOException {
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
        packer.packArrayHeader(2);
        packer.packDouble(distance);
        packer.packNil();
        packer.close();
        return packer.toByteArray();
    }

    /**
     * Encode try_pull event result (None if no event).
     */
    private byte[] encodeTryPullEvent(boolean hasEvent) throws IOException {
        MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
        if (hasEvent) {
            packer.packNil();
        } else {
            packer.packNil();
        }
        packer.close();
        return packer.toByteArray();
    }

    /**
     * Pack a value to MessagePack.
     */
    private void packValue(MessageBufferPacker packer, Object value) throws IOException {
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
        } else {
            packer.packString(value.toString());
        }
    }
}
