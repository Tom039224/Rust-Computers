package com.rustcomputers.peripheral.impl;

import com.rustcomputers.peripheral.MsgPack;
import com.rustcomputers.peripheral.PeripheralDisconnectedException;
import com.rustcomputers.peripheral.PeripheralException;
import com.rustcomputers.peripheral.PeripheralType;
import net.minecraft.core.BlockPos;
import net.minecraft.resources.ResourceLocation;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.level.block.entity.BlockEntity;
import net.minecraftforge.registries.ForgeRegistries;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.annotation.Nullable;
import java.lang.reflect.Method;
import java.util.ArrayDeque;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;

/**
 * CC:Tweaked モニター向けペリフェラル実装。
 * Peripheral implementation for CC:Tweaked Monitor blocks.
 *
 * <p>実行時リフレクションを使用して CC の内部クラスに直接アクセスするため、
 * ビルド時の CC:Tweaked 依存関係が不要。</p>
 *
 * <p>Uses runtime reflection to access CC internals directly,
 * so no compile-time CC:Tweaked dependency is required.</p>
 *
 * <h3>サポートするメソッド / Supported methods</h3>
 * <table border="1">
 * <tr><th>Method</th><th>Args (MsgPack)</th><th>Returns</th></tr>
 * <tr><td>getType</td><td>—</td><td>str "monitor"</td></tr>
 * <tr><td>getSize</td><td>—</td><td>array [width, height]</td></tr>
 * <tr><td>clear</td><td>—</td><td>nil</td></tr>
 * <tr><td>clearLine</td><td>—</td><td>nil</td></tr>
 * <tr><td>setCursorPos</td><td>[int x, int y]  (1-indexed)</td><td>nil</td></tr>
 * <tr><td>write</td><td>[str text]</td><td>nil</td></tr>
 * <tr><td>setTextColor / setTextColour</td><td>[int bitmask]</td><td>nil</td></tr>
 * <tr><td>setBackgroundColor / setBackgroundColour</td><td>[int bitmask]</td><td>nil</td></tr>
 * <tr><td>scroll</td><td>[int lines]</td><td>nil</td></tr>
 * <tr><td>setTextScale</td><td>[int scale×10]</td><td>nil</td></tr>
 * <tr><td>getTextScale</td><td>—</td><td>int scale×10</td></tr>
 * </table>
 *
 * <h3>色キー / Color bitmasks</h3>
 * <p>CC:Tweaked の Lua API と同じビットマスク形式で指定する。</p>
 * <p>Specify in bitmask form, same as CC:Tweaked's Lua API.</p>
 * <pre>
 * white=1, orange=2, magenta=4, light_blue=8, yellow=16,
 * lime=32, pink=64, gray=128, light_gray=256, cyan=512,
 * purple=1024, blue=2048, brown=4096, green=8192, red=16384, black=32768
 * </pre>
 */
public class CcMonitorPeripheral implements PeripheralType {

    private static final Logger LOGGER = LoggerFactory.getLogger(CcMonitorPeripheral.class);

    private static final String[] METHODS = {
            "getType",
            "getSize",
            "clear", "clearLine",
            "setCursorPos",
            "getCursorPos",
            "write",
            "blit",
            "setTextColor", "setTextColour",
            "getTextColor", "getTextColour",
            "setBackgroundColor", "setBackgroundColour",
            "getBackgroundColor", "getBackgroundColour",
            "isColor", "isColour",
            "scroll",
            "setTextScale", "getTextScale",
            "getCursorBlink",
            "setCursorBlink",
            "getPaletteColor", "getPaletteColour",
            "setPaletteColor", "setPaletteColour",
            "isAdvanced",    // 高度モニター判定 / Advanced monitor check
            "pollTouch",     // タッチイベントをデキュー / Dequeue a touch event (deprecated)
            "try_pull_monitor_touch",  // Event: monitor_touch イベント受信 / Receive monitor_touch event
    };

