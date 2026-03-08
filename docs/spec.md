# RustComputers 設計仕様書

> 最終更新: 2026-03-08  
> バージョン: 0.2.0

## 概要

**RustComputers** は、Minecraft 1.20.1 Forge (47.4.10) 向けの Mod です。  
CC:Tweaked (ComputerCraft) の Java ペリフェラルシステムと互換性を保ちつつ、Lua の代わりに **Rust (WASM)** でコンピューターのプログラムを記述できるようにすることを目的とします。

---

## 1. 設計コンセプト

### 1.1 1tick 遅れ原則

すべての情報取得 API は **1 Game Tick (GT) 遅れ**で結果が返ります。  
これにより Rust 実行環境を完全に Java/Minecraft から隔離し、時間軸のねじれを排除します。  
Lua実装で即時反映が保障されているものは_imm()がついた関数を別途用意し、これが即時実行となります。

```
GT:N   [Rust]  → 情報取得リクエスト発行 & ワールド干渉指示発行
GT:N   [Java]  → ワールド干渉指示を実行し、結果を保存
GT:N+1 [Java]  → 前 tick のリクエストに対する情報を収集し Rust に渡す
GT:N+1 [Rust]  → 情報を受け取り、次の処理を進める
```

### 1.2 Book-Read パターン (v0.2.0)

各コンピューターは独立した **WASM インスタンス** を持ち、1 tick に 1 回 poll されます。  
ユーザーは `async fn main()` を単一エントリーポイントとして記述します。

v0.2.0 で導入された **book-read パターン**により、1 ループ = 1 tick で動作します。

```rust
#[entry]
async fn main() {
    let mut radar = find_imm::<Radar>().unwrap();
    let mut motor = find_imm::<ElectricMotor>().unwrap();

    // 初回 book（結果はまだない）
    radar.book_next_scan_for_entities(100.0);
    wait_for_next_tick().await;

    loop {
        // 前 tick の結果を読み取り
        let entities = radar.read_last_scan_for_entities().unwrap_or_default();

        // 計算
        let control = calculate_control(&entities);

        // 次 tick のリクエストを予約
        motor.book_next_set_speed(control);
        radar.book_next_scan_for_entities(100.0);

        // tick 境界（全 book が一括 FFI 発行される）
        wait_for_next_tick().await;
    }
}
```

#### Book-Read フロー

```
GT N   [Rust]  → book_next_*() で予約 → wait_for_next_tick().await で FFI 一括発行
GT N+1 [Java]  → リクエスト実行
GT N+1 [Rust]  → read_last_*() で結果取得 → 新たに book_next_*() → wait_for_next_tick().await
```

- `book_next_*(&mut self, ...)`: リクエストを Rust 側のグローバルバッファに予約（FFI 呼び出しなし）
- `wait_for_next_tick().await`: 全予約を FFI 経由で一括発行し、次 tick まで yield
- `read_last_*(&self) -> Result<T, PeripheralError>`: 前 tick の結果を読み取り
- 同じメソッドへの複数回 book は**上書き**される（最後の book のみ有効）

#### イベント待機メソッド

`modem.receive_wait_raw()` のようなイベント駆動メソッドは引き続き **async** で提供されます。  
内部的に book → wait → read をループし、イベント発生まで自動的にブロックします。

```rust
let msg = modem.receive_wait_raw().await?; // イベント発生まで待機
```

`parallel!` マクロはイベント待機の並列実行に引き続き使用可能です。

#### 即時メソッド (_imm)

`_imm` サフィックスのメソッドは同 tick 内で即座に実行されます（book-read 不要）。

```rust
let kind = radar.get_type_imm()?; // 即座に返る
```

#### 結果バッファの配置

> **注意**: 結果バッファは現在 **Rust (WASM) 側**で確保しています。  
> パフォーマンス上の理由で将来 Java 側に移す可能性があります。

---

## 2. アーキテクチャ

### 2.1 1 tick 内のフェーズ

