# RustComputers 設計方針案（W-1 ～ W-5 rev.3）

> 2026-03-05 rev.3 更新。前版(rev.2)から以下を変更：  
> **W-1** WASI 廃止（no_std 採用により不要）、  
> **W-2** tick 実行モデル確定（`wasm_tick()` エクスポート + Rust mini-executor）、WASI 廃止→ホスト関数で代替、stdin を `Future<String>` に変更（Enter まで Pending）、result バッファを Rust 側で事前確保する方式に変更、ホスト関数全リスト整理、  
> **W-4** GUI レイアウト更新（ID / ステータス / ファイル名 / ログ 15 行スクロール 200 行バッファ / 入力欄）

---

## ▶ W-1: WASM ランタイム（確定: Chicory Runtime Compiler）

### 選定結果

**Chicory Runtime Compiler（MavenCentral: `com.dylibso.chicory:compiler`）を採用**

理由:
- ✅ **純 Java** → GraalVM・JNI・ネイティブバイナリ不要
- ✅ **Runtime Compilation** → メモリ上で WASM → Java Bytecode → JVM JIT
- ✅ **動的ロード対応** → Minecraft サーバーで WASM バイナリをロード・スワップ可能
- ✅ **完成度** → 2024 年に Compiler が experimental を脱ぎ安定化、テストスイート 100% パス
- ✅ **WASI 不要** → no_std 採用により `WasiPreview1` は使用しない。stdout/stdin はカスタムホスト関数で代替

### 実行モード選定根拠

| モード | 選択 | 理由 |
|---|---|---|
| Interpreter | ❌ | パフォーマンス不足（20 ticks/sec の Minecraft では遅すぎる） |
| **Runtime Compiler** | ✅ | 動的ロード + 高パフォーマンス + 外部ツール不要 |
| Build-time Compiler | ❌ | 動的ロード不可（サーバーでバイナリをスワップできない） |

### 技術的検証（完了）

| 項目 | 検証内容 | 結果 |
|---|---|---|
| **WASI stdout/stderr** | ByteArrayOutputStream でキャプチャ可能か | ✅ API あり。GUI ロード欄に直結可能 |
| **stdin サポート** | ByteArrayInputStream で入力可能か | ✅ API あり。GUI stdin 送信欄と連携可能 |
| **64 KB 超関数フォールバック** | インタプリタに自動フォールバックし続行可能か | ✅ フォールバック & 警告ログ出力。Rust 側で `codegen-units=1` + LTO で最小化。詳細は [chicory-investigation.md](chicory-investigation.md) 参照 |
| **スレッドセーフティ** | Minecraft MainThread 内での単一スレッド実行で問題ないか | ✅ Chicory インスタンスはスレッド安全。Server Thread での実行想定 |

### 依存構成

```xml
<!-- runtime core -->
<dependency>
    <groupId>com.dylibso.chicory</groupId>
    <artifactId>runtime</artifactId>
    <version>1.7.2</version>
</dependency>

<!-- JIT compilation -->
<dependency>
    <groupId>com.dylibso.chicory</groupId>
    <artifactId>compiler</artifactId>
    <version>1.7.2</version>
</dependency>

<!-- wasi は不使用（no_std 採用・カスタムホスト関数で代替） -->
```

### スパイク実装後の確認項目

- [x] Chicory Runtime Compiler で簡単な Rust WASM バイナリ実行
- [x] WASI stdout/stderr キャプチャ動作確認
- [x] 64 KB 超関数の自動フォールバック動作確認（68 KB バイナリで確認）
- [x] HostFunction (Rust→Java ポインタ渡し / 戻り値 / Java→WASM メモリ書き込み) 動作確認



## ▶ W-2: Java ↔ WASM ブリッジ（rev.3）

### 2-0. no_std 方針

**Rust プログラムは `#![no_std] + extern crate alloc` で動作させる。**

