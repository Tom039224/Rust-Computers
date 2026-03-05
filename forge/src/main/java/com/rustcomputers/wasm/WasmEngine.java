package com.rustcomputers.wasm;

import com.dylibso.chicory.compiler.MachineFactoryCompiler;
import com.dylibso.chicory.runtime.ExportFunction;
import com.dylibso.chicory.runtime.ImportValues;
import com.dylibso.chicory.runtime.Instance;
import com.dylibso.chicory.runtime.Memory;
import com.dylibso.chicory.wasm.Parser;
import com.dylibso.chicory.wasm.WasmModule;
import com.rustcomputers.Config;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.annotation.Nullable;
import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.time.LocalDateTime;
import java.time.format.DateTimeFormatter;

/**
 * 個々のコンピューターに対応する WASM 実行エンジン。
 * Per-computer WASM execution engine.
 *
 * <p>Chicory Runtime Compiler を使用して WASM バイナリを JIT コンパイルし、
 * 毎 tick {@code wasm_tick()} を呼び出す。</p>
 *
 * <p>Uses Chicory Runtime Compiler to JIT-compile WASM binaries and
 * calls {@code wasm_tick()} every server tick.</p>
 */
public final class WasmEngine {

    private static final Logger LOGGER = LoggerFactory.getLogger(WasmEngine.class);
    private static final DateTimeFormatter LOG_TIME_FMT = DateTimeFormatter.ofPattern("HH:mm:ss");

    // ------------------------------------------------------------------
    // フィールド / Fields
    // ------------------------------------------------------------------

    private final int computerId;
    private final Path computerDir;
    private final LogBuffer logBuffer;
    private final RequestManager requestManager;
    private final HostFunctions hostFunctions;

    private ComputerState state = ComputerState.STOPPED;
    @Nullable private String programName;
    @Nullable private Instance instance;
    @Nullable private ExportFunction wasmTick;
    private long tickCount;

    /** stdin 行入力の保留リクエスト ID（null = WASM が read_line を待っていない） */
    /** Pending stdin line request ID (null = WASM is not awaiting read_line) */
    @Nullable private Long pendingStdinRequestId;

    // ------------------------------------------------------------------
    // コンストラクタ / Constructor
    // ------------------------------------------------------------------

    /**
     * @param computerId  コンピューター ID / computer ID
     * @param computerDir コンピューターのファイルディレクトリ / computer file directory
     */
    public WasmEngine(int computerId, Path computerDir) {
        this.computerId = computerId;
        this.computerDir = computerDir;
        this.logBuffer = new LogBuffer(Config.LOG_BUFFER_SIZE.get());
        this.requestManager = new RequestManager();
        this.hostFunctions = new HostFunctions(this);
    }

    // ------------------------------------------------------------------
    // ライフサイクル / Lifecycle
    // ------------------------------------------------------------------