```
┌─────────────────────────────────────────────────────────┐
│  ServerTickEvent                                        │
│                                                         │
│  Phase 1: 前 tick リクエスト結果の収集 [Java]            │
│    └ 情報取得リクエストを処理し、結果を HashMap に格納   │
│                                                         │
│  Phase 2: Rust (WASM) の poll [Java → WASM]             │
│    └ 各コンピューターインスタンスを並列 poll             │
│    └ WASM が新たな情報リクエスト / 干渉指示を積む        │
│                                                         │
│  Phase 3: ワールド干渉指示の実行 [Java]                  │
│    └ 干渉系 API を実行し、結果を保存 (次 tick に渡す)    │
└─────────────────────────────────────────────────────────┘
```

### 2.2 返り値の扱い (v0.2.0 book-read)

| API 種別 | パターン | 結果取得タイミング |
|---|---|---|
| 情報取得系 (read) | `book_next_*` → `read_last_*` | 次 tick |
| ワールド干渉系 (write) | `book_next_*` → `read_last_*` | 次 tick |
| イベント待機 | `async` (await) | イベント発生時 |
| 即時取得 (_imm) | 同期呼び出し | 同 tick |

`PeripheralError::NotRequested` は `book_next_*` 未呼び出しで `read_last_*` を呼んだ場合に返される。

---

## 3. WASM ランタイム

### 3.1 選定: Chicory Runtime (純 Java)

| 項目 | 仕様 |
|---|---|
| **ランタイム** | **Chicory Runtime 1.7.2** (Dylibso / Apache-2.0) |
| **実行方式** | Interpreter モード（0.1.x）|
| **統合方式** | 純 Java （JNI 不要、GraalVM 不要） |
| **採用理由** | 純 Java で Forge 依存なし、JarInJar 可能、Minecraft Tick Loop と相性 |

**Runtime Compiler（WASM → JVM Bytecode JIT）について:**
- Chicory は `compiler` モジュールで JIT コンパイル機能を提供する
- Forge の `ModuleClassLoader` と `WasmClassLoader` の親ローダー不適合により現時点では競合発生
- 0.2.0 課題: カスタム ClassCollector による回避調査中
- 詳細: [docs/chicory-investigation.md](./chicory-investigation.md)

### 3.2 インスタンス分離

- **コンピューター 1 台** = **WASM インスタンス 1 つ** (メモリ完全分離)
- 複数コンピューターは スレッドプール で並列 poll
- 1 コンピューター内は シングルスレッド

### 3.3 ランタイム選定の詳細

詳細比較は [docs/wasm-runtime-comparison.md](./wasm-runtime-comparison.md) を参照。

Chicory 調査プロセス: [docs/chicory-investigation.md](./chicory-investigation.md)

**対象候補:**
- Chicory Runtime (選定: ✅ 純 Java・Forge 依存なし・JarInJar 対応)
- Wasmtime (旧候補: ❌ JNI 必須 → Forge から除外)
- Wasmer (検討: ⚠️ 公式Java JNI あるが保守停滞)
- GraalVM (検討: ⚠️ Java統合は簡単だが複雑性・GraalVM 依存)

---

## 4. WASM ホスト関数 (Java ↔ Rust ブリッジ)

### 4.1 概要

Rust コンピューター（WASM）と Java（Minecraft）の相互呼び出しを実現する機構。

**原則**:
- **情報取得 (Read)**: 1 Tick 遅延 (GT:N リクエスト → GT:N+1 結果)
- **ワールド干渉 (Write)**: 同様に 1 Tick 遅延
- **すべて Future ベース**: `parallel!` マクロで複数操作の Future を生成し `.await` で一括待機
- **タイムアウト&Fuel上限**: 悪意あるプログラム対策

### 4.2 ホスト関数の種類

すべて `"env"` モジュールに属する。引数・戻り値型は WASM i32/i64。

#### ログ出力

```
host_log(ptr: i32, len: i32)
  → void
  UTF-8 文字列をゲーム内 GUI に出力する。
```

#### 標準入力

```
host_stdin_read_line(result_ptr: i32, result_buf_size: i32) → i64
  → request_id (>0): Enter が押されるまで Pending
  結果ポーリングは host_poll_result を使用する。
```

#### 情報取得系（1tick 遅延）

