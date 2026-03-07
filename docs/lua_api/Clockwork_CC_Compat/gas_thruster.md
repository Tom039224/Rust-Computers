# GasThruster

**mod**: Clockwork CC Compat  
**peripheral type**: `cw_gas_thruster`  
**additional types**: `cw_gas_network`  
**source**: `GasThrusterPeripheral.java`

## 概要

ガススラスター。ガスを噴射して推力を生成する。[GasNetwork 基底](./gas_network.md)のメソッドを継承する。

## メソッド一覧

### 固有メソッド

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getThrust` | — | `number` | ✓ | 現在の推力 (N) を返す |
| `getFlowRate` | — | `number` | ✓ | ガス流量 (kg/s) を返す |
| `getGasMassFlow` | — | `{ [name]: number }` | ✓ | ガス名→流量 (kg/s) のテーブルを返す |
| `getFacing` | — | `string` | ✓ | スラスターの噴射向きを返す (`"north"`, `"south"`, `"east"`, `"west"` 等) |

### 継承メソッド（GasNetwork 共通）

→ [gas_network.md](./gas_network.md) 参照（`getTemperature`, `getPressure`, `getHeatEnergy`, `getGasMass`, `getPosition`, `getNetworkInfo`）

## 返値テーブル構造

### `getGasMassFlow` 例

```lua
{
  ["kelvin:air"]         = 0.42,
  ["vs_clockwork:steam"] = 0.08,
}
```
