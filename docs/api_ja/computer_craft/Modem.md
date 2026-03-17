# Modem

**Mod:** CC:Tweaked  
**ペリフェラルタイプ:** `modem`  
**ソース:** `ModemPeripheral.java`

## 概要

Modemペリフェラルは無線および有線ネットワーク通信を提供します。チャンネルを開いてメッセージを送受信できます。モデムは無線モード（範囲制限あり）または有線モード（接続されたネットワーク上で無制限）で動作します。

## 3つの関数パターン

Modem APIは全メソッドで3つの関数パターンを使用します：

1. **`book_next_*`** — 次のティックのリクエストをスケジュール
2. **`read_last_*`** — 前のティックの結果を読み取り
3. **`async_*`** — book、待機、読み取りを1つの呼び出しで行う便利メソッド

### パターン説明

```lua
-- 方法1: book_next / read_last パターン
modem.book_next_open(15)
wait_for_next_tick()
modem.read_last_open()

-- 方法2: async パターン（推奨）
modem.async_open(15)
```

## モデムの種類

CC:Tweakedは3種類のモデムを提供します：

1. **無線モデム** — 他の無線モデムと通信（デフォルト範囲64ブロック、y≥96で最大384ブロック）
2. **エンダーモデム** — 無線モデムで無制限範囲とクロスディメンション対応
3. **有線モデム** — ネットワークケーブルで接続されたネットワーク上で無制限範囲

## メソッド

### `open(channel)` / `book_next_open(channel)` / `read_last_open()` / `async_open(channel)`

チャンネルを開いてメッセージ受信をリッスンします。

**Lua署名:**
```lua
function open(channel: number) -> nil
```

**Rust署名:**
```rust
pub fn book_next_open(&mut self, channel: u32)
pub fn read_last_open(&self) -> Result<(), PeripheralError>
pub async fn async_open(&self, channel: u32) -> Result<(), PeripheralError>
```

**パラメータ:**
- `channel: number` — 開くチャンネル番号（0–65535）

**戻り値:** `nil`

**注記:**
- 最大128チャンネルを同時に開くことができます
- メッセージを送信するためにチャンネルを開く必要はありません

**例:**
```lua
local modem = peripheral.find("modem")
modem.async_open(15)
print("Channel 15 opened")
```

**エラーハンドリング:**
- チャンネル番号が範囲外の場合、エラーをスロー
- 既に128チャンネルが開いている場合、エラーをスロー

---

### `isOpen(channel)` / `book_next_is_open(channel)` / `read_last_is_open()` / `async_is_open(channel)`

チャンネルが現在開いているかを確認します。

**Lua署名:**
```lua
function isOpen(channel: number) -> boolean
```

**Rust署名:**
```rust
pub fn book_next_is_open(&mut self, channel: u32)
pub fn read_last_is_open(&self) -> Result<bool, PeripheralError>
pub async fn async_is_open(&self, channel: u32) -> Result<bool, PeripheralError>
```

**パラメータ:**
- `channel: number` — 確認するチャンネル番号（0–65535）

**戻り値:** `boolean` — 開いている場合は `true`、閉じている場合は `false`

**例:**
```lua
local modem = peripheral.find("modem")
if modem.async_is_open(15) then
  print("Channel 15 is open")
else
  print("Channel 15 is closed")
end
```

---

### `close(channel)` / `book_next_close(channel)` / `read_last_close()` / `async_close(channel)`

開いているチャンネルを閉じます。

**Lua署名:**
```lua
function close(channel: number) -> nil
```

**Rust署名:**
```rust
pub fn book_next_close(&mut self, channel: u32)
pub fn read_last_close(&self) -> Result<(), PeripheralError>
pub async fn async_close(&self, channel: u32) -> Result<(), PeripheralError>
```

**パラメータ:**
- `channel: number` — 閉じるチャンネル番号（0–65535）

**戻り値:** `nil`

