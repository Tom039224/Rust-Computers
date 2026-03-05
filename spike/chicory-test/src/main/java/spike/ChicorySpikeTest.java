package spike;

import com.dylibso.chicory.compiler.MachineFactoryCompiler;
import com.dylibso.chicory.runtime.ImportValues;
import com.dylibso.chicory.runtime.Instance;
import com.dylibso.chicory.wasi.WasiExitException;
import com.dylibso.chicory.wasi.WasiOptions;
import com.dylibso.chicory.wasi.WasiPreview1;
import com.dylibso.chicory.wasm.Parser;
import com.dylibso.chicory.wasm.WasmModule;

import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.File;
import java.util.List;

/**
 * RustComputers スパイク実装: Chicory Runtime Compiler + WASI 動作検証
 *
 * 検証項目:
 * 1. Rust WASM バイナリの動的ロード
 * 2. Runtime Compiler (AOT) での実行
 * 3. WASI stdout/stderr キャプチャ
 * 4. 環境変数渡し
 * 5. パフォーマンス測定（ロード時間 + コンパイル時間 + 実行時間）
 */
public class ChicorySpikeTest {

    private static final String WASM_RESOURCE = "/hello_wasm.wasm";

    public static void main(String[] args) throws Exception {
        System.out.println("=== RustComputers Chicory Spike Test ===\n");

        // --- Phase 1: WASM バイナリ解析 ---
        System.out.println("[Phase 1] WASM バイナリの解析");
        long t0 = System.nanoTime();

        var wasmStream = ChicorySpikeTest.class.getResourceAsStream(WASM_RESOURCE);
        if (wasmStream == null) {
            System.err.println("ERROR: WASM resource not found: " + WASM_RESOURCE);
            System.err.println("Run: cargo build --target wasm32-wasip1 --release in spike/rust-wasm-test/");
            System.exit(1);
        }
        WasmModule module = Parser.parse(wasmStream);
        long parseTime = System.nanoTime() - t0;
        System.out.printf("  Parse time: %.2f ms%n%n", parseTime / 1_000_000.0);

        // --- Phase 2: Interpreter モードで実行 ---
        System.out.println("[Phase 2] Interpreter モードで実行");
        runWithMode(module, "Interpreter", false);

        // --- Phase 3: Runtime Compiler モードで実行 ---
        System.out.println("[Phase 3] Runtime Compiler (AOT) モードで実行");
        runWithMode(module, "RuntimeCompiler", true);

        // --- Phase 4: 環境変数テスト ---
        System.out.println("[Phase 4] 環境変数テスト (Runtime Compiler)");
        runWithEnvVar(module);

        System.out.println("\n=== スパイクテスト完了 ===");
    }

    /**
     * 指定モードで WASM を実行し、stdout/stderr をキャプチャ + パフォーマンス測定
     */
    private static void runWithMode(WasmModule module, String modeName, boolean useCompiler) {
        var capturedStdout = new ByteArrayOutputStream();
        var capturedStderr = new ByteArrayOutputStream();

        var wasiOpts = WasiOptions.builder()
                .withStdout(capturedStdout)
                .withStderr(capturedStderr)
                .withArguments(List.of("hello_wasm"))
                .build();

        long compileStart = System.nanoTime();

        try (var wasi = WasiPreview1.builder().withOptions(wasiOpts).build()) {
            var builder = Instance.builder(module)
                    .withImportValues(
                            ImportValues.builder()
                                    .addFunction(wasi.toHostFunctions())
                                    .build());

            if (useCompiler) {
                builder.withMachineFactory(MachineFactoryCompiler::compile);
            }

            long buildStart = System.nanoTime();
            long compileTime = buildStart - compileStart;

            // build() で _start が自動実行される
            builder.build();

            long execTime = System.nanoTime() - buildStart;

            System.out.printf("  [%s] Compile/Setup time: %.2f ms%n", modeName, compileTime / 1_000_000.0);
            System.out.printf("  [%s] Execution time: %.2f ms%n", modeName, execTime / 1_000_000.0);
            System.out.printf("  [%s] Total time: %.2f ms%n", modeName, (compileTime + execTime) / 1_000_000.0);

        } catch (WasiExitException e) {
            // exit(0) でも WasiExitException がスローされる（デフォルト動作）
            long totalTime = System.nanoTime() - compileStart;
            System.out.printf("  [%s] Total time (with exit): %.2f ms (exit code: %d)%n",
                    modeName, totalTime / 1_000_000.0, e.exitCode());
        } catch (Exception e) {
            System.err.printf("  [%s] ERROR: %s%n", modeName, e.getMessage());
            e.printStackTrace();
            return;
        }

        // キャプチャ結果表示
        String stdout = capturedStdout.toString();
        String stderr = capturedStderr.toString();

        System.out.printf("  [%s] --- captured stdout ---%n", modeName);
        for (String line : stdout.split("\n")) {
            System.out.printf("  [%s]   > %s%n", modeName, line);
        }

        if (!stderr.isEmpty()) {
            System.out.printf("  [%s] --- captured stderr ---%n", modeName);
            for (String line : stderr.split("\n")) {
                System.out.printf("  [%s]   ! %s%n", modeName, line);
            }
        }

        System.out.println();
    }

    /**
     * 環境変数付きで WASM を実行（RC_TEST=hello を渡す）
     */
    private static void runWithEnvVar(WasmModule module) {
        var capturedStdout = new ByteArrayOutputStream();
        var capturedStderr = new ByteArrayOutputStream();

        var wasiOpts = WasiOptions.builder()
                .withStdout(capturedStdout)
                .withStderr(capturedStderr)
                .withArguments(List.of("hello_wasm"))
                .withEnvironment("RC_TEST", "hello_from_java")
                .build();

        try (var wasi = WasiPreview1.builder().withOptions(wasiOpts).build()) {
            Instance.builder(module)
                    .withMachineFactory(MachineFactoryCompiler::compile)
                    .withImportValues(
                            ImportValues.builder()
                                    .addFunction(wasi.toHostFunctions())
                                    .build())
                    .build();
        } catch (WasiExitException e) {
            // expected
        } catch (Exception e) {
            System.err.printf("  [EnvVar] ERROR: %s%n", e.getMessage());
            e.printStackTrace();
            return;
        }

        String stdout = capturedStdout.toString();
        System.out.println("  [EnvVar] --- captured stdout ---");
        for (String line : stdout.split("\n")) {
            System.out.printf("  [EnvVar]   > %s%n", line);
        }

        // 環境変数の値が出力に含まれるか確認
        if (stdout.contains("hello_from_java")) {
            System.out.println("  [EnvVar] ✅ 環境変数 RC_TEST の受け渡し成功");
        } else {
            System.out.println("  [EnvVar] ❌ 環境変数 RC_TEST が出力に含まれない");
        }
    }
}
