package com.rustcomputers.wasm;

import com.dylibso.chicory.runtime.HostFunction;
import com.dylibso.chicory.runtime.Instance;
import com.dylibso.chicory.wasm.types.FunctionType;
import com.dylibso.chicory.wasm.types.ValType;
import com.rustcomputers.peripheral.MsgPack;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.List;

/**
 * WASM にインポートされるホスト関数の定義。
 * Defines all host functions imported by the WASM module.
 *
 * <p>design-proposal W-2 に定義された全ホスト関数をここで実装する。
 * すべてのホスト関数は "env" モジュールに属する。</p>
 *
 * <p>Implements all host functions defined in design-proposal W-2.
 * All host functions belong to the "env" module.</p>
 */
public final class HostFunctions {

    private static final Logger LOGGER = LoggerFactory.getLogger(HostFunctions.class);

    /** ホスト関数のモジュール名 / Host function module name */
    private static final String MODULE = "env";

    private final WasmEngine engine;

    /**
     * @param engine このホスト関数群が紐づく WASM エンジン / the WASM engine these functions belong to
     */
    public HostFunctions(WasmEngine engine) {
        this.engine = engine;
    }

    /**
     * 全ホスト関数を配列として返す（Instance 構築時に渡す）。
     * Return all host functions as an array (passed to Instance builder).
     *
     * @return ホスト関数の配列 / array of host functions
     */
    public HostFunction[] toArray() {
        return new HostFunction[]{
            hostLog(),
            hostStdinReadLine(),
            hostRequestInfo(),
            hostDoAction(),
            hostRequestInfoImm(),
            hostPollResult(),
            hostFetchResult(),
            hostIsModAvailable(),
            hostGetComputerId(),
            hostFindPeripheralsByTypeImm(),
            hostGetTimeUtcMillis(),
            hostGetTimeIngameTicks(),
            hostBookEvent(),
            hostReadEvents(),
        };
    }

    // ==================================================================
    // host_log(ptr: i32, len: i32) → void
    // ==================================================================

    /**
     * ログ出力ホスト関数。println! / eprintln! の出力先。
     * Log output host function. Target for println! / eprintln!.
     */
    private HostFunction hostLog() {
        return new HostFunction(
            MODULE, "host_log",
            FunctionType.of(List.of(ValType.I32, ValType.I32), List.of()),
            (Instance inst, long... args) -> {
                int ptr = (int) args[0];
                int len = (int) args[1];
                String msg = inst.memory().readString(ptr, len);
                engine.appendLog(msg);
                return null;
            }
        );
    }

    // ==================================================================
    // host_stdin_read_line(result_ptr: i32, result_buf_size: i32) → i64
    // ==================================================================

    /**
     * 標準入力（行単位）のリクエスト。Enter が押されるまで Pending。
     * Request a line of standard input. Pending until Enter is pressed.
     *
     * <p>2 フェーズ取得方式: リクエスト時にはバッファ不要。結果は Java 側で䔍持し、
     * host_fetch_result 呼び出し時に WASM メモリに書き込む。</p>
     * <p>Two-phase fetch: no buffer at request time. Result is held by Java
     * and written to WASM memory when host_fetch_result is called.</p>
     */
    private HostFunction hostStdinReadLine() {
        return new HostFunction(
            MODULE, "host_stdin_read_line",
            FunctionType.of(List.of(), List.of(ValType.I64)),
            (Instance inst, long... args) -> {
                long requestId = engine.requestStdinLine();
                return new long[]{ requestId };
            }
        );
    }

    // ==================================================================
    // host_request_info(periph_id, method_id, args_ptr, args_len,
    //                   result_ptr, result_buf_size) → i64
    // ==================================================================