WASI を使わないため `WasiPreview1` は Java 側に不要。stdout/stdin をはじめ全ての I/O はカスタムホスト関数で実装する。
`rust-computers-api` クレートが以下をユーザーに提供する：

```rust
#![no_std]
extern crate alloc;   // Vec, String, HashMap などは alloc 経由で使える
use alloc::string::String;

// crate 提供マクロ（→ host_log に転送）
println!("hello {}", name);   // std の println! と同じ書き味
eprintln!("error: {}", msg);
```

カスタムアロケータ（`dlmalloc` または `wee_alloc`）をクレート内で設定し、ユーザーは意識しない。

---

### 2-1. tick 実行モデル（核心部）

Rust の `async/await` はコンパイル時に状態機械に変換されるため、スタック巻き戻し（Asyncify）は**不要**。

**WASM がエクスポートする関数:**

```
wasm_init()  → void   // main() を executor に spawn（最初の tick の前に1回）
wasm_tick()  → i32    // executor を一巡 poll: 1=継続, 0=main終了, -1=panic
```

**Java の ServerTickEvent での処理順:**

```
1. 前 tick に積まれた pending results（ペリフェラル応答 + stdin 入力）を
   HashMap<requestId, Result> に格納する

2. instance.export("wasm_tick").apply() を呼ぶ
   → Rust の mini-executor が全 Future を一巡 poll()
   → host_request_info 等が呼ばれた場合:
       request_id を HashMap に予約してすぐリターン（ブロックしない）
   → 全 Future が Pending になったら wasm_tick() からリターン

3. wasm_tick() の戻り値:
    1  → 次 tick も継続
    0  → main() 正常終了 → clean shutdown
   -1  → panic → クラッシュ処理（ログ保存 + GUI 表示）
```

**ユーザーが書く Rust（async/await は普通に使える）:**

```rust
async fn main() {
    let radar: Radar = peripheral::wrap("radar_0");
    loop {
        // stdin: Enter まで Pending、Enter されたら Ready
        let cmd = rc::read_line().await;

        // ペリフェラル: 次 tick まで Pending
        let blocks = radar.scan(64.0).await;

        println!("cmd={} blocks={}", cmd, blocks.len()); // → host_log
    }
}
```

---

### 2-2. ホスト関数シグネチャ（全リスト）

> `method_id`: u32（CRC32 フル幅, W-3 参照）  
> `request_id`: i64（モノトニック増加、ラップアラウンドなし）  
> result バッファは **Rust 側が事前確保**し、Java は書き込むだけ（Java が alloc しない）

```
// ── ログ出力 ──
host_log(ptr: i32, len: i32)
  println! / eprintln! → GUI ログ欄に表示

// ── stdin ──
host_stdin_read_line() → i64
  GUI 入力欄への Enter まで Pending の request_id を返す
  結果は host_poll_result で取得（UTF-8 文字列が result_ptr に書かれる）

// ── ペリフェラル操作 ──
host_request_info(
    periph_id: u32, method_id: u32,
    args_ptr: i32, args_len: i32,       // 引数（MessagePack）: Rust が alloc
    result_ptr: i32, result_buf_size: i32  // 結果バッファ: Rust が alloc
) → i64
  戻り値: request_id (>0) | error (<0)

host_do_action(
    periph_id: u32, method_id: u32,
    args_ptr: i32, args_len: i32,
    result_ptr: i32, result_buf_size: i32
) → i64
  戻り値: request_id (>0) | error (<0)

host_request_info_imm(
    periph_id: u32, method_id: u32,
    args_ptr: i32, args_len: i32,
    result_ptr: i32, result_buf_size: i32
) → i32
  戻り値: written_bytes (>=0: 即時書き込み済み) | error (<0)
  用途: @LuaFunction(immediate=true) のメソッドのみ（同 tick 内即返）

// ── ポーリング ──
host_poll_result(request_id: i64, written_bytes_ptr: i32) → i32
  written_bytes_ptr: Java が書いたバイト数を格納するアドレス（Rust が alloc）
  戻り値: 0=pending, 1=ready, <0=error

// ── メタ情報 ──
host_is_mod_available(mod_id: u16) → i32   // 1=available, 0=not available
host_get_computer_id() → i32
```