**例:**
```lua
local modem = peripheral.find("modem")
modem.async_close(15)
print("Channel 15 closed")
```

---

### `closeAll()` / `book_next_close_all()` / `read_last_close_all()` / `async_close_all()`

開いている全チャンネルを閉じます。

**Lua署名:**
```lua
function closeAll() -> nil
```

**Rust署名:**
```rust
pub fn book_next_close_all(&mut self)
pub fn read_last_close_all(&self) -> Result<(), PeripheralError>
pub async fn async_close_all(&self) -> Result<(), PeripheralError>
```

**戻り値:** `nil`

**例:**
```lua
local modem = peripheral.find("modem")
modem.async_close_all()
print("All channels closed")
```

---

### `transmit(channel, replyChannel, payload)` / `book_next_transmit(...)` / `read_last_transmit()` / `async_transmit(...)`

チャンネルにメッセージを送信します。

**Lua署名:**
```lua
function transmit(channel: number, replyChannel: number, payload: any) -> nil
```

**Rust署名:**
```rust
pub fn book_next_transmit<T: Serialize>(&mut self, channel: u32, reply_channel: u32, payload: &T)
pub fn read_last_transmit(&self) -> Result<(), PeripheralError>
pub async fn async_transmit<T: Serialize>(&self, channel: u32, reply_channel: u32, payload: &T) -> Result<(), PeripheralError>
```

**パラメータ:**
- `channel: number` — ターゲットチャンネル（0–65535）
- `replyChannel: number` — 応答用チャンネル（0–65535）
- `payload: any` — メッセージデータ（プリミティブ型、テーブル、またはシリアライズ可能な型）

**戻り値:** `nil`

**注記:**
- ターゲットチャンネルを開く必要はありません
- 応答チャンネルはあなたのモデムで開いておく必要があります
- ペイロードはLua値（boolean、number、string、table）です
- 関数とメタテーブルは送信されません

**例:**
```lua
local modem = peripheral.find("modem")
modem.async_open(43)  -- 応答チャンネルを開く

-- メッセージを送信
modem.async_transmit(15, 43, "Hello, world!")
```

---

### `isWireless()` / `book_next_is_wireless()` / `read_last_is_wireless()` / `async_is_wireless()`

このモデムが無線か有線かを確認します。

**Lua署名:**
```lua
function isWireless() -> boolean
```

**Rust署名:**
```rust
pub fn book_next_is_wireless(&mut self)
pub fn read_last_is_wireless(&self) -> Result<bool, PeripheralError>
pub async fn async_is_wireless(&self) -> Result<bool, PeripheralError>
```

**戻り値:** `boolean` — 無線の場合は `true`、有線の場合は `false`

**例:**
```lua
local modem = peripheral.find("modem")
if modem.async_is_wireless() then
  print("This is a wireless modem")
else
  print("This is a wired modem")
end
```

---

## イベント

### `modem_message`

開いているチャンネルでメッセージが受信されたときに発火します。

**イベントパラメータ:**
1. `string` — イベント名（`"modem_message"`）
2. `string` — モデムが接続されている側面
3. `number` — メッセージが送信されたチャンネル
4. `number` — 送信者が指定した応答チャンネル
5. `any` — メッセージペイロード
6. `number | nil` — 送信者までの距離（ブロック単位）、クロスディメンションの場合は `nil`

**例:**
```lua
local modem = peripheral.find("modem")
modem.async_open(0)

while true do
  local event, side, channel, replyChannel, message, distance = os.pullEvent("modem_message")
  print(("Received on channel %d: %s"):format(channel, tostring(message)))
  
  if distance then
    print(("Distance: %d blocks"):format(distance))
  else
    print("Cross-dimension message")
  end
end
```

---

## 使用例

### 例1: 基本的なメッセージ交換