```
host_request_info(
    periph_id:       u32,  // Direction (0=Down … 5=East)
    method_id:       u32,  // CRC32(method_name) — java.util.zip.CRC32 と同一
    args_ptr:        i32,  // MessagePack 引数バッファのアドレス
    args_len:        i32,  // 引数バッファのバイト長
    result_ptr:      i32,  // 結果を書き込むバッファのアドレス
    result_buf_size: i32   // 結果バッファのバイト長
) → i64 : request_id (>0) | error_code (<0)

host_poll_result(request_id: i64, written_bytes_ptr: i32) → i32
  → 0=pending, 1=ready, <0=error
  ready 時、result_ptr に written_bytes バイトが書き込まれている。
```

#### ワールド干渉系（1tick 遅延）

```
host_do_action(
    periph_id, method_id, args_ptr, args_len,
    result_ptr, result_buf_size
) → i64 : request_id (>0) | error_code (<0)
  シグネチャは host_request_info と同一。
  結果ポーリングは共通の host_poll_result を使用する。
```

#### 即時情報取得（同 tick・`@LuaFunction(immediate=true)` 専用）

```
host_request_info_imm(
    periph_id, method_id, args_ptr, args_len,
    result_ptr, result_buf_size
) → i32 : written_bytes (>=0) | error_code (<0)
  1tick 遅れ原則の意図的な例外。`_imm` サフィックスで明示する。
  `@LuaFunction(immediate=true)` として実装されたメソッド専用。
```

#### Mod 確認・コンピューター情報

```
host_is_mod_available(mod_id: u16) → i32
  → 1=available, 0=not available
  mod_id = CRC32(mod_name) as u16

host_get_computer_id() → i32
  → このコンピューターの整数 ID
```

#### ペリフェラル検索（即時）

```
host_find_peripherals_by_type_imm(
    name_ptr:        i32,  // type_name UTF-8 文字列アドレス
    name_len:        i32,  // type_name バイト長
    result_ptr:      i32,  // 結果バッファアドレス
    result_buf_size: i32   // 結果バッファサイズ
) → i32 : written_bytes (>=0) | error_code (<0)
  指定型名のペリフェラルを即時検索する（有線モデム経由を含む）。
  結果は MessagePack array[uint32] (periph_id のリスト) として書き込まれる。
  periph_id: 0-5 = 直接隣接 (Direction), 6+ = 有線モデム経由。
```

### 4.3 Shared Buffer

**固定サイズ: 64 KB** (案①採用)

- 引数領域 (16 KB): Rust が Java にリクエストを送る際のパラメータ
- 結果領域 1, 2 (各 8 KB): ダブルバッファで Java が結果を返す
- メタデータ領域 (8 KB): Request state table, timestamp 等
- 予約 (22 KB): 今後の拡張用

将来的に案②（動的割り当て）へ移行可能。その場合は allocate/deallocate を WASM モジュール側でエクスポート。

### 4.4 詳細設計

**[docs/w2-java-wasm-bridge.md](./w2-java-wasm-bridge.md) を参照**

内容:
- Request ID 管理戦略（シンプルインクリメント）
- エラーコード体系（-9 種）
- Thread safety（Synchronized state machine）
- Timeout（100 ticks）& Fuel limit（10M/tick）
- Feature-based mod detection
- 実装イメージ（Java + Rust）

---

## 5. `rust_computers_api` クレート

### 5.1 構造

```
rust-computers-api/
├── src/
│   ├── lib.rs          # pub use 再エクスポート, wait_for_next_tick(), entry!/parallel! マクロ
│   ├── ffi.rs          # extern "C" ホスト関数 FFI 宣言 (env モジュール)
│   ├── peripheral.rs   # PeriphAddr, Peripheral trait, book_request/book_action/read_result,
│   │                   # request_info/do_action/request_info_imm, find_imm/wrap_imm/wrap
│   ├── book_store.rs   # book-read パターンのグローバル状態管理 (pending/in_flight/results)
│   ├── future.rs       # WaitForNextTickFuture, RequestFuture (非同期 Poll 実装)
│   ├── error.rs        # BridgeError (9 コード), PeripheralError (Bridge/NotFound/DecodeFailed/Unexpected/NotRequested)
│   ├── executor.rs     # WASM async executor
│   ├── io.rs           # read_line (stdin)
│   └── msgpack.rs      # MessagePack シリアライズ・デシリアライズヘルパー
├── peripherals/
│   └── monitor.toml    # TOML マニフェスト → build.rs が Rust コードを自動生成
├── build.rs            # TOML → Rust コードジェネレーター
└── Cargo.toml          # no_std + alloc, dlmalloc, wasm32-unknown-unknown ターゲット
```

