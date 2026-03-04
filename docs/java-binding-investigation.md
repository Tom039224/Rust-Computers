# Wasmtime Java バインディング & Minecraft 統合調査

## 調査リポジトリ

### 1. wasmtime-java (kawamuray/wasmtime-java)

**URL**: https://github.com/kawamuray/wasmtime-java

### 重要な発見: 公式 Maven 配布あり！

```groovy
repositories {
    mavenCentral()
}

dependencies {
    implementation "io.github.kawamuray.wasmtime:wasmtime-java:$LATEST_VERSION"
}
```

- **JAR に Prebuilt JNI ライブラリが含まれている** ためお配布
- サポートプラットフォーム:
  - Linux (ELF) x86_64
  - Mac OS x86_64 / aarch64
  - Windows x86_64
  
- 自分たちで C/Rust 層の JNI を実装する必要がない！

### 2. wasmtime-java (bluejekyll/wasmtime-java)

**URL**: https://github.com/bluejekyll/wasmtime-java

- 別実装版
- Cargo.toml を含む（Rust JNI 実装が含まれているパターン）
- kawamuray 版との機能比較未調査（kawamuray 版の方が recent と思われる）

---

## 参考実装: fabric-wasmcraft-mod

**URL**: https://github.com/HashiCraft/fabric-wasmcraft-mod

**Minecraft**: 1.17.1 Fabric

### 実装特徴

#### 1. WASM ランタイム統合パターン

```java
// WasmRuntime.java
WasiCtx wasi = new WasiCtxBuilder()
    .inheritStdout()
    .inheritStderr()
    .preopenedDir(root, "/")  // ファイルシステムマップ
    .build();

Store<Void> store = Store.withoutData(wasi);
Engine engine = store.engine();
```

**ポイント**:
- WasiCtx で標準入出力・ファイルアクセスを統合
- Store/Engine は singleton で管理可能
- モジュールはキャッシング（ハッシュベース）

#### 2. Minecraft ブロック統合

```java
// WasmBlockEntity.java - ブロックのデータ持分
public ArrayList<String> modules;       // WASM モジュールパス
public ArrayList<String> names;         // モジュール名前
public String function;                  // 呼び出し関数名
public ArrayList<String> parameters;     // パラメータリスト
public String result;                    // 実行結果
public Integer redstonePower = 0;       // Redstone 出力
public boolean powered = false;          // ブロック状態
```

#### 3. Java ↔ WASM 相互呼び出しパターン

```java
// WasmFunctions.wrap() による型安全な関数ラッピング
Func helloFunc = WasmFunctions.wrap(store, () -> {
    System.err.println("Hello from WASM!");
});

// 15個まで の引数と戻り値に対応
// Function0<R0>, Function1<A0, R0>, ..., Function15<A0...A14, R0>
// Consumer0, Consumer1, ..., Consumer15
```

**メリット**:
- Lambda 式で Java 側のコールバック関数を定義
- 型チェックが compile time に行われる
- シンプルで readable な API

#### 4. メモリ操作パターン

```java
// StringFromMemory: WASM メモリ上の文字列を Java に読む
String string = getStringFromMemory(addr, module, linker);

// StringInMemory: Java 文字列を WASM メモリに書く
// allocate/deallocate 関数が WASM 側で必要
int resultAddr = setStringInMemory(string, name, linker);
```

---

## RustComputers への適用戦略

### 修正点 (自己実装JNI → 公式ライブラリ利用へ)

**Before**:
```
Wasmtime C API → 自作 JNI ラッパー → RustComputers Mod
```

**After**:
```
wasmtime-java (Maven) → RustComputers Mod
```

### 実装ロードマップ短縮版

| Phase | 内容 | 工数 |
|---|---|---|
| 1 | Gradle に `wasmtime-java` 依存を追加 | 1-2h |
| 2 | RustComputersComputerEntity + WasmRuntime 実装 | 1-2週間 |
| 3 | ペリフェラル API バインディング (`@LuaFunction`) | 2-3週間 |
| 4 | @LuaFunction 自動生成ツール | 1-2週間 |
| 5 | GUI / ユーザー UX | 1-2週間 |
| 6 | テスト・パフォーマンス最適化 | 1週間 |

