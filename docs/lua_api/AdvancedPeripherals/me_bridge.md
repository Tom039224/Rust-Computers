# MEBridge

**mod**: AdvancedPeripherals  
**peripheral type**: `me_bridge`  
**source**: `MEBridgePeripheral.java`

## 概要

ME ブリッジ。Applied Energistics 2 の ME ネットワークに接続し、アイテム・流体・ケミカルの読み取り・入出力・クラフト要求・ストレージ容量確認などを Lua から操作するペリフェラル。

## メソッド一覧

### アイテム操作

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `listItems` | — | `ItemEntry[]` | ✗ | ME ネットワーク内の全アイテム一覧を返す |
| `getItem` | `filter: table` | `ItemEntry` | ✗ | 条件に合う最初のアイテムを返す |
| `exportItem` | `filter: table`, `side: string` | `number` | ✗ | ME からアイテムを外部インベントリに出力する |
| `importItem` | `filter: table`, `side: string` | `number` | ✗ | 外部インベントリから ME にアイテムを取り込む |
| `exportItemToPeripheral` | `filter: table`, `targetName: string` | `number` | ✗ | 指定ペリフェラルへアイテムを出力する |
| `importItemFromPeripheral` | `filter: table`, `targetName: string` | `number` | ✗ | 指定ペリフェラルからアイテムを取り込む |

### 流体操作

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `listFluids` | — | `FluidEntry[]` | ✗ | ME ネットワーク内の全流体一覧を返す |
| `getFluid` | `filter: table` | `FluidEntry` | ✗ | 条件に合う最初の流体を返す |
| `exportFluid` | `filter: table`, `side: string` | `number` | ✗ | ME から流体を出力する |
| `importFluid` | `filter: table`, `side: string` | `number` | ✗ | 外部から ME に流体を取り込む |
| `exportFluidToPeripheral` | `filter: table`, `targetName: string` | `number` | ✗ | 指定ペリフェラルへ流体を出力する |
| `importFluidFromPeripheral` | `filter: table`, `targetName: string` | `number` | ✗ | 指定ペリフェラルから流体を取り込む |

### ケミカル操作（Mekanism 連携）

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `listChemicals` | — | `ChemicalEntry[]` | ✗ | ME ネットワーク内の全ケミカル一覧を返す |
| `getChemical` | `filter: table` | `ChemicalEntry` | ✗ | 条件に合う最初のケミカルを返す |
| `exportChemical` | `filter: table`, `side: string` | `number` | ✗ | ME からケミカルを出力する |
| `importChemical` | `filter: table`, `side: string` | `number` | ✗ | 外部から ME にケミカルを取り込む |
| `exportChemicalToPeripheral` | `filter: table`, `targetName: string` | `number` | ✗ | 指定ペリフェラルへケミカルを出力する |
| `importChemicalFromPeripheral` | `filter: table`, `targetName: string` | `number` | ✗ | 指定ペリフェラルからケミカルを取り込む |

### クラフト

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `craftItem` | `filter: table` | `boolean` | ✗ | 指定アイテムのクラフトを ME に要求する |
| `craftFluid` | `filter: table` | `boolean` | ✗ | 指定流体のクラフトを ME に要求する |
| `craftChemical` | `filter: table` | `boolean` | ✗ | 指定ケミカルのクラフトを ME に要求する |
| `isItemCrafting` | `filter: table` | `boolean` | ✗ | 指定アイテムのクラフトが進行中かを返す |
| `isFluidCrafting` | `filter: table` | `boolean` | ✗ | 指定流体のクラフトが進行中かを返す |

### ストレージ容量

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getEnergyStorage` | — | `number` | ✗ | ME ネットワークの蓄積エネルギーを返す |
| `getMaxEnergyStorage` | — | `number` | ✗ | ME ネットワークの最大エネルギー容量を返す |
| `getAvgPowerUsage` | — | `number` | ✗ | 平均電力使用量を返す |
| `getAvgPowerInjection` | — | `number` | ✗ | 平均電力注入量を返す |
| `getTotalItemStorage` | — | `number` | ✗ | アイテムストレージの総容量（バイト）を返す |
| `getUsedItemStorage` | — | `number` | ✗ | アイテムストレージの使用済み容量を返す |
| `getAvailableItemStorage` | — | `number` | ✗ | アイテムストレージの空き容量を返す |
| `getTotalFluidStorage` | — | `number` | ✗ | 流体ストレージの総容量を返す |
| `getUsedFluidStorage` | — | `number` | ✗ | 流体ストレージの使用済み容量を返す |
| `getAvailableFluidStorage` | — | `number` | ✗ | 流体ストレージの空き容量を返す |
| `getTotalChemicalStorage` | — | `number` | ✗ | ケミカルストレージの総容量を返す |
| `getUsedChemicalStorage` | — | `number` | ✗ | ケミカルストレージの使用済み容量を返す |
| `getAvailableChemicalStorage` | — | `number` | ✗ | ケミカルストレージの空き容量を返す |

## 返値テーブル構造

### `ItemEntry`

```lua
{
  name         = string,
  tags         = string[],
  count        = number,
  displayName  = string,
  maxStackSize = number,
  components   = table,
  fingerprint  = string,
}
```

### `FluidEntry`

```lua
{
  name        = string,
  tags        = string[],
  count       = number,
  displayName = string,
  fluidType   = { viscosity = number, density = number, ... },
  components  = table,
  fingerprint = string,
}
```

### `ChemicalEntry`

```lua
{
  name         = string,
  tags         = string[],
  isGaseous    = boolean,
  radioactivity= number,
  count        = number,
  displayName  = string,
  fingerprint  = string,
}
```
