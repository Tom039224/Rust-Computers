# ChatBox

**mod**: AdvancedPeripherals  
**peripheral type**: `chat_box`  
**source**: `ChatBoxPeripheral.java`

## 概要

チャットボックス。ゲームチャットへのメッセージ送信やトースト通知をサポートするペリフェラル。全メソッドはサーバーメインスレッドで実行される。

## メソッド一覧

| メソッド名 | 引数 | 戻り値 | flags | 説明 |
|---|---|---|---|---|
| `sendMessage` | `message: string`, `prefix?: string`, `brackets?: string`, `color?: string` | `boolean` | mainThread | 全プレイヤーにメッセージを送信する |
| `sendFormattedMessage` | `json: string`, `prefix?: string`, `brackets?: string`, `color?: string` | `boolean` | mainThread | JSON フォーマットのメッセージを全プレイヤーに送信する |
| `sendMessageToPlayer` | `message: string`, `player: string`, `prefix?: string`, `brackets?: string`, `color?: string` | `boolean` | mainThread | 指定プレイヤーにメッセージを送信する |
| `sendFormattedMessageToPlayer` | `json: string`, `player: string`, `prefix?: string`, `brackets?: string`, `color?: string` | `boolean` | mainThread | JSON フォーマットのメッセージを指定プレイヤーに送信する |
| `sendToastToPlayer` | `title: string`, `subtitle: string`, `player: string`, `prefix?: string`, `brackets?: string`, `color?: string` | `boolean` | mainThread | 指定プレイヤーにトースト通知を送信する |
| `sendFormattedToastToPlayer` | `jsonTitle: string`, `jsonSubtitle: string`, `player: string`, `prefix?: string`, `brackets?: string`, `color?: string` | `boolean` | mainThread | JSON フォーマットのトースト通知を指定プレイヤーに送信する |

## イベント

チャットボックスはプレイヤーのチャット発言を受信する:

| イベント名 | パラメーター | 説明 |
|---|---|---|
| `chat` | `username, message, uuid, isHidden` | プレイヤーがチャットにメッセージを送信したとき |

## 備考

- `prefix` / `brackets` / `color` はメッセージの装飾オプション（省略可）。
- `color` は `"&6"` のような Minecraft カラーコード形式。
