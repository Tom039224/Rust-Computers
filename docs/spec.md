# RustComputers 設計仕様書

> 最終更新: 2026-03-15  
> バージョン: 0.2.28

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
  - **情報取得系** (`book_next_request_*`): 複数回呼び出すと**最後のリクエストのみ有効**（上書き）
  - **ワールド干渉系** (`book_next_action_*`): 複数回呼び出すと**全て保存**（追記）
- `wait_for_next_tick().await`: 全予約を FFI 経由で一括発行し、次 tick まで yield
- `read_last_*(&self) -> Result<T, PeripheralError>`: 前 tick の結果を読み取り
  - 情報取得系は 1 つの結果（最新）を返す
  - ワールド干渉系は複数の結果（全操作の結果）を返す

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

> **v0.2.8 変更点**: 結果バッファを **two-phase fetch** 方式に変更しました。  
> Rust はリクエスト発行時にバッファを確保しません。代わりに `host_poll_result` でサイズを取得し、  
> 動的にバッファを確保してから `host_fetch_result` でデータを受け取ります。  
> これにより固定サイズバッファ不足による `ERR_RESULT_BUF_TOO_SMALL` エラーが解消されます。

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
host_stdin_read_line() → i64
  → request_id (>0): Enter が押されるまで Pending
  結果ポーリングは host_poll_result / host_fetch_result を使用する。
```

#### 情報取得系（1tick 遅延・book-read パターン）

```
host_request_info(
    periph_id:  u32,  // Direction (0=Down … 5=East)
    method_id:  u32,  // CRC32(method_name) — java.util.zip.CRC32 と同一
    args_ptr:   i32,  // MessagePack 引数バッファのアドレス
    args_len:   i32   // 引数バッファのバイト長
) → i64 : request_id (>0) | error_code (<0)
  情報取得リクエストを発行する（book フェーズ）。
  複数回呼び出した場合は最後のみ有効（上書きされる）。
```

#### ポーリング・結果取得（2-phase fetch）

```
// Phase 1: 結果サイズ確認（offset-by-1 エンコーディング）
host_poll_result(request_id: i64) → i64
  → 0=pending
     1+(size-1)=対応する結果が ready（size バイト）
     <0=error
  待機中は 0 を返す。実装完了時は「結果サイズ」を +1 エンコーディングで返す。
  例: 0 バイト結果が ready なら 1，3 バイト結果が ready なら 4。

// Phase 2: 動的確保バッファへ結果をコピー
host_fetch_result(request_id: i64, result_ptr: i32, result_buf_size: i32) → i32
  → written_bytes (>=0) | error_code (<0)
  Rust は host_poll_result の戻り値から実サイズを計算し、
  必要なバイト数のベクタを動的に確保してからこの関数を呼び出す。
  呼び出し後、request_id に対応するエントリは削除される。
  v0.2.8 以降、固定バッファ不足による ERR_BUFFER_TOO_SMALL は発生しない
  （Rust 側で動的確保）。
```

#### ワールド干渉系（1tick 遅延・book-read パターン）

```
host_do_action(
    periph_id:  u32,  // Direction (0=Down … 5=East)
    method_id:  u32,  // CRC32(method_name)
    args_ptr:   i32,  // MessagePack 引数バッファ
    args_len:   i32   // 引数バッファバイト長
) → i64 : request_id (>0) | error_code (<0)
  ワールド干渉（書き込み）リクエストを発行する（book フェーズ）。
  複数回呼び出した場合は全て保存される（追記される）。結果は
  host_poll_result + host_fetch_result で取得。
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

### 4.4 エラーコード体系

全ホスト関数が返すエラーコードは統一体系に従う。

| コード | 定数名 | 説明 |
|-------|--------|------|
| -1 | `InvalidRequestId` | request_id が無効（期限切れ等） |
| -2 | `InvalidPeripheral` | ペリフェラル ID が範囲外 (0-5 または有線 ID 外) |
| -3 | `MethodNotFound` | 指定メソッドが存在しない |
| -4 | `PeripheralNotFound` | ペリフェラルが接続されていない |
| -5 | `ArgumentError` | 引数形式が不正 |
| -6 | `ExecutionError` | メソッド実行エラー |
| -7 | `Timeout` | 100 tick タイムアウト |
| -8 | `SerializationError` | MessagePack エンコード/デコード失敗 |
| -9 | `BufferTooSmall` | 結果バッファサイズ不足（`_imm` 専用、通常は発生しない） |
| -10 | `RequestIdOverflow` | リクエスト ID シーケンス上限超過（極めて稀） |

