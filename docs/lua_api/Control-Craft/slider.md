# Slider

**mod**: Control-Craft  
**peripheral type**: `slider`  
**source**: `SliderPeripheral.java`

## 概要

スライダー。PID 制御による直線方向の精密位置制御ペリフェラル。与圧力・目標距離・現在値の読み取りが可能。

## メソッド一覧

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `setOutputForce` | `scale: number` | — | ✗ | 出力力スケールを設定する |
| `setPID` | `p: number`, `i: number`, `d: number` | — | ✗ | PID ゲインを設定する |
| `getDistance` | — | `number` | ✓ | 現在のスライダー距離 (blocks) を返す |
| `getCurrentValue` | — | `number` | ✓ | 現在の制御変数値を返す |
| `getTargetValue` | — | `number` | ✓ | 現在の目標値を返す |
| `setTargetValue` | `target: number` | — | ✗ | 目標値を設定する |
| `getPhysics` | — | `PhysicsInfo` | ✓ | 物理エンジンパラメーター（スライダー・コンパニオン）を返す |
| `lock` | — | — | ✗ | スライダーをロック（固定）する |
| `unlock` | — | — | ✗ | スライダーのロックを解除する |
| `isLocked` | — | `boolean` | ✓ | ロック状態かを返す |

## 返値テーブル構造

### `getPhysics` (`PhysicsInfo`)

```lua
{
  slider    = { -- スライダー側のパラメーター },
  companion = { -- コンパニオン側のパラメーター },
}
```
