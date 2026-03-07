# DuctTank

**mod**: Clockwork CC Compat  
**peripheral type**: `cw_duct_tank`  
**additional types**: `cw_gas_network`  
**source**: `DuctTankPeripheral.java`

## 概要

ガスダクトタンク。ガスを貯蔵するタンク状ブロック。[GasNetwork 基底](./gas_network.md)のメソッドを継承する。

## メソッド一覧

### 固有メソッド

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getHeight` | — | `number` | ✓ | タンクの高さ（ブロック数）を返す |
| `getWidth` | — | `number` | ✓ | タンクの幅（ブロック数）を返す |

### 継承メソッド（GasNetwork 共通）

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getTemperature` | — | `number` | ✓ | ガスの温度 (K) |
| `getPressure` | — | `number` | ✓ | ガスの圧力 |
| `getHeatEnergy` | — | `number` | ✓ | ガスの熱エネルギー |
| `getGasMass` | — | `{ [name]: mass }` | ✓ | ガス名→質量テーブル |
| `getPosition` | — | `{ x, y, z }` | ✓ | ブロックのワールド座標 |
| `getNetworkInfo` | — | `table[]` | ✗ | ネットワーク詳細情報 |
