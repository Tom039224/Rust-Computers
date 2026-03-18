package com.rustcomputers.peripheral.impl;

import com.rustcomputers.peripheral.PeripheralException;
import com.rustcomputers.peripheral.PeripheralType;
import dan200.computercraft.api.lua.LuaFunction;
import dan200.computercraft.api.peripheral.IPeripheral;
import dan200.computercraft.api.peripheral.IPeripheralProvider;
import net.minecraft.core.BlockPos;
import net.minecraft.core.Direction;
import net.minecraft.server.level.ServerLevel;
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
 * CC:Tweaked {@link IPeripheral} を RustComputers の {@link PeripheralType} にブリッジする汎用アダプター。
 *
 * <p>対象 mod の {@link IPeripheralProvider} を受け取り、{@code @LuaFunction} アノテーション付きメソッドを
 * リフレクションで直接呼び出して結果を msgpack にエンコードする。</p>
 *
 * <p>動作原理:
 * <ol>
 *   <li>{@link IPeripheralProvider#getPeripheral} で {@link IPeripheral} を取得する</li>
 *   <li>メソッド名に対応する {@code @LuaFunction} アノテーション付きメソッドをリフレクションで検索する</li>
 *   <li>メソッドを直接呼び出し、戻り値を msgpack にエンコードする</li>
 * </ol>
 * </p>
 */
public class CcPeripheralBridge implements PeripheralType {

    private static final Logger LOGGER = LoggerFactory.getLogger(CcPeripheralBridge.class);

    private final String typeName;
    private final String[] methods;
    private final Set<String> immMethods;
    private final IPeripheralProvider provider;

    public CcPeripheralBridge(String typeName, String[] methods, Set<String> immMethods,
                               IPeripheralProvider provider) {
        this.typeName = typeName;
        this.methods = methods.clone();
        this.immMethods = immMethods;
        this.provider = provider;
    }

    public CcPeripheralBridge(String typeName, String[] methods, IPeripheralProvider provider) {
        this(typeName, methods, Set.of(), provider);
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
            LOGGER.warn("CcPeripheralBridge[{}]: no CC peripheral at {}", typeName, pos);
            return encodeNil();
        }

        try {
            Method method = findLuaMethod(peripheral, methodName);
            if (method == null) {
                LOGGER.debug("CcPeripheralBridge[{}]: @LuaFunction '{}' not found", typeName, methodName);
                return encodeNil();
            }

            Object[] javaArgs = decodeMsgpackArgs(args, method);
            Object result = method.invoke(peripheral, javaArgs);
            return encodeResult(result);
        } catch (Exception e) {
            LOGGER.warn("CcPeripheralBridge[{}]: callMethod '{}' failed: {}", typeName, methodName, e.getMessage());
            return encodeNil();
        }
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
    // CC peripheral lookup via IPeripheralProvider
    // ------------------------------------------------------------------

    @Nullable
    private IPeripheral getPeripheral(ServerLevel level, BlockPos pos) {
        for (Direction dir : Direction.values()) {
            try {
                var opt = provider.getPeripheral(level, pos, dir);
                if (opt.isPresent()) return opt.resolve().orElse(null);
            } catch (Exception ignored) {}
        }
        // Direction なしでも試みる (null direction)
        try {
            var opt = provider.getPeripheral(level, pos, null);
            if (opt.isPresent()) return opt.resolve().orElse(null);
        } catch (Exception ignored) {}
        return null;
    }

    // ------------------------------------------------------------------
    // @LuaFunction method lookup via reflection
    // ------------------------------------------------------------------

    @Nullable
    private Method findLuaMethod(IPeripheral peripheral, String methodName) {
        Class<?> cls = peripheral.getClass();
        while (cls != null && cls != Object.class) {
            for (Method m : cls.getDeclaredMethods()) {
                LuaFunction ann = m.getAnnotation(LuaFunction.class);
                if (ann == null) continue;
                // アノテーションに明示的な名前があればそれを使う
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

    // ------------------------------------------------------------------
    // Args decoding: msgpack → Java args for the method
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
                for (int i = 0; i < count && u.hasNext(); i++) {
                    list.add(unpackValue(u));
                }
            } else {
                list.add(unpackValue(u, fmt));
            }
        } catch (IOException e) {
            LOGGER.debug("CcPeripheralBridge: failed to decode args: {}", e.getMessage());
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
            LOGGER.warn("CcPeripheralBridge: encode failed: {}", e.getMessage());
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

    private byte[] encodeNil() {
        return new byte[]{(byte) 0xC0};
    }

    // ------------------------------------------------------------------
    // Factory: リフレクションで IPeripheralProvider をインスタンス化する
    // ------------------------------------------------------------------

    /**
     * クラス名を指定して {@link IPeripheralProvider} をリフレクションでインスタンス化し、
     * {@link CcPeripheralBridge} を生成する。
     *
     * @param typeName         peripheral の型名
     * @param methods          メソッド名一覧
     * @param immMethods       即時実行メソッドセット
     * @param providerClassName {@link IPeripheralProvider} 実装クラスの完全修飾名
     * @return 生成した {@link CcPeripheralBridge}、またはプロバイダーが見つからない場合は {@code null}
     */
    @Nullable
    public static CcPeripheralBridge create(String typeName, String[] methods, Set<String> immMethods,
                                             String providerClassName) {
        try {
            Class<?> cls = Class.forName(providerClassName);
            IPeripheralProvider provider = (IPeripheralProvider) cls.getDeclaredConstructor().newInstance();
            return new CcPeripheralBridge(typeName, methods, immMethods, provider);
        } catch (ClassNotFoundException e) {
            LOGGER.debug("CcPeripheralBridge: provider class not found: {}", providerClassName);
            return null;
        } catch (Exception e) {
            LOGGER.warn("CcPeripheralBridge: failed to instantiate provider {}: {}", providerClassName, e.getMessage());
            return null;
        }
    }

    @Nullable
    public static CcPeripheralBridge create(String typeName, String[] methods, String providerClassName) {
        return create(typeName, methods, Set.of(), providerClassName);
    }
}
