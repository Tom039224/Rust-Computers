# Modem

**モジュール:** CC:Tweaked  
**ペリフェラルタイプ:** `modem`  
**ソース:** `ModemPeripheral.java`

## 概要

Modemペリフェラルは、ワイヤレスおよびワイヤードネットワーク通信を提供します。チャネルを開き、メッセージを送信し、他のモデムからデータを受信できます。モデムはワイヤレスモード（範囲制限あり）またはワイヤードモード（接続されたネットワーク上で無制限）で動作できます。

## 3つの関数パターン

Modem APIは、すべてのメソッドに対して3つの関数パターンを使用します：

1. **`book_next_*`** - 次のティックのリクエストをスケジュール
2. **`read_last_*`** - 前のティックの結果を読み取り
3. **`async_*`** - 便利なメソッド（book、待機、読み取りを1つの呼び出しで実行）

### パターンの説明

```rust
// Rust example to be added
```
## モデムの種類

CC:Tweakedは3種類のモデムを提供します：

1. **ワイヤレスモデム** - 他のワイヤレスモデムと範囲内（デフォルト64ブロック、y≥96で最大384ブロック）で通信
2. **エンダーモデム** - 無制限の範囲とクロスディメンション対応のワイヤレスモデム
3. **ワイヤードモデム** - ネットワークケーブルに接続して、接続されたネットワーク上で無制限の範囲を実現

## 実装状況

### ✅ 実装済み

- book_next_open / read_last_open / async_open
- book_next_is_open / read_last_is_open / async_is_open
- book_next_close / read_last_close / async_close
- book_next_close_all / read_last_close_all / async_close_all
- book_next_transmit / read_last_transmit / async_transmit
- book_next_transmit_raw / read_last_transmit_raw / async_transmit_raw
- book_next_try_receive_raw / read_last_try_receive_raw / async_try_receive_raw
- receive_wait_raw (async)

### 🚧 未実装

- is_wireless() method (all variants)
- get_names_remote() method (all variants)
- modem_message event system


## メソッド

### `open(channel)` / `book_next_open(channel)` / `read_last_open()` / `async_open(channel)`

受信メッセージをリッスンするためのチャネルを開きます。

**Lua署名:**
```rust
// Rust example to be added
```
**Rust署名:**
```rust
pub fn book_next_open(&mut self, channel: u32)
pub fn read_last_open(&self) -> Result<(), PeripheralError>
pub async fn async_open(&self, channel: u32) -> Result<(), PeripheralError>
```

**パラメータ:**
- `channel: number` — 開くチャネル番号（0–65535）

**戻り値:** `nil`

**注記:**
- 最大128個のチャネルを同時に開くことができます
- 送信するためにチャネルを開く必要はなく、受信するためにのみ必要です

**例:**
```rust
// Rust example to be added
```
**エラーハンドリング:**
- チャネル番号が範囲外の場合、エラーをスロー
- 128個のチャネルが既に開いている場合、エラーをスロー

---

### `isOpen(channel)` / `book_next_is_open(channel)` / `read_last_is_open()` / `async_is_open(channel)`

チャネルが現在開いているかどうかを確認します。

**Lua署名:**
```rust
// Rust example to be added
```
**Rust署名:**
```rust
pub fn book_next_is_open(&mut self, channel: u32)
pub fn read_last_is_open(&self) -> Result<bool, PeripheralError>
pub async fn async_is_open(&self, channel: u32) -> Result<bool, PeripheralError>
```

**パラメータ:**
- `channel: number` — 確認するチャネル番号（0–65535）

**戻り値:** `boolean` — 開いている場合は`true`、閉じている場合は`false`

**例:**
```rust
// Rust example to be added
```
---

### `close(channel)` / `book_next_close(channel)` / `read_last_close()` / `async_close(channel)`

開いているチャネルを閉じます。

**Lua署名:**
```rust
// Rust example to be added
```
**Rust署名:**
```rust
pub fn book_next_close(&mut self, channel: u32)
pub fn read_last_close(&self) -> Result<(), PeripheralError>
pub async fn async_close(&self, channel: u32) -> Result<(), PeripheralError>
```