---

## 次のアクション

1. ✅ wasmtime-java (kawamuray版) を Gradle 依存に追加
2. ✅ WasmRuntime 骨格実装 (fabric-wasmcraft-mod を参考)
3. ✅ ペリフェラル API bridge 実装
4. ✅ @LuaFunction 自動生成ツール設計

---

## 参考コード examples

### 基本的な WASM 実行 (fabric-wasmcraft-mod より)

```java
import io.github.kawamuray.wasmtime.*;

public class WasmCounter {
    public static void main(String[] args) {
        try (Store<Void> store = Store.withoutData();
             Engine engine = store.engine();
             Module module = Module.fromFile(engine, "./counter.wasm");
             
             // ホスト関数: inc を定義
             Func incFunc = WasmFunctions.wrap(store, () -> {
                 counter++;
             });
             
             Linker linker = new Linker(engine);
             linker.define("", "inc", Extern.fromFunc(incFunc));
             
             Instance instance = linker.instantiate(module)) {
            
            // get() →結果取得の例
            try (Func getFunc = instance.getFunc("get").get()) {
                WasmFunctions.Function0<Integer> fn = 
                    WasmFunctions.function(getFunc);
                System.out.println("Counter: " + fn.call());
            }
        }
    }
}
```

### メモリ操作 (文字列パラメータ)

```java
// WASM: pub extern fn allocate(len: usize) -> *mut u8
// WASM: pub extern fn deallocate(ptr: *mut u8, len: usize)
// WASM: pub extern fn process_string(ptr: *mut u8) -> i32

String input = "Hello, WASM!";
byte[] bytes = input.getBytes(StandardCharsets.UTF_8);

// 1. WASMメモリ確保
Func allocate = instance.getFunc("allocate").get();
WasmFunctions.Function1<Integer, Integer> allocateFn = 
    WasmFunctions.function(allocate);
int ptr = allocateFn.call(bytes.length);

// 2. UTF-8バイト列をWASMメモリに書込
Memory memory = instance.getMemory().get();
memory.write(ptr, bytes);

// 3. WASM関数呼び出し
Func processFunc = instance.getFunc("process_string").get();
WasmFunctions.Function1<Integer, Integer> processFn = 
    WasmFunctions.function(processFunc);
int result = processFn.call(ptr);

// 4. メモリ解放
Func deallocate = instance.getFunc("deallocate").get();
WasmFunctions.Consumer2<Integer, Integer> deallocateFn = 
    WasmFunctions.consumer(deallocate);
deallocateFn.accept(ptr, bytes.length);
```

---.

## 拡張参考: Rust (WASM) 側サンプル

```rust
// allocator (標準ライブラリが必要)
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// 文字列処理用の allocate/deallocate
#[no_mangle]
pub extern fn allocate(len: usize) -> *mut u8 {
    let mut buf = Vec::with_capacity(len);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);  // メモリ loss 注意！
    ptr
}

#[no_mangle]
pub extern fn deallocate(ptr: *mut u8, len: usize) {
    let _ = unsafe { Vec::from_raw_parts(ptr, 0, len) };
}

#[no_mangle]
pub extern fn process_string(ptr: *mut u8) -> i32 {
    // ptr の先頭をCStr として読む必要がある＋長さ情報が必要
    // => より高度な仲介層が必要
}

// より簡単: 固定バッファを使う
static mut SHARED_BUFFER: [u8; 4096] = [0; 4096];

#[no_mangle]
pub extern fn get_buffer_ptr() -> i32 {
    unsafe { &SHARED_BUFFER[0] as *const u8 } as i32
}

#[no_mangle]
pub extern fn process_shared() -> i32 {
    let text = unsafe {
        std::str::from_utf8(&SHARED_BUFFER).unwrap_or("")
    };
    text.len() as i32
}
```