#### 自動生成: TOML マニフェスト

`peripherals/*.toml` にメソッド定義を記述すると、`build.rs` が `<name>_gen.rs` を生成します。

```toml
# peripherals/monitor.toml
[[method]]
lua    = "getType"
ret    = "str"
imm    = true   # true → async + sync _imm バリアントを両方生成

[[method]]
lua    = "write"
action = true   # true → do_action (ワールド干渉系)
args   = [{ name = "text", ty = "str" }]
```

TOML フィールド:
- `lua`: Lua 側メソッド名 (必須)
- `action`: `true` = `do_action`（ワールド干渉系）、デフォルト `false` = `request_info`（情報取得系）
- `ret`: 戻り値型 `"i32"` / `"i64"` / `"bool"` / `"str"` / `"f64"` / `"(i32, i32)"` / `"bytes"` / 省略で `()`
  - `"bytes"`: MessagePack エンコード済み複雑型（v0.1.24+）→ `msgpack::Value` として自動デコード
  - `"i64"`: 64ビット整数型（v0.1.25+）
- `imm`: `true` = 同期 `_imm` バリアントも生成（`@LuaFunction(immediate=true)` 専用）
- `args`: 引数リスト `{ name, ty }` の配列

### 5.2 ペリフェラルのアドレッシング

```rust
use rust_computers_api::peripheral::Direction;
use rust_computers_api::monitor::Monitor;

// Direction enum で接続方向を指定
let mon = Monitor::new(Direction::South);
mon.write("hello").await?;

// 即時メソッド（@LuaFunction(immediate=true)）
let kind = mon.get_type_imm()?;   // async 不要、同 tick 内で返る
```

| `Direction` | `periph_id` |
|-------------|-------------|
| `Down`  | 0 |
| `Up`    | 1 |
| `North` | 2 |
| `South` | 3 |
| `West`  | 4 |
| `East`  | 5 |

### 5.3 `msgpack::Value` 型（v0.1.24+、v0.1.25 拡張）

`ret = "bytes"` と指定されたメソッドは、Java側で複雑な構造（List/Map）を MessagePack にエンコードして返します。  
Rust側は自動的にこれを `msgpack::Value` enum にデコードします。

```rust
pub enum Value {
    Nil,
    Bool(bool),
    Integer(i64),           // fixint, uint8/16/32, int8/16/32, int64
    Float(f64),             // float32, float64
    String(String),         // fixstr, str8, str16
    Binary(Vec<u8>),        // bin8, bin16
    Array(Vec<Value>),      // fixarray, array16
    Map(BTreeMap<String, Value>), // fixmap, map16 (キー常に String)
}
```

#### 型安全なアクセサメソッド（v0.1.25+）

`Value` には、各型を安全に抽出するコンビニエンスメソッドが用意されています：

| メソッド | 戻り値 | 説明 |
|---|---|---|
| `as_i64()` | `Option<i64>` | 整数値を取得 |
| `as_i32()` | `Option<i32>` | 整数値を i32 で取得 |
| `as_f64()` | `Option<f64>` | 浮動小数点を取得（Integer の場合もキャスト） |
| `as_bool()` | `Option<bool>` | ブール値を取得 |
| `as_str()` | `Option<&str>` | 文字列参照を取得 |
| `into_string()` | `Option<String>` | 所有権付き文字列を取得 |
| `as_array()` | `Option<&Vec<Value>>` | 配列参照を取得 |
| `into_array()` | `Option<Vec<Value>>` | 所有権付き配列を取得 |
| `as_map()` | `Option<&BTreeMap<...>>` | マップ参照を取得 |
| `into_map()` | `Option<BTreeMap<...>>` | 所有権付きマップを取得 |
| `as_bytes()` | `Option<&[u8]>` | バイナリ参照を取得 |
| `is_nil()` | `bool` | nil かどうか |
| `get(key)` | `Option<&Value>` | Map からキーで取得 |
| `at(index)` | `Option<&Value>` | Array からインデックスで取得 |
| `len()` | `usize` | Array/Map の要素数 |
| `is_empty()` | `bool` | コレクションが空か |

