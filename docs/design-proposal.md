# RustComputers 設計方針案（W-1 ～ W-5 rev.2）

> 2026-03-05 更新。前版から以下を変更：  
> W-1 ランタイム再検討（Chicory AOT 方向）、W-2 メモリ動的化・タイムアウト変更・即時関数追加・Request ID u64化・Unload区別・UI stdin、W-3 method_id u32化・クレート公開方針、W-4 ファイル配置をコンピューター別ディレクトリに変更・d&d 対応

---

## ▶ W-1: WASM ランタイム（再検討中）

### 背景

当初採用候補だった **kawamuray/wasmtime-java** は MavenCentral への publish こそあるが、  
リポジトリが更新停止状態であり、最新の WASM 仕様（component model、SIMD 等）に追従していない。

### 候補比較（再）

| ランタイム | JVM統合 | AOT/JIT | メモリコピー | 難易度 | 備考 |
|---|---|---|---|---|---|
| ~~wasmtime-java (kawamuray)~~ | JNI | Cranelift JIT | あり（FFI越え） | 低 | 更新停止、最新WASM非対応 |
| GraalWasm | Native | GraalVM JIT | ほぼなし | **高** | GraalVM JVM 必須 → ユーザー障壁大 |
| **Chicory (AOT/JIT)** | **純Java** | **AOT(JITAST変換)** | **なし（同一ヒープ）** | 中 | 64KB超関数はインタプリタ動作 |
| Chicory (インタプリタ) | 純Java | なし | なし | 低 | 常時インタプリタ、遅い |

### 方針決定: **Chicory AOT を採用方向で調査を進める**

**理由**:
- 純 Java 実装 → GraalVM 不要、ユーザー環境に依存しない
- AOT モード: WASM → Java バイトコード変換 → JVM JIT にかかる
- 同一 JVM ヒープ上でメモリを直接参照 → Shared Buffer の大きなコピーが発生しない
- Maven Central にライブラリあり: `com.dylibso.chicory:runtime`, `com.dylibso.chicory:aot`

**懸念点（要検証）**:

1. **64 KB 超の関数はインタプリタ動作になる**  
   - `rust_computers` クレートのコードが大きな関数を生成しないよう `codegen-units = 1` + LTO を活用  
   - 実測で性能が確認できた場合に正式採用

2. **WASI サポートの成熟度**  
   - `chicory-wasi` モジュールが存在するが、wasmtime ほど成熟していない可能性  
   - stdin/stdout キャプチャの実装可否を検証

3. **スレッドモデル**  
   - Chicory のインスタンスが Minecraft の Server Thread と同一スレッドで動作するか確認が必要

### 次のアクション（W-1 確定前）

- [ ] Chicory AOT で簡単な Rust WASM バイナリが動くかスパイク実装
- [ ] `chicory-wasi` の stdout キャプチャ可否を確認
- [ ] 64 KB 超関数の回避可能性を `wasm-opt` + LTO で実測



## ▶ W-2: Java ↔ WASM ブリッジ（rev.2）

### 2-1. ホスト関数シグネチャ

> `method_id` を u32 に拡張（CRC32 フル幅, W-3 参照）。  
> `request_id` を i64 にしラップアラウンド問題を排除。

```
host_request_info(peripheral_id: u32, method_id: u32, arg_count: u16, args_ptr: i32) → i64
  戻り値: request_id (>0) | error (<0)

host_do_action(peripheral_id: u32, method_id: u32, arg_count: u16, args_ptr: i32) → i64
  戻り値: request_id (>0) | error (<0)

host_poll_result(request_id: i64) → i32
  戻り値: result_addr (>0: ready) | 0 (pending) | error (<0)

host_is_mod_available(mod_id: u16) → u32
  戻り値: 1 (available) | 0 (not available)

host_request_info_imm(peripheral_id: u32, method_id: u32, arg_count: u16, args_ptr: i32) → i32
  戻り値: result_addr (>0: 即時結果) | error (<0)
  用途: @LuaFunction(immediate=true) のメソッドのみ
```

### 2-2. 即時読み取り関数（`_imm` サフィックス）

1tick 遅れなしで同期的に返せる情報（ペリフェラルの不変メタ情報など）は `_imm` を付与したホスト関数で即答する。

Rust API 側では `_imm` 版は `Future` 不要のブロッキング関数として生成：

```rust
// 通常: 1tick 遅れ（await 必要）
let temp: f64 = sensor.get_temp().await?;

// 即時: 同一tick 内に返る（await 不要）
let kind: &str = peripheral.get_type_imm()?;
```

ルール:
- `_imm` にできる条件: ワールドの状態変化が不要、かつ Java 側の実行コストが O(1)
- Mod 開発者がアノテーション `@LuaFunction(immediate = true)` を付けることで宣言
- 付けていないものは全て通常（1tick 遅れ）として扱う

### 2-3. parallel! マクロ使い方

`parallel!` の引数は **Future そのもの**（`.await` なし）。  
内部で `join!` 相当の処理をして、まとめて 1tick 待機する。

