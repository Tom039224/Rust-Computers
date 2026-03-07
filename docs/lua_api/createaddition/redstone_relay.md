# RedstoneRelay

**mod**: createaddition  
**peripheral type**: `redstone_relay`  
**source**: `RedstoneRelayPeripheral.java`

## 概要

レッドストーンリレー。FE エネルギーの流量をレッドストーン信号に変換する（またはその逆）。エネルギーモニタリング用途に使用する。

## メソッド一覧

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getMaxInsert` | — | `number` | ✗ | 1 tick あたりの最大入力 (FE/t) を返す |
| `getMaxExtract` | — | `number` | ✗ | 1 tick あたりの最大出力 (FE/t) を返す |
| `getThroughput` | — | `number` | ✗ | 現在の通過エネルギー量 (FE/t) を返す |
| `isPowered` | — | `boolean` | ✗ | レッドストーン信号を受信しているかを返す |
