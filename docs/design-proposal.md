# RustComputers 設計方針案（W-1 ～ W-5 確定版）

> レビュー用ドラフト。修正はこちらで掛けてください。

---

## ▶ W-1（既確定）: WASM ランタイム

**Wasmtime (kawamuray/wasmtime-java, MavenCentral)**

- Gradle 依存: `io.github.kawamuray.wasmtime:wasmtime-java`
- prebuilt JNI が JAR に同梱 → 自前 JNI 実装不要
- Cranelift JIT、形式検証済み、Apache-2.0

---

## ▶ W-2（既確定 + 修正）: Java ↔ WASM ブリッジ

### 2-1. ホスト関数シグネチャ（確定）

```
host_request_info(peripheral_id: u32, method_id: u16, arg_count: u16, args_ptr: i32) → i32
  戻り値: request_id (>0) | error (<0)

host_do_action(peripheral_id: u32, method_id: u16, arg_count: u16, args_ptr: i32) → i32
  戻り値: request_id (>0) | error (<0)

host_poll_result(request_id: i32) → i32
  戻り値: result_addr (>0: ready) | 0 (pending) | error (<0)

host_is_mod_available(mod_id: u16) → u32
  戻り値: 1 (available) | 0 (not available)
```

### 2-2. parallel! マクロ使い方訂正

`parallel!` の引数は **Futureそのもの**（`.await` なし）。
内部で `join!` 相当の処理をして、まとめて1tick待機する。

```rust
// ✅ 正しい: Future を渡す
let (a, b, c) = parallel!(
    radar.scan(64.0),
    sensor.get_temp(),
    lamp.set_on(true),   // 干渉もOK
);

// ❌ 誤り: .await を付けてはいけない
let (a, b) = parallel!(
    radar.scan(64.0).await,   // ← これは sequential に1個ずつ await している
    sensor.get_temp().await,
);
```

**parallel! の実際の動作**:

```
GT:N: parallel! 内の全 Future を一度に poll
      → 全員が host_request_info / host_do_action を呼び out して OK になる
      → まとめて Pending を返す
GT:N+1: Java が全リクエストを処理
        → 全 Future が Ready になる
        → parallel! が全結果をタプルで返す
```

### 2-3. Shared Buffer（確定：案①固定 64 KB）

```
0x00000 - 0x03FFF  引数領域  (16 KB)   Rust書き → Java読み
0x04000 - 0x05FFF  結果領域A (8 KB)    Java書き → Rust読み
0x06000 - 0x07FFF  結果領域B (8 KB)    ダブルバッファ用予備
0x08000 - 0x09FFF  メタデータ領域 (8 KB) request state table
0x0A000 - 0x0FFFF  予約 (22 KB)
```

**案②（動的 allocate/deallocate）への移行条件**:
- 一回のリクエストで 16 KB を超えるデータを扱う必要が出た場合

### 2-4. Request ID（確定）

シンプルインクリメント: `1, 2, 3, ...`  
ラップアラウンド時は1に戻す。`synchronized` でスレッドセーフ。

### 2-5. エラーコード（確定）

```java
ERR_INVALID_REQUEST_ID = -1
ERR_INVALID_PERIPHERAL = -2
ERR_METHOD_NOT_FOUND   = -3
ERR_JAVA_EXCEPTION     = -4
ERR_TIMEOUT            = -5
ERR_FUEL_EXHAUSTED     = -6
ERR_BUFFER_OVERFLOW    = -7
ERR_MOD_NOT_AVAILABLE  = -8
ERR_RESULT_LOST        = -9
```

### 2-6. タイムアウト・Fuel 切れ・Panic 時の挙動（確定）

**挙動**: WASM インスタンスを強制終了し、コンピューターを「クラッシュ状態」に移行。

```
通常状態 → [timeout / fuel切れ / panic] → クラッシュ状態
```

**デバッグ配慮**:

1. **クラッシュログを保存する**
   - `world/rustcomputers/crash/<computer_id>/<timestamp>.log`
   - 内容: クラッシュ原因, 最後のホスト関数呼び出し, Fuel残量, tick数
   - ゲーム内コマンドで参照可能: `/rc crash <computer_id>`

2. **コンピューターのディスプレイ（モニター）に最終状態を表示**
   - "CRASHED: FuelExhausted at tick 1234" 等

3. **再起動はユーザー操作で**
   - コンピューターを右クリック → "再起動" ボタン
   - 自動再起動はしない（バグのあるプログラムがサーバーを詰まらせないため）

4. **Panic メッセージを保持**
   - Rust の `panic!("message")` の内容を WASM Trap から抽出して log に含める

**タイムアウト**: 100 tick（約5秒）  
**Fuel 上限**: 10,000,000 命令 / Tick

---

## ▶ W-3: @LuaFunction → Rust バインディング自動生成

### 概要