    /**
     * 指定プログラムをロードし実行を開始する。
     * Load the specified program and start execution.
     *
     * @param wasmFileName .wasm ファイル名 / .wasm file name
     * @return 成功したら true / true if successful
     */
    public boolean start(String wasmFileName) {
        if (state == ComputerState.RUNNING) {
            LOGGER.warn("Computer #{}: already running, stopping first", computerId);
            shutdown();
        }

        Path wasmPath = computerDir.resolve(wasmFileName);
        if (!Files.exists(wasmPath)) {
            appendLog("[ERROR] File not found: " + wasmFileName);
            return false;
        }

        try {
            // バリデーション / Validation
            long size = Files.size(wasmPath);
            if (size > Config.MAX_WASM_SIZE.get()) {
                appendLog("[ERROR] File too large: " + size + " bytes (max " + Config.MAX_WASM_SIZE.get() + ")");
                return false;
            }

            byte[] bytes = Files.readAllBytes(wasmPath);
            if (!isValidWasm(bytes)) {
                appendLog("[ERROR] Invalid WASM binary: " + wasmFileName);
                return false;
            }

            // WASM モジュールをパース / Parse WASM module
            WasmModule module = Parser.parse(bytes);

            // Chicory Instance を構築（Runtime Compiler で JIT コンパイル）
            // Build Chicory Instance (JIT compile with Runtime Compiler)
            instance = Instance.builder(module)
                    .withMachineFactory(MachineFactoryCompiler::compile)
                    .withImportValues(
                            ImportValues.builder()
                                    .addFunction(hostFunctions.toArray())
                                    .build())
                    .build();

            // wasm_init() を呼び出す / Call wasm_init()
            ExportFunction wasmInit = instance.export("wasm_init");
            wasmInit.apply();

            // wasm_tick() のエクスポートを取得 / Get wasm_tick() export
            wasmTick = instance.export("wasm_tick");

            programName = wasmFileName;
            state = ComputerState.RUNNING;
            tickCount = 0;
            appendLog("Program started: " + wasmFileName);
            LOGGER.info("Computer #{}: started {}", computerId, wasmFileName);
            return true;

        } catch (IOException e) {
            appendLog("[ERROR] Failed to read: " + wasmFileName + " — " + e.getMessage());
            LOGGER.error("Computer #{}: failed to read WASM file", computerId, e);
            return false;
        } catch (Exception e) {
            crash("Failed to initialize: " + e.getMessage());
            LOGGER.error("Computer #{}: WASM init failed", computerId, e);
            return false;
        }
    }

    /**
     * 毎 tick 呼ばれるメインループ。
     * Main loop called every server tick.
     *
     * <p>design-proposal W-2 のフロー:
     * 1. 保留結果のタイムアウト処理
     * 2. wasm_tick() を呼ぶ
     * 3. 戻り値を確認</p>
     */
    public void tick() {
        if (state != ComputerState.RUNNING || wasmTick == null) {
            return;
        }

        tickCount++;
        requestManager.tick(tickCount, Config.REQUEST_TIMEOUT_TICKS.get());

        try {
            long[] result = wasmTick.apply();
            int returnCode = (int) result[0];

            switch (returnCode) {
                case 1 -> { /* 継続 / Continue */ }
                case 0 -> {
                    // main() 正常終了 / main() finished normally
                    appendLog("Program finished.");
                    LOGGER.info("Computer #{}: program finished normally", computerId);
                    shutdown();
                }
                default -> {
                    // -1 = panic, その他 = 不明なエラー / -1 = panic, other = unknown error
                    crash("wasm_tick returned " + returnCode + " at tick " + tickCount);
                }
            }
        } catch (Exception e) {
            crash("Exception during wasm_tick: " + e.getMessage());
            LOGGER.error("Computer #{}: wasm_tick exception", computerId, e);
        }
    }

    /**
     * 正常シャットダウン。
     * Graceful shutdown.
     */
    public void shutdown() {
        state = ComputerState.STOPPED;
        instance = null;
        wasmTick = null;
        pendingStdinRequestId = null;
        requestManager.clear();
        LOGGER.info("Computer #{}: shutdown", computerId);
    }

    /**
     * クラッシュ処理 — ログ保存して CRASHED 状態へ移行。
     * Crash handling — save log and transition to CRASHED state.
     *
     * @param reason クラッシュ理由 / crash reason
     */
    public void crash(String reason) {
        String msg = "CRASHED: " + reason;
        appendLog("[CRASH] " + msg);
        LOGGER.error("Computer #{}: {}", computerId, msg);

        state = ComputerState.CRASHED;
        instance = null;
        wasmTick = null;
        pendingStdinRequestId = null;
        requestManager.clear();

        // クラッシュログをファイルに保存 / Save crash log to file
        saveCrashLog(reason);
    }

    // ------------------------------------------------------------------
    // ログ / Logging
    // ------------------------------------------------------------------

    /**
     * ログ行を追加する（タイムスタンプ付き）。
     * Append a log line with timestamp.
     *
     * @param message ログメッセージ / log message
     */
    public void appendLog(String message) {
        String timestamp = LocalDateTime.now().format(LOG_TIME_FMT);
        logBuffer.append("[" + timestamp + "] " + message);
    }

