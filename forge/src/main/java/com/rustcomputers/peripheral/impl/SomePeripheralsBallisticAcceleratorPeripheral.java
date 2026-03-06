package com.rustcomputers.peripheral.impl;

import com.rustcomputers.peripheral.MsgPack;
import com.rustcomputers.peripheral.PeripheralException;
import com.rustcomputers.peripheral.PeripheralType;
import net.minecraft.core.BlockPos;
import net.minecraft.server.level.ServerLevel;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.annotation.Nullable;
import java.lang.reflect.Method;

/**
 * Some Peripherals の BallisticAccelerator ブロック向けペリフェラル。
 * Peripheral for Some Peripherals' BallisticAccelerator block
 * (some_peripherals:ballistic_accelerator).
 *
 * <p>ブロックエンティティなし。BallisticFunctions の静的計算のみ使用する。</p>
 * <p>No BlockEntity required. Delegates to BallisticFunctions static computations.</p>
 *
 * <h3>メソッド一覧 / Methods</h3>
 * <pre>
 * getDrag(base_drag, dimensional_drag_multiplier) → f64
 *   空気抵抗係数を計算して返す / Computes drag coefficient.
 * </pre>
 *
 * <h3>Kotlin 実装 / Kotlin implementation</h3>
 * <pre>
 * net.spaceeye.someperipherals.stuff.BallisticFunctions (Kotlin object / singleton)
 *   INSTANCE.getDrag(Double, Double): Double
 * </pre>
 */
public class SomePeripheralsBallisticAcceleratorPeripheral implements PeripheralType {

    private static final Logger LOGGER =
            LoggerFactory.getLogger(SomePeripheralsBallisticAcceleratorPeripheral.class);

    private static final String[] METHODS = {"getDrag"};

    // ------------------------------------------------------------------
    // リフレクションキャッシュ / Reflection cache
    // ------------------------------------------------------------------

    @Nullable private static Object   ballisticFunctionsInstance;
    @Nullable private static Method   mGetDrag;

    private static boolean reflectionInitialized = false;
    private static boolean reflectionOk = false;

    // ------------------------------------------------------------------
    // リフレクション初期化 / Reflection initialization
    // ------------------------------------------------------------------

    private static synchronized void ensureReflection() throws PeripheralException {
        if (reflectionInitialized) {
            if (!reflectionOk) throw new PeripheralException(
                    "Some Peripherals BallisticFunctions reflection unavailable");
            return;
        }
        reflectionInitialized = true;
        try {
            Class<?> bfCls = Class.forName(
                    "net.spaceeye.someperipherals.stuff.BallisticFunctions");

            // Kotlin object → INSTANCE フィールド / Kotlin object → INSTANCE field
            try {
                java.lang.reflect.Field instanceField = bfCls.getField("INSTANCE");
                ballisticFunctionsInstance = instanceField.get(null);
            } catch (NoSuchFieldException e) {
                // companion object や @JvmStatic の場合は null のまま
                // companion object or @JvmStatic — leave as null (static call)
                ballisticFunctionsInstance = null;
            }

            mGetDrag = bfCls.getMethod("getDrag", double.class, double.class);

            reflectionOk = true;
            LOGGER.info(
                    "SomePeripheralsBallisticAcceleratorPeripheral: reflection initialized");

        } catch (Exception e) {
            LOGGER.error("SomePeripheralsBallisticAcceleratorPeripheral: "
                    + "reflection init failed: {}", e.getMessage());
            throw new PeripheralException(
                    "SP BallisticFunctions reflection init failed: " + e.getMessage(), e);
        }
    }

    // ------------------------------------------------------------------
    // PeripheralType 実装 / PeripheralType implementation
    // ------------------------------------------------------------------

    @Override
    public String getTypeName() {
        return "ballistic_accelerator";
    }

    @Override
    public String[] getMethodNames() {
        return METHODS.clone();
    }

    @Override
    public byte[] callMethod(String methodName, byte[] args,
                             ServerLevel level, BlockPos peripheralPos)
            throws PeripheralException {
        ensureReflection();

        try {
            switch (methodName) {
                case "getDrag": {
                    int off0 = MsgPack.argOffset(args, 0);
                    int off1 = MsgPack.argOffset(args, 1);
                    if (off0 < 0 || off1 < 0) {
                        throw new PeripheralException(
                                "getDrag requires 2 arguments: base_drag, dimensional_drag_multiplier");
                    }
                    double baseDrag    = MsgPack.decodeF64(args, off0);
                    double dimDragMult = MsgPack.decodeF64(args, off1);
                    Object result = mGetDrag.invoke(ballisticFunctionsInstance,
                            baseDrag, dimDragMult);
                    return MsgPack.packAny(result);
                }

                default:
                    throw new PeripheralException("Unknown method: " + methodName);
            }
        } catch (PeripheralException ex) {
            throw ex;
        } catch (java.lang.reflect.InvocationTargetException ex) {
            Throwable cause = ex.getCause();
            throw new PeripheralException(
                    "BallisticAccelerator " + methodName + " failed: "
                    + (cause != null ? cause.getMessage() : ex.getMessage()), ex);
        } catch (Exception ex) {
            throw new PeripheralException(
                    "BallisticAccelerator reflection call failed: " + ex.getMessage(), ex);
        }
    }
}