CC: Tweaked 対応 Mod の Java コードに付いた `@LuaFunction` アノテーションから、対応する Rust ホスト関数ラッパーを自動生成する。

### 3-1. Method ID の決定方式

**CRC32 ハッシュ方式**

```
method_id = CRC32("ClassName.methodName") & 0xFFFF
```

- **なぜハッシュ**: Mod 間・バージョン間で method_id が安定する
- **衝突対策**: 衝突した場合はビルド時エラー（ツールが検出）

```java
// 例
BrewingStandPeripheral.getBrewingTime  → method_id = 0xA3F1
MonitorPeripheral.write                → method_id = 0x12BC
RadarPeripheral.scan                   → method_id = 0x7D44
```

```toml
# rust_computers/Cargo.toml に定数として埋め込む
[package.metadata.methods]
"BrewingStandPeripheral.getBrewingTime" = 0xA3F1
```

### 3-2. 型マッピング（Java → WASM → Rust）

| Java 型 | WASM 型 | Rust 型 | 転送方法 |
|---|---|---|---|
| `int` / `Integer` | `i32` | `i32` | レジスタ直 |
| `long` / `Long` | `i64` | `i64` | レジスタ直 |
| `double` / `Double` | `f64` | `f64` | レジスタ直 |
| `boolean` / `Boolean` | `i32` | `bool` | 0/1 変換 |
| `String` | `i32` (ptr), `i32` (len) | `&str` / `String` | Shared Buffer |
| `Object[]` / `IArguments` | `i32` (ptr), `i32` (len) | `&[u8]` | Shared Buffer (messagepack) |
| `void` | なし | `()` | — |

**複合型（配列・テーブル）のシリアライゼーション**: MessagePack を採用

- 理由: 軽量、バイト列に変換しやすい、Rust / Java 両方にライブラリあり
- Rust 側: `rmp-serde` クレート
- Java 側: `org.msgpack:msgpack-core`

### 3-3. 生成ツールの方式

**Gradle Annotation Processor + Rust codegen スクリプト** の2段構成

```
ステップ1: Gradle ビルド時
  @LuaFunction → Annotation Processor → JSON メタファイル
  
  生成物: build/generated/rust_bindings_meta.json
  内容:
  {
    "methods": [
      { "class": "RadarPeripheral", "method": "scan",
        "method_id": 32068, "args": ["double"], "returns": "Object[]" },
      ...
    ]
  }

ステップ2: Cargo ビルド時 (build.rs)
  rust_bindings_meta.json → Rust ソース生成
  
  生成物: OUT_DIR/generated_bindings.rs
  内容: extern "C" 宣言 + 型安全ラッパー関数
```

### 3-4. 生成コードの形式（Rust 側）

```rust
// generated_bindings.rs (自動生成、編集不可)
// Source: RadarPeripheral.scan(double range) -> Object[]

mod _generated {
    use super::*;

    pub const METHOD_RADAR_SCAN: u16 = 0x7D44;
    pub const METHOD_BREWING_GET_TIME: u16 = 0xA3F1;
    // ... 全メソッド

    extern "C" {
        fn host_request_info(periph_id: u32, method_id: u16, arg_count: u16, args_ptr: i32) -> i32;
        fn host_do_action(periph_id: u32, method_id: u16, arg_count: u16, args_ptr: i32) -> i32;
        fn host_poll_result(req_id: i32) -> i32;
        fn host_is_mod_available(mod_id: u16) -> u32;
    }
}

// 生成された型安全ラッパー

#[cfg(feature = "some_peripherals")]
impl RadarPeripheral {
    pub fn scan(&self, range: f64) -> impl Future<Output = Result<Vec<ScanResult>, BridgeError>> {
        // 1. range を Shared Buffer に書き込み
        // 2. host_request_info() 呼び出し
        // 3. poll loop Future を返す
        RequestFuture::new(self.id, METHOD_RADAR_SCAN, &[range.to_le_bytes().to_vec()])
    }
}

#[cfg(feature = "computer_craft")]
impl BrewingStandPeripheral {
    pub fn get_brewing_time(&self) -> impl Future<Output = Result<i32, BridgeError>> {
        RequestFuture::new(self.id, METHOD_BREWING_GET_TIME, &[])
    }
}
```

### 3-5. 手書きフォールバック方針

自動生成対応できないケース（複雑なオーバーロード等）は、手書きの `impl` ブロックで上書き可能。  
生成コードは `_generated` モジュール内のみ、ユーザー向け API は別ファイルで管理。

---

## ▶ W-4: WASM バイナリのアップロード UI

### 概要

コンパイル済み `.wasm` ファイルをコンピューターに紐付ける方法。

### 方式決定: ファイルシステム参照方式

**サーバー側ファイル配置 + 画面から ファイル名指定（CC スタイル）**

#### 配置ディレクトリ