Rust 側では各コードを `BridgeError` enum にマップする：

```rust
pub enum BridgeError {
    InvalidRequestId,
    InvalidPeripheral,
    MethodNotFound,
    PeripheralNotFound,
    ArgumentError,
    ExecutionError,
    Timeout,
    SerializationError,
    BufferTooSmall,
    RequestIdOverflow,
}
```

#### PeripheralError との関係

`PeripheralError` は高レベルエラー型で、実装層では `BridgeError` から変換される：

```rust
pub enum PeripheralError {
    Bridge(BridgeError),  // ホスト関数エラーをそのままラップ
    NotRequested,         // read_last_*() が book_next_*() なしで呼ばれた
    DecodeFailed,         // 結果デコード失敗（コンテンツ理由）
    Unexpected,           // 予期しないエラー
}
```

### 4.5 詳細設計
### 4.5 詳細設計

**[docs/w2-java-wasm-bridge.md](./w2-java-wasm-bridge.md) を参照**

内容:
- Request ID 管理戦略（シンプルインクリメント）
- エラーコード体系と実装詳細
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
│   ├── lib.rs              # pub use 再エクスポート, entry!/parallel! マクロ, wait_for_next_tick()
│   ├── ffi.rs              # extern "C" ホスト関数 FFI 宣言 (env モジュール)
│   ├── peripheral.rs       # Peripheral trait, book_request/book_action/read_result,
│   │                       # request_info/do_action/request_info_imm, PeriphAddr 等
│   ├── book_store.rs       # book-read パターンのグローバル状態管理
│   ├── executor.rs         # WASM async executor
│   ├── msgpack.rs          # MessagePack エンコード/デコード実装
│   ├── error.rs            # BridgeError, PeripheralError 定義
│   ├── io.rs               # 標準入出力
│   ├── computer_craft/     # CC:Tweaked ペリフェラル実装
│   │   ├── mod.rs
│   │   ├── modem.rs        # Modem ペリフェラル
│   │   ├── monitor.rs      # Monitor ペリフェラル
│   │   └── ...
│   └── ...その他ペリフェラル群
└── Cargo.toml              # no_std + alloc, dlmalloc, wasm32-unknown-unknown target
```

#### ペリフェラルの実装パターン

各ペリフェラルは **Rust ファイルとして直接実装** されます。例：

**ファイル**: [src/computer_craft/modem.rs](../../src/computer_craft/modem.rs)

```rust
pub struct Modem {
    addr: PeriphAddr,
}

impl Peripheral for Modem {
    const NAME: &'static str = "modem";
    
    fn new(addr: PeriphAddr) -> Self {
        Self { addr }
    }
}

impl Modem {
    // 読み取り系：複数回 book は last one のみ有効
    pub fn book_next_is_open(&mut self, channel: u32) {
        let args = msgpack::array(&[msgpack::int(channel as i32)]);
        peripheral::book_request(self.addr, "isOpen", &args);
    }
    
    pub async fn read_last_is_open(&self) -> Result<bool, PeripheralError> {
        let data = peripheral::read_result(self.addr, "isOpen").await?;
        msgpack::decode::<bool>(&data)
    }
    
    // 書き込み系：複数回 book は全件保存
    pub fn book_next_open(&mut self, channel: u32) {
        let args = msgpack::array(&[msgpack::int(channel as i32)]);
        peripheral::book_action(self.addr, "open", &args);
    }
    
    pub async fn read_last_open(&self) -> Result<Vec<()>, PeripheralError> {
        let results = peripheral::read_action_results(self.addr, "open").await?;
        Ok(results.into_iter().map(|_| ()).collect())
    }
}
```

**ペリフェラル実装の特徴**:
- 各ペリフェラルは `Peripheral` trait を実装
- `book_next_*(&mut self, ...)`: リクエスト予約（非同期）
- `read_last_*(& self) -> Result<T, ...>`: 前 tick 結果取得（非同期）
- **読み取り系メソッド**: 複数 book で last one が有効（BTreeMap by `periph_id` + `method_id`）
- **書き込み系メソッド**: 複数 book で全件が保存（Vec で順序保持）

### 5.2 ペリフェラルの取得方法

ペリフェラルの接続状態を確認し、インスタンスを生成します：

```rust
use rust_computers_api::peripheral::{Peripheral, PeriphAddr};
use rust_computers_api::computer_craft::Modem;

