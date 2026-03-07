# GasPump

**mod**: Clockwork CC Compat  
**peripheral type**: `cw_gas_pump`  
**additional types**: `cw_gas_network`  
**source**: `GasPumpPeripheral.java`

## 概要

ガスポンプ。ガスを能動的にネットワーク内で圧送する。[GasNetwork 基底](./gas_network.md)のメソッドを継承する。

## メソッド一覧

### 固有メソッド

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getPumpPressure` | — | `number` | ✓ | ポンプが発生させている圧力を返す |
| `getSpeed` | — | `number` | ✓ | ポンプの動作速度 (RPM) を返す |
| `getFacing` | — | `string` | ✓ | ポンプの向きを返す (`"north"`, `"south"`, `"east"`, `"west"` 等) |

### 継承メソッド（GasNetwork 共通）

→ [gas_network.md](./gas_network.md) 参照（`getTemperature`, `getPressure`, `getHeatEnergy`, `getGasMass`, `getPosition`, `getNetworkInfo`）
