# ChatBox

**モジュール:** AdvancedPeripherals  
**ペリフェラルタイプ:** `advancedPeripherals:chat_box`  
**ソース:** `ChatBoxPeripheral.java`

## 概要

ChatBoxペリフェラルを使用すると、サーバー上のプレイヤーにチャットメッセージとトースト通知を送信できます。プレーンテキストメッセージ、JSON形式のテキストコンポーネント、プレフィックス、括弧、色でカスタマイズ可能な形式をサポートしています。これはサーバーアナウンスメント、プレイヤー通知、インタラクティブシステムの作成に役立ちます。

## 3つの関数パターン

ChatBox APIは、すべてのメソッドに対して3つの関数パターンを使用します：

1. **`book_next_*`** - 次のティックのリクエストをスケジュール
2. **`read_last_*`** - 前のティックの結果を読み取り
3. **`async_*`** - 便利なメソッド（book、待機、読み取りを1つの呼び出しで実行）

### パターンの説明

```rust
// Rust example to be added
```
## 実装状況

### ✅ 実装済み

- All book_next_* / read_last_* methods

### 🚧 未実装

- async_* variants for all methods
- chat event


## メソッド

### `sendMessage(message, prefix?, brackets?, color?)` / `book_next_send_message(...)` / `read_last_send_message()` / `async_send_message(...)`

サーバー上のすべてのプレイヤーにプレーンテキストメッセージを送信します。

**Lua署名:**
```rust
// Rust example to be added
```
**Rust署名:**
```rust
pub fn book_next_send_message(&mut self, message: &str, prefix: Option<&str>, brackets: Option<&str>, color: Option<&str>)
pub fn read_last_send_message(&self) -> Result<bool, PeripheralError>
pub async fn async_send_message(&self, message: &str, prefix: Option<&str>, brackets: Option<&str>, color: Option<&str>) -> Result<bool, PeripheralError>
```

**パラメータ:**
- `message: string` — 送信するメッセージテキスト
- `prefix?: string` — オプションのプレフィックス（例：`"[サーバー]"`）
- `brackets?: string` — オプションの括弧スタイル（例：`"<>"`, `"[]"`）
- `color?: string` — オプションの色コード（例：`"red"`、`"#FF0000"`）

**戻り値:** `boolean` — メッセージが正常に送信された場合は`true`

**例:**
```rust
// Rust example to be added
```
---

### `sendFormattedMessage(json, prefix?, brackets?, color?)` / `book_next_send_formatted_message(...)` / `read_last_send_formatted_message()` / `async_send_formatted_message(...)`

JSON形式のテキストコンポーネントメッセージをすべてのプレイヤーに送信します。

**Lua署名:**
```rust
// Rust example to be added
```
**Rust署名:**
```rust
pub fn book_next_send_formatted_message(&mut self, json: &str, prefix: Option<&str>, brackets: Option<&str>, color: Option<&str>)
pub fn read_last_send_formatted_message(&self) -> Result<bool, PeripheralError>
pub async fn async_send_formatted_message(&self, json: &str, prefix: Option<&str>, brackets: Option<&str>, color: Option<&str>) -> Result<bool, PeripheralError>
```

**パラメータ:**
- `json: string` — JSONテキストコンポーネント（Minecraftチャット形式）
- `prefix?: string` — オプションのプレフィックス
- `brackets?: string` — オプションの括弧スタイル
- `color?: string` — オプションの色コード

**戻り値:** `boolean` — メッセージが正常に送信された場合は`true`

**JSON形式の例:**
```json
{
  "text": "クリックしてください！",
  "color": "blue",
  "clickEvent": {
    "action": "run_command",
    "value": "/say クリックされました"
  }
}
```

**例:**
```rust
// Rust example to be added
```
---

### `sendMessageToPlayer(message, player, prefix?, brackets?, color?)` / `book_next_send_message_to_player(...)` / `read_last_send_message_to_player()` / `async_send_message_to_player(...)`

特定のプレイヤーにプレーンテキストメッセージを送信します。

**Lua署名:**
```rust
// Rust example to be added
```
**Rust署名:**
```rust
pub fn book_next_send_message_to_player(&mut self, message: &str, player: &str, prefix: Option<&str>, brackets: Option<&str>, color: Option<&str>)
pub fn read_last_send_message_to_player(&self) -> Result<bool, PeripheralError>
pub async fn async_send_message_to_player(&self, message: &str, player: &str, prefix: Option<&str>, brackets: Option<&str>, color: Option<&str>) -> Result<bool, PeripheralError>
```

**パラメータ:**
- `message: string` — メッセージテキスト
- `player: string` — ターゲットプレイヤー名
- `prefix?: string` — オプションのプレフィックス
- `brackets?: string` — オプションの括弧スタイル
- `color?: string` — オプションの色コード

**戻り値:** `boolean` — メッセージが正常に送信された場合は`true`

**例:**
```rust
// Rust example to be added
```
---

### `sendFormattedMessageToPlayer(json, player, prefix?, brackets?, color?)` / `book_next_send_formatted_message_to_player(...)` / `read_last_send_formatted_message_to_player()` / `async_send_formatted_message_to_player(...)`

JSON形式のテキストコンポーネントメッセージを特定のプレイヤーに送信します。

