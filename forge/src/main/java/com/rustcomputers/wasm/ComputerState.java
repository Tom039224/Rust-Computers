package com.rustcomputers.wasm;

/**
 * コンピューターの実行状態。
 * Execution state of a computer.
 */
public enum ComputerState {

    /** 停止中 — プログラムが選択されていないか、実行前 / Stopped — no program selected or not yet started */
    STOPPED,

    /** 実行中 — wasm_tick() が毎 tick 呼ばれている / Running — wasm_tick() is called every tick */
    RUNNING,

    /** クラッシュ — Panic / Fuel 切れ / タイムアウトで異常終了 / Crashed — abnormal termination */
    CRASHED;

    /**
     * 整数値から状態を復元する（NBT 保存用）。
     * Restore state from an integer value (for NBT persistence).
     *
     * @param ordinal 保存された序数 / stored ordinal
     * @return 対応する状態、範囲外なら STOPPED / corresponding state, or STOPPED if out of range
     */
    public static ComputerState fromOrdinal(int ordinal) {
        ComputerState[] values = values();
        if (ordinal >= 0 && ordinal < values.length) {
            return values[ordinal];
        }
        return STOPPED;
    }
}
