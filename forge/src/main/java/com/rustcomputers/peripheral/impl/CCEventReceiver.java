package com.rustcomputers.peripheral.impl;

import com.rustcomputers.peripheral.MsgPack;
import net.minecraft.core.BlockPos;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.lang.reflect.Method;
import java.lang.reflect.Proxy;
import java.util.*;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentLinkedQueue;

/**
 * CC:Tweaked の IComputerAccess モックプロキシを使ったイベントレシーバー。
 * Event receiver using a mock proxy of CC:Tweaked's IComputerAccess.
 *
 * <p>CC:Tweaked の IPeripheral が {@code attach(IComputerAccess)} を呼ばれた際、
 * このモックコンピューターを渡すことで、ペリフェラルが発火する
 * {@code queueEvent()} をキャプチャしてキューに蓄積する。</p>
 *
 * <p>When CC:Tweaked's IPeripheral receives {@code attach(IComputerAccess)},
 * the mock computer is passed so that the peripheral's {@code queueEvent()} calls
 * are captured and enqueued per (BlockPos, eventName).</p>
 *
 * <h3>エンコード規則 / Encoding rules</h3>
 * <ul>
 *   <li>イベント引数 args[0] は attachment name なのでスキップ / args[0] = attachment name (skipped)</li>
 *   <li>UNIT_EVENTS: 常に empty-fixarray (0x90) → Rust の {@code Option<()>::Some(())} に対応</li>
 *   <li>実引数 0 個: 0x90</li>
 *   <li>実引数 1 個: {@code MsgPack.packAny(arg)}</li>
 *   <li>実引数 2 個以上: {@code MsgPack.array([packAny(arg1), ...])}</li>
 * </ul>
 */
public class CCEventReceiver {

    private static final Logger LOGGER = LoggerFactory.getLogger(CCEventReceiver.class);

    // ------------------------------------------------------------------
    // UNIT_EVENTS: queueEvent の引数を無視して常に Some(()) を返すイベント
    // These events return Some(()) regardless of actual args
    // Rust return type = Option<()>
    // ------------------------------------------------------------------
    private static final Set<String> UNIT_EVENTS = new HashSet<>(Arrays.asList(
            "train_arrive",
            "train_depart",
            "train_passing",
            "train_passed",
            "train_signal_state_change"
    ));

    /** empty fixarray (0x90) = rmp-serde の Some(()) / rmp-serde Some(()) */
    private static final byte[] EMPTY_FIXARRAY = {(byte) 0x90};

    // ------------------------------------------------------------------
    // キューとアタッチ状態 / Queues and attach state
    // ------------------------------------------------------------------

    /** "x,y,z:eventName" → キュー of MsgPack ペイロード */
    private static final ConcurrentHashMap<String, ConcurrentLinkedQueue<byte[]>>
            EVENT_QUEUES = new ConcurrentHashMap<>();

    /**
     * BlockPos → モックプロキシ。
     * BlockPos → mock proxy instance.
     * 値の存在 = アタッチ済み / value present = already attached.
     */
    private static final ConcurrentHashMap<BlockPos, Object> ATTACHED_PROXIES =
            new ConcurrentHashMap<>();

    // ------------------------------------------------------------------
    // IComputerAccess クラス遅延ロード / Lazy-load IComputerAccess class
    // ------------------------------------------------------------------

    private static volatile Class<?> iComputerAccessClass = null;
    private static volatile boolean iComputerAccessFailed = false;

    private static Class<?> getIComputerAccessClass() {
        if (iComputerAccessClass != null) return iComputerAccessClass;
        if (iComputerAccessFailed) return null;
        try {
            iComputerAccessClass = Class.forName(
                    "dan200.computercraft.api.peripheral.IComputerAccess");
            return iComputerAccessClass;
        } catch (ClassNotFoundException e) {
            iComputerAccessFailed = true;
            LOGGER.warn("CCEventReceiver: IComputerAccess not found (CC:T not loaded?)");
            return null;
        }
    }

    // ------------------------------------------------------------------
    // キーユーティリティ / Key utility
    // ------------------------------------------------------------------

    private static String queueKey(BlockPos pos, String eventName) {
        return pos.getX() + "," + pos.getY() + "," + pos.getZ() + ":" + eventName;
    }

    // ------------------------------------------------------------------
    // 公開 API / Public API
    // ------------------------------------------------------------------