```lua
-- 送信側
local modem = peripheral.find("modem")
modem.async_open(43)  -- 応答チャンネルを開く
modem.async_transmit(15, 43, "Hello!")

-- 受信側
local modem = peripheral.find("modem")
modem.async_open(15)

local event, side, channel, replyChannel, message, distance = os.pullEvent("modem_message")
print("Received: " .. tostring(message))

-- 応答を送信
modem.async_transmit(replyChannel, 15, "Hello back!")
```

### 例2: シンプルなサーバー

```lua
local modem = peripheral.find("modem")
modem.async_open(100)

print("Server listening on channel 100...")

while true do
  local event, side, channel, replyChannel, message, distance = os.pullEvent("modem_message")
  
  if channel == 100 then
    print("Request: " .. tostring(message))
    
    -- 応答を送信
    modem.async_transmit(replyChannel, 100, "Response: " .. tostring(message))
  end
end
```

### 例3: ブロードキャストリスナー

```lua
local modem = peripheral.find("modem")

-- 複数チャンネルをリッスン
for i = 1, 5 do
  modem.async_open(i)
end

while true do
  local event, side, channel, replyChannel, message, distance = os.pullEvent("modem_message")
  print(("Channel %d: %s (distance: %s)"):format(
    channel,
    tostring(message),
    distance and tostring(distance) or "unknown"
  ))
end
```

### 例4: リクエスト-応答パターン

```lua
local modem = peripheral.find("modem")
modem.async_open(43)

-- リクエストを送信
modem.async_transmit(15, 43, {action = "query", data = "test"})

-- タイムアウト付きで応答を待機
local timeout = os.startTimer(5)
while true do
  local event, arg1, arg2, arg3, arg4, arg5 = os.pullEvent()
  
  if event == "modem_message" then
    local channel, replyChannel, message = arg3, arg4, arg5
    if channel == 43 then
      print("Reply: " .. tostring(message))
      break
    end
  elseif event == "timer" and arg1 == timeout then
    print("Request timed out")
    break
  end
end
```

### 例5: 有線ネットワーク検出

```lua
local modem = peripheral.find("modem")

if modem.async_is_wireless() then
  print("Wireless modem detected")
  print("Range: 64 blocks (up to 384 at y≥96)")
else
  print("Wired modem detected")
  print("Range: Unlimited on connected network")
end
```

---

## エラーハンドリング

全メソッドは以下の場合にエラーをスロー可能です：

- **モデムが見つからない**: ペリフェラルが切断されている
- **無効なチャンネル**: チャンネル番号が範囲外（0–65535）
- **チャンネルが多すぎる**: 既に128チャンネルが開いている
- **ネットワークエラー**: 有線ネットワーク接続が切断されている

**エラーハンドリング例:**
```lua
local modem = peripheral.find("modem")
if not modem then
  error("No modem found")
end

local success, result = pcall(function()
  modem.async_open(15)
end)

if not success then
  print("Error: " .. result)
else
  print("Channel opened successfully")
end
```

---

## 型定義

### ReceiveData
```lua
{
  channel: number,      -- メッセージが送信されたチャンネル
  replyChannel: number, -- 送信者が指定した応答チャンネル
  payload: any,         -- メッセージデータ
  distance: number | nil, -- ブロック単位での距離（クロスディメンションの場合は nil）
}
```

---

## 注記

- チャンネル番号は0から65535の範囲
- 最大128チャンネルを同時に開くことができます
- 無線モデムは範囲制限があります。有線モデムはありません
- メッセージは即座に送信されますが、イベントとして受信されます
- 3つの関数パターンは効率的なバッチ操作を可能にします
- エンダーモデムはクロスディメンション通信に対応しています

---

## 関連

- [Inventory](./Inventory.md) — ネットワークアクセスに有線モデムが必要
- [CC:Tweaked Documentation](https://tweaked.cc/) — 公式ドキュメント
- `rednet` API — モデムの上に構築された高レベルネットワークAPI
