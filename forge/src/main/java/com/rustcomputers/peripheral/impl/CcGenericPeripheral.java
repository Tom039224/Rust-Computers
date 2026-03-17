package com.rustcomputers.peripheral.impl;

import com.rustcomputers.peripheral.MsgPack;
import com.rustcomputers.peripheral.PeripheralException;
import com.rustcomputers.peripheral.PeripheralType;
import net.minecraft.core.BlockPos;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.level.block.entity.BlockEntity;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.annotation.Nullable;
import java.lang.annotation.Annotation;
import java.lang.reflect.Field;
import java.lang.reflect.Method;
import java.util.*;
import java.util.concurrent.ConcurrentHashMap;
import net.minecraft.core.Direction;
import net.minecraft.world.level.Level;

/**
 * CC:Tweaked の IPeripheral を通じて他 Mod のペリフェラルメソッドを呼び出す汎用ブリッジ。
 * Generic bridge that calls other mods' peripheral methods through CC:Tweaked's IPeripheral.
 *
 * <p>CC:Tweaked がロードされている場合、Forge Capability を通じて BlockEntity から
 * IPeripheral を取得し、{@code @LuaFunction} アノテーション付きメソッドを
 * リフレクションで呼び出す。結果は MsgPack にエンコードして返す。</p>
 *
 * <p>When CC:Tweaked is loaded, obtains IPeripheral from BlockEntity via Forge Capability,
 * then calls {@code @LuaFunction}-annotated methods via reflection.
 * Results are encoded as MsgPack.</p>
 */
public class CcGenericPeripheral implements PeripheralType {

    private static final Logger LOGGER = LoggerFactory.getLogger(CcGenericPeripheral.class);

    // ------------------------------------------------------------------
    // インスタンスフィールド / Instance fields
    // ------------------------------------------------------------------

    private final String typeName;
    private final String[] methods;
    private final Set<String> immediateMethods;

    // ------------------------------------------------------------------
    // 静的リフレクションキャッシュ / Static reflection cache
    // ------------------------------------------------------------------

    /** CC:Tweaked Capabilities.CAPABILITY_PERIPHERAL フィールドへの参照 */
    @Nullable private static Object capabilityPeripheral;
    /** IPeripheral クラス */
    @Nullable private static Class<?> iPeripheralClass;
    /** @LuaFunction アノテーションクラス */
    @Nullable private static Class<? extends Annotation> luaFunctionAnnotation;

    private static boolean capabilityInitialized = false;
    private static boolean capabilityAvailable = false;

    /** peripheral class → (methodName → Method) */
    private static final ConcurrentHashMap<Class<?>, Map<String, Method>> METHOD_CACHE = new ConcurrentHashMap<>();

    // ------------------------------------------------------------------
    // コンストラクタ / Constructor
    // ------------------------------------------------------------------

    /**
     * 汎用ペリフェラルを構築する。
     * Construct a generic peripheral.
     *
     * @param typeName        ペリフェラル型名 / peripheral type name
     * @param methods         サポートメソッド名 / supported method names
     * @param immediateMethods callImmediate で安全なメソッド名セット / methods safe for callImmediate
     */
    public CcGenericPeripheral(String typeName, String[] methods, Set<String> immediateMethods) {
        this.typeName = typeName;
        this.methods = methods;
        this.immediateMethods = immediateMethods;
    }

    /**
     * 全メソッドを immediate 対応とする簡易コンストラクタ。
     * Convenience constructor where all methods support immediate calls.
     */
    public CcGenericPeripheral(String typeName, String[] methods) {
        this(typeName, methods, new HashSet<>(Arrays.asList(methods)));
    }