    // ----------------------------------------------------------------
    // リフレクション キャッシュ / Reflection caches
    // ----------------------------------------------------------------

    /** MonitorBlockEntity クラス / MonitorBlockEntity class */
    @Nullable private static Class<?> monitorBeClass;
    /** ServerMonitor クラス / ServerMonitor class */
    @Nullable private static Class<?> serverMonitorClass;
    /** Terminal クラス / Terminal class */
    @Nullable private static Class<?> terminalClass;

    @Nullable private static Method mGetServerMonitor;
    @Nullable private static Method mGetTerminal;

    @Nullable private static Method mTermWrite;
    @Nullable private static Method mTermBlit;
    @Nullable private static Method mTermClear;
    @Nullable private static Method mTermClearLine;
    @Nullable private static Method mTermSetCursorPos;
    @Nullable private static Method mTermGetCursorPos;
    @Nullable private static Method mTermGetWidth;
    @Nullable private static Method mTermGetHeight;
    @Nullable private static Method mTermScroll;
    @Nullable private static Method mTermSetTextColour;
    @Nullable private static Method mTermGetTextColour;
    @Nullable private static Method mTermSetBgColour;
    @Nullable private static Method mTermGetBgColour;
    @Nullable private static Method mTermGetCursorBlink;
    @Nullable private static Method mTermSetCursorBlink;
    @Nullable private static Method mTermGetPalette;

    @Nullable private static Method mBEGetTextScale;
    @Nullable private static Method mBESetTextScale;

    /**
     * CC の private メソッド createServerTerminal()。
     * createServerMonitor() を呼んで ServerMonitor を生成し、rebuild() でターミナルを初期化する。
     * CC's private method createServerTerminal().
     * Calls createServerMonitor() to create the ServerMonitor, then rebuild() to initialize the terminal.
     * 新しく置いたモニターは xIndex=0, yIndex=0, width=1, height=1 がデフォルトなので
     * CC コンピューターなしでも正常に動作する。
     * A freshly-placed monitor defaults to xIndex=0, yIndex=0, width=1, height=1 so this
     * works correctly even without a CC computer attached.
     */
    @Nullable private static Method mCreateServerTerminal;
    /** MonitorBlockEntity.toWorldPos(int, int) — マルチブロックモニターのオリジン位置取得 */
    @Nullable private static Method mToWorldPos;

    private static boolean reflectionInitialized = false;
    private static boolean reflectionOk = false;

    // ----------------------------------------------------------------
    // タッチイベントキュー / Touch event queue
    // ----------------------------------------------------------------

    /**
     * ブロック位置 → タッチイベントキュー（[u, v] 0.0–1.0 の正規化座標）。
     * Block pos → touch event queue ([u, v] normalized 0.0–1.0 coordinates).
     */
    private static final Map<BlockPos, ArrayDeque<float[]>> TOUCH_QUEUE = new ConcurrentHashMap<>();

    /**
     * 右クリックイベントからタッチを積む（サーバーイベントハンドラーから呼ぶ）。
     * Enqueue a touch event from a right-click (called from server event handler).
     *
     * @param pos ブロック位置 / block position
     * @param u   面の水平方向 0.0–1.0 / horizontal face fraction
     * @param v   面の垂直方向 0.0–1.0 / vertical face fraction
     */
    public static void queueTouchEvent(BlockPos pos, float u, float v) {
        TOUCH_QUEUE.computeIfAbsent(pos, k -> new ArrayDeque<>()).offer(new float[]{u, v});
    }

