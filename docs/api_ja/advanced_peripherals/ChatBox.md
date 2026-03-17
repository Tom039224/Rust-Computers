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

```lua
-- 方法1: book_next / read_last パターン
chat.book_next_send_message("こんにちは！")
wait_for_next_tick()
local ok = chat.read_last_send_message()

-- 方法2: async パターン（推奨）
local ok = chat.async_send_message("こんにちは！")
```

## メソッド

### `sendMessage(message, prefix?, brackets?, color?)` / `book_next_send_message(...)` / `read_last_send_message()` / `async_send_message(...)`

サーバー上のすべてのプレイヤーにプレーンテキストメッセージを送信します。

**Lua署名:**
```lua
function sendMessage(message: string, prefix?: string, brackets?: string, color?: string) -> boolean
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
```lua
local chat = peripheral.find("advancedPeripherals:chat_box")
local ok = chat.async_send_message("サーバーが再起動します！", "[サーバー]", "[]", "red")
print("メッセージ送信: " .. tostring(ok))
```

---

### `sendFormattedMessage(json, prefix?, brackets?, color?)` / `book_next_send_formatted_message(...)` / `read_last_send_formatted_message()` / `async_send_formatted_message(...)`

JSON形式のテキストコンポーネントメッセージをすべてのプレイヤーに送信します。

**Lua署名:**
```lua
function sendFormattedMessage(json: string, prefix?: string, brackets?: string, color?: string) -> boolean
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
```lua
local chat = peripheral.find("advancedPeripherals:chat_box")
local json = '{"text":"重要！","color":"red","bold":true}'
local ok = chat.async_send_formatted_message(json, "[アラート]")
```

---

### `sendMessageToPlayer(message, player, prefix?, brackets?, color?)` / `book_next_send_message_to_player(...)` / `read_last_send_message_to_player()` / `async_send_message_to_player(...)`

特定のプレイヤーにプレーンテキストメッセージを送信します。

**Lua署名:**
```lua
function sendMessageToPlayer(message: string, player: string, prefix?: string, brackets?: string, color?: string) -> boolean
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
```lua
local chat = peripheral.find("advancedPeripherals:chat_box")
local ok = chat.async_send_message_to_player("ようこそ！", "Steve", "[サーバー]")
```

---

### `sendFormattedMessageToPlayer(json, player, prefix?, brackets?, color?)` / `book_next_send_formatted_message_to_player(...)` / `read_last_send_formatted_message_to_player()` / `async_send_formatted_message_to_player(...)`

JSON形式のテキストコンポーネントメッセージを特定のプレイヤーに送信します。

**Lua署名:**
```lua
function sendFormattedMessageToPlayer(json: string, player: string, prefix?: string, brackets?: string, color?: string) -> boolean
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
```lua
function sendToastToPlayer(title: string, subtitle: string, player: string, prefix?: string, brackets?: string, color?: string) -> boolean
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
```lua
local chat = peripheral.find("advancedPeripherals:chat_box")
local ok = chat.async_send_toast_to_player("達成！", "ダイヤモンドを見つけました！", "Steve")
```

---

### `sendFormattedToastToPlayer(jsonTitle, jsonSubtitle, player, prefix?, brackets?, color?)` / `book_next_send_formatted_toast_to_player(...)` / `read_last_send_formatted_toast_to_player()` / `async_send_formatted_toast_to_player(...)`

JSON形式のトースト通知を特定のプレイヤーに送信します。

**Lua署名:**
```lua
function sendFormattedToastToPlayer(jsonTitle: string, jsonSubtitle: string, player: string, prefix?: string, brackets?: string, color?: string) -> boolean
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

```lua
local chat = peripheral.find("advancedPeripherals:chat_box")

chat.async_send_message("サーバーメンテナンスが5分後に行われます！", "[サーバー]", "[]", "red")
sleep(5)
chat.async_send_message("サーバーが今再起動しています！", "[サーバー]", "[]", "red")
```

### 例2: ウェルカムメッセージ

```lua
local chat = peripheral.find("advancedPeripherals:chat_box")

local function welcome_player(name)
  chat.async_send_message_to_player("サーバーへようこそ！", name, "[サーバー]", "[]", "green")
  chat.async_send_toast_to_player("ようこそ！", "楽しんでください！", name)
end

welcome_player("Steve")
```

### 例3: ホバーテキスト付きの形式付きメッセージ

```lua
local chat = peripheral.find("advancedPeripherals:chat_box")

local json = {
  text = "情報をクリック",
  color = "blue",
  underlined = true,
  hoverEvent = {
    action = "show_text",
    contents = "これはツールチップです"
  }
}

local json_str = textutils.serialiseJSON(json)
chat.async_send_formatted_message(json_str, "[情報]")
```

### 例4: バッチ通知

```lua
local chat = peripheral.find("advancedPeripherals:chat_box")

local players = {"Steve", "Alex", "Notch"}
for _, player in ipairs(players) do
  chat.async_send_message_to_player("こんにちは " .. player .. "！", player, "[サーバー]")
end
```

### 例5: ステータス更新

```lua
local chat = peripheral.find("advancedPeripherals:chat_box")

local function send_status(status, color)
  local json = {
    text = "ステータス: " .. status,
    color = color,
    bold = true
  }
  local json_str = textutils.serialiseJSON(json)
  chat.async_send_formatted_message(json_str, "[システム]")
end

send_status("オンライン", "green")
sleep(2)
send_status("ビジー", "yellow")
sleep(2)
send_status("オフライン", "red")
```

---

## エラーハンドリング

すべてのメソッドは以下の場合にエラーをスローする可能性があります：

- **プレイヤーが見つからない**: ターゲットプレイヤーがオンラインではない
- **ペリフェラル切断**: ChatBoxにアクセスできなくなった
- **無効なJSON**: JSON形式が不正
- **メッセージが長すぎる**: メッセージが文字数制限を超えている

**エラーハンドリングの例:**
```lua
local chat = peripheral.find("advancedPeripherals:chat_box")
if not chat then
  error("ChatBoxが見つかりません")
end

local success, result = pcall(function()
  return chat.async_send_message("こんにちは！")
end)

if not success then
  print("エラー: " .. result)
else
  print("メッセージ送信: " .. tostring(result))
end
```

---

## 型定義

### TextComponent (JSON)
```lua
{
  text: string,                    -- 表示テキスト
  color?: string,                  -- 色名または16進数（#RRGGBB）
  bold?: boolean,                  -- 太字テキスト
  italic?: boolean,                -- イタリック体テキスト
  underlined?: boolean,            -- 下線付きテキスト
  strikethrough?: boolean,         -- 打ち消し線テキスト
  obfuscated?: boolean,            -- 難読化テキスト
  clickEvent?: {
    action: string,                -- "open_url", "run_command", "suggest_command", "copy_to_clipboard"
    value: string                  -- アクション値
  },
  hoverEvent?: {
    action: string,                -- "show_text", "show_item", "show_entity"
    contents: string | table       -- イベント内容
  }
}
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