**result バッファ確保フロー（Java は書くだけ）:**

```
1. Rust: args_buf = alloc(args_len)  に引数を書き込む
   Rust: result_buf = alloc(result_buf_size)  を事前確保
   Rust: host_request_info(periph_id, method_id,
                            args_ptr, args_len,
                            result_ptr, result_buf_size) → request_id

2. Java: args_ptr から args_len バイトを読んでペリフェラルを呼ぶ
   Java: 結果を result_ptr に書き込む（result_buf_size を超えない範囲で）
   Java: HashMap<request_id, written_bytes> に格納

3. 次 tick: Rust が host_poll_result(request_id, written_bytes_ptr) を呼ぶ
   Java: HashMap に結果があれば written_bytes を written_bytes_ptr に書く → 1 を返す
   Rust: result_ptr から written_bytes 分を読む → args_buf / result_buf を free
```

---

### 2-3. 即時関数（`_imm` サフィックス）

| 種別 | API | 実装 | 用途 |
|---|---|---|----- |
| **通常関数** | `host_request_info` / `host_do_action` | Future（1tick 遅れ）| ワールド状態変化・読み取り全般 |
| **即時関数** | `host_request_info_imm` | 同 tick 即返 | 変化なし・O(1) のメタ情報のみ |

条件: Mod 開発者が `@LuaFunction(immediate = true)` を付けることで宣言。未付加は全て通常扱い。

---

### 2-4. parallel! マクロ

`parallel!` の引数は **Future そのもの**（`.await` なし）。内部で join! 相当の処理をして、まとめて 1tick 待機する。

```rust
// ✅ 正しい: Future を渡す
let (a, b, c) = parallel!(
    radar.scan(64.0),
    sensor.get_temp(),
    lamp.set_on(true),
);
```

---

### 2-5. Request ID 管理

```java
private final AtomicLong nextRequestId = new AtomicLong(1L);
public long issueRequestId() { return nextRequestId.getAndIncrement(); }
// 使用中 ID は HashMap<Long, PendingResult> で管理。Timeout 後に自動破棄。
```

`host_stdin_read_line()` の request_id も同じ HashMap で管理する。
GUI 入力欄で Enter が押された時点で該当 ID の result に文字列を格納する。

---

### 2-6. エラーコード

```java
ERR_INVALID_REQUEST_ID = -1
ERR_INVALID_PERIPHERAL = -2
ERR_METHOD_NOT_FOUND   = -3
ERR_JAVA_EXCEPTION     = -4
ERR_TIMEOUT            = -5
ERR_FUEL_EXHAUSTED     = -6
ERR_RESULT_BUF_TOO_SMALL = -7  // result_buf_size が結果サイズより小さい
ERR_MOD_NOT_AVAILABLE  = -8
ERR_RESULT_LOST        = -9
```

---

### 2-7. タイムアウト・Fuel 切れ・Panic 時の挙動

**タイムアウト発動条件（変更）**: ビジー状態（レスポンス待ち中）が **20 tick（約 1 秒）** 継続した場合。  
ビジーでない tick 中はカウントしない（sleep や通常処理は対象外）。

**異常終了 vs 正常 Unload を区別**:

```
異常終了（Panic / Fuel切れ / Timeout）:
  → インスタンス強制終了
  → "クラッシュ状態"へ移行
  → ログ保存 + GUI に表示
  → 手動での再起動が必要（自動再起動しない）

正常 Unload（チャンクのアンロード / サーバー停止）:
  → インスタンスを clean shutdown
  → 次回チャンクロード時に自動で main() からゼロ再実行
  → 永続化（再開）はしない: main() をゼロから開始
```

