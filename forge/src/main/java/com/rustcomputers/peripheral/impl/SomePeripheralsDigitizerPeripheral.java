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
import java.lang.reflect.Constructor;
import java.lang.reflect.Method;
import java.util.ArrayList;
import java.util.List;

/**
 * Some Peripherals の Digitizer ブロック向けペリフェラル。
 * Peripheral for Some Peripherals' Digitizer block (some_peripherals:digitizer).
 *
 * <p>DigitizerBlockEntity と DigitalItemsSavedData を利用する。
 * Uses DigitizerBlockEntity and DigitalItemsSavedData.</p>
 *
 * <p>CC:Tweaked の {@code ObjectArguments} でリフレクション経由の引数渡しを行う。
 * Uses CC:Tweaked's {@code ObjectArguments} for reflection-based argument passing.</p>
 *
 * <h3>メソッド一覧 / Methods</h3>
 * <pre>
 * digitizeAmount(amount?)              → str (UUID) | map (error)
 * rematerializeAmount(uuid, amount?)   → bool | map (error)
 * mergeDigitalItems(into, from, amt?)  → bool | map (error)
 * separateDigitalItem(from, amount)    → str (UUID) | map (error)
 * getItemLimitInSlot()                 → i32
 * </pre>
 *
 * <h3>Kotlin 実装クラス / Kotlin implementation class</h3>
 * <pre>
 * net.spaceeye.someperipherals.integrations.cc.peripherals.DigitizerPeripheral
 *   constructor(Level, BlockPos, BlockEntity)
 * </pre>
 */
public class SomePeripheralsDigitizerPeripheral implements PeripheralType {

    private static final Logger LOGGER =
            LoggerFactory.getLogger(SomePeripheralsDigitizerPeripheral.class);

    private static final String[] METHODS = {
            "digitizeAmount", "rematerializeAmount", "mergeDigitalItems",
            "separateDigitalItem", "getItemLimitInSlot"
    };

    // ------------------------------------------------------------------
    // リフレクションキャッシュ / Reflection cache
    // ------------------------------------------------------------------

    @Nullable private static Class<?>     levelClass;
    @Nullable private static Class<?>     blockPosClass;
    @Nullable private static Class<?>     blockEntityClass;
    @Nullable private static Class<?>     digitizerPeriphClass;
    @Nullable private static Constructor<?> digitizerCtor;

    // IArguments を使うメソッド / methods that use IArguments
    @Nullable private static Class<?>     iArgumentsClass;
    @Nullable private static Class<?>     objectArgsClass;
    @Nullable private static Constructor<?> objectArgsCtor; // ObjectArguments(List<Object>)

    @Nullable private static Method mDigitizeAmount;
    @Nullable private static Method mRematerializeAmount;
    @Nullable private static Method mMergeDigitalItems;

    // IArguments を使わないメソッド / methods without IArguments
    @Nullable private static Method mSeparateDigitalItem; // (String, int)
    @Nullable private static Method mGetItemLimitInSlot;  // ()

    private static boolean reflectionInitialized = false;
    private static boolean reflectionOk = false;

    // ------------------------------------------------------------------
    // リフレクション初期化 / Reflection initialization
    // ------------------------------------------------------------------

    private static synchronized void ensureReflection() throws PeripheralException {
        if (reflectionInitialized) {
            if (!reflectionOk) throw new PeripheralException(
                    "Some Peripherals Digitizer reflection unavailable");
            return;
        }
        reflectionInitialized = true;
        try {
            levelClass       = Class.forName("net.minecraft.world.level.Level");
            blockPosClass    = Class.forName("net.minecraft.core.BlockPos");
            blockEntityClass = Class.forName(
                    "net.minecraft.world.level.block.entity.BlockEntity");

            iArgumentsClass = Class.forName("dan200.computercraft.api.lua.IArguments");
            objectArgsClass = Class.forName("dan200.computercraft.api.lua.ObjectArguments");
            objectArgsCtor  = objectArgsClass.getConstructor(List.class);

            digitizerPeriphClass = Class.forName(
                    "net.spaceeye.someperipherals.integrations.cc.peripherals"
                    + ".DigitizerPeripheral");
            digitizerCtor = digitizerPeriphClass.getConstructor(
                    levelClass, blockPosClass, blockEntityClass);

            // IArguments 引数を取るメソッド / methods with IArguments param
            mDigitizeAmount       = digitizerPeriphClass.getMethod("digitizeAmount",       iArgumentsClass);
            mRematerializeAmount  = digitizerPeriphClass.getMethod("rematerializeAmount",  iArgumentsClass);
            mMergeDigitalItems    = digitizerPeriphClass.getMethod("mergeDigitalItems",    iArgumentsClass);

            // 直接引数のメソッド / direct-argument methods
            mSeparateDigitalItem  = digitizerPeriphClass.getMethod("separateDigitalItem",
                    String.class, int.class);
            mGetItemLimitInSlot   = digitizerPeriphClass.getMethod("getItemLimitInSlot");

            reflectionOk = true;
            LOGGER.info("SomePeripheralsDigitizerPeripheral: reflection initialized");

        } catch (Exception e) {
            LOGGER.error("SomePeripheralsDigitizerPeripheral: "
                    + "reflection init failed: {}", e.getMessage());
            throw new PeripheralException(
                    "SP Digitizer reflection init failed: " + e.getMessage(), e);
        }
    }