    // ------------------------------------------------------------------
    // PeripheralType 実装 / PeripheralType implementation
    // ------------------------------------------------------------------

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
        return invokeViaPeripheral(methodName, args, level, peripheralPos);
    }

    @Override
    public byte[] callImmediate(String methodName, byte[] args,
                                ServerLevel level, BlockPos peripheralPos)
            throws PeripheralException {
        if (!immediateMethods.contains(methodName)) {
            return null; // 非対応メソッド — 通常の callMethod にフォールバック
        }
        return invokeViaPeripheral(methodName, args, level, peripheralPos);
    }

    // ------------------------------------------------------------------
    // CC:Tweaked Capability 初期化 / CC:T Capability initialization
    // ------------------------------------------------------------------

    @SuppressWarnings("unchecked")
    private static synchronized void ensureCapability() throws PeripheralException {
        if (capabilityInitialized) {
            if (!capabilityAvailable) {
                throw new PeripheralException("CC:Tweaked peripheral capability not available");
            }
            return;
        }
        capabilityInitialized = true;
        try {
            // IPeripheral クラスのロード
            iPeripheralClass = Class.forName("dan200.computercraft.api.peripheral.IPeripheral");

            // @LuaFunction アノテーションのロード
            luaFunctionAnnotation = (Class<? extends Annotation>)
                    Class.forName("dan200.computercraft.api.lua.LuaFunction");

            // Capabilities.CAPABILITY_PERIPHERAL フィールドの取得
            Class<?> capClass = Class.forName("dan200.computercraft.shared.Capabilities");
            Field field = capClass.getDeclaredField("CAPABILITY_PERIPHERAL");
            field.setAccessible(true);
            capabilityPeripheral = field.get(null);

            capabilityAvailable = true;
            LOGGER.info("CcGenericPeripheral: CC:Tweaked capability initialized");
        } catch (Exception e) {
            LOGGER.error("CcGenericPeripheral: CC:Tweaked capability init failed: {}", e.getMessage());
            throw new PeripheralException("CC:Tweaked capability init failed: " + e.getMessage(), e);
        }
    }

    // ------------------------------------------------------------------
    // IPeripheral 取得 / Get IPeripheral
    // ------------------------------------------------------------------

    /**
     * ブロックエンティティから CC:Tweaked の IPeripheral を取得する。
     * Get CC:Tweaked's IPeripheral from a BlockEntity.
     *
     * <p>BlockEntity.getCapability(cap, null) を呼び出し、
     * LazyOptional から IPeripheral を取り出す。</p>
     */
    @Nullable
    private static Object getPeripheral(BlockEntity be) throws Exception {
        // be.getCapability(capabilityPeripheral, null)
        Method getCapMethod = be.getClass().getMethod("getCapability",
                Class.forName("net.minecraftforge.common.capabilities.Capability"),
                net.minecraft.core.Direction.class);
        Object lazyOpt = getCapMethod.invoke(be, capabilityPeripheral, (Object) null);

        // LazyOptional.orElse(null)
        Method orElse = lazyOpt.getClass().getMethod("orElse", Object.class);
        return orElse.invoke(lazyOpt, (Object) null);
    }

    /**
     * CC:Tweaked の IPeripheralProvider チェーンを反復して IPeripheral を取得する。
     * BlockEntity を持たないペリフェラル (Some Peripherals の Radar 等) に使用。
     *
     * Get IPeripheral by iterating CC:Tweaked's IPeripheralProvider chain.
     * Used for non-BlockEntity peripherals (e.g. Some Peripherals Radar).
     */
    @Nullable
    private static Object getPeripheralViaProviders(Level level, BlockPos pos) {
        try {
            Class<?> peripheralsClass = Class.forName("dan200.computercraft.impl.Peripherals");
            Field providersField = peripheralsClass.getDeclaredField("providers");
            providersField.setAccessible(true);
            java.util.Collection<?> providers = (java.util.Collection<?>) providersField.get(null);

            for (Object provider : providers) {
                // getPeripheral(Level, BlockPos, Direction) を探す
                Method getPeripheralMethod = null;
                for (Method m : provider.getClass().getMethods()) {
                    if (m.getName().equals("getPeripheral") && m.getParameterCount() == 3) {
                        getPeripheralMethod = m;
                        break;
                    }
                }
                if (getPeripheralMethod == null) continue;

                Object lazyOpt = getPeripheralMethod.invoke(provider, level, pos, Direction.NORTH);
                // LazyOptional.isPresent()
                Method isPresent = lazyOpt.getClass().getMethod("isPresent");
                if ((boolean) isPresent.invoke(lazyOpt)) {
                    Method orElse = lazyOpt.getClass().getMethod("orElse", Object.class);
                    Object result = orElse.invoke(lazyOpt, (Object) null);
                    LOGGER.debug("getPeripheralViaProviders: found IPeripheral at {} via {}", pos, provider.getClass().getSimpleName());
                    return result;
                }
            }
        } catch (Exception e) {
            LOGGER.debug("getPeripheralViaProviders failed at {}: {}", pos, e.getMessage());
        }
        return null;
    }

    // ------------------------------------------------------------------
    // @LuaFunction メソッド検索 / Find @LuaFunction methods
    // ------------------------------------------------------------------

    private static boolean hasLuaAnnotation(Method m) {
        if (luaFunctionAnnotation != null && m.isAnnotationPresent(luaFunctionAnnotation)) {
            return true;
        }
        for (Annotation ann : m.getAnnotations()) {
            String name = ann.annotationType().getSimpleName();
            if (name.equals("LuaFunction") || name.equals("LuaMethod")) {
                return true;
            }
        }
        return false;
    }

    /**
     * 対象クラスの @LuaFunction メソッドをキャッシュ付きで検索する。
     * Find @LuaFunction methods on the target class (with caching).
     */
    private static Map<String, Method> findLuaMethods(Class<?> clazz) {
        return METHOD_CACHE.computeIfAbsent(clazz, cls -> {
            Map<String, Method> map = new HashMap<>();
            for (Method m : cls.getMethods()) {
                if (hasLuaAnnotation(m)) {
                    m.setAccessible(true);
                    map.put(m.getName(), m);
                }
            }
            // スーパークラスも走査 / Also scan superclasses
            Class<?> parent = cls.getSuperclass();
            while (parent != null && parent != Object.class) {
                for (Method m : parent.getDeclaredMethods()) {
                    if (hasLuaAnnotation(m) && !map.containsKey(m.getName())) {
                        m.setAccessible(true);
                        map.put(m.getName(), m);
                    }
                }
                parent = parent.getSuperclass();
            }
            return map;
        });
    }

    // ------------------------------------------------------------------
    // メソッド呼び出し / Method invocation
    // ------------------------------------------------------------------

    /**
     * CC:Tweaked IPeripheral を介してメソッドを呼び出す。
     * Invoke a method through CC:Tweaked's IPeripheral.
     */
    private byte[] invokeViaPeripheral(String methodName, byte[] args,
                                       ServerLevel level, BlockPos peripheralPos)
            throws PeripheralException {
        ensureCapability();

        // BlockEntity は持たないペリフェラル (RadarPeripheral 等) の場合 null になり得る
        // BlockEntity may be null for non-BE peripherals (e.g. RadarPeripheral)
        BlockEntity be = level.getBlockEntity(peripheralPos);

        try {
            // CC:T IPeripheral を取得 / Get CC:T IPeripheral
            // まず BE から capability で取得、なければ IPeripheralProvider チェーンを試みる
            // First try BE capability, then fall back to IPeripheralProvider chain
            Object peripheral = null;
            if (be != null) {
                peripheral = getPeripheral(be);
            }
            if (peripheral == null) {
                peripheral = getPeripheralViaProviders(level, peripheralPos);
            }

            // ペリフェラルが一切見つからない場合はエラー
            // Error if no peripheral found by any means
            if (peripheral == null && be == null) {
                CCEventReceiver.detach(null, peripheralPos);
                throw new PeripheralException("No peripheral at " + peripheralPos);
            }

            // IComputerAccess モックをアタッチして events を受信できるようにする (初回のみ)
            // Attach mock IComputerAccess so events can be received (idempotent)
            if (peripheral != null) {
                CCEventReceiver.ensureAttached(peripheral, peripheralPos);
            }

            // try_pull_* はイベントキューから返す (@LuaFunction として存在しないため)
            // try_pull_* methods return from event queue (they don't exist as @LuaFunction)
            if (methodName.startsWith("try_pull_")) {
                return CCEventReceiver.tryPull(methodName, peripheralPos);
            }

            // ターゲット: IPeripheral → BE → null (後で nil を返す)
            // Target: IPeripheral → BE → null (returns nil later)
            Object target = peripheral != null ? peripheral : be;

            Map<String, Method> luaMethods = findLuaMethods(target.getClass());
            Method method = luaMethods.get(methodName);

            if (method == null) {
                // @LuaFunction が見つからない場合、BE 上のメソッドも検索 (BE が存在する場合のみ)
                // Also search BE for @LuaFunction (only when BE exists)
                if (peripheral != null && be != null) {
                    luaMethods = findLuaMethods(be.getClass());
                    method = luaMethods.get(methodName);
                    if (method != null) {
                        target = be;
                    }
                }
            }

            if (method == null) {
                // メソッドが見つからない場合は nil を返す (互換性のため)
                // Return nil if method not found (for compatibility)
                LOGGER.warn("{}: method '{}' not found via reflection, returning nil", typeName, methodName);
                return MsgPack.nil();
            }

            // 引数を構築 / Build arguments
            Object[] javaArgs = buildArgs(method, args);

            // メソッド呼び出し / Invoke method
            Object result = method.invoke(target, javaArgs);

            // 結果をエンコード / Encode result
            return encodeResult(result);

        } catch (PeripheralException ex) {
            throw ex;
        } catch (java.lang.reflect.InvocationTargetException ex) {
            Throwable cause = ex.getCause();
            String msg = cause != null ? cause.getMessage() : ex.getMessage();
            throw new PeripheralException(typeName + "." + methodName + " failed: " + msg, ex);
        } catch (Exception ex) {
            throw new PeripheralException(
                    typeName + "." + methodName + " reflection call failed: " + ex.getMessage(), ex);
        }
    }

    // ------------------------------------------------------------------
    // 引数構築 / Argument building
    // ------------------------------------------------------------------

    /**
     * @LuaFunction メソッドの引数を MsgPack データから構築する。
     * Build method arguments from MsgPack data based on @LuaFunction signature.
     *
     * <p>CC:Tweaked の @LuaFunction メソッド引数パターン:
     * - プリミティブ型: int, long, double, boolean, float
     * - String
     * - IComputerAccess, ILuaContext → null を渡す（使用しない）
     * - IArguments → MsgPack ベースのラッパーを渡す
     * - Optional<type> → 値があれば渡す
     * - Map<?, ?> → MsgPack マップをデコード</p>
     */
    private static Object[] buildArgs(Method method, byte[] args) throws PeripheralException {
        Class<?>[] paramTypes = method.getParameterTypes();
        Object[] result = new Object[paramTypes.length];
        int argIndex = 0; // MsgPack 引数のインデックス

        for (int i = 0; i < paramTypes.length; i++) {
            Class<?> pt = paramTypes[i];
            String ptName = pt.getName();

            // CC:T 内部型 — null を渡す
            if (ptName.equals("dan200.computercraft.api.peripheral.IComputerAccess")
                || ptName.equals("dan200.computercraft.api.lua.ILuaContext")) {
                result[i] = null;
                continue;
            }

            // IArguments — 簡易ラッパーを渡す
            if (ptName.equals("dan200.computercraft.api.lua.IArguments")) {
                result[i] = null; // IArguments ラッパーは未実装 — null で渡す
                continue;
            }

            int offset = MsgPack.argOffset(args, argIndex);
            if (offset < 0) {
                // 引数不足 — Optional なら null、プリミティブならデフォルト値
                if (pt == Optional.class) {
                    result[i] = Optional.empty();
                } else if (pt == int.class || pt == Integer.class) {
                    result[i] = 0;
                } else if (pt == long.class || pt == Long.class) {
                    result[i] = 0L;
                } else if (pt == double.class || pt == Double.class) {
                    result[i] = 0.0;
                } else if (pt == float.class || pt == Float.class) {
                    result[i] = 0.0f;
                } else if (pt == boolean.class || pt == Boolean.class) {
                    result[i] = false;
                } else if (pt == String.class) {
                    result[i] = null;
                } else {
                    result[i] = null;
                }
                argIndex++;
                continue;
            }

            // プリミティブ型のデコード
            if (pt == int.class || pt == Integer.class) {
                result[i] = MsgPack.decodeInt(args, offset);
            } else if (pt == long.class || pt == Long.class) {
                result[i] = (long) MsgPack.decodeInt(args, offset);
            } else if (pt == double.class || pt == Double.class) {
                result[i] = MsgPack.decodeF64(args, offset);
            } else if (pt == float.class || pt == Float.class) {
                result[i] = (float) MsgPack.decodeF64(args, offset);
            } else if (pt == boolean.class || pt == Boolean.class) {
                result[i] = decodeBool(args, offset);
            } else if (pt == String.class) {
                result[i] = MsgPack.decodeStr(args, offset);
            } else if (pt == Object[].class) {
                // Read all remaining arguments into Object[] (usually for varargs or LuaMethod wrappers)
                List<Object> list = new ArrayList<>();
                int[] offsetInOut = new int[]{offset};
                while (offsetInOut[0] < args.length && offsetInOut[0] > 0) {
                    list.add(MsgPack.decodeAny(args, offsetInOut));
                }
                result[i] = list.toArray(new Object[0]);
                argIndex = 999; // mark all processed
                continue;
            } else if (pt == Optional.class) {
                // Optional<T> — 中身の型を推定して渡す
                // Note: 実行時にジェネリクス型消去されるため、簡易的に Object にフォールバック
                if (isNil(args, offset)) {
                    result[i] = Optional.empty();
                } else {
                    // try to decode as number or string
                    try {
                        result[i] = Optional.of(MsgPack.decodeF64(args, offset));
                    } catch (Exception e) {
                        try {
                            result[i] = Optional.of(MsgPack.decodeStr(args, offset));
                        } catch (Exception e2) {
                            result[i] = Optional.empty();
                        }
                    }
                }
            } else {
                // 未対応型 — null
                result[i] = null;
            }
            argIndex++;
        }

        return result;
    }

    // ------------------------------------------------------------------
    // 結果エンコード / Result encoding
    // ------------------------------------------------------------------

    /**
     * Java の返り値を MsgPack にエンコードする。
     * Encode a Java return value to MsgPack.
     */
    private static byte[] encodeResult(@Nullable Object result) throws PeripheralException {
        if (result == null) {
            return MsgPack.nil();
        }

        // MethodResult の処理 — CC:T の MethodResult を使っている場合
        String className = result.getClass().getName();
        if (className.contains("MethodResult")) {
            try {
                // MethodResult.of() は Object[] を内部に持つ
                // getResult() で Object[] を取得
                Method getResult = result.getClass().getMethod("getResult");
                Object[] values = (Object[]) getResult.invoke(result);
                if (values == null || values.length == 0) {
                    return MsgPack.nil();
                }
                if (values.length == 1) {
                    return MsgPack.packAny(values[0]);
                }
                // 複数値 → 配列としてエンコード
                List<byte[]> encoded = new ArrayList<>();
                for (Object v : values) {
                    encoded.add(MsgPack.packAny(v));
                }
                return MsgPack.array(encoded);
            } catch (Exception e) {
                LOGGER.warn("Failed to decode MethodResult: {}", e.getMessage());
                return MsgPack.nil();
            }
        }

        return MsgPack.packAny(result);
    }

    // ------------------------------------------------------------------
    // ヘルパー / Helpers
    // ------------------------------------------------------------------

    /** MsgPack の nil チェック */
    private static boolean isNil(byte[] data, int offset) {
        return data != null && offset < data.length && (data[offset] & 0xFF) == 0xC0;
    }

    /** MsgPack の bool デコード */
    private static boolean decodeBool(byte[] data, int offset) {
        if (data == null || offset >= data.length) return false;
        int b = data[offset] & 0xFF;
        return b == 0xC3; // true
    }

    // ------------------------------------------------------------------
    // ファクトリメソッド / Factory methods
    // ------------------------------------------------------------------

    /**
     * 読み取り専用メソッドのみの汎用ペリフェラルを生成するファクトリ。
     * Factory for a generic peripheral where all methods are read-only (safe for IMM).
     */
    public static CcGenericPeripheral readOnly(String typeName, String... methods) {
        return new CcGenericPeripheral(typeName, methods);
    }

    /**
     * メソッド毎に IMM 対応を指定するファクトリ。
     * Factory with per-method IMM specification.
     */
    public static CcGenericPeripheral withActions(String typeName, String[] allMethods, String... immMethods) {
        return new CcGenericPeripheral(typeName, allMethods, new HashSet<>(Arrays.asList(immMethods)));
    }
}
