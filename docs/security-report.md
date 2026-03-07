# RustComputers セキュリティレポート

> 作成日: 2026-03-07  
> 対象バージョン: 0.1.24  
> 調査対象: forge/ および crates/ ディレクトリ全体

---

## 1. 概要

RustComputers は Minecraft 1.20.1 Forge 上で動作する Mod であり、  
ユーザーが記述した **Rust プログラムを WASM にコンパイルして Minecraft 内のコンピューターブロックで実行する** という機能を提供します。  
ユーザーが任意のコードを実行できる環境であるため、サーバー・他プレイヤーへの影響を最小化する設計が求められます。

本レポートでは、現在の実装に存在するセキュリティリスクと、それぞれの改善方法を整理します。

---

## 2. アーキテクチャとセキュリティ境界

```
┌─────────────────────────────────────────────────────────────────┐
│  ユーザーが記述した Rust コード                                   │
│  └─ wasm32-unknown-unknown にコンパイル → .wasm ファイル         │
│                          │                                       │
│                          ▼                                       │
│  WasmEngine (Java)                                               │
│  ├─ Chicory (pure-Java WASM インタープリタ)                      │
│  │   └─ WASM 線形メモリ (各インスタンスで完全分離)               │
│  └─ HostFunctions (Java)  ← ここが信頼境界                       │
│      ├─ host_log                                                 │
│      ├─ host_stdin_read_line                                     │
│      ├─ host_request_info / host_do_action                       │
│      ├─ host_request_info_imm                                    │
│      ├─ host_poll_result                                         │
│      ├─ host_is_mod_available                                    │
│      └─ host_get_computer_id                                     │
│                          │                                       │
│                          ▼                                       │
│  PeripheralProvider / PeripheralType                             │
│  └─ Minecraft ワールド・サーバーリソース                         │
└─────────────────────────────────────────────────────────────────┘
```

**セキュリティの主防衛線はホスト関数 (Java 実装) に置かれています。**  
WASM バイナリ自体は Chicory のサンドボックス内で動作し、WASM 線形メモリは各インスタンスで完全分離されています。

---

## 3. 現在の実装に存在するセキュリティリスク

### 3.1 ホスト関数呼び出しのレート制限が未実装 🔴 高

**リスク**: design-proposal W-3 では「1tick あたりのホスト関数呼び出し数上限」が設計として定義されていますが、現在の実装 (`HostFunctions.java`, `WasmEngine.java`) にはこの制御が存在しません。

**影響**: 悪意あるプログラムが 1 tick に大量のペリフェラルリクエストを発行することで、Java サーバースレッドに過剰な負荷をかけられます。

**該当コード**:
```java
// WasmEngine.java: issuePeripheralRequest()
// レート制限なしに何度でも呼び出せる
public long issuePeripheralRequest(PendingResult.Type type, int periphId, ...)
```

**改善案**:
```java
// WasmEngine に per-tick カウンターを追加し、上限を設ける
private int hostCallsThisTick = 0;
private static final int MAX_HOST_CALLS_PER_TICK = 64; // Config から読み込むのが望ましい

public long issuePeripheralRequest(...) {
    if (hostCallsThisTick >= MAX_HOST_CALLS_PER_TICK) {
        pr.completeWithError(ErrorCodes.ERR_RATE_LIMIT_EXCEEDED);
        return id;
    }
    hostCallsThisTick++;
    // ...
}

// tick() の先頭でカウンターをリセット
public void tick() {
    hostCallsThisTick = 0;
    // ...
}
```

---

### 3.2 引数サイズの検証なし 🔴 高

**リスク**: `HostFunctions.java` のペリフェラル系ホスト関数で、WASM から渡される `argsLen` の値を検証せずに `memory.readBytes(argsPtr, argsLen)` を呼び出しています。

**影響**: Chicory が WASM 線形メモリの境界外アクセスを防ぐため直接的なメモリ破壊は起きませんが、非常に大きな `argsLen` (例: `Integer.MAX_VALUE`) を指定することで大量のヒープ確保を引き起こし、Java プロセスの OOM (Out of Memory) につながる可能性があります。