// Direction enum で直接指定
let modem = Modem::new(PeriphAddr(3_u32)); // Direction::South = 3
modem.write("hello").await?;

// または接続確認後に取得（有線ネットワーク含む）
let found_modems = Modem::find_all().await?;
for modem in found_modems {
    modem.transmit(1, 0, "data").await?;
}
```

| ペリフェラル型 | NAME | ファイル |
|---|---|---|
| `Modem` | `"modem"` | `src/computer_craft/modem.rs` |
| `Monitor` | `"monitor"` | `src/computer_craft/monitor.rs` |
| `Monitor2` | `"monitor"` | (Monitor と同一) |
| ... | ... | ... |

**Direction 値の対応**:

| `Direction` | ID |
|---|---|
| `Down` | 0 |
| `Up` | 1 |
| `North` | 2 |
| `South` | 3 |
| `West` | 4 |
| `East` | 5 |

6以上は有線モデム経由での接続を表します。

### 5.3 `msgpack::Value` 型（v0.1.24+、v0.1.25 拡張）

Java 側で複雑な構造（List/Map）を返す場合、`ret = "bytes"` と指定します。  
このとき Java は結果を MessagePack にエンコードして返し、  
Rust 側は自動的にこれを `msgpack::Value` enum にデコードします。

```rust
pub enum Value {
    Nil,
    Bool(bool),
    Integer(i64),           // fixint, uint8/16/32, int8/16/32, int64
    Float(f64),             // float32, float64
    String(String),         // fixstr, str8, str16, str32
    Binary(Vec<u8>),        // bin8, bin16, bin32
    Array(Vec<Value>),      // fixarray（最大 15 要素）
    Map(BTreeMap<String, Value>), // fixmap, map16（キー常に String）
}
```

**注**: 配列は `fixarray` 形式のみに対応しています。実務上は 1～5 要素の配列がほとんどなため、問題になることはありません。

#### 型安全なアクセサメソッド（v0.1.25+）


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

### 6.2 ペリフェラルの接続確認

現在、ペリフェラルが接続されているかどうかは `host_is_mod_available` または
`host_request_info` の返り値エラーコード（`-4: PeripheralNotFound`）で確認する。

```rust
// 接続確認パターン
match peripheral::request_info(Direction::South, "getType", &[]).await {
    Ok(data) => { /* 接続中 */ }
    Err(BridgeError::PeripheralNotFound) => { /* 未接続 */ }
    Err(e) => { /* その他エラー */ }
}
```

エラーコード体系の詳細は [セクション 4.4](#44-エラーコード体系) を参照。

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
- **ホスト関数**: `host_request_info`, `host_do_action`, `host_request_info_imm`, `host_poll_result`, `host_fetch_result`, `host_is_mod_available`, `host_get_computer_id`, `host_log`, `host_stdin_read_line`
- **`_imm` API**: `@LuaFunction(immediate=true)` 向け同期 API。1tick 遅れ原則の意図的例外。`_imm` サフィックスが明示的な例外承認シグナル
- **ペリフェラルアドレッシング**: `Direction` 列挙型（Down/Up/North/South/West/East = 0-5）
- **メソッド ID**: CRC32(method_name) — `java.util.zip.CRC32` と同一アルゴリズム
- **シリアライゼーション**: MessagePack（Rust 側: 手書き + 自動デコード, Java 側: msgpack-core 0.9.8）
- **複雑型の扱い（v0.1.24+）**: `ret = "bytes"` 指定時、Java が List/Map を MessagePack エンコード → Rust が自動で `msgpack::Value` enum にデコード
- **Value 型安全アクセサ（v0.1.25）**: `as_i64()`, `as_str()`, `get()`, `at()` 等のコンビニエンスメソッド、`From<T>` / `Display` / `PartialEq` 実装
- **エラーコード**: -1 〜 -10 の 10 種（`BridgeError` 列挙型、詳細は [セクション 4.4](#44-エラーコード体系)）
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
