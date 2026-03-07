# Boiler

**mod**: Clockwork CC Compat  
**peripheral type**: `Create_Boiler`  
**source**: `BoilerPeripheral.java`

## 概要

ボイラー。Create mod のボイラーマルチブロックを制御・監視する。GasNetwork 基底とは独立した実装であり、GasNetwork メソッドは**継承しない**。

## メソッド一覧

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `isActive` | — | `boolean` | ✓ | ボイラーが動作中かを返す |
| `getHeatLevel` | — | `number` | ✓ | 現在の熱レベルを返す |
| `getActiveHeat` | — | `number` | ✓ | 現在の有効熱量を返す |
| `isPassiveHeat` | — | `boolean` | ✓ | パッシブ加熱を使用しているかを返す |
| `getWaterSupply` | — | `number` | ✓ | 現在の水供給量を返す |
| `getAttachedEngines` | — | `number` | ✓ | 接続されているエンジン数を返す |
| `getAttachedWhistles` | — | `number` | ✓ | 接続されているホイッスル数を返す |
| `getEngineEfficiency` | — | `number` | ✓ | エンジンの効率を返す |
| `getBoilerSize` | — | `number` | ✓ | ボイラーのサイズ（スケール係数）を返す |
| `getWidth` | — | `number` | ✓ | ボイラーの幅（ブロック数）を返す |
| `getHeight` | — | `number` | ✓ | ボイラーの高さ（ブロック数）を返す |
| `getMaxHeatForSize` | — | `number` | ✓ | このサイズで到達できる最大熱量を返す |
| `getMaxHeatForWater` | — | `number` | ✓ | 現在の水量に対応する最大熱量を返す |
| `getFillState` | — | `number` | ✓ | 充填状態 (0.0〜1.0) を返す |
| `getFluidContents` | — | `FluidInfo` | ✓ | ボイラー内の流体情報を返す |
| `getControllerPos` | — | `{ x, y, z }` | ✓ | ボイラーコントローラーのワールド座標を返す |

## 返値テーブル構造

### `getFluidContents` (`FluidInfo`)

```lua
{
  fluid    = string,  -- 流体名（例: "minecraft:water"）
  amount   = number,  -- 現在量 (mB)
  capacity = number,  -- 最大容量 (mB)
}
```

### `getControllerPos`

```lua
{ x = number, y = number, z = number }
```