**該当コード**:
```java
// WasmEngine.java
byte[] argsData = argsLen > 0 ? memory.readBytes(argsPtr, argsLen) : new byte[0];
// argsLen の上限チェックなし
```

**改善案**:
```java
private static final int MAX_ARGS_LEN = 65536; // 64 KB

if (argsLen < 0 || argsLen > MAX_ARGS_LEN) {
    pr.completeWithError(ErrorCodes.ERR_INVALID_ARGS);
    return id;
}
byte[] argsData = argsLen > 0 ? memory.readBytes(argsPtr, argsLen) : new byte[0];
```

---

### 3.3 ログ出力の長さ制限なし 🟡 中

**リスク**: `host_log` ホスト関数で、WASM から渡される `len` の値を検証せずにログ文字列を読み込んでいます。

**影響**: 非常に長いログメッセージを大量に送信することで、ログバッファとメモリを圧迫できます。また、ログ文字列にゲームチャット経由で表示される Minecraft フォーマットコード（§ など）が含まれる場合、視覚的な混乱が生じる可能性があります。

**該当コード**:
```java
// HostFunctions.java: hostLog()
String msg = inst.memory().readString(ptr, len);
engine.appendLog(msg);
// 長さチェックなし
```

**改善案**:
```java
private static final int MAX_LOG_MSG_LEN = 4096; // 4 KB

if (len <= 0 || len > MAX_LOG_MSG_LEN) {
    // 無効または過大なログは無視
    return null;
}
String msg = inst.memory().readString(ptr, len);
// ゲームフォーマットコードのサニタイズ
msg = msg.replace("§", "");
engine.appendLog(msg);
```

---

### 3.4 stdin 入力の長さ制限なし 🟡 中

**リスク**: `WasmEngine.submitStdinLine()` でプレイヤーからの入力行をバイト配列に変換して `PendingResult` に渡しますが、入力長の制限がありません。

**影響**: プレイヤーが非常に長い入力を送信した場合、`resultBufSize` を超えてもクライアント側でクラッシュを引き起こす可能性があります（ただし `ERR_RESULT_BUF_TOO_SMALL` で制御はされています）。

**改善案**:
```java
public void submitStdinLine(String line) {
    // 入力長を WASM 側のバッファサイズに収まる範囲に制限
    if (line.length() > 1024) {
        line = line.substring(0, 1024);
    }
    // ...
}
```

---

### 3.5 ファイル名サニタイズの不完全性 🟡 中

**リスク**: `WasmEngine.sanitizeFileName()` でパス区切り文字と `..` を除去していますが、実装が正規表現ベースであり、エッジケースが存在します。

**影響**: Null バイト (`\0`) を含むファイル名や、Windows 環境固有のデバイス名 (`CON`, `NUL`, `PRN` など) を使ったパスが未対策です。

**該当コード**:
```java
// WasmEngine.java
String sanitized = name.replaceAll("[/\\\\]", "").replace("..", "");
if (sanitized.isBlank() || !sanitized.endsWith(".wasm")) return null;
return sanitized;
```

**改善案**:
```java
public static String sanitizeFileName(String name) {
    if (name == null || name.isBlank()) return null;
    // Null バイト除去
    name = name.replace("\0", "");
    // パス区切りと ".." の除去
    String sanitized = name.replaceAll("[/\\\\]", "").replace("..", "");
    // .wasm 拡張子チェック
    if (sanitized.isBlank() || !sanitized.endsWith(".wasm")) return null;
    // 有効なファイル名文字のみを許可（英数字・アンダースコア・ハイフン・ピリオド）
    if (!sanitized.matches("[a-zA-Z0-9_\\-\\.]+\\.wasm")) return null;
    // Windows デバイス名を拒否
    String base = sanitized.replaceAll("\\.wasm$", "").toUpperCase();
    if (base.matches("CON|PRN|AUX|NUL|COM[1-9]|LPT[1-9]")) return null;
    return sanitized;
}
```

---

### 3.6 /rustcomputers log コマンドの権限制御なし 🟡 中

**リスク**: `/rustcomputers log <computerId> true` コマンドは、任意のプレイヤーが任意のコンピューター ID のログを取得できます。権限チェックが実装されていません。

