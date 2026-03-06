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
 * Some Peripherals の Radar ブロック向けペリフェラル。
 * Peripheral for Some Peripherals' Radar block (some_peripherals:radar).
 *
 * <p>コンパイル時に Some Peripherals への依存を持たないよう、
 * リフレクションで Kotlin 静的ユーティリティ関数を呼び出す。</p>
 *
 * <p>Uses reflection to call Kotlin static utility functions so that
 * Some Peripherals is not a compile-time dependency.</p>
 *
 * <h3>メソッド一覧 / Methods</h3>
 * <pre>
 * scan(radius)             → array   全エンティティ + 船のスキャン
 * scanForEntities(radius)  → array   エンティティのみ
 * scanForShips(radius)     → array   Valkyrien Skies の船のみ
 * scanForPlayers(radius)   → array   プレイヤーのみ
 * getConfigInfo()          → map     レーダー設定情報
 *
 * scan(radius)             → array   all entities + ships in radius
 * scanForEntities(radius)  → array   entities only
 * scanForShips(radius)     → array   Valkyrien Skies ships only
 * scanForPlayers(radius)   → array   players only
 * getConfigInfo()          → map     radar configuration info
 * </pre>
 *
 * <h3>Kotlin 実装クラス / Kotlin implementation classes</h3>
 * <pre>
 * net.spaceeye.someperipherals.stuff.radar.ScanInRadiusKt
 *   - scanInRadius(Double, Level, BlockPos): Any
 *   - scanForEntitiesInRadius(Double, Level, BlockPos): Any
 *   - scanForShipsInRadius(Double, Level, BlockPos): Any
 *   - scanForPlayersInRadius(Double, Level, BlockPos): Any
 *
 * net.spaceeye.someperipherals.stuff.configToMap.RadarKt
 *   - makeRadarConfigInfo(): MutableMap&lt;String, Any&gt;
 * </pre>
 */
public class SomePeripheralsRadarPeripheral implements PeripheralType {

    private static final Logger LOGGER = LoggerFactory.getLogger(SomePeripheralsRadarPeripheral.class);

    private static final String[] METHODS = {
            "scan", "scanForEntities", "scanForShips", "scanForPlayers", "getConfigInfo"
    };

    // ------------------------------------------------------------------
    // リフレクションキャッシュ / Reflection cache
    // ------------------------------------------------------------------

    @Nullable private static Class<?> levelClass;
    @Nullable private static Class<?> blockPosClass;

    @Nullable private static Method mScanInRadius;
    @Nullable private static Method mScanForEntities;
    @Nullable private static Method mScanForShips;
    @Nullable private static Method mScanForPlayers;
    @Nullable private static Method mGetConfigInfo;

    private static boolean reflectionInitialized = false;
    private static boolean reflectionOk = false;

    // ------------------------------------------------------------------
    // リフレクション初期化 / Reflection initialization
    // ------------------------------------------------------------------

    /**
     * リフレクション初期化。最初の呼び出し時に一度だけ実行する。
     * Initialize reflection — runs once on first call.
     */
    private static synchronized void ensureReflection() throws PeripheralException {
        if (reflectionInitialized) {
            if (!reflectionOk) {
                throw new PeripheralException("Some Peripherals radar reflection unavailable");
            }
            return;
        }
        reflectionInitialized = true;
        try {
            // Minecraft クラス（引数型として使用） / Minecraft classes used as parameter types
            levelClass    = Class.forName("net.minecraft.world.level.Level");
            blockPosClass = Class.forName("net.minecraft.core.BlockPos");

            // ScanInRadiusKt — Kotlin ファイルからコンパイルされた静的メソッド群
            // ScanInRadiusKt — static methods compiled from Kotlin top-level functions
            Class<?> scanKt = Class.forName(
                    "net.spaceeye.someperipherals.stuff.radar.ScanInRadiusKt");

            mScanInRadius   = scanKt.getMethod("scanInRadius",
                    double.class, levelClass, blockPosClass);
            mScanForEntities = scanKt.getMethod("scanForEntitiesInRadius",
                    double.class, levelClass, blockPosClass);
            mScanForShips    = scanKt.getMethod("scanForShipsInRadius",
                    double.class, levelClass, blockPosClass);
            mScanForPlayers  = scanKt.getMethod("scanForPlayersInRadius",
                    double.class, levelClass, blockPosClass);

            // RadarKt — makeRadarConfigInfo() の静的メソッド
            // RadarKt — static method makeRadarConfigInfo()
            Class<?> radarConfigKt = Class.forName(
                    "net.spaceeye.someperipherals.stuff.configToMap.RadarKt");
            mGetConfigInfo = radarConfigKt.getMethod("makeRadarConfigInfo");

            reflectionOk = true;
            LOGGER.info("SomePeripheralsRadarPeripheral: reflection initialized successfully");

        } catch (Exception e) {
            LOGGER.error("SomePeripheralsRadarPeripheral: reflection init failed: {}", e.getMessage());
            throw new PeripheralException(
                    "Some Peripherals radar reflection init failed: " + e.getMessage(), e);
        }
    }

    // ------------------------------------------------------------------
    // PeripheralType 実装 / PeripheralType implementation
    // ------------------------------------------------------------------

    @Override
    public String getTypeName() {
        return "sp_radar";
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
                case "scan": {
                    double radius = getRadiusArg(args, 0);
                    Object result = mScanInRadius.invoke(null, radius, level, peripheralPos);
                    return MsgPack.packAny(result);
                }
                case "scanForEntities": {
                    double radius = getRadiusArg(args, 0);
                    Object result = mScanForEntities.invoke(null, radius, level, peripheralPos);
                    return MsgPack.packAny(result);
                }
                case "scanForShips": {
                    double radius = getRadiusArg(args, 0);
                    Object result = mScanForShips.invoke(null, radius, level, peripheralPos);
                    return MsgPack.packAny(result);
                }
                case "scanForPlayers": {
                    double radius = getRadiusArg(args, 0);
                    Object result = mScanForPlayers.invoke(null, radius, level, peripheralPos);
                    return MsgPack.packAny(result);
                }
                case "getConfigInfo": {
                    Object result = mGetConfigInfo.invoke(null);
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
                    "Radar " + methodName + " failed: "
                    + (cause != null ? cause.getMessage() : ex.getMessage()), ex);
        } catch (Exception ex) {
            throw new PeripheralException("Radar reflection call failed: " + ex.getMessage(), ex);
        }
    }

    // ------------------------------------------------------------------
    // ヘルパー / Helpers
    // ------------------------------------------------------------------

    /**
     * 引数バイト列から i 番目の引数を double として取り出す。
     * Extract the i-th argument as a double from the encoded args bytes.
     *
     * @param args  MsgPack エンコード済み引数配列 / msgpack-encoded args array
     * @param index 引数インデックス (0-based) / argument index (0-based)
     * @return double 値 / double value
     * @throws PeripheralException 引数が存在しない場合 / if arg is missing
     */
    private static double getRadiusArg(byte[] args, int index) throws PeripheralException {
        int offset = MsgPack.argOffset(args, index);
        if (offset < 0) {
            throw new PeripheralException("Missing argument " + index + " (radius expected)");
        }
        return MsgPack.decodeF64(args, offset);
    }
}