    /**
     * クラッシュログをファイルに保存する。
     * Save the crash log to a file.
     */
    private void saveCrashLog(String reason) {
        try {
            Path logDir = computerDir.resolve("log");
            Files.createDirectories(logDir);

            String filename = LocalDateTime.now()
                    .format(DateTimeFormatter.ofPattern("yyyy-MM-dd_HH-mm-ss")) + ".log";
            Path logFile = logDir.resolve(filename);

            StringBuilder sb = new StringBuilder();
            sb.append("=== RustComputers Crash Log ===\n");
            sb.append("Computer ID: ").append(computerId).append('\n');
            sb.append("Program: ").append(programName != null ? programName : "(none)").append('\n');
            sb.append("Tick: ").append(tickCount).append('\n');
            sb.append("Reason: ").append(reason).append('\n');
            sb.append("\n--- Log Buffer ---\n");
            for (String line : logBuffer.snapshot()) {
                sb.append(line).append('\n');
            }

            Files.writeString(logFile, sb.toString(), StandardCharsets.UTF_8);
            LOGGER.info("Computer #{}: crash log saved to {}", computerId, logFile);
        } catch (IOException e) {
            LOGGER.error("Computer #{}: failed to save crash log", computerId, e);
        }
    }

    // ------------------------------------------------------------------
    // ホスト関数からのコールバック / Host function callbacks
    // ------------------------------------------------------------------

    /**
     * stdin 行入力リクエストを発行する。
     * Issue a stdin line input request.
     *
     * @param resultPtr    Rust 側が確保した結果バッファのアドレス / WASM-allocated result buffer address
     * @param resultBufSize 結果バッファのサイズ / result buffer size
     * @return request_id
     */
    public long requestStdinLine(int resultPtr, int resultBufSize) {
        // 既存の stdin リクエストがあれば破棄 / Cancel existing stdin request if any
        if (pendingStdinRequestId != null) {
            requestManager.remove(pendingStdinRequestId);
        }

        long id = requestManager.nextRequestId();
        // Rust 側で result バッファを確保済み。Java はそこに UTF-8 文字列を書き込む。
        // Result buffer is pre-allocated by Rust. Java writes UTF-8 string data there.
        PendingResult pr = new PendingResult(id, PendingResult.Type.STDIN, resultPtr, resultBufSize, tickCount);
        requestManager.register(pr);
        pendingStdinRequestId = id;
        return id;
    }

    /**
     * GUI から stdin 入力行を受け取る。
     * Receive a stdin input line from the GUI.
     *
     * <p>WASM が read_line().await していない場合は破棄する。</p>
     * <p>Discarded if WASM is not currently awaiting read_line().</p>
     *
     * @param line 入力行 / input line
     */
    public void submitStdinLine(String line) {
        if (pendingStdinRequestId == null) {
            // WASM が read_line を待っていない → 破棄
            // WASM is not awaiting read_line → discard
            LOGGER.debug("Computer #{}: stdin discarded (no pending read_line)", computerId);
            return;
        }

        PendingResult pr = requestManager.get(pendingStdinRequestId);
        if (pr != null && !pr.isCompleted()) {
            byte[] data = line.getBytes(StandardCharsets.UTF_8);
            pr.complete(data);
        }
        pendingStdinRequestId = null;
    }

    /**
     * ペリフェラルリクエストを発行する（host_request_info / host_do_action 用）。
     * Issue a peripheral request (for host_request_info / host_do_action).
     *
     * @return request_id (>0) またはエラーコード (<0) / request_id (>0) or error code (<0)
     */
    public long issuePeripheralRequest(
            PendingResult.Type type,
            int periphId, int methodId,
            Memory memory, int argsPtr, int argsLen,
            int resultPtr, int resultBufSize) {

        // 引数を読み取る / Read arguments from WASM memory
        byte[] argsData = argsLen > 0 ? memory.readBytes(argsPtr, argsLen) : new byte[0];

        long id = requestManager.nextRequestId();
        PendingResult pr = new PendingResult(id, type, resultPtr, resultBufSize, tickCount);
        requestManager.register(pr);

        // TODO: ペリフェラルへのディスパッチを実装
        // TODO: Dispatch to peripheral implementation
        // 現時点ではスタブ: 即座に空結果で完了する
        // For now, stub: complete immediately with empty result
        LOGGER.debug("Computer #{}: peripheral request id={} periph={} method={}",
                computerId, id, periphId, methodId);
        pr.complete(new byte[0]);

        return id;
    }