**影響**: 別プレイヤーのコンピューターログを閲覧でき、プログラムの内容や動作情報が漏洩します。

**該当コード**:
```java
// RustComputersCommand.java: executeSet()
// .requires() による権限チェックなし
.then(Commands.literal("log")
    .then(Commands.argument("computerId", ...))
```

**改善案**:
1. コンピューターのオーナー情報をデータベース化し、オーナー本人または OP のみがログを閲覧可能にする。
2. または、`get-dev` と同様に `.requires(src -> src.hasPermission(2))` で OP 必須にする。

---

### 3.7 WASM ファイルの完全性検証なし 🟡 中

**リスク**: WASM ファイルはマジックナンバーチェック（`0x00 0x61 0x73 0x6D`）のみで検証されており、ファイルの完全性（チェックサム・デジタル署名）は確認されていません。

**影響**: サーバー管理者の権限を持つ攻撃者がファイルシステム上の WASM を差し替えた場合、次回ロード時に悪意あるコードが実行されます（信頼境界の前提を満たす場合）。

**改善案**:
- サーバー設定で SHA-256 チェックサムをホワイトリスト管理し、一致しない WASM のロードを拒否する（高セキュリティ環境向け）。
- 少なくとも Chicory の WASM バリデーション（モジュール構造検証）に任せる現状は最低ラインの許容範囲。

---

### 3.8 クラッシュログへの情報漏洩 🟢 低

**リスク**: `WasmEngine.saveCrashLog()` はログバッファ全体をファイルに書き込みます。ログには `println!` 経由でユーザーが出力した内容が含まれます。

**影響**: ユーザーが意図的に機密情報（パスワード・シークレット等）をプログラムに埋め込んでログ出力した場合、クラッシュログとして永続化されます。これはユーザー自身の自己責任ですが、ログファイルのアクセス権管理が重要です。

**改善案**:
- クラッシュログの保存先ディレクトリのパーミッションを適切に設定するドキュメントを追加する。
- ログの最大行数を設定（`Config.LOG_BUFFER_SIZE`）しており、一定の制御は既にある。

---

### 3.9 method_id (CRC32) の衝突リスク 🟢 低

**リスク**: ペリフェラルメソッドの識別に `CRC32(メソッド名)` の 32bit ハッシュを使用しています。CRC32 の衝突確率は SHA などより高く、理論的には衝突を利用した不正なメソッド呼び出しが可能です。

**影響**: 実際にはホスト側で既知メソッド名との照合を行っており、CRC32 衝突で未知のメソッドを呼び出すことはできません。**現時点でのリスクは低い** と評価しますが、ペリフェラルメソッド数が増えると衝突確率が高まります。

**改善案**:
- ビルド時に全メソッド名の CRC32 衝突を検出してビルドエラーとする仕組みは design-proposal に記載済み。実装を確認・徹底する。
- 長期的には方式を MurmurHash3 (32bit) や FNV-1a に変更することを検討する（衝突の性質が CRC32 より良好）。

---

## 4. WASM サンドボックスで防止できる攻撃

以下の攻撃は WASM の仕様または Chicory の実装により**原理的に防止されています**。

| 攻撃シナリオ | 防止理由 |
|---|---|
| サーバーファイルシステムへの直接アクセス | WASI は未使用。`no_std` 環境なのでファイル操作 API は存在しない |
| 他コンピューターの WASM メモリ読み取り | WASM 線形メモリは各インスタンスで完全分離 |
| Fuel カウントの改ざん | Fuel は Java/Chicory 側で管理 |
| Java ヒープ外のメモリアクセス | Chicory が WASM 線形メモリ境界を強制 |
| スレッドの生成 | WASM `wasm32-unknown-unknown` はシングルスレッド |
| ネットワーク直接アクセス | ホスト関数に存在しない |
| JNI 経由のエスケープ | Chicory は pure-Java (JNI 不要) |

---

## 5. 既存のセキュリティ対策（現在実装済み）

