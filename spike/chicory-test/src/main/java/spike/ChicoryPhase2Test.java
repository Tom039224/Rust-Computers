package spike;

import com.dylibso.chicory.compiler.MachineFactoryCompiler;
import com.dylibso.chicory.runtime.HostFunction;
import com.dylibso.chicory.runtime.ImportValues;
import com.dylibso.chicory.runtime.Instance;
import com.dylibso.chicory.wasi.WasiExitException;
import com.dylibso.chicory.wasi.WasiOptions;
import com.dylibso.chicory.wasi.WasiPreview1;
import com.dylibso.chicory.wasm.Parser;
import com.dylibso.chicory.wasm.WasmModule;
import com.dylibso.chicory.wasm.types.FunctionType;
import com.dylibso.chicory.wasm.types.ValType;

import java.io.ByteArrayOutputStream;
import java.nio.charset.StandardCharsets;
import java.util.List;
import java.util.concurrent.atomic.AtomicInteger;

/**
 * Phase 2 スパイク: Java HostFunction + WASM Memory API 検証
 *
 * 検証項目:
 * 1. Rust->Java ポインタ渡し: log_str(ptr, len) → Java が WASM メモリを読む
 * 2. Rust->Java->Rust 戻り値: compute(a, b) → i32 を Java が計算して返す
 * 3. Java->WASM メモリ書き込み: fill_buf(out_ptr, max_len) → Java が WASM メモリに書く
 */
public class ChicoryPhase2Test {

    private static final String WASM_RESOURCE = "/host_funcs.wasm";

    // Java 側で受け取ったログを記録するリスト
    private static final java.util.List<String> hostLogs = new java.util.ArrayList<>();
    private static int computeCallCount = 0;

    public static void main(String[] args) throws Exception {
        System.out.println("=== Phase 2: HostFunction + Memory API Spike ===\n");

        var wasmStream = ChicoryPhase2Test.class.getResourceAsStream(WASM_RESOURCE);
        if (wasmStream == null) {
            System.err.println("ERROR: WASM resource not found: " + WASM_RESOURCE);
            System.err.println("Run: cargo build --bin host_funcs --target wasm32-wasip1 --release");
            System.exit(1);
        }

        WasmModule module = Parser.parse(wasmStream);
        System.out.println("[Setup] WASM module parsed OK\n");

        // --- HostFunction 定義 ---

        // 1. log_str(ptr: i32, len: i32) -> void
        //    Rust 側の文字列ポインタを Java が WASM メモリから読む
        var logStrFn = new HostFunction(
            "env",
            "log_str",
            FunctionType.of(List.of(ValType.I32, ValType.I32), List.of()),
            (Instance instance, long... fnArgs) -> {
                int ptr = (int) fnArgs[0];
                int len = (int) fnArgs[1];
                String msg = instance.memory().readString(ptr, len);
                hostLogs.add(msg);
                System.out.printf("  [Java HostFn] log_str received: \"%s\"%n", msg);
                return null;
            }
        );

        // 2. compute(a: i32, b: i32) -> i32
        //    Java 側で a * b + 1 を計算して返す
        var computeFn = new HostFunction(
            "env",
            "compute",
            FunctionType.of(List.of(ValType.I32, ValType.I32), List.of(ValType.I32)),
            (Instance instance, long... fnArgs) -> {
                int a = (int) fnArgs[0];
                int b = (int) fnArgs[1];
                int result = a * b + 1;
                computeCallCount++;
                System.out.printf("  [Java HostFn] compute(%d, %d) = %d%n", a, b, result);
                return new long[]{ result };
            }
        );

        // 3. fill_buf(out_ptr: i32, max_len: i32) -> i32
        //    Java が WASM の線形メモリに文字列を書き込み、書き込みバイト数を返す
        var fillBufFn = new HostFunction(
            "env",
            "fill_buf",
            FunctionType.of(List.of(ValType.I32, ValType.I32), List.of(ValType.I32)),
            (Instance instance, long... fnArgs) -> {
                int outPtr = (int) fnArgs[0];
                int maxLen = (int) fnArgs[1];
                byte[] payload = "message_from_java".getBytes(StandardCharsets.UTF_8);
                int writeLen = Math.min(payload.length, maxLen);
                instance.memory().write(outPtr, payload, 0, writeLen);
                System.out.printf("  [Java HostFn] fill_buf wrote %d bytes to WASM memory @ 0x%x%n",
                        writeLen, outPtr);
                return new long[]{ writeLen };
            }
        );

        // --- WASI + HostFunctions を組み合わせて Instance 生成 ---
        var capturedStdout = new ByteArrayOutputStream();
        var capturedStderr = new ByteArrayOutputStream();

        var wasiOpts = WasiOptions.builder()
                .withStdout(capturedStdout)
                .withStderr(capturedStderr)
                .withArguments(List.of("host_funcs"))
                .build();

        System.out.println("[Execution] Running host_funcs.wasm with Runtime Compiler...\n");
        long startNs = System.nanoTime();

        try (var wasi = WasiPreview1.builder().withOptions(wasiOpts).build()) {
            Instance.builder(module)
                    .withMachineFactory(MachineFactoryCompiler::compile)
                    .withImportValues(
                            ImportValues.builder()
                                    .addFunction(wasi.toHostFunctions())
                                    .addFunction(logStrFn, computeFn, fillBufFn)
                                    .build())
                    .build();  // _start 自動実行
        } catch (WasiExitException e) {
            // exit(0) は想定通り
        } catch (Exception e) {
            System.err.println("ERROR: " + e.getMessage());
            e.printStackTrace();
            System.exit(1);
        }

        long elapsedMs = (System.nanoTime() - startNs) / 1_000_000;

        // --- WASM 側の WASI stdout 出力 ---
        String wasmStdout = capturedStdout.toString();
        System.out.println("[WASM stdout]");
        for (String line : wasmStdout.split("\n")) {
            System.out.println("  > " + line);
        }
        System.out.println();

        // --- 検証 ---
        System.out.println("[Validation]");

        // Test 1: log_str
        boolean logOk = !hostLogs.isEmpty() && hostLogs.stream()
                .anyMatch(s -> s.contains("Hello from Rust WASM (via pointer)"));
        System.out.printf("  Test 1 (log_str / Rust->Java string): %s%n",
                logOk ? "✅ PASS" : "❌ FAIL (logs=" + hostLogs + ")");

        // Test 2: compute
        boolean computeOk = computeCallCount > 0 && wasmStdout.contains("✅ compute result correct");
        System.out.printf("  Test 2 (compute / Rust->Java->Rust i32): %s (called %d times)%n",
                computeOk ? "✅ PASS" : "❌ FAIL", computeCallCount);

        // Test 3: fill_buf (WASM stdout に "from_java" が含まれるか)
        boolean memOk = wasmStdout.contains("✅ Java->WASM memory write correct");
        System.out.printf("  Test 3 (fill_buf / Java->WASM memory write): %s%n",
                memOk ? "✅ PASS" : "❌ FAIL");

        System.out.printf("%n  Total elapsed: %d ms%n", elapsedMs);

        boolean allPassed = logOk && computeOk && memOk;
        System.out.println("\n" + (allPassed ? "✅ Phase 2 ALL PASSED" : "❌ Phase 2 SOME FAILED"));
        System.exit(allPassed ? 0 : 1);
    }
}