    /**
     * 即時ペリフェラル情報取得を実行する（host_request_info_imm 用）。
     * Execute an immediate peripheral info request (for host_request_info_imm).
     *
     * @return 書き込みバイト数 (>=0) またはエラーコード (<0) / written bytes (>=0) or error code (<0)
     */
    public int executeImmediate(
            int periphId, int methodId,
            Memory memory, int argsPtr, int argsLen,
            int resultPtr, int resultBufSize) {

        // TODO: 即時ペリフェラル呼び出しを実装
        // TODO: Implement immediate peripheral call
        LOGGER.debug("Computer #{}: immediate request periph={} method={}", computerId, periphId, methodId);
        return 0; // 0 バイト書き込み（スタブ） / 0 bytes written (stub)
    }

    /**
     * 指定 Mod ID が利用可能か確認する。
     * Check if the specified mod ID is available.
     *
     * @param modId Mod 識別子 / mod identifier
     * @return 利用可能なら true / true if available
     */
    public boolean isModAvailable(int modId) {
        // TODO: 実際の Mod 存在確認を実装
        // TODO: Implement actual mod availability check
        return false;
    }

    // ------------------------------------------------------------------
    // WASM バリデーション / WASM validation
    // ------------------------------------------------------------------

    /**
     * WASM バイナリの基本的なバリデーション。
     * Basic validation of a WASM binary.
     *
     * @param bytes バイナリデータ / binary data
     * @return 有効なら true / true if valid
     */
    public static boolean isValidWasm(byte[] bytes) {
        // WASM マジックナンバー: \0asm
        return bytes.length >= 4
                && bytes[0] == 0x00
                && bytes[1] == 'a'
                && bytes[2] == 's'
                && bytes[3] == 'm';
    }

    /**
     * ファイル名のサニタイズ（パストラバーサル対策）。
     * Sanitize a file name (prevent path traversal).
     *
     * @param name 元のファイル名 / original file name
     * @return サニタイズ済みファイル名、無効なら null / sanitized name, or null if invalid
     */
    @Nullable
    public static String sanitizeFileName(String name) {
        if (name == null || name.isBlank()) return null;
        // パス区切り文字と ".." を除去 / Remove path separators and ".."
        String sanitized = name.replaceAll("[/\\\\]", "").replace("..", "");
        if (sanitized.isBlank() || !sanitized.endsWith(".wasm")) return null;
        return sanitized;
    }

    // ------------------------------------------------------------------
    // アクセサ / Accessors
    // ------------------------------------------------------------------

    public int getComputerId()           { return computerId; }
    public ComputerState getState()      { return state; }
    @Nullable public String getProgramName() { return programName; }
    public LogBuffer getLogBuffer()      { return logBuffer; }
    public RequestManager getRequestManager() { return requestManager; }
    public long getTickCount()           { return tickCount; }
    public Path getComputerDir()         { return computerDir; }

    /**
     * コンピューターディレクトリ内の .wasm ファイル一覧を返す。
     * List .wasm files in the computer directory.
     *
     * @return ファイル名の配列 / array of file names
     */
    public String[] listPrograms() {
        try {
            if (!Files.isDirectory(computerDir)) {
                return new String[0];
            }
            return Files.list(computerDir)
                    .filter(p -> p.toString().endsWith(".wasm"))
                    .map(p -> p.getFileName().toString())
                    .sorted()
                    .toArray(String[]::new);
        } catch (IOException e) {
            LOGGER.error("Computer #{}: failed to list programs", computerId, e);
            return new String[0];
        }
    }
}
