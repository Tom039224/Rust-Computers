# GasNetwork（基底クラス）

**mod**: Clockwork CC Compat  
**peripheral type**: `cw_gas_network`  
**source**: `GasNetworkPeripheral.java`

## 概要

全ガス系ペリフェラルの基底クラス。下記の全ペリフェラルはこのメソッドを継承する:
- [air_compressor](./air_compressor.md)
- [coal_burner](./coal_burner.md)
- [duct_tank](./duct_tank.md)
- [exhaust](./exhaust.md)
- [gas_engine](./gas_engine.md)
- [gas_nozzle](./gas_nozzle.md)
- [gas_pump](./gas_pump.md)
- [gas_thruster](./gas_thruster.md)
- [gas_valve](./gas_valve.md)
- [radiator](./radiator.md)
- [redstone_duct](./redstone_duct.md)

## 共通メソッド（全 GasNetwork ペリフェラルで利用可能）

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getTemperature` | — | `number` | ✓ | ガスネットワークの現在温度 (K) を返す |
| `getPressure` | — | `number` | ✓ | ガスネットワークの現在圧力を返す |
| `getHeatEnergy` | — | `number` | ✓ | ガスネットワークの熱エネルギーを返す |
| `getGasMass` | — | `{ [gasName: string]: number }` | ✓ | ガス名→質量 (kg) のテーブルを返す |
| `getPosition` | — | `{ x, y, z }` | ✓ | このブロックのワールド座標を返す |
| `getNetworkInfo` | — | `table[]` | ✗ | ガスネットワークの詳細情報リストを返す |

## `getGasMass` 返値例

```lua
{
  ["kelvin:air"]          = 5.2,
  ["vs_clockwork:steam"]  = 1.8,
}
```

## `getNetworkInfo` 返値

ガスネットワークの各ノードに関する複合テーブルのリスト。フィールドは接続ネットワークの構成によって異なる。
