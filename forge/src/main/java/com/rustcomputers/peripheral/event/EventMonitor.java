package com.rustcomputers.peripheral.event;

import com.rustcomputers.peripheral.AttachedPeripheral;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.*;
import java.util.concurrent.ConcurrentHashMap;

/**
 * イベント監視とキュー管理。
 * Event monitoring and queue management.
 *
 * <p>ペリフェラルからのイベントを監視し、キューに保存する。
 * Rust側からのリクエストに応じてイベントを返す。</p>
 *
 * <p>Monitors events from peripherals and stores them in queues.
 * Returns events in response to requests from Rust.</p>
 */
public class EventMonitor {
    private static final Logger LOGGER = LoggerFactory.getLogger(EventMonitor.class);

    /**
     * イベントリスナー: periph_id → event_name → listener
     * Event listeners: periph_id → event_name → listener
     */
    private final Map<Integer, Map<String, EventListener>> listeners = new ConcurrentHashMap<>();

    /**
     * イベントキュー: periph_id → event_name → queue of events
     * Event queue: periph_id → event_name → queue of events
     */
    private final Map<Integer, Map<String, Queue<Object>>> eventQueues = new ConcurrentHashMap<>();

    /**
     * イベントリスナーを登録する。
     * Register an event listener.
     *
     * @param periphId  ペリフェラルID / peripheral ID
     * @param eventName イベント名 / event name
     */
    public void registerListener(int periphId, String eventName) {
        listeners.computeIfAbsent(periphId, k -> new ConcurrentHashMap<>())
                .put(eventName, new EventListener(eventName));
        LOGGER.debug("Registered event listener: periphId={}, eventName={}", periphId, eventName);
    }

    /**
     * イベントリスナーを解除する。
     * Unregister an event listener.
     *
     * @param periphId  ペリフェラルID / peripheral ID
     * @param eventName イベント名 / event name
     */
    public void unregisterListener(int periphId, String eventName) {
        Map<String, EventListener> periphListeners = listeners.get(periphId);
        if (periphListeners != null) {
            periphListeners.remove(eventName);
            LOGGER.debug("Unregistered event listener: periphId={}, eventName={}", periphId, eventName);
        }
    }

    /**
     * 全ペリフェラルのイベントをポーリングする。
     * Poll events from all peripherals.
     *
     * @param peripherals 接続されているペリフェラルのマップ / map of attached peripherals
     */
    public void pollEvents(Map<Integer, AttachedPeripheral> peripherals) {
        for (Map.Entry<Integer, Map<String, EventListener>> entry : listeners.entrySet()) {
            int periphId = entry.getKey();
            Map<String, EventListener> periphListeners = entry.getValue();
            AttachedPeripheral periph = peripherals.get(periphId);

            if (periph == null) {
                continue;
            }

            for (Map.Entry<String, EventListener> listenerEntry : periphListeners.entrySet()) {
                String eventName = listenerEntry.getKey();

                try {
                    // イベントをポーリング（実装はペリフェラルタイプに依存）
                    // Poll for event (implementation depends on peripheral type)
                    Object event = pollEventFromPeripheral(periph, eventName);

                    if (event != null) {
                        queueEvent(periphId, eventName, event);
                    }
                } catch (Exception e) {
                    LOGGER.warn("Event polling failed for periphId={}, eventName={}: {}",
                            periphId, eventName, e.getMessage());
                }
            }
        }
    }

    /**
     * ペリフェラルからイベントをポーリングする。
     * Poll an event from a peripheral.
     *
     * @param peripheral ペリフェラル / peripheral
     * @param eventName  イベント名 / event name
     * @return イベントデータ、なければnull / event data or null if none
     */
    private Object pollEventFromPeripheral(AttachedPeripheral peripheral, String eventName) {
        // TODO: ペリフェラルタイプごとのイベントポーリング実装
        // TODO: Implement event polling for each peripheral type
        // 現在は未実装（Phase 2の後続タスクで実装）
        // Currently not implemented (will be implemented in subsequent Phase 2 tasks)
        return null;
    }

    /**
     * イベントをキューに追加する。
     * Queue an event.
     *
     * @param periphId  ペリフェラルID / peripheral ID
     * @param eventName イベント名 / event name
     * @param event     イベントデータ / event data
     */
    public void queueEvent(int periphId, String eventName, Object event) {
        eventQueues.computeIfAbsent(periphId, k -> new ConcurrentHashMap<>())
                .computeIfAbsent(eventName, k -> new LinkedList<>())
                .offer(event);
        LOGGER.debug("Queued event: periphId={}, eventName={}", periphId, eventName);
    }

    /**
     * イベントキューから全イベントを取得してクリアする。
     * Get all events from the queue and clear it.
     *
     * @param periphId  ペリフェラルID / peripheral ID
     * @param eventName イベント名 / event name
     * @return イベントのリスト / list of events
     */
    public List<Object> getEvents(int periphId, String eventName) {
        Queue<Object> queue = eventQueues.getOrDefault(periphId, new HashMap<>())
                .getOrDefault(eventName, new LinkedList<>());

        List<Object> result = new ArrayList<>(queue);
        queue.clear();
        return result;
    }

    /**
     * 特定のペリフェラルの全イベントをクリアする。
     * Clear all events for a specific peripheral.
     *
     * @param periphId ペリフェラルID / peripheral ID
     */
    public void clearEvents(int periphId) {
        eventQueues.remove(periphId);
        LOGGER.debug("Cleared all events for periphId={}", periphId);
    }

    /**
     * 全イベントをクリアする。
     * Clear all events.
     */
    public void clearAllEvents() {
        eventQueues.clear();
        LOGGER.debug("Cleared all events");
    }

    /**
     * リスナーマップを取得する（テスト用）。
     * Get the listeners map (for testing).
     */
    Map<Integer, Map<String, EventListener>> getListeners() {
        return listeners;
    }
}
