# Modem

**mod**: CC:Tweaked  
**peripheral type**: `modem`  
**source**: `ModemPeripheral.java`

## 概要

モデム（無線・有線）へのペリフェラル。チャンネルを開いてメッセージを送受信する。  
有線モデムの拡張メソッドは [wired_modem.md](./wired_modem.md) を参照。

## メソッド一覧

| メソッド名 | 引数 | 戻り値 | flags | imm | 説明 |
|---|---|---|---|---|---|
| `open` | `channel: number` | `nil` | — | ✗ | 指定チャンネルを開く（0〜65535、最大 128 チャンネル同時） |
| `isOpen` | `channel: number` | `boolean` | — | ✓ | 指定チャンネルが開いているか返す |
| `close` | `channel: number` | `nil` | — | ✗ | 指定チャンネルを閉じる |
| `closeAll` | — | `nil` | — | ✗ | 全チャンネルを閉じる |
| `transmit` | `channel: number, replyChannel: number, payload: any` | `nil` | — | ✗ | チャンネルにメッセージを送信する（payload は boolean/number/string/table） |
| `isWireless` | — | `boolean` | — | ✓ | このモデムが無線モデムかどうかを返す |

## イベント

受信側で発生するイベント:

| イベント名 | 引数 | 説明 |
|---|---|---|
| `modem_message` | `side, channel, replyChannel, payload, distance` | チャンネルにメッセージが届いたとき |