```rust
// ✅ 正しい: Future を渡す
let (a, b, c) = parallel!(
    radar.scan(64.0),
    sensor.get_temp(),
    lamp.set_on(true),
);

// ❌ 誤り: .await を付けてはいけない
let (a, b) = parallel!(
    radar.scan(64.0).await,
    sensor.get_temp().await,
);
```

### 2-4. メモリ管理方式（動的割り当て 案②）

**固定 Shared Buffer ではなく、動的 alloc/free 方式を採用する。**

理由:
- 大量スキャンデータ等が 64 KB を超えるケースに最初から対応
- Chicory の場合は同一ヒープのため、動的割り当てのコストが FFI 越えコピーに比べて小さい

```
呼び出しフロー:
  1. Rust が引数バッファを alloc() して args_ptr として渡す
  2. Java が args_ptr から読み、result_ptr を alloc() して結果を書き込む
  3. Rust が poll_result() → result_ptr を得て read → free
```

### 2-5. Request ID 管理（u64 モノトニック）

```java
private final AtomicLong nextRequestId = new AtomicLong(1L);
public long issueRequestId() { return nextRequestId.getAndIncrement(); }
```

使用中 ID は HashMap で管理。Timeout 後に自動破棄。

### 2-6. エラーコード

```java
ERR_INVALID_REQUEST_ID = -1
ERR_INVALID_PERIPHERAL = -2
ERR_METHOD_NOT_FOUND   = -3
ERR_JAVA_EXCEPTION     = -4
ERR_TIMEOUT            = -5
ERR_FUEL_EXHAUSTED     = -6
ERR_ALLOC_FAILED       = -7   // 動的確保失敗（旧 BUFFER_OVERFLOW）
ERR_MOD_NOT_AVAILABLE  = -8
ERR_RESULT_LOST        = -9
```

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

### 3-2. セキュリティ考慮

**問題**: バインディングクレートを自作できると、ホスト関数を直接呼び出して不正操作が可能になる恐れがある。

**対策案（要検討）**:

| 案 | 内容 | 実現性 |
|---|---|---|
| **A: クレート配布のみ** | 公式クレートのみ使用可能・ソースは公開するが社会的なプレッシャーでの対策 | 低（open source なので自作可能） |
| **B: method_id を秘匿** | ホスト関数の ID を WASM ロード時に動的に渡し、静的に解析できないようにする | 中（難読化程度） |
| **C: capability token** | ロード時にサーバーが capability token を WASM に渡し、全呼び出しで検証 | 高（実装コスト大） |
| **D: サーバー側許可リスト** | サーバー設定で「使用可能なペリフェラル API」を明示的に列挙。それ以外は ERR_NOT_PERMITTED | 高 |

**現時点の方針**: 案 **D（サーバー側許可リスト）+ A（公式クレート配布）** の組み合わせで進める。  
完全な防止は不可能だが、明示的な許可リストにより「意図しないAPIアクセス」は阻止できる。

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
┌────────────────────────────────────────────┐
│  RustComputer #12                          │
├────────────────────────────────────────────┤
│  Program: [my_program.wasm              ▼] │  ← ドロップダウン（d&d で追加）
│                                            │
│  Status:  RUNNING                          │
│  Fuel used last tick:  1,234,567           │
│  Uptime:  384 ticks                        │
├────────────────────────────────────────────┤
│  ── Log ──────────────────────────────── ↑ │
│  [11:32:01] Program started               │  ← 最新10行表示
│  [11:32:02] Hello, world!                 │    スクロールで上下
│  [11:32:03] Scanning at range 64...       │
│  [11:32:04] Found 342 blocks              │
│  [11:32:05] ...                           │
│  [11:32:06] ...                           │
│  [11:32:07] ...                           │
│  [11:32:08] ...                           │
│  [11:32:09] ...                           │
│  [11:32:10] ...                           │ ↓ │
├────────────────────────────────────────────┤
│  stdin>  [                              ] │  ← stdin 送信欄
│                                            │
├────────────────────────────────────────────┤
│  [Load]    [Restart]    [Stop]            │
└────────────────────────────────────────────┘
```

**ログ欄**:
- 直近 10 行を常時表示（上下スクロール可）
- `println!` / `eprintln!` のキャプチャ出力 + クラッシュ情報を流す
- タイムスタンプ付き

**stdin 送信欄**:
- `IO::stdin()` の読み込み待ちのとき、この欄からテキストを送信可能
- Enter キー or 送信ボタンで WASM の stdin バッファに書き込み
- 実装上はホスト関数 `host_stdin_write(ptr: i32, len: i32)` を追加

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
| **W-1** | WASM ランタイム | **Chicory AOT 方向で検討中**（スパイク実装で検証） |
| **W-2** | ホスト関数 | method_id=u32, request_id=i64, + _imm 即時関数追加 |
| | メモリ管理 | **動的割り当て（案②）** |
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
| | GUI | ログ 10 行表示（スクロール）+ **stdin 送信欄** |
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
