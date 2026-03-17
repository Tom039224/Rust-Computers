package com.rustcomputers.peripheral.event;

/**
 * イベント型定義。
 * Event type definitions for CC:Tweaked compatible peripherals.
 *
 * <p>各イベントは固有のIDと名前を持ち、Rust側とのFFI通信で使用される。</p>
 * <p>Each event has a unique ID and name used for FFI communication with Rust.</p>
 */
public enum EventType {
    MODEM_MESSAGE(0, "modem_message"),
    MONITOR_TOUCH(1, "monitor_touch"),
    CHAT(2, "chat"),
    PLAYER_JOIN(3, "playerJoin"),
    PLAYER_LEAVE(4, "playerLeave"),
    KEY(5, "key"),
    KEY_UP(6, "key_up"),
    TRAIN_PASS(7, "train_pass");

    private final int id;
    private final String eventName;

    EventType(int id, String eventName) {
        this.id = id;
        this.eventName = eventName;
    }

    /**
     * イベントIDを取得する（FFI通信用）。
     * Get the event ID (for FFI communication).
     */
    public int getId() {
        return id;
    }

    /**
     * イベント名を取得する。
     * Get the event name.
     */
    public String getEventName() {
        return eventName;
    }

    /**
     * イベント名からEventTypeを取得する。
     * Get EventType from event name.
     *
     * @param eventName イベント名 / event name
     * @return EventType、見つからない場合はnull / EventType or null if not found
     */
    public static EventType fromName(String eventName) {
        for (EventType type : values()) {
            if (type.eventName.equals(eventName)) {
                return type;
            }
        }
        return null;
    }

    /**
     * イベントIDからEventTypeを取得する。
     * Get EventType from event ID.
     *
     * @param id イベントID / event ID
     * @return EventType、見つからない場合はnull / EventType or null if not found
     */
    public static EventType fromId(int id) {
        for (EventType type : values()) {
            if (type.id == id) {
                return type;
            }
        }
        return null;
    }
}