**ログ保存先**: `saves/<world>/rust computers/computer/<id>/log/<timestamp>.log`  
（W-4 のファイル配置と同じルートに統一）

**デバッグ配慮**:
1. **クラッシュログ** — Panic メッセージ, 最後のホスト関数呼び出し, Fuel 残量, tick 数
2. **GUI に最終状態表示** — "CRASHED: FuelExhausted at tick 1234"
3. **再起動はユーザー操作のみ** — バグプログラムによるサーバー詰まり防止
4. **stdout キャプチャ → GUI リアルタイムログ** — `print!`/`println!` の出力を GUI に流す

**Fuel 上限**: 10,000,000 命令 / tick

---

## ▶ W-3: @LuaFunction → Rust バインディング・クレート公開方針（rev.2）

### 概要

CC:Tweaked 対応 Mod の Java コードに付いた `@LuaFunction` アノテーションから、対応する Rust バインディングクレートを生成・公開する。

**方針変更（セキュリティ上の理由）**:  
自動生成コードをユーザーが直接編集できる環境に置くと、バインディングの改ざんによる悪用が可能になる。  
そのため **生成物は crates.io 公開クレートとして提供**し、プレイヤーは Cargo.toml に依存追加するだけで使用できるようにする。

```toml
# Rust プログラムの Cargo.toml
[dependencies]
rust-computers-api = "0.1"          # コアランタイム（parallel!, RequestFuture 等）
rc-bindings-cc-tweaked = "0.1"      # CC:Tweaked ペリフェラルバインディング
rc-bindings-advanced-peripherals = "0.1"   # Optional: Advanced Peripherals
```

### 3-1. クレート構成

```
rust-computers-api          ← コアランタイム（手書き、mod が配布）
  parallel!                 ← マクロ
  RequestFuture<T>          ← 非同期 poll ループ
  PeripheralHandle<T>       ← ペリフェラル参照
  BridgeError               ← エラー型
  _imm 版関数のサポート

rc-bindings-<mod-id>        ← 各 Mod 用バインディングクレート（生成 + 公開）
  src/generated.rs          ← build.rs で生成（ユーザー編集不可）
  src/lib.rs                ← 公開 API（Peripheral 型ラッパー等）
```

### 3-2. セキュリティ考慮（詳細検討）

#### 問題の本質

Rust では `unsafe extern "C"` ブロックを書けば、公式クレートを使わずともホスト関数を直接呼び出せる。  
オープンソースである以上、**クレートの配布方法でこれを防ぐことは原理的に不可能**。

```rust
// 悪意あるユーザーが公式クレートを使わず直接呼ぶことができる
unсafe extern "C" {
    fn host_do_action(periph_id: u32, method_id: u32, arg_count: u16, args_ptr: i32) -> i64;
}
// → コンパイルさえ通ればサーバーに飛ばせる
```

**したがって、セキュリティの防衛線はクレートではなく「Java 側のホスト関数実装」に置くべき。**

#### Java 側で実施するアクセス制御（確定）

```
ホスト関数が呼ばれるたびに Java 側で以下を検証:

1. peripheral_id オーナー確認
   → そのコンピューターが peripheral_id と隣接しているか
   → 他プレイヤーのペリフェラルへのアクセスを遮断

2. method_id 許可リスト確認
   → config/rustcomputers-server.toml の allowed_methods に含まれるか
   → 未登録の method_id は ERR_NOT_PERMITTED を返す

3. per-tick レート制限
   → 1tick あたりの呼び出し数上限（Fuel と別に）
   → ホスト関数スパムによる Java スレッドの圧迫を防止
```

#### WASM の構造上防げること・防げないこと