    /**
     * モニター BE の toWorldPos(0, 0) を使ってオリジン（xIndex=0, yIndex=0）ブロック位置を返す。
     * マルチブロックモニターでタッチキューの位置を統一するために使用する。
     * Returns the origin block (xIndex=0, yIndex=0) of a monitor block entity.
     * Used to unify touch-queue keys for multi-block monitors.
     *
     * @param be       MonitorBlockEntity インスタンス（または任意オブジェクト）
     * @param fallback toWorldPos が失敗した場合に返す位置
     */
    public static BlockPos resolveMonitorOrigin(Object be, BlockPos fallback) {
        if (mToWorldPos == null || be == null) return fallback;
        try {
            return (BlockPos) mToWorldPos.invoke(be, 0, 0);
        } catch (Exception e) {
            return fallback;
        }
    }

    /**
     * MonitorBlockEntity から ServerMonitor を取得する（リフレクション用ヘルパー）。
     * Get ServerMonitor from MonitorBlockEntity (reflection helper).
     */
    public static Object getServerMonitorReflection(Object be) throws Exception {
        if (mGetServerMonitor == null) return null;
        return mGetServerMonitor.invoke(be);
    }

    /**
     * ServerMonitor から Terminal を取得する（リフレクション用ヘルパー）。
     * Get Terminal from ServerMonitor (reflection helper).
     */
    public static Object getTerminalReflection(Object serverMonitor) throws Exception {
        if (mGetTerminal == null) return null;
        return mGetTerminal.invoke(serverMonitor);
    }

    /**
     * Terminal の幅を取得する（リフレクション用ヘルパー）。
     * Get Terminal width (reflection helper).
     */
    public static int getTerminalWidth(Object terminal) throws Exception {
        if (mTermGetWidth == null) return 0;
        return (int) mTermGetWidth.invoke(terminal);
    }

    /**
     * Terminal の高さを取得する（リフレクション用ヘルパー）。
     * Get Terminal height (reflection helper).
     */
    public static int getTerminalHeight(Object terminal) throws Exception {
        if (mTermGetHeight == null) return 0;
        return (int) mTermGetHeight.invoke(terminal);
    }

    // ----------------------------------------------------------------
    // 初期化 / Initialization
    // ----------------------------------------------------------------

