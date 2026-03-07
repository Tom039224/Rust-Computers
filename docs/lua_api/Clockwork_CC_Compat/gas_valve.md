# GasValve

**mod**: Clockwork CC Compat  
**peripheral type**: `cw_gas_valve`  
**additional types**: `cw_gas_network`  
**source**: `GasValvePeripheral.java`

## 概要

ガスバルブ。ガスネットワーク間のガス流量を調整する。[GasNetwork 基底](./gas_network.md)のメソッドを継承する。

## メソッド一覧

### 固有メソッド

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getAperture` | — | `number` | ✓ | バルブの開度 (0.0〜1.0) を返す |
| `getFacing` | — | `string` | ✓ | バルブの向きを返す (`"north"`, `"south"`, `"east"`, `"west"` 等) |

### 継承メソッド（GasNetwork 共通）

→ [gas_network.md](./gas_network.md) 参照（`getTemperature`, `getPressure`, `getHeatEnergy`, `getGasMass`, `getPosition`, `getNetworkInfo`）
