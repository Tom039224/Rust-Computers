# Transmitter

**mod**: Control-Craft  
**peripheral type**: `transmitter`  
**source**: `TransmitterPeripheral.java`

## 概要

トランスミッター。CC-Tweaked の有線モデムに類似した、Control-Craft 独自の遠隔メソッド呼び出しペリフェラル。アクセスキー・コンテキスト・スロット名を使って周辺機器のメソッドをリモート呼び出しできる。

## メソッド一覧

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `callRemote` | `access: string`, `ctx: string`, `args...: any` | `any` | ✗ | 指定アクセスキーとコンテキストでリモートメソッドを同期呼び出しする |
| `callRemoteAsync` | `access: string`, `ctx: string`, `slotName: string`, `remoteName: string`, `method: string`, `args...: any` | — | ✗ | 非同期でリモートメソッドを呼び出し、結果を `slotName` イベントで受け取る |
| `setProtocol` | `protocol: number` | — | ✗ | 使用する通信プロトコル番号 (long) を設定する |

## 備考

- `callRemoteAsync` は結果を即座に返さず、指定 `slotName` のイベントとして後で受け取る。
- `setProtocol` は long 整数値を受け取る。