    /**
     * リフレクション初期化。最初の呼び出し時に一度だけ実行する。
     * Initialize reflection. Runs once on first call.
     */
    private static synchronized void ensureReflection() throws PeripheralException {
        if (reflectionInitialized) {
            if (!reflectionOk) {
                throw new PeripheralException("CC:Tweaked reflection unavailable");
            }
            return;
        }
        reflectionInitialized = true;
        try {
            monitorBeClass = Class.forName(
                    "dan200.computercraft.shared.peripheral.monitor.MonitorBlockEntity");
            serverMonitorClass = Class.forName(
                    "dan200.computercraft.shared.peripheral.monitor.ServerMonitor");
            terminalClass = Class.forName(
                    "dan200.computercraft.core.terminal.Terminal");

            // getServerMonitor は private なので getDeclaredMethod を使う
            // getServerMonitor is private — must use getDeclaredMethod
            mGetServerMonitor = monitorBeClass.getDeclaredMethod("getServerMonitor");
            mGetServerMonitor.setAccessible(true);

            mGetTerminal      = serverMonitorClass.getMethod("getTerminal");
            mGetTerminal.setAccessible(true);

            mTermWrite        = terminalClass.getMethod("write", String.class);
            mTermBlit         = terminalClass.getMethod("blit", String.class, String.class, String.class);
            mTermClear        = terminalClass.getMethod("clear");
            mTermClearLine    = terminalClass.getMethod("clearLine");
            mTermSetCursorPos = terminalClass.getMethod("setCursorPos", int.class, int.class);
            mTermGetCursorPos = terminalClass.getMethod("getCursorPos");
            mTermGetWidth     = terminalClass.getMethod("getWidth");
            mTermGetHeight    = terminalClass.getMethod("getHeight");
            mTermScroll       = terminalClass.getMethod("scroll", int.class);
            mTermSetTextColour = terminalClass.getMethod("setTextColour", int.class);
            mTermGetTextColour = terminalClass.getMethod("getTextColour");
            mTermSetBgColour   = terminalClass.getMethod("setBackgroundColour", int.class);
            mTermGetBgColour   = terminalClass.getMethod("getBackgroundColour");
            mTermGetCursorBlink = terminalClass.getMethod("getCursorBlink");
            mTermSetCursorBlink = terminalClass.getMethod("setCursorBlink", boolean.class);
            mTermGetPalette     = terminalClass.getMethod("getPalette");

            // OptionalなscaleBE系メソッド — 無ければスキップ / optional, skip if absent
            try {
                mBEGetTextScale = monitorBeClass.getMethod("getTextScale");
                mBESetTextScale = monitorBeClass.getMethod("setTextScale", int.class);
                mBEGetTextScale.setAccessible(true);
                mBESetTextScale.setAccessible(true);
            } catch (NoSuchMethodException ex) {
                LOGGER.debug("CcMonitorPeripheral: setTextScale not found, skipping");
            }

            // createServerTerminal() は private メソッド。
            // createServerMonitor() を呼んで ServerMonitor を生成し、rebuild() でターミナルを初期化する。
            // 新しく置いたモニターは xIndex=0, yIndex=0, width=1, height=1 がデフォルトのため
            // CC コンピューターなしでも正常に動作する。(serverTick() は存在しないため v0.1.8 では無効だった)
            // createServerTerminal() is a private method.
            // It calls createServerMonitor() to set up the ServerMonitor, then rebuild() to build the terminal.
            // A freshly-placed monitor defaults to xIndex=0, yIndex=0, width=1, height=1 so it works
            // without a CC computer. (serverTick() doesn't exist — was a no-op in v0.1.8)
            try {
                mCreateServerTerminal = monitorBeClass.getDeclaredMethod("createServerTerminal");
                mCreateServerTerminal.setAccessible(true);
            } catch (NoSuchMethodException ex) {
                LOGGER.warn("CcMonitorPeripheral: createServerTerminal() not found — monitor init will fail");
            }

            try {
                mToWorldPos = monitorBeClass.getDeclaredMethod("toWorldPos", int.class, int.class);
                mToWorldPos.setAccessible(true);
            } catch (NoSuchMethodException ex) {
                LOGGER.warn("CcMonitorPeripheral: toWorldPos() not found — multi-block monitor touch origin falls back to peripheral pos");
            }

            reflectionOk = true;
            LOGGER.info("CcMonitorPeripheral: reflection initialized successfully");

        } catch (Exception e) {
            LOGGER.error("CcMonitorPeripheral: reflection init failed: {}", e.getMessage());
            throw new PeripheralException("CC:Tweaked reflection init failed: " + e.getMessage(), e);
        }
    }

    // ----------------------------------------------------------------
    // PeripheralType 実装 / PeripheralType implementation
    // ----------------------------------------------------------------

