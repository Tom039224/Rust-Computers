# GasEngine

**mod**: Clockwork CC Compat  
**peripheral type**: `cw_gas_engine`  
**additional types**: `cw_gas_network`  
**source**: `GasEnginePeripheral.java`

## 概要

ガスエンジン。ガスを消費して運動エネルギーに変換する。[GasNetwork 基底](./gas_network.md)のメソッドを継承する。

## メソッド一覧

### 固有メソッド

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getAttachedEngines` | — | `number` | ✓ | 接続されているエンジン数を返す |
| `getTotalEfficiency` | — | `number` | ✓ | 全接続エンジンの合計効率を返す |

### 継承メソッド（GasNetwork 共通）

→ [gas_network.md](./gas_network.md) 参照（`getTemperature`, `getPressure`, `getHeatEnergy`, `getGasMass`, `getPosition`, `getNetworkInfo`）
