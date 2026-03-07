# CoalBurner

**mod**: Clockwork CC Compat  
**peripheral type**: `cw_coal_burner`  
**additional types**: `cw_gas_network`  
**source**: `CoalBurnerPeripheral.java`

## 概要

石炭バーナー。石炭等の固形燃料を燃焼させガスネットワークに熱を供給する。[GasNetwork 基底](./gas_network.md)のメソッドを継承する。

## メソッド一覧

### 固有メソッド

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getFuelTicks` | — | `number` | ✓ | 残り燃焼時間 (ticks) を返す |
| `getMaxBurnTime` | — | `number` | ✓ | 現在の燃料の最大燃焼時間 (ticks) を返す |
| `isBurning` | — | `boolean` | ✓ | 燃焼中かどうかを返す |

### 継承メソッド（GasNetwork 共通）

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getTemperature` | — | `number` | ✓ | ガスの温度 (K) |
| `getPressure` | — | `number` | ✓ | ガスの圧力 |
| `getHeatEnergy` | — | `number` | ✓ | ガスの熱エネルギー |
| `getGasMass` | — | `{ [name]: mass }` | ✓ | ガス名→質量テーブル |
| `getPosition` | — | `{ x, y, z }` | ✓ | ブロックのワールド座標 |
| `getNetworkInfo` | — | `table[]` | ✗ | ネットワーク詳細情報 |