`From<T>` 実装: `i32`, `i64`, `f64`, `bool`, `&str`, `String` から `Value` への変換が可能です。  
`Display` 実装: `Value` は `{}` でフォーマット可能です。  
`PartialEq` 実装: `Value` 同士の比較が可能です。

**使用例**:  
```rust
use rust_computers_api::msgpack as m;

let result: m::Value = radar.scan(100.0).await?;

// 型安全なアクセサでデータを抽出
if let Some(items) = result.as_array() {
    for item in items {
        // Map から直接キーで取得
        if let Some(true) = item.get("is_entity").and_then(|v| v.as_bool()) {
            let name = item.get("name").and_then(|v| v.as_str()).unwrap_or("unknown");
            let x = item.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0);
            rc::println!("Entity: {} at x={}", name, x);
        }
    }
}
```

### 5.4 feature フラグ（将来）

現在は Monitor のみ。将来的に以下を追加予定:

```toml
[features]
default           = []
computer_craft    = []   # Monitor / DiskDrive 等
some_peripherals  = []   # Radar 等
cc_vs             = []   # Ship 等
control_craft     = []   # Jet 等
```

---

## 6. ペリフェラルアドレッシング（Direction 列挙型）

### 6.1 設計

`PeripheralHandle<T>` による文字列アドレッシング（旧設計）を廃止し、
コンピューターの**隣接 6 方向**を `Direction` 列挙型で直接指定する。

```rust
#[repr(u32)]
pub enum Direction {
    Down  = 0,
    Up    = 1,
    North = 2,
    South = 3,
    West  = 4,
    East  = 5,
}
```

`periph_id` フィールド（`host_request_info` 等の第 1 引数）は `Direction as u32` の値をそのまま使用する。

### 6.2 接続状態の確認

現在、ペリフェラルが接続されているかどうかは `host_is_mod_available` または
`host_request_info` の返り値エラーコード（`-3: PeripheralNotFound`）で確認する。

```rust
// 接続確認パターン
match peripheral::request_info(Direction::South, "getType", &[]).await {
    Ok(data) => { /* 接続中 */ }
    Err(BridgeError::PeripheralNotFound) => { /* 未接続 */ }
    Err(e) => { /* その他エラー */ }
}
```

### 6.3 BridgeError コード体系

| コード | 定数名 | 説明 |
|-------|--------|------|
| -1 | `InvalidRequest` | リクエスト不正 |
| -2 | `BufferTooSmall` | 結果バッファが不足 |
| -3 | `PeripheralNotFound` | ペリフェラル未接続 |
| -4 | `MethodNotFound` | メソッド不明 |
| -5 | `ExecutionError` | メソッド実行エラー |
| -6 | `Timeout` | 100 tick タイムアウト |
| -7 | `Serialization` | シリアライズ失敗 |
| -8 | `RequestIdOverflow` | ID 上限超過 |
| -9 | `Unknown` | その他 |

---

## 7. CC 互換ペリフェラルシステムとの関係

| 要素 | 扱い |
|---|---|
| CCの Lua VM | **使用しない** (完全遮断) |
| CCの Java ペリフェラルシステム (`IPeripheral`, `@LuaFunction` 等) | **そのまま流用** |
| CCのモデム / ペリフェラルブロック等の Java 実装 | **一部流用** |
| Lua が絡む部分で重要なもの | 必要に応じて Rust で再実装 (FFI 経由) |

他 Mod の CC 対応は主に以下のパターンです（参照実装より）：

| パターン | 例 Mod | 概要 |
|---|---|---|
| `IPeripheral` + Forge Capability | Control-Craft (`JetPeripheral`) | BlockEntity に Capability として紐付け |
| `GenericPeripheral` | Some-Peripherals (`RadarPeripheral`) | `registerGenericSource()` で登録 |
| `ILuaAPI` | CC-VS (`ShipAPI`) | コンピューターに API を注入 |

RustComputers では、これら全パターンの `@LuaFunction` メソッドを **自動生成ツールで Rust 側バインディングに変換** することを目標とします。

---

## 8. ユーザープログラムライフサイクル

1. ユーザーが `rust_computers` クレートを依存に追加し、`async fn main()` を実装
2. Rust → WASM にコンパイル（ユーザー側ビルド）
3. WASM バイナリをゲーム内 UI (CC に近い形) でサーバーにアップロード
4. サーバーがサイズ制限内か確認し、コンピューターに紐付け
5. コンピューター起動で WASM インスタンス生成 → tick ループ開始