| 対策 | 実装箇所 | 説明 |
|---|---|---|
| WASM バイナリサイズ上限 | `Config.MAX_WASM_SIZE` | デフォルト 4MB。巨大ファイルの読み込みを拒否 |
| Fuel 上限 (per-tick) | `Config.FUEL_PER_TICK` | デフォルト 10,000,000 命令/tick |
| tick タイムアウト | `Config.TICK_TIMEOUT_MS` | デフォルト 200ms。超過でクラッシュ状態へ |
| 同時実行コンピューター数上限 | `Config.MAX_COMPUTERS` | デフォルト 16 台 |
| ファイル名サニタイズ | `WasmEngine.sanitizeFileName()` | パストラバーサル対策 (一部不完全: §3.5 参照) |
| WASM マジックナンバー検証 | `WasmEngine.isValidWasm()` | 非 WASM ファイルの実行拒否 |
| ペリフェラル隣接チェック | `PeripheralProvider.scanAdjacent()` | コンピューターに隣接するブロックのみアクセス可 |
| CRC32 メソッド逆引き | `WasmEngine.resolveMethodName()` | 未知の method_id は拒否 |
| OP 権限チェック (`get-dev`) | `RustComputersCommand.java:101` | `/rustcomputers get-dev` は OP レベル 2 以上が必要 |
| クラッシュログ保存 | `WasmEngine.saveCrashLog()` | 問題発生時のデバッグ情報を記録 |
| リクエストタイムアウト | `Config.REQUEST_TIMEOUT_TICKS` | 未完了リクエストを自動破棄 |

---

## 6. 改善優先度マトリクス

| # | リスク | 深刻度 | 実装コスト | 優先度 |
|---|---|---|---|---|
| 3.1 | per-tick ホスト関数呼び出しレート制限 | 高 | 低 | **最優先** |
| 3.2 | 引数サイズ (`argsLen`) 上限検証 | 高 | 低 | **最優先** |
| 3.3 | ログメッセージ長さ制限 | 中 | 低 | 優先 |
| 3.5 | ファイル名サニタイズ強化 | 中 | 低 | 優先 |
| 3.6 | ログ閲覧コマンドの権限制御 | 中 | 中 | 優先 |
| 3.4 | stdin 入力長制限 | 中 | 低 | 通常 |
| 3.7 | WASM ファイル完全性検証 | 中 | 高 | 将来対応 |
| 3.8 | クラッシュログのアクセス管理 | 低 | 低 | 通常 |
| 3.9 | CRC32 衝突対策のビルド検証 | 低 | 中 | 将来対応 |

---

## 7. 追加で検討すべきセキュリティ対策

### 7.1 WASM エクスポート関数の検証

現在、`wasm_init()` と `wasm_tick()` のエクスポートが存在しなければ実行時例外で失敗しますが、事前にエクスポート一覧を検証する処理が明示されていません。不要なエクスポート関数（例: `_start` など WASI 系）を検出して警告するバリデーションを追加することで、意図しない関数呼び出しを防げます。

### 7.2 ペリフェラルアクセスの監査ログ

どのコンピューターがどのペリフェラルのどのメソッドを呼んだかをサーバーログに記録することで、不正利用の事後調査が可能になります。

### 7.3 コンピューターオーナー管理

現在の実装では、誰でも任意のコンピューター ID のログをストリーミングできます（§3.6）。  
コンピューターブロックを設置したプレイヤー UUID をオーナーとして記録し、オーナーのみがそのコンピューターを操作・ログ閲覧できる仕組みを設けることを推奨します。

### 7.4 サーバー管理者向けグローバル WASM 実行無効化スイッチ

緊急時にすべての WASM 実行を即停止できる `/rustcomputers admin disable` コマンドを実装することで、問題発生時の被害拡大を防止できます。

---

## 8. まとめ

RustComputers は WASM サンドボックス + Chicory インタープリタという設計により、Java サーバープロセスへの直接攻撃は原理的に防止できています。  
一方で、**ホスト関数レイヤーのレート制限と引数バリデーション** が現時点では不足しており、DoS 系攻撃への耐性が低い状態です。

**最低限、§3.1 と §3.2 の対策を実施することを強く推奨します。**  
その後、§3.3～§3.6 を順次対応することで、マルチプレイヤー環境での安全性が大幅に向上します。

---

*本レポートは v0.1.24 時点のコード `forge/` および `crates/` を対象として作成されました。*
