package com.rustcomputers.peripheral.impl;

import com.rustcomputers.peripheral.PeripheralException;
import com.rustcomputers.peripheral.PeripheralType;
import dan200.computercraft.api.lua.LuaFunction;
import dan200.computercraft.api.peripheral.IPeripheral;
import net.minecraft.core.BlockPos;
import net.minecraft.core.Direction;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.level.block.entity.BlockEntity;
import org.msgpack.core.MessageBufferPacker;
import org.msgpack.core.MessagePack;
import org.msgpack.core.MessageUnpacker;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.annotation.Nullable;
import java.io.IOException;
import java.lang.reflect.Field;
import java.lang.reflect.Method;
import java.util.*;

/**
 * BlockEntity の {@code Capabilities.CAPABILITY_PERIPHERAL} から {@link IPeripheral} を取得し、
 * {@code @LuaFunction} アノテーション付きメソッドをリフレクションで呼び出すブリッジ。
 *
 * <p>Control-Craft や Create の speedometer/stressometer のように、
 * {@link dan200.computercraft.api.peripheral.IPeripheralProvider} ではなく
 * BlockEntity が直接 capability として {@link IPeripheral} を提供するケースに使用する。</p>
 */
public class CcBlockEntityBridge implements PeripheralType {

    private static final Logger LOGGER = LoggerFactory.getLogger(CcBlockEntityBridge.class);

    private final String typeName;
    private final String[] methods;
    private final Set<String> immMethods;

    public CcBlockEntityBridge(String typeName, String[] methods, Set<String> immMethods) {
        this.typeName = typeName;
        this.methods = methods.clone();
        this.immMethods = immMethods;
    }

    public CcBlockEntityBridge(String typeName, String[] methods) {
        this(typeName, methods, Set.of());
    }

    @Override
    public String getTypeName() { return typeName; }

    @Override
    public String[] getMethodNames() { return methods.clone(); }

    @Override
    public byte[] callMethod(String methodName, byte[] args,
                             ServerLevel level, BlockPos pos) throws PeripheralException {
        // try_pull_* イベント系は CCEventReceiver に委譲する
        if (methodName.startsWith("try_pull_")) {
            return CCEventReceiver.tryPull(methodName, pos);
        }

        IPeripheral peripheral = getPeripheral(level, pos);
        if (peripheral == null) {
            LOGGER.warn("CcBlockEntityBridge[{}]: no CC peripheral at {}", typeName, pos);
            return encodeNil();
        }
        return invokeMethod(peripheral, methodName, args);
    }

    @Override
    public byte[] callImmediate(String methodName, byte[] args,
                                ServerLevel level, BlockPos pos) throws PeripheralException {
        if (immMethods.contains(methodName)) {
            return callMethod(methodName, args, level, pos);
        }
        return null;
    }

    // ------------------------------------------------------------------
    // BlockEntity capability lookup (reflection — CC:Tweaked is compile-only)
    // ------------------------------------------------------------------

    private static volatile boolean capInitTried = false;
    @Nullable private static volatile Object ccCapabilityPeripheral = null;

    @Nullable
    private IPeripheral getPeripheral(ServerLevel level, BlockPos pos) {
        BlockEntity be = level.getBlockEntity(pos);
        if (be == null) return null;

        try {
            if (!capInitTried) {
                capInitTried = true;
                Class<?> capClass = Class.forName("dan200.computercraft.shared.Capabilities");
                Field f = capClass.getDeclaredField("CAPABILITY_PERIPHERAL");
                f.setAccessible(true);
                ccCapabilityPeripheral = f.get(null);
            }
            if (ccCapabilityPeripheral == null) return null;

            Method getCap = be.getClass().getMethod(
                    "getCapability",
                    Class.forName("net.minecraftforge.common.capabilities.Capability"),
                    Direction.class
            );

            for (Direction dir : Direction.values()) {
                try {
                    Object p = unwrapLazyOptional(getCap.invoke(be, ccCapabilityPeripheral, dir));
                    if (p instanceof IPeripheral ip) return ip;
                } catch (Throwable ignored) {}
            }
            try {
                Object p = unwrapLazyOptional(getCap.invoke(be, ccCapabilityPeripheral, (Object) null));
                if (p instanceof IPeripheral ip) return ip;
            } catch (Throwable ignored) {}
        } catch (Throwable e) {
            LOGGER.debug("CcBlockEntityBridge[{}]: capability lookup failed at {}: {}", typeName, pos, e.getMessage());
        }
        return null;
    }

    @Nullable
    private static Object unwrapLazyOptional(@Nullable Object lazyOptional) {
        if (lazyOptional == null) return null;
        try {
            Method orElse = lazyOptional.getClass().getMethod("orElse", Object.class);
            return orElse.invoke(lazyOptional, (Object) null);
        } catch (Throwable ignored) {
            return null;
        }
    }

    // ------------------------------------------------------------------
    // @LuaFunction method invocation via reflection
    // ------------------------------------------------------------------

    private byte[] invokeMethod(IPeripheral peripheral, String methodName, byte[] args) {
        try {
            Method method = findLuaMethod(peripheral, methodName);
            if (method == null) {
                LOGGER.debug("CcBlockEntityBridge[{}]: @LuaFunction '{}' not found", typeName, methodName);
                return encodeNil();
            }
            Object[] javaArgs = decodeMsgpackArgs(args, method);
            Object result = method.invoke(peripheral, javaArgs);
            return encodeResult(result);
        } catch (Exception e) {
            LOGGER.warn("CcBlockEntityBridge[{}]: callMethod '{}' failed: {}", typeName, methodName, e.getMessage());
            return encodeNil();
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
                        if (name.equals(methodName)) { m.setAccessible(true); return m; }
                    }
                } else if (m.getName().equals(methodName)) {
                    m.setAccessible(true); return m;
                }
            }
            cls = cls.getSuperclass();
        }
        return null;
    }

    // ------------------------------------------------------------------
    // Args decoding: msgpack → Java
    // ------------------------------------------------------------------

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
                for (int i = 0; i < count && u.hasNext(); i++) list.add(unpackValue(u));
            } else {
                list.add(unpackValue(u, fmt));
            }
        } catch (IOException e) {
            LOGGER.debug("CcBlockEntityBridge: failed to decode args: {}", e.getMessage());
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
                for (int i = 0; i < n; i++) { map.put(unpackValue(u), unpackValue(u)); }
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

    // ------------------------------------------------------------------
    // Result encoding: Object → msgpack
    // ------------------------------------------------------------------

    private byte[] encodeResult(@Nullable Object value) {
        try {
            MessageBufferPacker packer = MessagePack.newDefaultBufferPacker();
            packValue(packer, value);
            packer.close();
            return packer.toByteArray();
        } catch (IOException e) {
            LOGGER.warn("CcBlockEntityBridge: encode failed: {}", e.getMessage());
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
            for (var entry : map.entrySet()) { packValue(p, entry.getKey()); packValue(p, entry.getValue()); }
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

    private byte[] encodeNil() { return new byte[]{(byte) 0xC0}; }
}