**Lua署名:**
```rust
// Rust example to be added
```
**Rust署名:**
```rust
pub fn book_next_send_formatted_message_to_player(&mut self, json: &str, player: &str, prefix: Option<&str>, brackets: Option<&str>, color: Option<&str>)
pub fn read_last_send_formatted_message_to_player(&self) -> Result<bool, PeripheralError>
pub async fn async_send_formatted_message_to_player(&self, json: &str, player: &str, prefix: Option<&str>, brackets: Option<&str>, color: Option<&str>) -> Result<bool, PeripheralError>
```

**パラメータ:**
- `json: string` — JSONテキストコンポーネント
- `player: string` — ターゲットプレイヤー名
- `prefix?: string` — オプションのプレフィックス
- `brackets?: string` — オプションの括弧スタイル
- `color?: string` — オプションの色コード

**戻り値:** `boolean` — メッセージが正常に送信された場合は`true`

---

### `sendToastToPlayer(title, subtitle, player, prefix?, brackets?, color?)` / `book_next_send_toast_to_player(...)` / `read_last_send_toast_to_player()` / `async_send_toast_to_player(...)`

特定のプレイヤーにトースト通知を送信します。

**Lua署名:**
```rust
// Rust example to be added
```
**Rust署名:**
```rust
pub fn book_next_send_toast_to_player(&mut self, title: &str, subtitle: &str, player: &str, prefix: Option<&str>, brackets: Option<&str>, color: Option<&str>)
pub fn read_last_send_toast_to_player(&self) -> Result<bool, PeripheralError>
pub async fn async_send_toast_to_player(&self, title: &str, subtitle: &str, player: &str, prefix: Option<&str>, brackets: Option<&str>, color: Option<&str>) -> Result<bool, PeripheralError>
```

**パラメータ:**
- `title: string` — トーストタイトルテキスト
- `subtitle: string` — トーストサブタイトルテキスト
- `player: string` — ターゲットプレイヤー名
- `prefix?: string` — オプションのプレフィックス
- `brackets?: string` — オプションの括弧スタイル
- `color?: string` — オプションの色コード

**戻り値:** `boolean` — トーストが正常に送信された場合は`true`

**例:**
```rust
// Rust example to be added
```
---

### `sendFormattedToastToPlayer(jsonTitle, jsonSubtitle, player, prefix?, brackets?, color?)` / `book_next_send_formatted_toast_to_player(...)` / `read_last_send_formatted_toast_to_player()` / `async_send_formatted_toast_to_player(...)`

JSON形式のトースト通知を特定のプレイヤーに送信します。

**Lua署名:**
```rust
// Rust example to be added
```
**Rust署名:**
```rust
pub fn book_next_send_formatted_toast_to_player(&mut self, json_title: &str, json_subtitle: &str, player: &str, prefix: Option<&str>, brackets: Option<&str>, color: Option<&str>)
pub fn read_last_send_formatted_toast_to_player(&self) -> Result<bool, PeripheralError>
pub async fn async_send_formatted_toast_to_player(&self, json_title: &str, json_subtitle: &str, player: &str, prefix: Option<&str>, brackets: Option<&str>, color: Option<&str>) -> Result<bool, PeripheralError>
```

**パラメータ:**
- `jsonTitle: string` — JSONタイトルコンポーネント
- `jsonSubtitle: string` — JSONサブタイトルコンポーネント
- `player: string` — ターゲットプレイヤー名
- `prefix?: string` — オプションのプレフィックス
- `brackets?: string` — オプションの括弧スタイル
- `color?: string` — オプションの色コード

**戻り値:** `boolean` — トーストが正常に送信された場合は`true`

---

## イベント

ChatBoxペリフェラルはイベントを生成しません。

---

## 使用例

### 例1: サーバーアナウンスメント

```rust
// Rust example to be added
```
### 例2: ウェルカムメッセージ

```rust
// Rust example to be added
```
### 例3: ホバーテキスト付きの形式付きメッセージ

```rust
// Rust example to be added
```
### 例4: バッチ通知

```rust
// Rust example to be added
```
### 例5: ステータス更新

```rust
// Rust example to be added
```
---

## エラーハンドリング

すべてのメソッドは以下の場合にエラーをスローする可能性があります：

- **プレイヤーが見つからない**: ターゲットプレイヤーがオンラインではない
- **ペリフェラル切断**: ChatBoxにアクセスできなくなった
- **無効なJSON**: JSON形式が不正
- **メッセージが長すぎる**: メッセージが文字数制限を超えている

**エラーハンドリングの例:**
```rust
// Rust example to be added
```
---

## 型定義

### TextComponent (JSON)
```rust
// Rust example to be added
```
---

## 注記

- メッセージは非同期で送信され、すぐに表示されない場合があります
- プレイヤー名は大文字と小文字を区別します
- JSON形式はMinecraftのテキストコンポーネント形式に従います
- トースト通知はプレイヤーの画面の右上に表示されます
- 色コードは色名（例：「red」、「blue」）または16進数コード（例：「#FF0000」）です
- 3つの関数パターンは効率的なバッチ操作を可能にします

---

## 関連

- [Modem](../computer_craft/Modem.md) — ネットワーク通信用
- [AdvancedPeripherals ドキュメント](https://advancedperipherals.readthedocs.io/) — 公式ドキュメント