    /**
     * ペリフェラル情報取得リクエスト（非同期、次 tick 以降に結果）。
     * Peripheral info request (async, result available next tick or later).
     *
     * <p>2 フェーズ取得方式: リクエスト時にはバッファ不要。</p>
     * <p>Two-phase fetch: no result buffer at request time.</p>
     */
    private HostFunction hostRequestInfo() {
        return new HostFunction(
            MODULE, "host_request_info",
            FunctionType.of(
                List.of(ValType.I32, ValType.I32, ValType.I32, ValType.I32),
                List.of(ValType.I64)
            ),
            (Instance inst, long... args) -> {
                int periphId    = (int) args[0];
                int methodId    = (int) args[1];
                int argsPtr     = (int) args[2];
                int argsLen     = (int) args[3];

                long requestId = engine.issuePeripheralRequest(
                    PendingResult.Type.INFO,
                    periphId, methodId,
                    inst.memory(), argsPtr, argsLen
                );
                return new long[]{ requestId };
            }
        );
    }

    // ==================================================================
    // host_do_action(periph_id, method_id, args_ptr, args_len,
    //                result_ptr, result_buf_size) → i64
    // ==================================================================

    /**
     * ペリフェラルアクション実行リクエスト（非同期）。
     * Peripheral action request (async).
     *
     * <p>2 フェーズ取得方式: リクエスト時にはバッファ不要。</p>
     * <p>Two-phase fetch: no result buffer at request time.</p>
     */
    private HostFunction hostDoAction() {
        return new HostFunction(
            MODULE, "host_do_action",
            FunctionType.of(
                List.of(ValType.I32, ValType.I32, ValType.I32, ValType.I32),
                List.of(ValType.I64)
            ),
            (Instance inst, long... args) -> {
                int periphId    = (int) args[0];
                int methodId    = (int) args[1];
                int argsPtr     = (int) args[2];
                int argsLen     = (int) args[3];

                long requestId = engine.issuePeripheralRequest(
                    PendingResult.Type.ACTION,
                    periphId, methodId,
                    inst.memory(), argsPtr, argsLen
                );
                return new long[]{ requestId };
            }
        );
    }

    // ==================================================================
    // host_request_info_imm(periph_id, method_id, args_ptr, args_len,
    //                       result_ptr, result_buf_size) → i32
    // ==================================================================

    /**
     * 即時ペリフェラル情報取得（同 tick 内即返）。
     * Immediate peripheral info request (returns within the same tick).
     *
     * <p>@LuaFunction(immediate=true) のメソッドのみ対応。</p>
     * <p>Only supports methods annotated with @LuaFunction(immediate=true).</p>
     */
    private HostFunction hostRequestInfoImm() {
        return new HostFunction(
            MODULE, "host_request_info_imm",
            FunctionType.of(
                List.of(ValType.I32, ValType.I32, ValType.I32, ValType.I32, ValType.I32, ValType.I32),
                List.of(ValType.I32)
            ),
            (Instance inst, long... args) -> {
                int periphId    = (int) args[0];
                int methodId    = (int) args[1];
                int argsPtr     = (int) args[2];
                int argsLen     = (int) args[3];
                int resultPtr   = (int) args[4];
                int resultBufSz = (int) args[5];

                int writtenBytes = engine.executeImmediate(
                    periphId, methodId,
                    inst.memory(), argsPtr, argsLen,
                    resultPtr, resultBufSz
                );
                return new long[]{ writtenBytes };
            }
        );
    }

    // ==================================================================
    // host_poll_result(request_id: i64) → i64
    // ==================================================================

    /**
     * 保留結果をポーリングする（2 フェーズ取得フェーズ 1）。
     * Poll a pending result (two-phase fetch phase 1).
     *
     * <p>戻り値 = 0: pending / 正値 (payload.length + 1): ready / 負: error</p>
     * <p>Return: 0 = pending / positive (payload.length + 1) = ready / negative = error</p>
     *
     * <p>offset-by-1 方式で 0 (空結果) と pending (0) を区別する。</p>
     * <p>offset-by-1 encoding distinguishes ready-with-empty from pending.</p>
     */
    private HostFunction hostPollResult() {
        return new HostFunction(
            MODULE, "host_poll_result",
            FunctionType.of(List.of(ValType.I64), List.of(ValType.I64)),
            (Instance inst, long... args) -> {
                long requestId = args[0];

                PendingResult pr = engine.getRequestManager().get(requestId);
                if (pr == null) {
                    return new long[]{ (long) ErrorCodes.ERR_INVALID_REQUEST_ID };
                }

                if (!pr.isCompleted()) {
                    return new long[]{ 0L }; // Pending
                }

                // エラーチェック / Check for error
                if (pr.getErrorCode() != 0) {
                    engine.getRequestManager().remove(requestId);
                    return new long[]{ (long) pr.getErrorCode() };
                }

                // offset-by-1 で結果サイズを返す（0 の空結果でも 1 と返り pending、0 と区別する）
                // Return result size with offset-by-1 (empty result returns 1, not 0)
                byte[] payload = pr.getPayload();
                long resultSize = (payload != null) ? payload.length : 0L;
                // host_fetch_result でデータを読み出すまでマネージャを削除しない
                // Do NOT remove from manager yet; host_fetch_result will remove it
                return new long[]{ resultSize + 1L };
            }
        );
    }

