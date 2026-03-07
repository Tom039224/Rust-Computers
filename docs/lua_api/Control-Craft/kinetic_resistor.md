# KineticResistor

**mod**: Control-Craft  
**peripheral type**: `resistor`  
**source**: `KineticResistorPeripheral.java`

## 概要

キネティックレジスター。Create mod のキネティックネットワークに対する抵抗比率を Lua から制御する。

## メソッド一覧

| メソッド名 | 引数 | 戻り値 | flags | imm | 説明 |
|---|---|---|---|---|---|
| `getRatio` | — | `number` | — | ✓ | 現在の抵抗比率を返す |
| `setRatio` | `ratio: number` | — | mainThread | ✗ | 抵抗比率を設定する（値域は実装依存） |
