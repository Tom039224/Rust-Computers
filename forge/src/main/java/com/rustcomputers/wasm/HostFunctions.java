package com.rustcomputers.wasm;

import com.dylibso.chicory.runtime.HostFunction;
import com.dylibso.chicory.runtime.Instance;
import com.dylibso.chicory.wasm.types.FunctionType;
import com.dylibso.chicory.wasm.types.ValType;
import com.rustcomputers.peripheral.MsgPack;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.nio.charset.StandardCharsets;
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
            hostIsModAvailable(),
            hostGetComputerId(),
            hostFindPeripheralsByTypeImm(),
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
     * <p>Rust 側が確保した result バッファのアドレスとサイズを受け取る。</p>
     * <p>Receives the address and size of the result buffer allocated by Rust.</p>
     */
    private HostFunction hostStdinReadLine() {
        return new HostFunction(
            MODULE, "host_stdin_read_line",
            FunctionType.of(List.of(ValType.I32, ValType.I32), List.of(ValType.I64)),
            (Instance inst, long... args) -> {
                int resultPtr   = (int) args[0];
                int resultBufSz = (int) args[1];
                long requestId = engine.requestStdinLine(resultPtr, resultBufSz);
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
     */
    private HostFunction hostRequestInfo() {
        return new HostFunction(
            MODULE, "host_request_info",
            FunctionType.of(
                List.of(ValType.I32, ValType.I32, ValType.I32, ValType.I32, ValType.I32, ValType.I32),
                List.of(ValType.I64)
            ),
            (Instance inst, long... args) -> {
                int periphId    = (int) args[0];
                int methodId    = (int) args[1];
                int argsPtr     = (int) args[2];
                int argsLen     = (int) args[3];
                int resultPtr   = (int) args[4];
                int resultBufSz = (int) args[5];

                long requestId = engine.issuePeripheralRequest(
                    PendingResult.Type.INFO,
                    periphId, methodId,
                    inst.memory(), argsPtr, argsLen,
                    resultPtr, resultBufSz
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
     */
    private HostFunction hostDoAction() {
        return new HostFunction(
            MODULE, "host_do_action",
            FunctionType.of(
                List.of(ValType.I32, ValType.I32, ValType.I32, ValType.I32, ValType.I32, ValType.I32),
                List.of(ValType.I64)
            ),
            (Instance inst, long... args) -> {
                int periphId    = (int) args[0];
                int methodId    = (int) args[1];
                int argsPtr     = (int) args[2];
                int argsLen     = (int) args[3];
                int resultPtr   = (int) args[4];
                int resultBufSz = (int) args[5];

                long requestId = engine.issuePeripheralRequest(
                    PendingResult.Type.ACTION,
                    periphId, methodId,
                    inst.memory(), argsPtr, argsLen,
                    resultPtr, resultBufSz
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
    // host_poll_result(request_id: i64, written_bytes_ptr: i32) → i32
    // ==================================================================

    /**
     * 保留結果をポーリングする。
     * Poll a pending result.
     *
     * @return 0=pending, 1=ready, 負=error
     */
    private HostFunction hostPollResult() {
        return new HostFunction(
            MODULE, "host_poll_result",
            FunctionType.of(List.of(ValType.I64, ValType.I32), List.of(ValType.I32)),
            (Instance inst, long... args) -> {
                long requestId      = args[0];
                int writtenBytesPtr = (int) args[1];

                PendingResult pr = engine.getRequestManager().get(requestId);
                if (pr == null) {
                    return new long[]{ ErrorCodes.ERR_INVALID_REQUEST_ID };
                }

                if (!pr.isCompleted()) {
                    return new long[]{ 0 }; // Pending
                }

                // エラーチェック / Check for error
                if (pr.getErrorCode() != 0) {
                    engine.getRequestManager().remove(requestId);
                    return new long[]{ pr.getErrorCode() };
                }

                // 結果を WASM メモリに書き込む / Write result to WASM memory
                byte[] payload = pr.getPayload();
                if (payload != null && payload.length > 0) {
                    if (payload.length > pr.getResultBufSize()) {
                        // バッファ不足 — エラーを返す / Buffer too small — return error
                        engine.getRequestManager().remove(requestId);
                        return new long[]{ ErrorCodes.ERR_RESULT_BUF_TOO_SMALL };
                    }
                    inst.memory().write(pr.getResultPtr(), payload);

                    // written_bytes を書き込む / Write written_bytes
                    byte[] lenBytes = ByteBuffer.allocate(4)
                            .order(ByteOrder.LITTLE_ENDIAN)
                            .putInt(payload.length)
                            .array();
                    inst.memory().write(writtenBytesPtr, lenBytes);
                } else {
                    // ペイロードが空（void 戻り値等） / Empty payload (void return, etc.)
                    byte[] zeroBytes = new byte[]{ 0, 0, 0, 0 };
                    inst.memory().write(writtenBytesPtr, zeroBytes);
                }

                engine.getRequestManager().remove(requestId);
                return new long[]{ 1 }; // Ready
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
}
