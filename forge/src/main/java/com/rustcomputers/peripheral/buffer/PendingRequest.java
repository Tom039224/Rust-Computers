package com.rustcomputers.peripheral.buffer;

/**
 * 保留中のリクエスト。
 * Pending request record.
 *
 * @param methodName メソッド名 / method name
 * @param args       引数（MessagePack形式） / arguments (MessagePack format)
 * @param isAction   アクション系メソッドかどうか / whether this is an action method
 */
public record PendingRequest(
        String methodName,
        byte[] args,
        boolean isAction
) {
    /**
     * クエリリクエストを作成する。
     * Create a query request.
     */
    public static PendingRequest query(String methodName, byte[] args) {
        return new PendingRequest(methodName, args, false);
    }

    /**
     * アクションリクエストを作成する。
     * Create an action request.
     */
    public static PendingRequest action(String methodName, byte[] args) {
        return new PendingRequest(methodName, args, true);
    }
}
