# Modem

**mod**: CC:Tweaked  
**peripheral type**: `modem`  
**source**: `ModemPeripheral.java`

## 概要

モデム（無線・有線）へのペリフェラル。チャンネルを開いてメッセージを送受信する。

モデムは0〜65535の範囲のチャンネルで動作し、最大128チャンネルを同時に開くことができる。メッセージを受信するには、チャンネルを開いておく必要がある。

## モデムの種類

CC:Tweakedには3種類のモデムがある：

1. **無線モデム (Wireless Modem)**: 
   - 他の無線モデムにメッセージを送信
   - 範囲制限あり（通常64ブロック、y=96以上で最大384ブロック）
   - コンピュータの隣に設置、またはポケットコンピュータ/タートルのアップグレードとして装備

2. **エンダーモデム (Ender Modem)**:
   - 無線モデムの上位版
   - 距離制限なし、ディメンション間通信可能

3. **有線モデム (Wired Modem)**:
   - ネットワークケーブルで接続された他の有線モデムにメッセージを送信
   - 追加のペリフェラルをコンピュータに接続可能
   - 有線モデム固有のメソッドは [wired_modem.md](./wired_modem.md) を参照

## メソッド一覧

| メソッド名 | 引数 | 戻り値 | 説明 |
|---|---|---|---|
| `open` | `channel: number` | `nil` | 指定チャンネルを開く（0〜65535、最大128チャンネル同時） |
| `isOpen` | `channel: number` | `boolean` | 指定チャンネルが開いているか返す |
| `close` | `channel: number` | `nil` | 指定チャンネルを閉じる |
| `closeAll` | — | `nil` | 全チャンネルを閉じる |
| `transmit` | `channel: number, replyChannel: number, payload: any` | `nil` | チャンネルにメッセージを送信する |
| `isWireless` | — | `boolean` | このモデムが無線モデムかどうかを返す |

## メソッド詳細

### open(channel)

モデムのチャンネルを開く。メッセージを受信するには、チャンネルを開いておく必要がある。

最大128チャンネルを同時に開くことができる。

**引数:**
- `channel: number` - 開くチャンネル番号（0〜65535）

**戻り値:**
- なし

**例:**
```lua
local modem = peripheral.find("modem")
modem.open(15)  -- チャンネル15を開く
```

**エラー:**
- チャンネル番号が範囲外（0〜65535以外）の場合
- 既に128チャンネルが開いている場合

---

### isOpen(channel)

指定チャンネルが開いているかどうかを確認する。

**引数:**
- `channel: number` - 確認するチャンネル番号（0〜65535）

**戻り値:**
- `boolean` - チャンネルが開いている場合は `true`、閉じている場合は `false`

**例:**
```lua
local modem = peripheral.find("modem")
if modem.isOpen(15) then
  print("Channel 15 is open")
else
  print("Channel 15 is closed")
end
```

**エラー:**
- チャンネル番号が範囲外の場合

---

### close(channel)

開いているチャンネルを閉じる。閉じたチャンネルではメッセージを受信できなくなる。

**引数:**
- `channel: number` - 閉じるチャンネル番号（0〜65535）

**戻り値:**
- なし

**例:**
```lua
local modem = peripheral.find("modem")
modem.close(15)  -- チャンネル15を閉じる
```

**エラー:**
- チャンネル番号が範囲外の場合

---

### closeAll()

開いている全チャンネルを閉じる。

**引数:**
- なし

**戻り値:**
- なし

**例:**
```lua
local modem = peripheral.find("modem")
modem.closeAll()  -- 全チャンネルを閉じる
```

---

### transmit(channel, replyChannel, payload)

指定チャンネルにメッセージを送信する。

チャンネルを開いているモデムは `modem_message` イベントを受信する。

**注意:** メッセージを送信するためにチャンネルを開く必要はない。

**引数:**
- `channel: number` - メッセージを送信するチャンネル（0〜65535）
- `replyChannel: number` - 返信用チャンネル（0〜65535）。送信側がこのチャンネルを開いていれば返信を受信できる
- `payload: any` - 送信するデータ。プリミティブ型（boolean、number、string）およびテーブルが使用可能。関数やメタテーブルは送信されない

**戻り値:**
- なし

