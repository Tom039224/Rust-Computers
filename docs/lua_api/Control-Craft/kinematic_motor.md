# KinematicMotor

**mod**: Control-Craft  
**peripheral type**: `servo`  
**source**: `KinematicMotorPeripheral.java`

## 概要

キネマティックモーター。指定した角度に直接移動するサーボペリフェラル。`DynamicMotor` とは異なり物理トルクをシミュレートせず、制御ターゲットに従って直接移動する。

## メソッド一覧

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `setTargetAngle` | `value: number` | — | ✗ | 目標角度 (deg) を設定する |
| `getTargetAngle` | — | `number` | ✓ | 現在の目標角度を返す |
| `getControlTarget` | — | `string` | ✓ | 現在の制御ターゲット名を返す |
| `setControlTarget` | `target: string` | — | ✗ | 制御ターゲットを設定する |
| `getPhysics` | — | `table` | ✓ | 物理エンジンパラメーターを返す |
| `getAngle` | — | `number` | ✓ | 現在の角度 (deg) を返す |
| `getRelative` | — | `number[3][3]` | ✓ | 相対変換行列 (3×3) を返す |
| `setIsForcingAngle` | `enabled: boolean` | — | ✗ | 強制角度モードのオン/オフを切り替える |

## 備考

- `DynamicMotor` と同じ peripheral type `servo` を持つが、メソッドセットが異なる。
- `getRelative` は 3×3 の二次元配列（Lua テーブル）を返す。
