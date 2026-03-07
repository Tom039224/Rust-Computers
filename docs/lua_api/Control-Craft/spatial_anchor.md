# SpatialAnchor

**mod**: Control-Craft  
**peripheral type**: `spatial`  
**source**: `SpatialAnchorPeripheral.java`

## 概要

スペーシャルアンカー。VS2 ship の静的配置・PPID/QPID 制御・チャンネル設定を管理するペリフェラル。位置 (Position) と回転 (Quaternion) の両軸に PID 制御を設定できる。

## メソッド一覧

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `setStatic` | `enabled: boolean` | — | ✗ | 船を静的（動かない）状態に設定する |
| `setRunning` | `enabled: boolean` | — | ✗ | アンカーの動作状態を設定する |
| `setOffset` | `offset: number` | — | ✗ | アンカーのオフセット距離を設定する |
| `setPPID` | `p: number`, `i: number`, `d: number` | — | ✗ | 位置制御 (PPID) のゲインを設定する |
| `setQPID` | `p: number`, `i: number`, `d: number` | — | ✗ | 回転制御 (QPID) のゲインを設定する |
| `setChannel` | `channel: number` | — | ✗ | チャンネル番号 (long) を設定する |

## 備考

- `setPPID` は位置の PID、`setQPID` はクォータニオンベースの回転 PID。
- `setChannel` は long 整数値を受け取る。