    // ==================================================================
    // host_fetch_result(request_id: i64, result_ptr: i32, result_buf_size: i32) → i32
    // ==================================================================

    /**
     * 完了した結果データを WASM メモリに転送する（2 フェーズ取得フェーズ 2）。
     * Transfer completed result data into WASM memory (two-phase fetch phase 2).
     *
     * <p>{@code host_poll_result} が正値を返した直後に呼び出す。
     * Rust 側が result のサイズ分だけ動的確保したバッファに書き込む。</p>
     * <p>Called immediately after {@code host_poll_result} returns a positive value.
     * Writes to a Rust-dynamically-allocated buffer of exactly the right size.</p>
     *
     * @return 書き込みバイト数 (&gt;=0) | エラーコード (&lt;0) /
     *         written bytes (≥0) | error code (&lt;0)
     */
    private HostFunction hostFetchResult() {
        return new HostFunction(
            MODULE, "host_fetch_result",
            FunctionType.of(
                List.of(ValType.I64, ValType.I32, ValType.I32),
                List.of(ValType.I32)
            ),
            (Instance inst, long... args) -> {
                long requestId   = args[0];
                int resultPtr    = (int) args[1];
                int resultBufSz  = (int) args[2];

                PendingResult pr = engine.getRequestManager().get(requestId);
                if (pr == null) {
                    return new long[]{ ErrorCodes.ERR_INVALID_REQUEST_ID };
                }

                byte[] payload = pr.getPayload();
                engine.getRequestManager().remove(requestId);

                if (payload == null || payload.length == 0) {
                    // 空ペイロード / Empty payload
                    return new long[]{ 0 };
                }

                if (payload.length > resultBufSz) {
                    // バッファ不足（Rust 側のバグ: host_poll_result の値 -1 分だけ確保するはず）
                    // Buffer too small (Rust bug: should allocate host_poll_result value - 1 bytes)
                    LOGGER.warn("Computer #{}: host_fetch_result: buffer too small ({} < {})",
                            engine.getComputerId(), resultBufSz, payload.length);
                    return new long[]{ ErrorCodes.ERR_RESULT_BUF_TOO_SMALL };
                }

                // WASM メモリに書き込む / Write to WASM memory
                inst.memory().write(resultPtr, payload);
                return new long[]{ payload.length };
            }
        );
    }

    // ==================================================================
    // host_is_mod_available(mod_id: u16) → i32
    // ==================================================================

    /**
     * 指定 Mod が利用可能か確認する。
     * Check whether the specified mod is available.
     */
    private HostFunction hostIsModAvailable() {
        return new HostFunction(
            MODULE, "host_is_mod_available",
            FunctionType.of(List.of(ValType.I32), List.of(ValType.I32)),
            (Instance inst, long... args) -> {
                int modId = (int) args[0] & 0xFFFF;
                boolean available = engine.isModAvailable(modId);
                return new long[]{ available ? 1 : 0 };
            }
        );
    }

    // ==================================================================
    // host_get_computer_id() → i32
    // ==================================================================

    /**
     * このコンピューターの ID を返す。
     * Return the ID of this computer.
     */
    private HostFunction hostGetComputerId() {
        return new HostFunction(
            MODULE, "host_get_computer_id",
            FunctionType.of(List.of(), List.of(ValType.I32)),
            (Instance inst, long... args) -> {
                return new long[]{ engine.getComputerId() };
            }
        );
    }

