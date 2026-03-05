package com.rustcomputers.peripheral;

import net.minecraft.core.BlockPos;
import net.minecraft.server.level.ServerLevel;

import javax.annotation.Nullable;

/**
 * ペリフェラル型の定義インターフェース。
 * Interface defining a peripheral type.
 *
 * <p>各 Minecraft ブロックが提供するペリフェラル機能を定義する。
 * サードパーティ Mod は {@link PeripheralProvider#register} で登録できる。</p>
 *
 * <p>Defines the peripheral capabilities provided by a Minecraft block.
 * Third-party mods can register implementations via {@link PeripheralProvider#register}.</p>
 */
public interface PeripheralType {

    /**
     * ペリフェラルの型名（例: "chest", "furnace", "redstone"）。
     * Peripheral type name (e.g., "chest", "furnace", "redstone").
     *
     * @return 型名文字列 / type name string
     */
    String getTypeName();

    /**
     * サポートするメソッド名の一覧を返す。
     * Return the list of supported method names.
     *
     * @return メソッド名の配列 / array of method names
     */
    String[] getMethodNames();

    /**
     * メソッドを呼び出す（非同期リクエスト用）。
     * Call a method (for async requests).
     *
     * <p>host_request_info / host_do_action から呼ばれる。
     * 結果は MessagePack バイト列で返す（空結果は空配列）。</p>
     *
     * <p>Called from host_request_info / host_do_action.
     * Returns the result as MessagePack bytes (empty array for void).</p>
     *
     * @param methodName メソッド名 / method name
     * @param args       MessagePack 引数 / MessagePack arguments
     * @param level      サーバーレベル / server level
     * @param peripheralPos ペリフェラルブロックの座標 / peripheral block position
     * @return 結果バイト列 / result bytes
     * @throws PeripheralException メソッド実行エラー / method execution error
     */
    byte[] callMethod(String methodName, byte[] args,
                      ServerLevel level, BlockPos peripheralPos) throws PeripheralException;

    /**
     * 即時メソッド呼び出し（host_request_info_imm 用）。
     * Immediate method call (for host_request_info_imm).
     *
     * <p>デフォルトでは通常の callMethod に委譲する。
     * 安全な読み取り専用操作のみオーバーライドすること。</p>
     *
     * <p>By default, delegates to the regular callMethod.
     * Override only for safe, read-only operations.</p>
     *
     * @param methodName メソッド名 / method name
     * @param args       引数 / arguments
     * @param level      サーバーレベル / server level
     * @param peripheralPos ペリフェラルブロックの座標 / peripheral block position
     * @return 結果バイト列、null なら unsupported / result bytes, null if unsupported
     */
    @Nullable
    default byte[] callImmediate(String methodName, byte[] args,
                                 ServerLevel level, BlockPos peripheralPos) throws PeripheralException {
        return callMethod(methodName, args, level, peripheralPos);
    }
}
