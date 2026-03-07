# DynamicMotor

**mod**: Control-Craft  
**peripheral type**: `servo`  
**source**: `DynamicMotorPeripheral.java`

## 概要

ダイナミックモーター。PID 制御による物理ベースのサーボ制御ペリフェラル。トルク・角度・回転速度の精密な制御が可能。`KinematicMotor` とは異なり物理トルクをシミュレートする。

## メソッド一覧

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `setPID` | `p: number`, `i: number`, `d: number` | — | ✗ | PID ゲインを設定する |
| `setTargetValue` | `value: number` | — | ✗ | 目標値（角度 deg）を設定する |
| `getTargetValue` | — | `number` | ✓ | 現在の目標値を返す |
| `getPhysics` | — | `PhysicsInfo` | ✓ | 物理エンジンパラメーター（サーボ・コンパニオン）を返す |
| `getAngle` | — | `number` | ✓ | 現在の角度 (deg) を返す |
| `getAngularVelocity` | — | `number` | ✓ | 現在の角速度 (deg/s) を返す |
| `getCurrentValue` | — | `number` | ✓ | 現在の制御変数値を返す |
| `getRelative` | — | `number[3][3]` | ✓ | 相対変換行列 (3×3) を返す |
| `setOutputTorque` | `scale: number` | — | ✗ | 出力トルクスケールを設定する |
| `setIsAdjustingAngle` | `enabled: boolean` | — | ✗ | 角度調整モードのオン/オフを切り替える |
| `lock` | — | — | ✗ | モーターをロック（固定）する |
| `unlock` | — | — | ✗ | モーターのロックを解除する |
| `isLocked` | — | `boolean` | ✓ | ロック状態かを返す |

## 返値テーブル構造

### `getPhysics` (`PhysicsInfo`)

```lua
{
  servomotor = { -- サーボ側のパラメーター },
  companion  = { -- コンパニオン側のパラメーター },
}
```

## 備考

- `getRelative` は 3×3 の二次元配列（Lua のテーブル）を返す。
- `KinematicMotor` も同じ peripheral type `servo` を使用するが、メソッドセットが異なる。