**パラメータ:**
- `channel: number` — 閉じるチャネル番号（0–65535）

**戻り値:** `nil`

**例:**
```rust
// Rust example to be added
```
---

### `closeAll()` / `book_next_close_all()` / `read_last_close_all()` / `async_close_all()`

すべての開いているチャネルを閉じます。

**Lua署名:**
```rust
// Rust example to be added
```
**Rust署名:**
```rust
pub fn book_next_close_all(&mut self)
pub fn read_last_close_all(&self) -> Result<(), PeripheralError>
pub async fn async_close_all(&self) -> Result<(), PeripheralError>
```

**戻り値:** `nil`

**例:**
```rust
// Rust example to be added
```
---

### `transmit(channel, replyChannel, payload)` / `book_next_transmit(...)` / `read_last_transmit()` / `async_transmit(...)`

チャネルにメッセージを送信します。

**Lua署名:**
```rust
// Rust example to be added
```
**Rust署名:**
```rust
pub fn book_next_transmit<T: Serialize>(&mut self, channel: u32, reply_channel: u32, payload: &T)
pub fn read_last_transmit(&self) -> Result<(), PeripheralError>
pub async fn async_transmit<T: Serialize>(&self, channel: u32, reply_channel: u32, payload: &T) -> Result<(), PeripheralError>
```

**パラメータ:**
- `channel: number` — ターゲットチャネル（0–65535）
- `replyChannel: number` — 応答用のリプライチャネル（0–65535）
- `payload: any` — メッセージデータ（プリミティブ、テーブル、またはシリアライズ可能な型）

**戻り値:** `nil`

**注記:**
- 送信するためにターゲットチャネルを開く必要はありません
- リプライチャネルは応答を受信するためにあなたのモデムで開いている必要があります
- ペイロードは任意のLua値（ブール値、数値、文字列、テーブル）です
- 関数とメタテーブルは送信されません

**例:**
```rust
// Rust example to be added
```
## イベント

### `modem_message` 🚧

開いているチャネルでメッセージが受信されたときに発生します。

**イベントパラメータ:**
1. `string` — イベント名（`"modem_message"`）
2. `string` — モデムが接続されている側
3. `number` — メッセージが送信されたチャネル
4. `number` — 送信者が指定したリプライチャネル
5. `any` — メッセージペイロード
6. `number | nil` — ブロック単位での送信者までの距離（クロスディメンションの場合はnil）

**例:**
```rust
// Rust example to be added
```
---

## 使用例

### 例1: 基本的なメッセージ交換

```rust
// Rust example to be added
```
### 例2: シンプルなサーバー

```rust
// Rust example to be added
```
### 例3: ブロードキャストリスナー

```rust
// Rust example to be added
```
### 例4: リクエスト-リプライパターン

```rust
// Rust example to be added
```
### 例5: ワイヤードネットワーク検出

```rust
// Rust example to be added
```
---

## エラーハンドリング

すべてのメソッドは以下の場合にエラーをスローする可能性があります：

- **モデムが見つからない**: ペリフェラルが切断されている
- **無効なチャネル**: チャネル番号が範囲外（0–65535）
- **チャネルが多すぎる**: 128個のチャネルが既に開いている
- **ネットワークエラー**: ワイヤードネットワーク接続が切断されている

**エラーハンドリングの例:**
```rust
// Rust example to be added
```
---

## 型定義

### ReceiveData
```rust
// Rust example to be added
```
---

## 注記

- チャネル番号は0から65535の範囲です
- 最大128個のチャネルを同時に開くことができます
- ワイヤレスモデムは範囲制限があります。ワイヤードモデムはありません
- メッセージは即座に送信されますが、イベントとして受信されます
- 3つの関数パターンは効率的なバッチ操作を可能にします
- エンダーモデムはディメンション間で通信できます

---

## 関連

- [Inventory](./Inventory.md) — ネットワークアクセスにはワイヤードモデムが必要
- [CC:Tweaked ドキュメント](https://tweaked.cc/) — 公式ドキュメント
- `rednet` API — モデムの上に構築された高レベルのネットワーキングAPI
