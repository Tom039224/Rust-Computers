# ChatBox

**モジュール:** `advanced_peripherals::chat_box`  
**ペリフェラルタイプ:** `advancedPeripherals:chat_box`

AdvancedPeripherals チャットボックス ペリフェラル。プレイヤーにチャットメッセージやトースト通知を送信します。プレーンテキストとJSONフォーマットのメッセージに対応し、プレフィックス、括弧、色のカスタマイズもサポートします。

## ブックリードメソッド

### `book_next_send_message` / `read_last_send_message`
全プレイヤーにメッセージを送信します。
```rust
pub fn book_next_send_message(
    &mut self,
    message: &str,
    prefix: Option<&str>,
    brackets: Option<&str>,
    color: Option<&str>,
)
pub fn read_last_send_message(&self) -> Result<bool, PeripheralError>
```
**パラメータ:**
- `message: &str` — メッセージテキスト
- `prefix: Option<&str>` — オプションのプレフィックス
- `brackets: Option<&str>` — オプションの括弧スタイル
- `color: Option<&str>` — オプションの色

**戻り値:** `bool` — 成功ステータス

---

### `book_next_send_formatted_message` / `read_last_send_formatted_message`
全プレイヤーにJSONフォーマットのメッセージを送信します。
```rust
pub fn book_next_send_formatted_message(
    &mut self,
    json: &str,
    prefix: Option<&str>,
    brackets: Option<&str>,
    color: Option<&str>,
)
pub fn read_last_send_formatted_message(&self) -> Result<bool, PeripheralError>
```
**パラメータ:**
- `json: &str` — JSONテキストコンポーネント
- `prefix: Option<&str>` — オプションのプレフィックス
- `brackets: Option<&str>` — オプションの括弧スタイル
- `color: Option<&str>` — オプションの色

**戻り値:** `bool`

---

### `book_next_send_message_to_player` / `read_last_send_message_to_player`
特定のプレイヤーにメッセージを送信します。
```rust
pub fn book_next_send_message_to_player(
    &mut self,
    message: &str,
    player: &str,
    prefix: Option<&str>,
    brackets: Option<&str>,
    color: Option<&str>,
)
pub fn read_last_send_message_to_player(&self) -> Result<bool, PeripheralError>
```
**パラメータ:**
- `message: &str` — メッセージテキスト
- `player: &str` — 対象プレイヤー名
- `prefix: Option<&str>` — オプションのプレフィックス
- `brackets: Option<&str>` — オプションの括弧スタイル
- `color: Option<&str>` — オプションの色

**戻り値:** `bool`

---

### `book_next_send_formatted_message_to_player` / `read_last_send_formatted_message_to_player`
特定のプレイヤーにJSONフォーマットのメッセージを送信します。
```rust
pub fn book_next_send_formatted_message_to_player(
    &mut self,
    json: &str,
    player: &str,
    prefix: Option<&str>,
    brackets: Option<&str>,
    color: Option<&str>,
)
pub fn read_last_send_formatted_message_to_player(&self) -> Result<bool, PeripheralError>
```
**パラメータ:**
- `json: &str` — JSONテキストコンポーネント
- `player: &str` — 対象プレイヤー名
- `prefix / brackets / color` — オプションのカスタマイズ

**戻り値:** `bool`

---

### `book_next_send_toast_to_player` / `read_last_send_toast_to_player`
特定のプレイヤーにトースト通知を送信します。
```rust
pub fn book_next_send_toast_to_player(
    &mut self,
    title: &str,
    subtitle: &str,
    player: &str,
    prefix: Option<&str>,
    brackets: Option<&str>,
    color: Option<&str>,
)
pub fn read_last_send_toast_to_player(&self) -> Result<bool, PeripheralError>
```
**パラメータ:**
- `title: &str` — トーストタイトル
- `subtitle: &str` — トーストサブタイトル
- `player: &str` — 対象プレイヤー名
- `prefix / brackets / color` — オプションのカスタマイズ

**戻り値:** `bool`

---

### `book_next_send_formatted_toast_to_player` / `read_last_send_formatted_toast_to_player`
特定のプレイヤーにJSONフォーマットのトースト通知を送信します。
```rust
pub fn book_next_send_formatted_toast_to_player(
    &mut self,
    json_title: &str,
    json_subtitle: &str,
    player: &str,
    prefix: Option<&str>,
    brackets: Option<&str>,
    color: Option<&str>,
)
pub fn read_last_send_formatted_toast_to_player(&self) -> Result<bool, PeripheralError>
```
**パラメータ:**
- `json_title: &str` — JSONタイトルコンポーネント
- `json_subtitle: &str` — JSONサブタイトルコンポーネント
- `player: &str` — 対象プレイヤー名
- `prefix / brackets / color` — オプションのカスタマイズ

**戻り値:** `bool`

## イミディエイトメソッド

なし。

## 型定義

なし。

## 使用例

```rust
use rust_computers_api::advanced_peripherals::ChatBox;
use rust_computers_api::peripheral::Peripheral;

let mut chat = ChatBox::wrap(addr);

chat.book_next_send_message("こんにちは！", Some("サーバー"), None, None);
wait_for_next_tick().await;
let ok = chat.read_last_send_message();

chat.book_next_send_message_to_player("プライベートメッセージ", "Steve", None, None, None);
wait_for_next_tick().await;
```
