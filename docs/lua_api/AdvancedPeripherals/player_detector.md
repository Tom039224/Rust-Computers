# PlayerDetector

**mod**: AdvancedPeripherals  
**peripheral type**: `player_detector`  
**source**: `PlayerDetectorPeripheral.java`

## 概要

プレイヤー検出器。オンラインプレイヤーの検索・位置検出・詳細情報取得が可能なペリフェラル。

## メソッド一覧

### プレイヤーリスト

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getOnlinePlayers` | — | `string[]` | ✗ | オンラインプレイヤー名の一覧を返す |

### 座標範囲検索

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getPlayersInRange` | `radius: number` | `string[]` | ✗ | 指定半径内のプレイヤー名を返す |
| `getPlayersInCoords` | `x1, y1, z1, x2, y2, z2: number` | `string[]` | ✗ | 指定座標範囲内のプレイヤー名を返す |
| `getPlayersInCubic` | `dx, dy, dz: number` | `string[]` | ✗ | 指定の立方体範囲内のプレイヤー名を返す |
| `isPlayersInRange` | `radius: number` | `boolean` | ✗ | 指定半径内にプレイヤーがいるかを返す |
| `isPlayersInCoords` | `x1, y1, z1, x2, y2, z2: number` | `boolean` | ✗ | 指定座標範囲内にプレイヤーがいるかを返す |
| `isPlayersInCubic` | `dx, dy, dz: number` | `boolean` | ✗ | 指定立方体範囲内にプレイヤーがいるかを返す |
| `isPlayerInRange` | `player: string`, `radius: number` | `boolean` | ✗ | 特定プレイヤーが半径内にいるかを返す |
| `isPlayerInCoords` | `player: string`, `x1, y1, z1, x2, y2, z2: number` | `boolean` | ✗ | 特定プレイヤーが座標範囲内にいるかを返す |
| `isPlayerInCubic` | `player: string`, `dx, dy, dz: number` | `boolean` | ✗ | 特定プレイヤーが立方体範囲内にいるかを返す |

### プレイヤー詳細

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getPlayerPos` | `player: string`, `decimals?: number` | `PlayerInfo` | ✗ | プレイヤーの座標（省略可: 小数点桁数）を返す |
| `getPlayer` | `player: string`, `decimals?: number` | `PlayerInfo` | ✗ | プレイヤーの詳細情報を返す |

## 返値テーブル構造

### `getPlayer` / `getPlayerPos` (`PlayerInfo`)

```lua
{
  x            = number,
  y            = number,
  z            = number,
  -- getPlayer の追加フィールド:
  name         = string,
  uuid         = string,
  health       = number,
  maxHealth    = number,
  isFlying     = boolean,
  isSprinting  = boolean,
  isSneaking   = boolean,
  gameMode     = string,  -- "survival", "creative", etc.
  experience   = number,
  level        = number,
  pitch        = number,
  yaw          = number,
  dimension    = string,
}
```
