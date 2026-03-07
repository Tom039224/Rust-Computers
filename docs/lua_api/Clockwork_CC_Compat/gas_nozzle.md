# GasNozzle

**mod**: Clockwork CC Compat  
**peripheral type**: `cw_gas_nozzle`  
**additional types**: `cw_gas_network`  
**source**: `GasNozzlePeripheral.java`

## 概要

ガスノズル（バルーン制御）。気球への注入・排気を制御し、浮力/応力/ガス状態の詳細なモニタリングができる大型ペリフェラル。[GasNetwork 基底](./gas_network.md)のメソッドを継承する。

## メソッド一覧

### 固有メソッド

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `setPointer` | `value: number` | — | ✗ | ポインター値（注入率制御）を設定する |
| `getPointer` | — | `number` | ✓ | 現在のポインター値を返す |
| `getPointerSpeed` | — | `number` | ✓ | ポインターの変化速度を返す |
| `getPocketTemperature` | — | `number` | ✓ | 気球内部ポケットの温度 (K) を返す |
| `getDuctTemperature` | — | `number` | ✓ | 接続ダクトの温度 (K) を返す |
| `getTargetTemperature` | — | `number` | ✓ | 目標温度 (K) を返す |
| `getBalloonVolume` | — | `number` | ✓ | 気球の体積 (m³) を返す |
| `getBuoyancyForce` | — | `number` | ✗ | 現在の浮力 (N) を返す |
| `getLeaks` | — | `table[]` | ✓ | リーク箇所のリストを返す |
| `getBalloonPressure` | — | `number` | ✗ | 気球内圧力を返す |
| `getBalloonGasContents` | — | `{ [name]: mass }` | ✗ | 気球内ガス名→質量テーブルを返す |
| `getLossRate` | — | `number` | ✗ | ガス損失レート (kg/s) を返す |
| `getInflowRate` | — | `number` | ✗ | ガス流入レート (kg/s) を返す |
| `getMissingPositions` | — | `{ x, y, z }[]` | ✗ | 気球に必要だが欠けているブロック座標リストを返す |
| `getTotalGasMass` | — | `number` | ✗ | 気球内の全ガス質量 (kg) を返す |
| `getLeakIntegrity` | — | `number` | ✗ | リーク耐性（0〜1）を返す |
| `getMaxLeaks` | — | `number` | ✗ | 許容最大リーク数を返す |
| `getInternalDensity` | — | `number` | ✗ | 気球内部の密度を返す |
| `getTemperatureDelta` | — | `number` | ✓ | 気球内外の温度差 (K) を返す |
| `hasBalloon` | — | `boolean` | ✓ | 気球が接続されているかを返す |

### 継承メソッド（GasNetwork 共通）

→ [gas_network.md](./gas_network.md) 参照（`getTemperature`, `getPressure`, `getHeatEnergy`, `getGasMass`, `getPosition`, `getNetworkInfo`）

## 返値テーブル構造

### `getLeaks` 要素

```lua
{
  x = number,
  y = number,
  z = number,
  -- 漏洩位置の詳細情報（実装依存）
}
```
