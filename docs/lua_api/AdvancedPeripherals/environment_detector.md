# EnvironmentDetector

**mod**: AdvancedPeripherals  
**peripheral type**: `environment_detector`  
**source**: `EnvironmentDetectorPeripheral.java`

## 概要

環境検出器。バイオーム・光レベル・時間・天候・月相・スライムチャンク・ディメンション・近隣エンティティスキャンなど環境情報を総合的に取得するペリフェラル。

## メソッド一覧

### 環境・天候

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getBiome` | — | `string` | ✗ | バイオーム ID を返す（例: `"minecraft:plains"`） |
| `getDimension` | — | `string` | ✗ | ディメンション ID を返す（例: `"minecraft:overworld"`） |
| `isDimension` | `dim: string` | `boolean` | ✗ | 指定ディメンションにいるかを返す |
| `listDimensions` | — | `string[]` | ✗ | 全ディメンション ID のリストを返す |
| `isRaining` | — | `boolean` | ✗ | 雨が降っているかを返す |
| `isThunder` | — | `boolean` | ✗ | 雷雨かを返す |
| `isSunny` | — | `boolean` | ✗ | 晴れかを返す |

### 光・時間

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getSkyLightLevel` | — | `number` | ✗ | 現在の空のライトレベル (0〜15) を返す |
| `getBlockLightLevel` | — | `number` | ✗ | 現在のブロックライトレベル (0〜15) を返す |
| `getDayLightLevel` | — | `number` | ✗ | 昼光レベル (0〜15) を返す |
| `getTime` | — | `number` | ✗ | ワールド時間 (ticks) を返す |

### 月相

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getMoonId` | — | `number` | ✗ | 現在の月相 ID (0〜7) を返す |
| `getMoonName` | — | `string` | ✗ | 月相名（例: `"Full Moon"`）を返す |
| `isMoon` | `phase: string` | `boolean` | ✗ | 指定月相かどうかを返す |

### 地形

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `isSlimeChunk` | — | `boolean` | ✗ | スライムチャンクかどうかを返す |

### 睡眠

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `canSleepHere` | — | `boolean` | ✗ | このディメンションでスリープ可能かを返す |
| `canSleepPlayer` | `name: string` | `boolean` | ✗ | 指定プレイヤーがスリープ可能かを返す |

### エンティティスキャン

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `scanEntities` | `radius: number` | `EntityInfo[]` | ✗ | 指定半径内のエンティティ一覧を返す |
| `scanCost` | `radius: number` | `number` | ✓ | `scanEntities` の演算コストを返す |

## 返値テーブル構造

### `scanEntities` 要素 (`EntityInfo`)

```lua
{
  id              = string,   -- エンティティ ID
  uuid            = string,   -- UUID
  name            = string,   -- 名前
  tags            = string[], -- タグリスト
  canFreeze       = boolean,
  isGlowing       = boolean,
  isInWall        = boolean,
  health          = number,
  maxHealth       = number,
  lastDamageSource= string,
  x               = number,
  y               = number,
  z               = number,
}
```