| シナリオ | 対策 | 根拠 |
|---|---|---|
| 公式クレートなしに直接 host_do_action を呼ぶ | Java 側の権限チェックで制御 | ホスト関数実装が最終ゲート |
| 隣のコンピューターのペリフェラルを操作 | peripheral_id オーナー確認 | Java 管理の peripheral→computer マップ |
| 他コンピューターの WASM メモリを読む | **原理的に不可能** | WASM 線形メモリは各インスタンスで完全分離 |
| Fuel を無効化する | **原理的に不可能** | Fuel カウントは Java/Chicory 側で管理 |
| 許可された操作を高頻度で呼んで荒らす | Fuel 制限 + per-tick レート制限 | 許可された操作でも過剰呼び出しはブロック |
| サーバーファイルシステムを直接操作 | **原理的に不可能** | WASI のファイルアクセスは事前マウント設定内のみ |

#### クレート自作を「できるが無意味」にする設計

クレートを自作して直接呼んでも、**Java 側の権限チェックを突破できない限り同じ結果**になるよう設計する。  
「公式クレートを使うことのメリット = 型安全・エラーハンドリング・生産性」にとどめ、  
「公式クレートを使わないことのリスク = Java 側が何も信頼せずチェックするため変わらない」という構造。

#### 方針確定

- **主防衛**: Java 側ホスト関数での (1) オーナー確認 (2) 許可リスト確認 (3) レート制限
- **補助**: クレートを crates.io 公開して「正規ルートを使いやすくする」
- **明示**: ドキュメントに「クレートを自作しても Java 側のチェックは迂回できない」と記載し、改ざん意欲を低下させる

### 3-3. Method ID の決定方式（変更: u32 フル幅）

```
method_id = CRC32("ClassName.methodName")   // u32 フル幅（旧: & 0xFFFF の切り捨てなし）
```

- 32bit CRC32 なので衝突確率が 16bit より大幅に低下
- ビルドツールが衝突を検出した場合はビルドエラー（`CRC32("ClassName.methodName#overload_index")` で回避）

```java
// 例
BrewingStandPeripheral.getBrewingTime  → method_id = 0xC3A1F4E2
MonitorPeripheral.write                → method_id = 0x712BCC4A
RadarPeripheral.scan                   → method_id = 0x1D7E44A0
```

### 3-4. 型マッピング（Java → WASM → Rust）

| Java 型 | WASM 型 | Rust 型 | 転送方法 |
|---|---|---|---|
| `int` / `Integer` | `i32` | `i32` | レジスタ直 |
| `long` / `Long` | `i64` | `i64` | レジスタ直 |
| `double` / `Double` | `f64` | `f64` | レジスタ直 |
| `boolean` / `Boolean` | `i32` | `bool` | 0/1 変換 |
| `String` | `i32` (ptr), `i32` (len) | `&str` / `String` | 動的ヒープ |
| `Object[]` / `IArguments` | `i32` (ptr), `i32` (len) | `&[u8]` | 動的ヒープ (MessagePack) |
| `void` | なし | `()` | — |

**複合型シリアライゼーション**: MessagePack（`rmp-serde` / `msgpack-core`）

### 3-5. 生成ツールの方式

**Gradle Annotation Processor（ビルド時） + Rust codegen（クレートリリース時）** の2段構成

この処理はサーバー動作時には行わない。Mod ごとにクレートを事前生成・公開しておく。

```
[Mod 開発者 / RustComputers メンテナ]

ステップ1: Gradle ビルド（Mod のビルド）
  @LuaFunction + @LuaFunction(immediate=true)
    → Annotation Processor
    → build/generated/rust_bindings_meta.json

ステップ2: クレート生成スクリプト実行（手動 or CI）
  rust_bindings_meta.json
    → codegen ツール (build.rs / proc-macro)
    → rc-bindings-<mod-id>/src/generated.rs

ステップ3: crates.io / GitHub に公開
  → プレイヤーは Cargo.toml に追加するだけ
```

---

## ▶ W-4: WASM バイナリのファイル配置・UI（rev.2）

### 4-1. ファイル配置（CC:Tweaked 方式に統一）