    @Override
    public String getTypeName() {
        return "monitor";
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

        BlockEntity be = level.getBlockEntity(peripheralPos);
        if (be == null || !monitorBeClass.isInstance(be)) {
            throw new PeripheralDisconnectedException(
                    "Monitor disconnected: no MonitorBlockEntity at " + peripheralPos);
        }

        // terminal が不要なメソッドを先に処理
        // Handle methods that don't need the terminal up-front
        if ("getType".equals(methodName)) {
            // MonitorBlockEntity であることは上で確認済み / Already confirmed it's a MonitorBlockEntity
            return MsgPack.str("monitor");
        }

        // isAdvanced / pollTouch / try_pull_monitor_touch は terminal 取得が不要なので先に処理
        // Handle isAdvanced, pollTouch, and try_pull_monitor_touch before requiring the terminal
        if ("isAdvanced".equals(methodName)) {
            ResourceLocation rl = ForgeRegistries.BLOCKS.getKey(
                    level.getBlockState(peripheralPos).getBlock());
            return MsgPack.bool(rl != null && "monitor_advanced".equals(rl.getPath()));
        }
        if ("pollTouch".equals(methodName)) {
            BlockPos originPos = resolveMonitorOrigin(be, peripheralPos);
            ArrayDeque<float[]> q = TOUCH_QUEUE.get(originPos);
            if (q == null || q.isEmpty()) return MsgPack.nil();
            float[] frac = q.poll();
            // ターミナルサイズでキャラクター座標変換 / convert fraction to char coords
            try {
                Object serverMonitor = mGetServerMonitor.invoke(be);
                if (serverMonitor != null) {
                    Object terminal = mGetTerminal.invoke(serverMonitor);
                    if (terminal != null) {
                        int w = (int) mTermGetWidth.invoke(terminal);
                        int h = (int) mTermGetHeight.invoke(terminal);
                        int cx = Math.max(1, Math.min(w, (int) (frac[0] * w) + 1));
                        int cy = Math.max(1, Math.min(h, (int) (frac[1] * h) + 1));
                        return MsgPack.array(MsgPack.int32(cx), MsgPack.int32(cy));
                    }
                }
            } catch (Exception ignored) {}
            // フォールバック: 正規化値を整数 ×100 で返す / fallback: fraction × 100
            return MsgPack.array(
                    MsgPack.int32((int) (frac[0] * 100)),
                    MsgPack.int32((int) (frac[1] * 100)));
        }
        if ("try_pull_monitor_touch".equals(methodName)) {
            // CCEventReceiver を使ってイベントキューから取得
            // Use CCEventReceiver to get event from queue
            return CCEventReceiver.tryPull(methodName, peripheralPos);
        }

        try {
            Object serverMonitor = mGetServerMonitor.invoke(be);

            // serverMonitor が null、または terminal が null の場合は createServerTerminal() で初期化する。
            //
            // terminal が null になるケース:
            //   モニターがRustコンピューターに隣接しない場所に拡張された場合、CC:Tweaked は
            //   resize() → needsTerminal=false（CCペリフェラル未接続）→ ServerMonitor.reset()
            //   の流れでターミナルをリセットする。この場合 serverMonitor 自体は null でないが
            //   getTerminal() が null を返す。createServerTerminal() を呼ぶと内部で
            //   terminal==null を検知して rebuild() してくれるため初期化が完了する。
            //
            // If serverMonitor is null OR terminal is null, call createServerTerminal() to re-initialize.
            // When the monitor is expanded at a position not adjacent to the Rust computer, CC:Tweaked
            // calls resize() → needsTerminal=false (no CC peripheral) → ServerMonitor.reset(), which
            // clears the terminal. serverMonitor itself is non-null in this case, but getTerminal()
            // returns null. createServerTerminal() detects terminal==null and calls rebuild(), restoring it.
            if (mCreateServerTerminal != null) {
                boolean terminalMissing = serverMonitor == null
                        || mGetTerminal.invoke(serverMonitor) == null;
                if (terminalMissing) {
                    LOGGER.debug("CcMonitorPeripheral: serverMonitor/terminal null — calling createServerTerminal() to init");
                    mCreateServerTerminal.invoke(be);
                    serverMonitor = mGetServerMonitor.invoke(be);
                }
            }

            if (serverMonitor == null) {
                throw new PeripheralException(
                        "Monitor not initialized — createServerTerminal() did not produce a ServerMonitor. "
                      + "CC:Tweaked version mismatch? Expected dan200.computercraft 1.116.x");
            }
            
            // IComputerAccess モックをアタッチして monitor_touch イベントを受信できるようにする (初回のみ)
            // Attach mock IComputerAccess so monitor_touch events can be received (idempotent)
            CCEventReceiver.ensureAttached(serverMonitor, peripheralPos);
            
            Object terminal = mGetTerminal.invoke(serverMonitor);
            if (terminal == null) {
                throw new PeripheralException("Monitor terminal is null");
            }

            return dispatch(methodName, args, be, terminal);

        } catch (PeripheralException e) {
            throw e;
        } catch (Exception e) {
            throw new PeripheralException("Monitor method '" + methodName + "' failed: " + e.getMessage(), e);
        }
    }

