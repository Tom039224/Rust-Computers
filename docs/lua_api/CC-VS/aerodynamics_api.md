# AerodynamicsAPI

**mod**: CC:Valkyrien Skies (CC-VS)  
**peripheral type**: グローバル API (`aerodynamics.*` / `aero.*`)  
**source**: `AerodynamicsAPI.kt`

## 概要

CC:Tweaked グローバル API。`aerodynamics.xxx()` または `aero.xxx()` の形で呼び出す。  
大気モデルの定数取得および現在の高度での大気パラメータを取得できる。

## メソッド一覧

### プロパティ系（引数なし、定数値を返す）

| メソッド名 | 戻り値 | imm | 説明 |
|---|---|---|---|
| `defaultMax` | `number` | ✓ | `AerodynamicUtils.DEFAULT_MAX`（大気モデルの最大高度定数） |
| `defaultSeaLevel` | `number` | ✓ | `AerodynamicUtils.DEFAULT_SEA_LEVEL`（デフォルト海面高度） |
| `dragCoefficient` | `number` | ✓ | サーバー設定の `dragCoefficient` 値 |
| `gravitationalAcceleration` | `number` | ✓ | 重力加速度定数 |
| `universalGasConstant` | `number` | ✓ | 気体定数 R |
| `airMolarMass` | `number` | ✓ | 空気のモル質量 |

### 関数メソッド

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getAtmosphericParameters` | — | `{maxY, seaLevel, gravity}` または `nil` | ✓ | 現在ディメンションの大気パラメータを返す（未初期化時は `nil`） |
| `getAirDensity` | `y?: number` | `number \| nil` | ✓ | 指定 Y 座標の空気密度を返す（省略時はコンピューターの Y 座標） |
| `getAirPressure` | `y?: number` | `number \| nil` | ✓ | 指定 Y 座標の大気圧を返す |
| `getAirTemperature` | `y?: number` | `number \| nil` | ✓ | 指定 Y 座標の気温を返す |

## `getAtmosphericParameters` 返値

```lua
{
  maxY:      number,  -- 大気の最大高度
  seaLevel:  number,  -- 海面高度
  gravity:   number,  -- 重力加速度
}
```