```
<world>/rustcomputers/programs/
  my_program.wasm
  radar_scan.wasm
  ship_controller.wasm
```

- サーバー管理者 or プレイヤーが SCP / FTP 等で `.wasm` を置く
- サーバーの設定で「どのプレイヤーが自分のプログラムを置けるか」を制御

#### コンピューター GUI

```
┌───────────────────────────────────┐
│  RustComputer                    │
├───────────────────────────────────┤
│  Program: [my_program.wasm      ] │  ← テキスト入力（オートコンプリート）
│                                   │
│  Status: RUNNING                  │
│  Fuel used last tick: 1,234,567  │
│  Uptime: 384 ticks                │
├───────────────────────────────────┤
│  [Load]  [Restart]  [Crash Log]  │
└───────────────────────────────────┘
```

#### 操作フロー

1. プレイヤーが `.wasm` ファイルをサーバーの `programs/` フォルダに配置
2. コンピューターを右クリックして GUI を開く
3. ファイル名を入力 or オートコンプリートで選択
4. `[Load]` ボタン押下 → バリデーション（サイズ・フォーマット確認）
5. バリデーション通過後、コンピューター起動

#### 将来オプション（後期フェーズ）

- CC の フロッピーディスクアイテム に相当する「Rust Disk」アイテム
  - `.wasm` バイトをエンコードしてアイテムの NBT に保持
  - コンピューターに差し込む動作でロード

### セキュリティ考慮

- ファイル名のパストラバーサル対策: `../` を含むパスを拒否
- ファイル拡張子: `.wasm` のみ許可
- バリデーション: WASM マジックナンバー `\0asm` を確認

---

## ▶ W-5: WASM バイナリのサイズ上限

### 上限値

**デフォルト: 4 MB**

根拠:
- 標準的な Rust プログラム（依存クレートなし）: 数十 KB ～ 数百 KB
- `rust_computers` クレート + 標準ライブラリ (wee_alloc): 1 ～ 2 MB 程度
- 余裕をもって 4 MB に設定

### サーバー設定でオーバーライド可能

```toml
# config/rustcomputers-server.toml

[limits]
# WASM バイナリの最大サイズ (bytes)
# デフォルト: 4194304 (4 MB)
max_wasm_size = 4194304

# 1 Tick あたりの Fuel 上限
# デフォルト: 10000000
fuel_per_tick = 10000000

# リクエストのタイムアウト (ticks)
# デフォルト: 100 (~5秒)
request_timeout_ticks = 100

# 同時実行コンピューター数の上限
# デフォルト: 16
max_computers = 16
```

### アップロード時バリデーション

```java
// ロード時にチェック
void validateWasm(Path wasmPath, long maxSize) throws ValidationException {
    long size = Files.size(wasmPath);
    if (size > maxSize) {
        throw new ValidationException(
            String.format("Binary too large: %d bytes (max %d)", size, maxSize)
        );
    }

    // WASM マジックナンバー確認
    byte[] magic = Files.readAllBytes(wasmPath, 0, 4);
    if (!Arrays.equals(magic, new byte[]{0, 'a', 's', 'm'})) {
        throw new ValidationException("Not a valid WASM binary");
    }
}
```

---

## ▶ まとめ（全決定事項）

| # | 項目 | 決定内容 |
|---|---|---|
| **W-1** | WASM ランタイム | Wasmtime via wasmtime-java (MavenCentral) / Cranelift JIT |
| **W-2** | Java ↔ WASM ブリッジ | Shared Buffer 固定 64 KB / request_id インクリメント / 4 ホスト関数 |
| | parallel! マクロ | 引数は Future（`.await` なし）/ 内部で join! 相当 |
| | クラッシュ時 | インスタンス強制終了 + ログ保存 + 手動再起動 |
| **W-3** | 型バインディング | CRC32 method_id / MessagePack 複合型 / Gradle Processor + build.rs |
| **W-4** | アップロード UI | ファイルシステム参照方式 (programs/ フォルダ) + GUI でファイル名入力 |
| **W-5** | バイナリサイズ上限 | デフォルト 4 MB / config.toml でオーバーライド可 |

---

## ▶ 次フェーズ（実装準備）

全 W-* が確定したため、**実装フェーズへ移行可能**。

推奨実装順序:

1. **Gradle プロジェクト構築** (Forge 1.20.1, wasmtime-java 依存追加)
2. **WasmRuntime.java** 骨格 (Engine, Store, Linker 初期化)
3. **WasmComputer.java** 骨格 (tick 呼び出し, Fuel refuel, crash handling)
4. **ホスト関数登録** (4 関数 + WasmFunctions.wrap)
5. **rust_computers クレート** 骨格 + `RequestFuture` 実装
6. **Gradle Annotation Processor** (@LuaFunction → JSON metadata)
7. **build.rs codegen** (JSON → .rs)
8. **GUI** (ファイル選択, クラッシュログ表示)
