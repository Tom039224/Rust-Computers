# RSBridge

**mod**: AdvancedPeripherals  
**peripheral type**: `rs_bridge`  
**source**: `RSBridgePeripheral.java`

## 概要

RS ブリッジ。Refined Storage ネットワークに接続し、アイテム・流体・ケミカルの CRUD 操作およびクラフト要求を行うペリフェラル。メソッドセットは `me_bridge` とほぼ同一。

## メソッド一覧

[MEBridge のメソッド一覧](./me_bridge.md) と同一のインターフェースを持つ。

| カテゴリ | メソッド群 |
|---|---|
| アイテム操作 | `listItems`, `getItem`, `exportItem`, `importItem`, `exportItemToPeripheral`, `importItemFromPeripheral` |
| 流体操作 | `listFluids`, `getFluid`, `exportFluid`, `importFluid`, `exportFluidToPeripheral`, `importFluidFromPeripheral` |
| ケミカル操作 | `listChemicals`, `getChemical`, `exportChemical`, `importChemical`, `exportChemicalToPeripheral`, `importChemicalFromPeripheral` |
| クラフト | `craftItem`, `craftFluid`, `craftChemical`, `isItemCrafting`, `isFluidCrafting` |
| ストレージ容量 | `getEnergyStorage`, `getMaxEnergyStorage`, `getAvgPowerUsage`, `getAvgPowerInjection`, `getTotalItemStorage`, `getUsedItemStorage`, `getAvailableItemStorage`, `getTotalFluidStorage`, `getUsedFluidStorage`, `getAvailableFluidStorage`, `getTotalChemicalStorage`, `getUsedChemicalStorage`, `getAvailableChemicalStorage` |

## 注意点・RS 固有の差異

| メソッド | 挙動 |
|---|---|
| `getAverageEnergyInput` | Refined Storage は未対応のため常に `0` を返す |

## 返値テーブル構造

[me_bridge.md の返値テーブル構造](./me_bridge.md#返値テーブル構造)と同一。