**例:**
```lua
local modem = peripheral.find("modem")
modem.open(43)  -- 返信を受信するためにチャンネル43を開く

-- チャンネル15にメッセージを送信、返信はチャンネル43で受信
modem.transmit(15, 43, "Hello, world!")
```

**エラー:**
- チャンネル番号が範囲外の場合

---

### isWireless()

このモデムが無線モデムか有線モデムかを判定する。

有線ネットワークやリモートペリフェラルに関するメソッドは、有線モデムでのみ使用可能。

**引数:**
- なし

**戻り値:**
- `boolean` - 無線モデムの場合は `true`、有線モデムの場合は `false`

**例:**
```lua
local modem = peripheral.find("modem")
if modem.isWireless() then
  print("This is a wireless modem")
else
  print("This is a wired modem")
end
```

---

## イベント

### modem_message

チャンネルにメッセージが届いたときに発生するイベント。

**パラメータ:**
1. `string` - イベント名（`"modem_message"`）
2. `string` - メッセージを受信したモデムの側面
3. `number` - メッセージが送信されたチャンネル
4. `number` - 送信者が設定した返信チャンネル
5. `any` - 送信者が送信したメッセージ
6. `number | nil` - 送信者と受信者の距離（ブロック単位）。ディメンション間通信の場合は `nil`

**例:**
```lua
local modem = peripheral.find("modem")
modem.open(0)

while true do
  local event, side, channel, replyChannel, message, distance = os.pullEvent("modem_message")
  print(("Message received on side %s on channel %d (reply to %d) from %f blocks away with message %s"):format(
    side, channel, replyChannel, distance or -1, tostring(message)
  ))
end
```

---

## 使用例

### 例1: 基本的なメッセージ送受信

```lua
-- 送信側
local modem = peripheral.find("modem")
modem.open(43)  -- 返信用チャンネルを開く
modem.transmit(15, 43, "Hello, world!")

-- 受信側
local modem = peripheral.find("modem")
modem.open(15)  -- チャンネル15を開く

local event, side, channel, replyChannel, message, distance = os.pullEvent("modem_message")
print("Received: " .. tostring(message))

-- 返信
modem.transmit(replyChannel, 15, "Hello back!")
```

### 例2: メッセージの待機と返信

```lua
local modem = peripheral.find("modem") or error("No modem attached", 0)
modem.open(43)  -- 返信用チャンネルを開く

-- メッセージを送信
modem.transmit(15, 43, "Hello, world!")

-- チャンネル43で返信を待つ
local event, side, channel, replyChannel, message, distance
repeat
  event, side, channel, replyChannel, message, distance = os.pullEvent("modem_message")
until channel == 43

print("Received a reply: " .. tostring(message))
```

### 例3: ブロードキャストサーバー

```lua
local modem = peripheral.find("modem")
modem.open(100)  -- リクエスト用チャンネル

print("Server listening on channel 100...")

while true do
  local event, side, channel, replyChannel, message, distance = os.pullEvent("modem_message")
  
  if channel == 100 then
    print("Received request: " .. tostring(message))
    
    -- 返信を送信
    modem.transmit(replyChannel, 100, "Response: " .. tostring(message))
  end
end
```

### 例4: 複数チャンネルの監視

```lua
local modem = peripheral.find("modem")

-- 複数チャンネルを開く
for i = 1, 5 do
  modem.open(i)
end

while true do
  local event, side, channel, replyChannel, message, distance = os.pullEvent("modem_message")
  print("Channel " .. channel .. ": " .. tostring(message))
end
```

---

## 備考

- チャンネル番号は0〜65535の範囲
- 最大128チャンネルを同時に開くことができる
- メッセージを送信するためにチャンネルを開く必要はない
- 無線モデムの範囲は通常64ブロック、y=96以上で最大384ブロック
- エンダーモデムは距離制限なし、ディメンション間通信可能
- 有線モデムはネットワークケーブルで接続された他の有線モデムと通信
- `rednet` APIはモデムの上に構築された、より使いやすいインターフェース

---

## 関連項目

- [wired_modem.md](./wired_modem.md) - 有線モデム固有のメソッド
- [CC:Tweaked Documentation](https://tweaked.cc/) - 公式ドキュメント
- `rednet` API - モデムの上に構築されたネットワークAPI