    /**
     * ペリフェラルにモックコンピューターをアタッチする (初回のみ実行)。
     * Attach a mock computer to the peripheral (idempotent; no-op after first call).
     *
     * @param peripheral CC:Tweaked の IPeripheral インスタンス
     * @param pos        ブロック座標
     */
    public static void ensureAttached(Object peripheral, BlockPos pos) {
        if (ATTACHED_PROXIES.containsKey(pos)) return;

        Class<?> ica = getIComputerAccessClass();
        if (ica == null) return;

        try {
            // ---- プロキシ生成 / Create IComputerAccess proxy ----
            final BlockPos capturedPos = pos;
            Object proxy = Proxy.newProxyInstance(
                    ica.getClassLoader(),
                    new Class<?>[]{ica},
                    (proxyObj, method, args) -> {
                        switch (method.getName()) {
                            case "queueEvent":
                                // args[0] = String eventName
                                // args[1] = Object[] arguments  (varargs backing array)
                                if (args != null && args.length >= 1) {
                                    String eventName = (String) args[0];
                                    Object[] eventArgs = (args.length > 1 && args[1] instanceof Object[])
                                            ? (Object[]) args[1]
                                            : new Object[0];
                                    enqueueEvent(capturedPos, eventName, eventArgs);
                                }
                                return null;

                            case "getAttachmentName":
                                return "rust_computer";

                            case "getID":
                                return 0;

                            case "getAvailablePeripherals":
                                return Collections.emptyMap();

                            case "getAvailablePeripheral":
                                return null;

                            case "getMainThreadMonitor":
                                return null;

                            case "mount":
                            case "mountWritable":
                                return null;

                            case "unmount":
                                return null;

                            // Object methods
                            case "equals":
                                return proxyObj == (args != null && args.length > 0 ? args[0] : null);
                            case "hashCode":
                                return System.identityHashCode(proxyObj);
                            case "toString":
                                return "CCEventReceiver$MockComputer@" + capturedPos;

                            default:
                                return null;
                        }
                    }
            );

            // アタッチ済みとして記録してから attach() を呼ぶ
            // Record as attached BEFORE calling attach() to avoid reentrancy
            ATTACHED_PROXIES.put(pos, proxy);

            // ---- peripheral.attach(proxy) を呼び出す / Call peripheral.attach(proxy) ----
            callAttach(peripheral, proxy);

        } catch (Exception e) {
            LOGGER.warn("CCEventReceiver: ensureAttached failed at {}: {}", pos, e.getMessage());
            ATTACHED_PROXIES.remove(pos);
        }
    }

    /**
     * ブロック座標のアタッチ状態とキューをクリアする。
     * Clear attach state and queues for a block position.
     *
     * <p>BlockEntity がアンロードされた際などに呼び出す。
     * Call when the BlockEntity is unloaded or becomes unavailable.</p>
     *
     * @param peripheral アタッチされていたペリフェラル (detach 呼び出し用; null 可)
     * @param pos        ブロック座標
     */
    public static void detach(Object peripheral, BlockPos pos) {
        Object proxy = ATTACHED_PROXIES.remove(pos);
        if (proxy != null && peripheral != null) {
            try {
                callDetach(peripheral, proxy);
            } catch (Exception e) {
                LOGGER.debug("CCEventReceiver: detach call failed at {}: {}", pos, e.getMessage());
            }
        }
        // キューをクリア / Clear queues
        String prefix = pos.getX() + "," + pos.getY() + "," + pos.getZ() + ":";
        EVENT_QUEUES.keySet().removeIf(k -> k.startsWith(prefix));
    }

    /**
     * 手動でイベントをキューに追加する（CC:Tweaked が自動生成しないイベント用）。
     * Manually enqueue an event (for events that CC:Tweaked doesn't auto-generate).
     *
     * @param pos       ブロック座標
     * @param eventName イベント名
     * @param eventArgs イベント引数（attachment name を含まない）
     */
    public static void queueEventManually(BlockPos pos, String eventName, Object... eventArgs) {
        try {
            byte[] encoded = encodeEventPayload(eventName, prependAttachmentName(eventArgs));
            String key = queueKey(pos, eventName);
            EVENT_QUEUES
                    .computeIfAbsent(key, k -> new ConcurrentLinkedQueue<>())
                    .offer(encoded);
        } catch (Exception e) {
            LOGGER.warn("CCEventReceiver: failed to manually queue event '{}' at {}: {}",
                    eventName, pos, e.getMessage());
        }
    }

    /**
     * イベント引数の先頭に attachment name を追加する。
     * Prepend attachment name to event arguments.
     */
    private static Object[] prependAttachmentName(Object[] args) {
        Object[] result = new Object[args.length + 1];
        result[0] = "rust_computer";
        System.arraycopy(args, 0, result, 1, args.length);
        return result;
    }

