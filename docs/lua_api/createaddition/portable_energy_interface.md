# PortableEnergyInterface

**mod**: createaddition  
**peripheral type**: `portable_energy_interface`  
**source**: `PortableEnergyInterfacePeripheral.java`

## 概要

ポータブルエネルギーインターフェース。Create mod のコントラプション（組み立て体）上の機械と外部 FE ネットワーク間でエネルギーの送受信を行う。

## メソッド一覧

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getEnergy` | — | `number` | ✗ | インターフェースのバッファ内エネルギー (FE) を返す |
| `getCapacity` | — | `number` | ✗ | バッファの最大容量 (FE) を返す |
| `isConnected` | — | `boolean` | ✗ | コントラプションに接続されているかを返す |
| `getMaxInsert` | — | `number` | ✗ | 1 tick あたりの最大入力 (FE/t) を返す |
| `getMaxExtract` | — | `number` | ✗ | 1 tick あたりの最大出力 (FE/t) を返す |