    // ==================================================================
    // host_find_peripherals_by_type_imm(name_ptr: i32, name_len: i32,
    //                                   result_ptr: i32, result_buf_size: i32) → i32
    // ==================================================================

    /**
     * 型名に一致するペリフェラルの periph_id リストを即時返す。
     * Immediately return the list of periph_ids matching the given type name.
     *
     * <p>結果は msgpack の uint32 配列としてエンコードされる。</p>
     * <p>Result is encoded as a msgpack array of uint32 periph_ids.</p>
     *
     * @return 結果バッファに書き込んだバイト数、バッファ不足なら負値
     *         / bytes written to result buffer, negative if buffer too small
     */
    private HostFunction hostFindPeripheralsByTypeImm() {
        return new HostFunction(
            MODULE, "host_find_peripherals_by_type_imm",
            FunctionType.of(
                List.of(ValType.I32, ValType.I32, ValType.I32, ValType.I32),
                List.of(ValType.I32)
            ),
            (Instance inst, long... args) -> {
                int namePtr     = (int) args[0];
                int nameLen     = (int) args[1];
                int resultPtr   = (int) args[2];
                int resultBufSz = (int) args[3];

                // WASM メモリから型名を読み取る / Read type name from WASM memory
                String typeName = inst.memory().readString(namePtr, nameLen);

                // 型名に一致する periph_id をリストアップ / Find matching periph_ids
                java.util.List<Integer> ids = engine.findPeripheralsByType(typeName);

                // msgpack 配列としてエンコード / Encode as msgpack array of uint32
                byte[][] encoded = new byte[ids.size()][];
                for (int i = 0; i < ids.size(); i++) {
                    encoded[i] = MsgPack.int32(ids.get(i));
                }
                byte[] payload = MsgPack.array(encoded);

                // バッファサイズチェック / Check buffer size
                if (payload.length > resultBufSz) {
                    LOGGER.warn("Computer #{}: host_find_peripherals_by_type_imm: " +
                            "result buffer too small ({} < {})",
                            engine.getComputerId(), resultBufSz, payload.length);
                    return new long[]{ ErrorCodes.ERR_RESULT_BUF_TOO_SMALL };
                }

                // WASM メモリに書き込む / Write to WASM memory
                inst.memory().write(resultPtr, payload);
                LOGGER.debug("Computer #{}: host_find_peripherals_by_type_imm: " +
                        "type='{}', found={}, written={}",
                        engine.getComputerId(), typeName, ids.size(), payload.length);
                return new long[]{ payload.length };
            }
        );
    }

    // ==================================================================
    // host_get_time_utc_millis() → i64
    // ==================================================================

    /**
     * UTC時刻をミリ秒単位で返す。
     * Return UTC time in milliseconds.
     *
     * @return UTC time (millis since Unix epoch)
     */
    private HostFunction hostGetTimeUtcMillis() {
        return new HostFunction(
            MODULE, "host_get_time_utc_millis",
            FunctionType.of(List.of(), List.of(ValType.I64)),
            (Instance inst, long... args) -> {
                long timeMillis = System.currentTimeMillis();
                return new long[]{ timeMillis };
            }
        );
    }

    // ==================================================================
    // host_get_time_ingame_ticks() → i64
    // ==================================================================

    /**
     * ゲーム内時刻をticks単位で返す。
     * Return in-game time in ticks.
     *
     * @return in-game time (ticks since level creation)
     */
    private HostFunction hostGetTimeIngameTicks() {
        return new HostFunction(
            MODULE, "host_get_time_ingame_ticks",
            FunctionType.of(List.of(), List.of(ValType.I64)),
            (Instance inst, long... args) -> {
                long timeTicks = engine.getGameTicks();
                return new long[]{ timeTicks };
            }
        );
    }

    // ==================================================================
    // host_book_event(periph_id: i32, event_name_ptr: i32, event_name_len: i32) → void
    // ==================================================================