    /**
     * try_pull_X メソッドに対応するイベントデータをデキューして返す。
     * Dequeue and return event data for a try_pull_X method.
     *
     * @param tryPullMethodName "try_pull_modem_message" 形式のメソッド名
     * @param pos               ブロック座標
     * @return MsgPack エンコード済みペイロード (イベントなし時は nil)
     */
    public static byte[] tryPull(String tryPullMethodName, BlockPos pos) {
        if (!tryPullMethodName.startsWith("try_pull_")) return MsgPack.nil();

        String eventName = tryPullMethodName.substring("try_pull_".length());
        String key = queueKey(pos, eventName);

        ConcurrentLinkedQueue<byte[]> queue = EVENT_QUEUES.get(key);
        if (queue == null) return MsgPack.nil();

        byte[] payload = queue.poll();
        return payload != null ? payload : MsgPack.nil();
    }

    // ------------------------------------------------------------------
    // 内部実装 / Internal implementation
    // ------------------------------------------------------------------

    /**
     * イベントをエンコードしてキューに追加する。
     * Encode event arguments and push to queue.
     *
     * <p>SyncedPeripheral パターン: args[0] = attachmentName → スキップ
     * ModemPeripheral パターン: args[0] = attachmentName → スキップ</p>
     */
    private static void enqueueEvent(BlockPos pos, String eventName, Object[] eventArgs) {
        try {
            byte[] encoded = encodeEventPayload(eventName, eventArgs);
            String key = queueKey(pos, eventName);
            EVENT_QUEUES
                    .computeIfAbsent(key, k -> new ConcurrentLinkedQueue<>())
                    .offer(encoded);
        } catch (Exception e) {
            LOGGER.warn("CCEventReceiver: failed to encode event '{}' at {}: {}",
                    eventName, pos, e.getMessage());
        }
    }

    /**
     * イベントペイロードを Rust の serde 期待形式にエンコードする。
     * Encode event payload into the format expected by Rust's serde deserialization.
     *
     * <ul>
     *   <li>UNIT_EVENTS (train_arrive 等): 常に 0x90 (empty fixarray = Some(()))</li>
     *   <li>実引数 0 個: 0x90 (Some(()) 相当)</li>
     *   <li>実引数 1 個: packAny(arg)</li>
     *   <li>実引数 2 個以上: array([packAny(arg), ...])</li>
     * </ul>
     *
     * @param eventName イベント名 (attachment name なし)
     * @param rawArgs   args passed to queueEvent() — rawArgs[0] = attachment name
     */
    private static byte[] encodeEventPayload(String eventName, Object[] rawArgs) {
        // UNIT_EVENTS: 常にイベント発生シグナル (Some(())) を返す
        if (UNIT_EVENTS.contains(eventName)) {
            return EMPTY_FIXARRAY.clone();
        }

        // rawArgs[0] = attachment name → スキップして実引数を取得
        // rawArgs[0] = attachment name → skip to get real args
        int start = rawArgs.length > 0 ? 1 : 0;
        int realCount = rawArgs.length - start;

        if (realCount == 0) {
            // 実引数なし → Some(()) = 0x90
            return EMPTY_FIXARRAY.clone();
        }

        if (realCount == 1) {
            // 単一引数 → そのままエンコード
            return MsgPack.packAny(rawArgs[start]);
        }

        // 複数引数 → 配列としてエンコード
        List<byte[]> items = new ArrayList<>(realCount);
        for (int i = start; i < rawArgs.length; i++) {
            items.add(MsgPack.packAny(rawArgs[i]));
        }
        return MsgPack.array(items);
    }

    // ------------------------------------------------------------------
    // リフレクションヘルパー / Reflection helpers
    // ------------------------------------------------------------------

    /** peripheral.attach(proxy) を呼び出す */
    private static void callAttach(Object peripheral, Object proxy) throws Exception {
        Method m = findMethodByName(peripheral.getClass(), "attach");
        if (m != null) {
            m.invoke(peripheral, proxy);
        } else {
            LOGGER.debug("CCEventReceiver: no attach() method found on {}",
                    peripheral.getClass().getSimpleName());
        }
    }

    /** peripheral.detach(proxy) を呼び出す */
    private static void callDetach(Object peripheral, Object proxy) throws Exception {
        Method m = findMethodByName(peripheral.getClass(), "detach");
        if (m != null) {
            m.invoke(peripheral, proxy);
        }
    }

    /**
     * クラスから引数 1 個のメソッドを名前で検索する。
     * Find a 1-parameter method by name on a class (public methods only).
     */
    private static Method findMethodByName(Class<?> cls, String name) {
        for (Method m : cls.getMethods()) {
            if (m.getName().equals(name) && m.getParameterCount() == 1) {
                m.setAccessible(true);
                return m;
            }
        }
        return null;
    }
}
