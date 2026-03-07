# AirCompressor

**mod**: Clockwork CC Compat  
**peripheral type**: `cw_air_compressor`  
**additional types**: `cw_gas_network`  
**source**: `AirCompressorPeripheral.java`

## 概要

空気圧縮機。ガスネットワークに空気を圧縮供給する。[GasNetwork 基底](./gas_network.md)のメソッドを継承する。

## メソッド一覧

### 固有メソッド

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getStatus` | — | `string` | ✓ | 圧縮機の動作状態を返す |
| `getSpeed` | — | `number` | ✓ | 圧縮機の動作速度 (RPM) を返す |
| `getFacing` | — | `string` | ✓ | 圧縮機の向きを返す (`"north"`, `"south"`, `"east"`, `"west"` 等) |

### 継承メソッド（GasNetwork 共通）

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getTemperature` | — | `number` | ✓ | ガスの温度 (K) |
| `getPressure` | — | `number` | ✓ | ガスの圧力 |
| `getHeatEnergy` | — | `number` | ✓ | ガスの熱エネルギー |
| `getGasMass` | — | `{ [name]: mass }` | ✓ | ガス名→質量テーブル |
| `getPosition` | — | `{ x, y, z }` | ✓ | ブロックのワールド座標 |
| `getNetworkInfo` | — | `table[]` | ✗ | ネットワーク詳細情報 |