---

## 9. 未決定事項（決定済み部分を更新）

| # | 項目 | 状態 | 詳細 |
|---|---|---|---|
| W-1 | WASM ランタイムの選定 | **✅ 決定済み** | Chicory Runtime 1.7.2 (Pure Java) |
| W-2 | Java ↔ WASM ブリッジ | **✅ 決定済み** | 詳細は [docs/w2-java-wasm-bridge.md](./w2-java-wasm-bridge.md) |
| W-3 | `@LuaFunction` → Rust 自動生成ツール | **✅ 運用中（v0.1.24+）** | build.rs で TOML マニフェストから生成 |
| W-4 | MessagePack Value デシリアライズ | **✅ 実装完了（v0.1.24+）** | `ret = "bytes"` → `msgpack::Value` 自動デコード |
| W-4b | Value 型安全アクセサ | **✅ 実装完了（v0.1.25）** | `as_i64()`, `get()`, `at()` 等のコンビニエンスメソッド |
| W-5 | WASM バイナリのアップロード UI | **✅ 実装完了**| UIに直接ドロップ |
| W-6 | WASM バイナリのサイズ上限値 | **✅ 実装完了** | コンフィグから変更可能 |
| W-7 | Chicory Runtime Compiler (JIT) 統合 | **✅ 実装完了** | JITによる高速化に対応。 |

### W-1 決定内容

- **ランタイム**: Chicory Runtime 1.7.2 (Apache-2.0 / 純 Java, JarInJar)
- **実行モード**: Interpreter（0.1.x 系固定）
- **採用根拠**: JNI 不要、GraalVM 不要、Forge との相性が良い、スパイク実証済み
- **詳細調査**: [docs/chicory-investigation.md](./chicory-investigation.md)

### W-2 決定内容

- **Shared Buffer**: 固定 64 KB （案①） 実装
- **ホスト関数**: `host_request_info`, `host_do_action`, `host_request_info_imm`, `host_poll_result`, `host_is_mod_available`, `host_get_computer_id`, `host_log`, `host_stdin_read_line`
- **`_imm` API**: `@LuaFunction(immediate=true)` 向け同期 API。1tick 遅れ原則の意図的例外。`_imm` サフィックスが明示的な例外承認シグナル
- **ペリフェラルアドレッシング**: `Direction` 列挙型（Down/Up/North/South/West/East = 0-5）
- **メソッド ID**: CRC32(method_name) — `java.util.zip.CRC32` と同一アルゴリズム
- **シリアライゼーション**: MessagePack（Rust 側: 手書き + 自動デコード, Java 側: msgpack-core 0.9.8）
- **複雑型の扱い（v0.1.24+）**: `ret = "bytes"` 指定時、Java が List/Map を MessagePack エンコード → Rust が自動で `msgpack::Value` enum にデコード
- **Value 型安全アクセサ（v0.1.25）**: `as_i64()`, `as_str()`, `get()`, `at()` 等のコンビニエンスメソッド、`From<T>` / `Display` / `PartialEq` 実装
- **エラーコード**: -1 〜 -9 の 9 種（`BridgeError` 列挙型）
- **Timeout**: 100 ticks (≈5 秒)
- **Fuel Limit**: 10M / Tick（悪意あるプログラム対策）

---

## 10. 参照リポジトリ

| リポジトリ | 目的 |
|---|---|
| [CC-Tweaked](https://github.com/cc-tweaked/CC-Tweaked) | Java ペリフェラル API の調査 |
| [CC-VS](https://github.com/TechTastic/CC-VS) | `ILuaAPI` パターンの参照実装 |
| [Some-Peripherals](https://github.com/SuperSpaceEye/Some-Peripherals) | `IPeripheral` / `GenericPeripheral` パターンの参照実装 |
| [Control-Craft](https://github.com/Rew1nd-dev/Control-Craft) | `IPeripheral` + Capability パターンの参照実装 |
| [Chicory](https://github.com/dylibso/chicory) | 採用 WASM ランタイム (Pure Java, Apache-2.0) |
| [fabric-wasmcraft-mod](https://github.com/HashiCraft/fabric-wasmcraft-mod) | Minecraft WASM 統合パターン |

```
