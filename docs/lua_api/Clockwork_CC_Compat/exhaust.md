# Exhaust

**mod**: Clockwork CC Compat  
**peripheral type**: `cw_exhaust`  
**additional types**: `cw_gas_network`  
**source**: `ExhaustPeripheral.java`

## 概要

排気口。ガスネットワークから余剰ガスを排出する。[GasNetwork 基底](./gas_network.md)のメソッドを継承する。

## メソッド一覧

### 固有メソッド

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getFacing` | — | `string` | ✓ | 排気口の向きを返す (`"north"`, `"south"`, `"east"`, `"west"` 等) |

### 継承メソッド（GasNetwork 共通）

→ [gas_network.md](./gas_network.md) 参照（`getTemperature`, `getPressure`, `getHeatEnergy`, `getGasMass`, `getPosition`, `getNetworkInfo`）
