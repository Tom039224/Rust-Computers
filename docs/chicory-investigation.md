# Chicory AOT 調査レポート（2026-03-05）

## 概要

Chicory は **JVM ネイティブ WebAssembly ランタイム**。  
Java + WASM を使う場合の選択肢として検討中。

- 公式: https://chicory.dev/
- リポジトリ: https://github.com/dylibso/chicory
- MavenCentral: `com.dylibso.chicory:*`
- ライセンス: Apache 2.0

---

## 実行モード比較

| モード | パフォーマンス | 動的ロード | 要件 | 出力形式 | RustComputers向け |
|---|---|---|---|---|---|
| **Interpreter** | 🐢 遅い | ✅ 可 | なし | なし（完全解釈） | ❌ パフォーマンス不足 |
| **Runtime Compiler** | 🐇 高速 | ✅ **可** | compiler + ASM + reflect | メモリ上 Java Bytecode | ✅ **推奨** |
| **Build-time Compiler** | 🐇 最速 | ❌ 不可 | Maven/Gradle plugin | Java Bytecode ファイル | ❌ 動的ロード不可 |

**RustComputers には Runtime Compiler が最適**理由：
- Minecraft サーバーで WASM バイナリを動的にロード・スワップする必要がある
- ランタイムパフォーマンスが重要（Minecraft のゲームループ内で実行）
- 外部ツール（コンパイラ）に依存したくない

---

## Runtime Compiler の実装パターン

```java
import com.dylibso.chicory.compiler.MachineFactoryCompiler;
import com.dylibso.chicory.wasm.Parser;
import com.dylibso.chicory.runtime.Instance;

var wasmModule = Parser.parse(wasmFile);
var instance = Instance.builder(wasmModule)
    .withMachineFactory(MachineFactoryCompiler::compile)
    .build();

// 関数呼び出し
var exportFunc = instance.export("my_func");
var result = exportFunc.apply(arg1, arg2);
```

### インタプリタフォールバック

関数が JVM のメソッドサイズ上限（64 KB 程度）を超えた場合、自動的にインタプリタモードにフォールバック。  
ログに警告が出る（`InterpreterFallback.WARN` デフォルト）。

```
Warning: using interpreted mode for WASM function index: 232
```

**RustComputers への影響**:  
`rust_computers` クレート側で `codegen-units = 1` + LTO を使い、大きすぎる関数を生成しないよう配慮が必要だが、フォールバックがあるので完全には防げずとも問題なし。

---

## WASI P1 サポート（stdout/stdin キャプチャ）

✅ **完全実装済み**

### 依存追加

```xml
<dependency>
    <groupId>com.dylibso.chicory</groupId>
    <artifactId>wasi</artifactId>
</dependency>
```

### stdout キャプチャの実装

```java
import com.dylibso.chicory.wasi.WasiOptions;
import com.dylibso.chicory.wasi.WasiPreview1;
import java.io.ByteArrayOutputStream;

// OutputStream を用意（stdout キャプチャ用）
var capturedStdout = new ByteArrayOutputStream();
var capturedStderr = new ByteArrayOutputStream();

// WASI オプション設定
var wasiOpts = WasiOptions.builder()
    .withStdout(capturedStdout)
    .withStderr(capturedStderr)
    .build();

// WASI インスタンス構築
var wasi = WasiPreview1.builder().withOptions(wasiOpts).build();

// ホスト関数として登録
var store = new Store().addFunction(wasi.toHostFunctions());
store.instantiate("my-mod", Parser.parse(wasmFile));

// 実行後、キャプチャ内容を取得
String output = capturedStdout.toString();
```

### RustComputers への応用

GUI のログ欄に `print!` / `println!` 出力を流すには：

```java
class RustComputerLogStream extends OutputStream {
    private List<String> logLines;
    
    @Override
    public void write(int b) {
        // GUI ログに追加（改行単位でバッファ）
    }
}

var wasiOpts = WasiOptions.builder()
    .withStdout(new RustComputerLogStream())
    .build();
```

---

## stdin サポート

✅ **実装済み**

```java
var fakeStdin = new ByteArrayInputStream("input text".getBytes());
var wasiOpts = WasiOptions.builder()
    .withStdin(fakeStdin)
    .build();
```

WASM 側で `std::io::stdin()` を使うと読み込める。  
RustComputers の GUI stdin 送信欄と組み合わせてリアルタイム通信が可能。

---

## パフォーマンス懸念 ✅ 解消

設計提案の「64 KB 超関数でインタプリタ動作」という懸念について：

1. **Chicory がフォールバック対応**  
   - 大きな関数は自動的にインタプリタにフォールバック
   - ただしログに警告が出る

2. **Rust 側での対策**  
   - `codegen-units = 1` + LTO で生成された関数を小さく保つ
   - `wasm-opt` で最適化
   - 実測テストで確認可能

3. **minecraft tick 内での実行可能性**  
   - Runtime Compiler はメモリ上で Java Bytecode に変換
   - JVM の JIT がさらに最適化
   - 20 ticks/sec の Minecraft サーバーには十分な性能

---

## 次のステップ（実装前の必須検証）

### Phase 1: スパイク実装（**優先度: 高**）

```
[ ] 簡単な Rust WASM バイナリを用意
    - プロジェクト: rustcomputers-test-wasm (新規)
    - コード: fn main() { println!("Hello from WASM!"); }
    - ビルド: cargo build --target wasm32-wasi --release
    
[ ] Chicory Runtime Compiler で実行確認
    - Java テストコード作成
    - stdout キャプチャ動作確認
    - 実行時間測定
    
[ ] WASI stdin/stdout integrated test
    - Rust: std::io::{stdin, stdout} 使用
    - Java: ByteArrayInputStream/OutputStream で入出力確認
```

### Phase 2: Java ホスト関数統合（実装本編の要）

```
[ ] Rust => Java 呼び出し
    - unsafe extern "C" でホスト関数 FFI
    - ホスト関数シグネチャの確認
    
[ ] メモリ管理
    - WASM 線形メモリへの直接アクセス
    - Chicory の Memory API 確認
```

### Phase 3: Minecraft 統合（実装本編）

```
[ ] WasmRuntime.java に Chicory 統合
[ ] WasmComputer.java に Fuel/Timeout 実装
[ ] GUI stdout/stdin パイプ
```

---

## 結論

✅ **Chicory AOT（= Runtime Compiler）は RustComputers W-1 に完全に対応する**

- 純 Java ✓ GraalVM 不要 ✓
- 動的ロード ✓
- WASI サポート（stdout/stdin） ✓
- パフォーマンス（メモリ上 JIT） ✓
- テスト可能性 ✓

**リスク**:
- 64 KB 超関数でインタプリタフォールバック（ただし自動対応 + 警告出力）
  → Rust 側で `codegen-units/LTO` で対策、実測で確認 → **許容可能**

**方針確定**: Chicory Runtime Compiler を W-1 として採用する方向で進める