    /**
     * イベントリスナーを登録する。
     * Register an event listener.
     *
     * <p>指定されたペリフェラルの指定されたイベントを監視開始する。</p>
     * <p>Start monitoring the specified event on the specified peripheral.</p>
     *
     * @param periph_id ペリフェラルID / peripheral ID
     * @param event_name_ptr イベント名のポインタ / event name pointer
     * @param event_name_len イベント名の長さ / event name length
     */
    private HostFunction hostBookEvent() {
        return new HostFunction(
            MODULE, "host_book_event",
            FunctionType.of(List.of(ValType.I32, ValType.I32, ValType.I32), List.of()),
            (Instance inst, long... args) -> {
                int periphId = (int) args[0];
                int eventNamePtr = (int) args[1];
                int eventNameLen = (int) args[2];

                // WASM メモリからイベント名を読み取る / Read event name from WASM memory
                String eventName = inst.memory().readString(eventNamePtr, eventNameLen);

                // イベントリスナーを登録 / Register event listener
                engine.getPeripheralRequestManager().registerEventListener(periphId, eventName);

                LOGGER.debug("Computer #{}: host_book_event: periphId={}, eventName={}",
                        engine.getComputerId(), periphId, eventName);
                return null;
            }
        );
    }

    // ==================================================================
    // host_read_events(periph_id: i32, event_name_ptr: i32, event_name_len: i32,
    //                  result_ptr: i32, result_buf_size: i32) → i32
    // ==================================================================

    /**
     * イベントキューから全イベントを取得する。
     * Get all events from the event queue.
     *
     * <p>結果は msgpack の配列としてエンコードされる。
     * 各要素は Option&lt;Event&gt; を表現する（イベント未発生時は null）。</p>
     * <p>Result is encoded as a msgpack array.
     * Each element represents Option&lt;Event&gt; (null if no event occurred).</p>
     *
     * @return 結果バッファに書き込んだバイト数、バッファ不足なら負値
     *         / bytes written to result buffer, negative if buffer too small
     */
    private HostFunction hostReadEvents() {
        return new HostFunction(
            MODULE, "host_read_events",
            FunctionType.of(
                List.of(ValType.I32, ValType.I32, ValType.I32, ValType.I32, ValType.I32),
                List.of(ValType.I32)
            ),
            (Instance inst, long... args) -> {
                int periphId = (int) args[0];
                int eventNamePtr = (int) args[1];
                int eventNameLen = (int) args[2];
                int resultPtr = (int) args[3];
                int resultBufSz = (int) args[4];

                // WASM メモリからイベント名を読み取る / Read event name from WASM memory
                String eventName = inst.memory().readString(eventNamePtr, eventNameLen);

                // イベントキューから全イベントを取得 / Get all events from queue
                java.util.List<Object> events = engine.getPeripheralRequestManager()
                        .getEvents(periphId, eventName);

                // msgpack 配列としてエンコード / Encode as msgpack array
                // Vec<Option<T>> 形式: 各イベントを msgpack でエンコード
                // Vec<Option<T>> format: encode each event as msgpack
                byte[][] encoded = new byte[events.size()][];
                for (int i = 0; i < events.size(); i++) {
                    Object event = events.get(i);
                    if (event instanceof byte[]) {
                        encoded[i] = (byte[]) event;
                    } else if (event instanceof com.rustcomputers.peripheral.buffer.PeripheralError) {
                        // エラーの場合は null として扱う / Treat errors as null
                        encoded[i] = MsgPack.nil();
                    } else {
                        // その他の場合も null として扱う / Treat other cases as null
                        encoded[i] = MsgPack.nil();
                    }
                }
                byte[] payload = MsgPack.array(encoded);

                // バッファサイズチェック / Check buffer size
                if (payload.length > resultBufSz) {
                    LOGGER.warn("Computer #{}: host_read_events: " +
                            "result buffer too small ({} < {})",
                            engine.getComputerId(), resultBufSz, payload.length);
                    return new long[]{ ErrorCodes.ERR_RESULT_BUF_TOO_SMALL };
                }

                // WASM メモリに書き込む / Write to WASM memory
                inst.memory().write(resultPtr, payload);
                LOGGER.debug("Computer #{}: host_read_events: " +
                        "periphId={}, eventName='{}', count={}, written={}",
                        engine.getComputerId(), periphId, eventName, events.size(), payload.length);
                return new long[]{ payload.length };
            }
        );
    }
}