CC:Tweaked が `saves/<world>/computercraft/computer/<id>/<file>.lua` にプログラムを格納するのと同様に、RustComputers も **コンピューターごとのディレクトリ**に保存する。

```
saves/<world>/rust computers/computer/<id>/
  <program>.wasm          ← 実行バイナリ（複数可）
  log/
    <timestamp>.log       ← クラッシュ・ランログ
```

- `<id>` はコンピューターブロックに割り当てられる整数ID（CC と同一方式）
- `.wasm` ファイルは複数置けるが、実行するのは GUI で選択した1つ

### 4-2. アップロード方法

#### 方法①: サーバーファイル直置き（従来通り）
SCP / FTP / SFTP 等で `saves/<world>/rust computers/computer/<id>/` に `.wasm` を置く。

#### 方法②: GUI への Drag & Drop（新規追加）

- Minecraft のコンピューター GUI に `.wasm` ファイルをデスクトップからドラッグ&ドロップ
- CC:Tweaked の Disk Drive と同様の実装方針
- ドロップしたファイルは自動バリデーション後、`computer/<id>/` に**上書き保存**
- マルチプレイヤーでは接続先 → サーバーへ転送（クライアント → サーバーのパケット送信）

```
[単独プレイ / Singleplayer]
  ドロップ → バリデーション → saves/.../computer/<id>/<name>.wasm に保存

[マルチプレイヤー]
  ドロップ → クライアント側でサイズ/フォーマット検証
           → S2C パケット: UploadWasm { computer_id, filename, bytes }
           → サーバー側で再バリデーション + 保存
```

### 4-3. コンピューター GUI

```
┌──────────────────────────────────────────────┐
│  Computer #12                                │  ← コンピューター ID
│  Status: RUNNING   my_program.wasm           │  ← ステータス + 実行ファイル名
├──────────────────────────────────────────────┤
│  ── Log ────────────────────────────────── ↑ │
│  [11:32:01]  Program started                 │  ← 15行表示（バッファ200行）
│  [11:32:02]  Hello, world!                   │    スクロールで上下
│  [11:32:03]  Scanning at range 64...         │
│  [11:32:04]  Found 342 blocks                │
│  [11:32:05]  ...                             │
│  [11:32:06]  ...                             │
│  [11:32:07]  ...                             │
│  [11:32:08]  ...                             │
│  [11:32:09]  ...                             │
│  [11:32:10]  ...                             │
│  [11:32:11]  ...                             │
│  [11:32:12]  ...                             │
│  [11:32:13]  ...                             │
│  [11:32:14]  ...                             │
│  [11:32:15]  ...                             │ ↓
├──────────────────────────────────────────────┤
│  > [                                      ]  │  ← 入力欄（Enter で送信）
└──────────────────────────────────────────────┘
```

**レイアウト（上から順）:**
1. `Computer #<id>` — ブロックに割り当てられた ID
2. `Status: <状態>  <実行ファイル名>` — RUNNING / STOPPED / CRASHED + 選択中ファイル名
3. ログ欄 — 表示 15 行、バッファ 200 行（古い行は自動削除）。スクロール可。`println!` / `eprintln!` 出力 + クラッシュ情報を流す。タイムスタンプ付き
4. 入力欄 — Enter で送信。`host_stdin_read_line()` が返した request_id に対して Enter が押された時点で結果を確定させる。WASM が `read_line().await` していない間の入力は**破棄**（バッファリングしない）

**実行ファイル選択:**  
画面外（別ボタンまたは右クリックメニュー等）でドロップダウン選択 + Drag & Drop による追加。  
メイン画面はシンプルに保ち、ファイル管理 UI は分離する（詳細は別途）。

### 4-4. Drag & Drop 実装方針（Java）

