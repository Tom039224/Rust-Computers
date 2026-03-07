# ColonyIntegrator

**mod**: AdvancedPeripherals  
**peripheral type**: `colony_integrator`  
**source**: `ColonyIntegratorPeripheral.java`

## 概要

コロニーインテグレーター。MineColonies mod のコロニー情報を Lua から参照するペリフェラル。建物・市民・ワークオーダー・レクエスト等を取得できる。

## メソッド一覧

### コロニー情報

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `isInColony` | — | `boolean` | ✗ | このペリフェラルがコロニー内にあるかを返す |
| `getColonyID` | — | `number` | ✗ | コロニー ID を返す |
| `getColonyName` | — | `string` | ✗ | コロニー名を返す |
| `getColonyStyle` | — | `string` | ✗ | コロニーのスタイル（建築セット名）を返す |
| `isActive` | — | `boolean` | ✗ | コロニーがアクティブかを返す |
| `getAmountOfCitizens` | — | `number` | ✗ | 現在の市民数を返す |
| `getMaxCitizens` | — | `number` | ✗ | 最大市民数を返す |
| `getHappiness` | — | `number` | ✗ | コロニーの幸福度を返す |
| `getPosition` | — | `{ x, y, z }` | ✗ | コロニーのタウンホール座標を返す |

### 市民

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getCitizens` | — | `CitizenInfo[]` | ✗ | 全市民の情報リストを返す |
| `getCitizenInfo` | `id: number` | `CitizenInfo` | ✗ | 指定 ID の市民の情報を返す |

### 建物

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getBuildings` | — | `BuildingInfo[]` | ✗ | 全建物の情報リストを返す |
| `getBuildingInfo` | `x: number`, `y: number`, `z: number` | `BuildingInfo` | ✗ | 指定座標の建物情報を返す |

### ワークオーダー・リクエスト

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getWorkOrders` | — | `WorkOrderInfo[]` | ✗ | 現在のワークオーダー（建設・修理依頼）一覧を返す |
| `getRequests` | — | `RequestInfo[]` | ✗ | コロニー内の未解決リクエスト一覧を返す |
| `getBuilderResources` | `x, y, z: number` | `ResourceInfo[]` | ✗ | 指定ビルダーに必要なリソース一覧を返す |

### 攻撃・侵入

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `isUnderAttack` | — | `boolean` | ✗ | コロニーが攻撃を受けているかを返す |

## 返値テーブル構造

### `CitizenInfo`

```lua
{
  id       = number,
  name     = string,
  job      = string,
  level    = number,
  health   = number,
  maxHealth= number,
  happiness= number,
  x        = number,
  y        = number,
  z        = number,
  bedPos   = { x, y, z },
}
```

### `BuildingInfo`

```lua
{
  type     = string,   -- 建物種別
  location = { x, y, z },
  level    = number,
  maxLevel = number,
  style    = string,
}
```

### `WorkOrderInfo`

```lua
{
  id       = number,
  type     = string,
  builder  = { x, y, z },
  location = { x, y, z },
  priority = number,
}
```