    // getSize / getType / getTextScale / getCursorPos / getTextColor / getBackgroundColor / isColor / getCursorBlink / getPaletteColor は読み取り専用 → callImmediate で使用可能
    @Nullable
    @Override
    public byte[] callImmediate(String methodName, byte[] args,
                                ServerLevel level, BlockPos peripheralPos)
            throws PeripheralException {
        switch (methodName) {
            case "getType":
            case "getSize":
            case "getTextScale":
            case "getCursorPos":
            case "getTextColor":
            case "getTextColour":
            case "getBackgroundColor":
            case "getBackgroundColour":
            case "isColor":
            case "isColour":
            case "getCursorBlink":
            case "getPaletteColor":
            case "getPaletteColour":
            case "isAdvanced":  // 読み取り専用 / read-only
            case "pollTouch":   // キューからのデキュー / dequeue from queue
            case "try_pull_monitor_touch":  // イベント受信 / event receive
                return callMethod(methodName, args, level, peripheralPos);
            default:
                return null; // 書き込み系は即時呼び出し不可
        }
    }

    // ----------------------------------------------------------------
    // メソッドディスパッチ / Method dispatch
    // ----------------------------------------------------------------

    private byte[] dispatch(String name, byte[] args, Object be, Object term) throws Exception {
        switch (name) {

            case "getType":
                return MsgPack.str("monitor");

            case "getSize": {
                int w = (int) mTermGetWidth.invoke(term);
                int h = (int) mTermGetHeight.invoke(term);
                return MsgPack.array(MsgPack.int32(w), MsgPack.int32(h));
            }

            case "clear":
                mTermClear.invoke(term);
                return MsgPack.nil();

            case "clearLine":
                mTermClearLine.invoke(term);
                return MsgPack.nil();

            case "setCursorPos": {
                // Lua API は 1-indexed、Terminal 内部は 0-indexed
                // Lua API is 1-indexed, Terminal internally is 0-indexed
                int x = getIntArg(args, 0, "x");
                int y = getIntArg(args, 1, "y");
                mTermSetCursorPos.invoke(term, x - 1, y - 1);
                return MsgPack.nil();
            }

            case "write": {
                String text = getStrArg(args, 0, "text");
                mTermWrite.invoke(term, text);
                return MsgPack.nil();
            }

            case "blit": {
                String text = getStrArg(args, 0, "text");
                String textColor = getStrArg(args, 1, "textColor");
                String backgroundColor = getStrArg(args, 2, "backgroundColor");
                mTermBlit.invoke(term, text, textColor, backgroundColor);
                return MsgPack.nil();
            }

            case "getCursorPos": {
                int[] pos = (int[]) mTermGetCursorPos.invoke(term);
                // Terminal は 0-indexed、Lua API は 1-indexed
                // Terminal is 0-indexed, Lua API is 1-indexed
                return MsgPack.array(MsgPack.int32(pos[0] + 1), MsgPack.int32(pos[1] + 1));
            }

            case "getCursorBlink": {
                boolean blink = (boolean) mTermGetCursorBlink.invoke(term);
                return MsgPack.bool(blink);
            }

            case "setCursorBlink": {
                int offset = MsgPack.argOffset(args, 0);
                if (offset < 0) throw new PeripheralException("Missing argument: blink");
                boolean blink = decodeBool(args, offset);
                mTermSetCursorBlink.invoke(term, blink);
                return MsgPack.nil();
            }

            case "isColor":
            case "isColour": {
                // Advanced monitor かどうかで判定
                // Check if advanced monitor by checking if BE is an advanced monitor
                // We need to get the block type from the BE
                try {
                    // MonitorBlockEntity.getBlockState().getBlock()
                    Method getBlockState = be.getClass().getMethod("getBlockState");
                    Object blockState = getBlockState.invoke(be);
                    Method getBlock = blockState.getClass().getMethod("getBlock");
                    Object block = getBlock.invoke(blockState);
                    
                    ResourceLocation rl = ForgeRegistries.BLOCKS.getKey((net.minecraft.world.level.block.Block) block);
                    return MsgPack.bool(rl != null && "monitor_advanced".equals(rl.getPath()));
                } catch (Exception e) {
                    return MsgPack.bool(false);
                }
            }

            case "getTextColor":
            case "getTextColour": {
                int colourIndex = (int) mTermGetTextColour.invoke(term);
                int bitmask = 1 << colourIndex;
                return MsgPack.int32(bitmask);
            }

            case "getBackgroundColor":
            case "getBackgroundColour": {
                int colourIndex = (int) mTermGetBgColour.invoke(term);
                int bitmask = 1 << colourIndex;
                return MsgPack.int32(bitmask);
            }

            case "getPaletteColor":
            case "getPaletteColour": {
                int bitmask = getIntArg(args, 0, "color");
                int colourIndex = colourFromBitmask(bitmask);
                
                // getPalette() returns Palette object
                Object palette = mTermGetPalette.invoke(term);
                if (palette == null) {
                    return MsgPack.array(MsgPack.float64(0.0), MsgPack.float64(0.0), MsgPack.float64(0.0));
                }
                
                // Palette.getColour(int) returns double[3] (r, g, b)
                Method getColour = palette.getClass().getMethod("getColour", int.class);
                double[] rgb = (double[]) getColour.invoke(palette, colourIndex);
                return MsgPack.array(MsgPack.float64(rgb[0]), MsgPack.float64(rgb[1]), MsgPack.float64(rgb[2]));
            }

            case "setPaletteColor":
            case "setPaletteColour": {
                int bitmask = getIntArg(args, 0, "color");
                int colourIndex = colourFromBitmask(bitmask);
                
                // Get RGB values
                int offset1 = MsgPack.argOffset(args, 1);
                int offset2 = MsgPack.argOffset(args, 2);
                int offset3 = MsgPack.argOffset(args, 3);
                if (offset1 < 0 || offset2 < 0 || offset3 < 0) {
                    throw new PeripheralException("Missing RGB arguments");
                }
                double r = MsgPack.decodeF64(args, offset1);
                double g = MsgPack.decodeF64(args, offset2);
                double b = MsgPack.decodeF64(args, offset3);
                
                // getPalette() returns Palette object
                Object palette = mTermGetPalette.invoke(term);
                if (palette != null) {
                    // Palette.setColour(int, double, double, double)
                    Method setColour = palette.getClass().getMethod("setColour", 
                            int.class, double.class, double.class, double.class);
                    setColour.invoke(palette, colourIndex, r, g, b);
                }
                return MsgPack.nil();
            }

            case "setTextColor":
            case "setTextColour": {
                int bitmask = getIntArg(args, 0, "color");
                int colourIndex = colourFromBitmask(bitmask);
                mTermSetTextColour.invoke(term, colourIndex);
                return MsgPack.nil();
            }

            case "setBackgroundColor":
            case "setBackgroundColour": {
                int bitmask = getIntArg(args, 0, "color");
                int colourIndex = colourFromBitmask(bitmask);
                mTermSetBgColour.invoke(term, colourIndex);
                return MsgPack.nil();
            }

            case "scroll": {
                int n = getIntArg(args, 0, "n");
                mTermScroll.invoke(term, n);
                return MsgPack.nil();
            }

            case "setTextScale": {
                if (mBESetTextScale == null) {
                    return MsgPack.nil(); // unsupported, silently ignore
                }
                // Rust は float64 でスケール値 (0.5〜5.0) を送る → CC内部形式 scale*2 に変換
                // Rust sends float64 scale value (0.5–5.0) → convert to CC internal format scale*2
                int offset = MsgPack.argOffset(args, 0);
                if (offset < 0) throw new PeripheralException("Missing argument: scale");
                double scale = MsgPack.decodeF64(args, offset);
                int ccScale = (int) Math.round(scale * 2);
                ccScale = Math.max(1, Math.min(10, ccScale)); // clamp: 0.5x〜5.0x
                mBESetTextScale.invoke(be, ccScale);
                return MsgPack.nil();
            }

            case "getTextScale": {
                if (mBEGetTextScale == null) {
                    return MsgPack.int32(10); // 1.0x as default
                }
                int ccScale = (int) mBEGetTextScale.invoke(be);
                // ccScale は scale*2 → scaleX10 に変換 / ccScale is scale*2 → scaleX10
                int scaleX10 = (int) Math.round(ccScale / 2.0 * 10);
                return MsgPack.int32(scaleX10);
            }

            default:
                throw new PeripheralException("Unknown monitor method: " + name);
        }
    }