    // ------------------------------------------------------------------
    // PeripheralType 実装 / PeripheralType implementation
    // ------------------------------------------------------------------

    @Override
    public String getTypeName() {
        return "digitizer";
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

        // BlockEntity を取得 / Get the BlockEntity
        BlockEntity be = level.getBlockEntity(peripheralPos);
        if (be == null) {
            throw new PeripheralException("No BlockEntity at Digitizer position " + peripheralPos);
        }

        try {
            // DigitizerPeripheral(level, pos, be) を生成 / Instantiate DigitizerPeripheral
            Object periph = digitizerCtor.newInstance(level, peripheralPos, be);

            switch (methodName) {
                case "digitizeAmount": {
                    // arg 0: optional int amount
                    List<Object> argList = new ArrayList<>();
                    int off0 = MsgPack.argOffset(args, 0);
                    if (off0 >= 0) argList.add(MsgPack.decodeInt(args, off0));
                    Object iArgs = objectArgsCtor.newInstance(argList);
                    Object result = mDigitizeAmount.invoke(periph, iArgs);
                    return MsgPack.packAny(result);
                }

                case "rematerializeAmount": {
                    // arg 0: String uuid, arg 1: optional int amount
                    int off0 = MsgPack.argOffset(args, 0);
                    if (off0 < 0) throw new PeripheralException(
                            "rematerializeAmount requires argument: uuid (str)");
                    String uuid = decodeStr(args, off0);
                    List<Object> argList = new ArrayList<>();
                    argList.add(uuid);
                    int off1 = MsgPack.argOffset(args, 1);
                    if (off1 >= 0) argList.add(MsgPack.decodeInt(args, off1));
                    Object iArgs = objectArgsCtor.newInstance(argList);
                    Object result = mRematerializeAmount.invoke(periph, iArgs);
                    return MsgPack.packAny(result);
                }

                case "mergeDigitalItems": {
                    // arg 0: String into_uuid, arg 1: String from_uuid, arg 2: optional int amount
                    int off0 = MsgPack.argOffset(args, 0);
                    int off1 = MsgPack.argOffset(args, 1);
                    if (off0 < 0 || off1 < 0) throw new PeripheralException(
                            "mergeDigitalItems requires 2 arguments: into_uuid, from_uuid");
                    String into = decodeStr(args, off0);
                    String from = decodeStr(args, off1);
                    List<Object> argList = new ArrayList<>();
                    argList.add(into);
                    argList.add(from);
                    int off2 = MsgPack.argOffset(args, 2);
                    if (off2 >= 0) argList.add(MsgPack.decodeInt(args, off2));
                    Object iArgs = objectArgsCtor.newInstance(argList);
                    Object result = mMergeDigitalItems.invoke(periph, iArgs);
                    return MsgPack.packAny(result);
                }

                case "separateDigitalItem": {
                    // arg 0: String from_uuid, arg 1: int amount
                    int off0 = MsgPack.argOffset(args, 0);
                    int off1 = MsgPack.argOffset(args, 1);
                    if (off0 < 0 || off1 < 0) throw new PeripheralException(
                            "separateDigitalItem requires 2 arguments: from_uuid (str), amount (int)");
                    String from = decodeStr(args, off0);
                    int amount  = MsgPack.decodeInt(args, off1);
                    Object result = mSeparateDigitalItem.invoke(periph, from, amount);
                    return MsgPack.packAny(result);
                }

                case "getItemLimitInSlot": {
                    Object result = mGetItemLimitInSlot.invoke(periph);
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
                    "Digitizer " + methodName + " failed: "
                    + (cause != null ? cause.getMessage() : ex.getMessage()), ex);
        } catch (Exception ex) {
            throw new PeripheralException(
                    "Digitizer reflection call failed: " + ex.getMessage(), ex);
        }
    }

    // ------------------------------------------------------------------
    // MsgPack 文字列デコード / MsgPack string decode
    // ------------------------------------------------------------------

    /**
     * MsgPack バイト列の offset 位置から文字列を読む。
     * Decode a MsgPack string at the given offset.
     */
    private static String decodeStr(byte[] data, int offset) throws PeripheralException {
        if (data == null || offset < 0 || offset >= data.length) {
            throw new PeripheralException("Cannot decode string: invalid offset " + offset);
        }
        int b = data[offset] & 0xFF;
        int len;
        int start;
        if ((b & 0xE0) == 0xA0) {   // fixstr
            len   = b & 0x1F;
            start = offset + 1;
        } else if (b == 0xD9) {      // str 8
            len   = data[offset + 1] & 0xFF;
            start = offset + 2;
        } else if (b == 0xDA) {      // str 16
            len   = ((data[offset + 1] & 0xFF) << 8) | (data[offset + 2] & 0xFF);
            start = offset + 3;
        } else {
            throw new PeripheralException(
                    "Expected MsgPack string at offset " + offset + ", got 0x"
                    + Integer.toHexString(b));
        }
        if (start + len > data.length) {
            throw new PeripheralException("MsgPack string truncated at offset " + offset);
        }
        return new String(data, start, len, java.nio.charset.StandardCharsets.UTF_8);
    }
}