```java
// Forge は Screen の onEvent で FileDraggingEvent を受け取れる（1.20.1 Forge 対応）
@Override
public boolean onDragDrop(List<Path> paths, double mouseX, double mouseY) {
    for (Path p : paths) {
        String name = p.getFileName().toString();
        if (!name.endsWith(".wasm")) continue;
        byte[] bytes = Files.readAllBytes(p);
        validateWasm(bytes, configMaxSize);   // サイズ・マジックナンバー確認
        if (isMultiplayer) {
            sendUploadPacket(computerId, name, bytes);
        } else {
            saveWasmLocal(computerId, name, bytes);
        }
        refreshProgramList();
        return true;
    }
    return false;
}
```

### 4-5. セキュリティ

- パストラバーサル対策: ファイル名から `/`, `..` を除去
- 拡張子: `.wasm` のみ許可
- マジックナンバー: ` asm` を先頭4バイトで確認
- マルチプレイヤー: サーバー側でも再バリデーション必須
- 将来オプション: CC の Rust Disk アイテム相当（NBT に bytes を格納）

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

## ▶ まとめ（全決定事項 rev.2）

| # | 項目 | 決定内容 |
|---|---|---|
| **W-1** | WASM ランタイム | **Chicory Runtime Compiler 1.7.2 確定**（スパイク Phase 1/2 完了） |
| **W-2** | Rust 方針 | **no_std + alloc**（WASI 廃止） |
| | tick 実行モデル | **`wasm_tick()` エクスポート + Rust mini-executor** |
| | ホスト関数 | method_id=u32, request_id=i64, `_imm` 即時関数, `host_log`, `host_stdin_read_line` |
| | メモリ管理 | **result バッファは Rust が事前確保**、Java は書き込むだけ |
| | stdin | **`Future<String>`（Enter まで Pending）** |
| | タイムアウト | **ビジー 20tick（1秒）** |
| | parallel! | 引数は Future（`.await` なし） |
| | クラッシュ時 | 強制終了+ログ保存+手動再起動 |
| | Unload 時 | **clean shutdown + 次回チャンクロードで自動再実行** |
| **W-3** | method_id | **CRC32 u32 フル幅**（16bit 切り捨てなし） |
| | クレート公開 | **crates.io 公開クレート**（自作不可・サーバー許可リスト） |
| | 型シリアライズ | MessagePack (rmp-serde / msgpack-core) |
| **W-4** | ファイル配置 | `saves/<world>/rust computers/computer/<id>/<name>.wasm` |
| | ログ配置 | `computer/<id>/log/<timestamp>.log` |
| | アップロード | サーバー直置き + **GUI Drag & Drop（自動上書き保存）** |
| | GUI | **ID / ステータス+ファイル名 / ログ 15 行（バッファ 200 行）/ 入力欄** |
| **W-5** | バイナリサイズ | デフォルト 4 MB / config.toml でオーバーライド可 |

---

## ▶ 次フェーズ（実装準備）

全 W-* の方針が揃ったため、**スパイク + 実装フェーズへ移行可能**。

### 優先順（updated）

1. **Chicory AOT スパイク** — 簡単な Rust WASM バイナリを Chicory AOT で動作確認 + stdout キャプチャ確認
2. **Gradle プロジェクト構築** — Forge 1.20.1 + Chicory 依存追加
3. **WasmRuntime.java** 骨格 — Engine, Store, Linker 初期化（Chicory API）
4. **WasmComputer.java** 骨格 — tick 呼び出し, Fuel, 異常/正常終了判別
5. **ホスト関数登録** — 5 関数（+imm）+ 動的メモリ管理
6. **rust-computers-api クレート** 骨格 + `RequestFuture` + `parallel!`
7. **Gradle Annotation Processor** — @LuaFunction + immediate → JSON metadata
8. **codegen** — JSON → rc-bindings-cc-tweaked クレート
9. **GUI** — ファイル選択 (d&d), ログ 10 行, stdin 送信欄
10. **通信パケット** — マルチプレイヤー WASM アップロード