    // ----------------------------------------------------------------
    // ユーティリティ / Utilities
    // ----------------------------------------------------------------

    /**
     * CC:Tweaked カラービットマスク → 0-15 インデックスに変換。
     * Convert CC:Tweaked color bitmask to 0-15 index.
     *
     * <p>white=1→0, orange=2→1, …, black=32768→15</p>
     */
    private static int colourFromBitmask(int bitmask) throws PeripheralException {
        if (bitmask <= 0 || Integer.bitCount(bitmask) != 1) {
            throw new PeripheralException(
                    "Invalid color bitmask: " + bitmask + " (must be a power of 2)");
        }
        return Integer.numberOfTrailingZeros(bitmask);
    }

    /** MsgPack の bool デコード */
    private static boolean decodeBool(byte[] data, int offset) {
        if (data == null || offset >= data.length) return false;
        int b = data[offset] & 0xFF;
        return b == 0xC3; // true
    }

    private static int getIntArg(byte[] args, int index, String argName) throws PeripheralException {
        int offset = MsgPack.argOffset(args, index);
        if (offset < 0) {
            throw new PeripheralException("Missing argument: " + argName);
        }
        return MsgPack.decodeInt(args, offset);
    }

    private static String getStrArg(byte[] args, int index, String argName) throws PeripheralException {
        // fixarray header から文字列データを読み取る
        // Read string data from fixarray
        if (args == null || args.length == 0) {
            throw new PeripheralException("Missing argument: " + argName);
        }

        // fixarray の index 番目の要素オフセットを取得
        int offset = MsgPack.argOffset(args, index);
        if (offset < 0) {
            throw new PeripheralException("Missing argument: " + argName + " at index " + index);
        }

        // 文字列を解析 / Parse string
        int b = args[offset] & 0xFF;
        int strLen;
        int dataOffset;

        if ((b & 0xE0) == 0xA0) {         // fixstr
            strLen = b & 0x1F;
            dataOffset = offset + 1;
        } else if (b == 0xD9) {           // str 8
            strLen = args[offset + 1] & 0xFF;
            dataOffset = offset + 2;
        } else if (b == 0xDA) {           // str 16
            strLen = ((args[offset + 1] & 0xFF) << 8) | (args[offset + 2] & 0xFF);
            dataOffset = offset + 3;
        } else {
            throw new PeripheralException(
                    "Argument '" + argName + "' is not a string (tag=0x" + Integer.toHexString(b) + ")");
        }

        if (dataOffset + strLen > args.length) {
            throw new PeripheralException("String argument '" + argName + "' truncated");
        }
        return new String(args, dataOffset, strLen, java.nio.charset.StandardCharsets.UTF_8);
    }
}
