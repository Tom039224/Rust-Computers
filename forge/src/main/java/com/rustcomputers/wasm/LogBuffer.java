package com.rustcomputers.wasm;

import java.util.ArrayDeque;
import java.util.Deque;

/**
 * 固定容量の循環ログバッファ。
 * Fixed-capacity circular log buffer.
 *
 * <p>GUI のログ欄で使用する。容量を超えると古い行が自動的に削除される。</p>
 * <p>Used for the GUI log panel. Older lines are automatically evicted when capacity is exceeded.</p>
 */
public final class LogBuffer {

    private final int capacity;
    private final Deque<String> lines;

    /**
     * @param capacity 最大行数 / maximum number of lines
     */
    public LogBuffer(int capacity) {
        this.capacity = capacity;
        this.lines = new ArrayDeque<>(capacity);
    }

    /**
     * ログ行を追加する。容量超過時は先頭（最古）行を削除。
     * Append a log line. Evicts the oldest line if capacity is exceeded.
     *
     * @param line 追加する行 / line to append
     */
    public synchronized void append(String line) {
        if (lines.size() >= capacity) {
            lines.pollFirst();
        }
        lines.addLast(line);
    }

    /**
     * 全ログ行のスナップショットを配列として返す。
     * Return a snapshot of all log lines as an array.
     *
     * @return ログ行の配列（古い順） / array of log lines (oldest first)
     */
    public synchronized String[] snapshot() {
        return lines.toArray(new String[0]);
    }

    /**
     * 現在の行数を返す。
     * Return the current number of lines.
     */
    public synchronized int size() {
        return lines.size();
    }

    /**
     * 全行を消去する。
     * Clear all lines.
     */
    public synchronized void clear() {
        lines.clear();
    }

    /**
     * 最新 n 行を返す（GUI 表示用）。
     * Return the latest n lines (for GUI display).
     *
     * @param n 取得する行数 / number of lines to retrieve
     * @return 最新 n 行の配列 / array of the latest n lines
     */
    public synchronized String[] tail(int n) {
        int size = lines.size();
        int skip = Math.max(0, size - n);
        return lines.stream().skip(skip).toArray(String[]::new);
    }
}
