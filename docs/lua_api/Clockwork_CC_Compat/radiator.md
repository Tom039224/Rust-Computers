# Radiator

**mod**: Clockwork CC Compat  
**peripheral type**: `cw_radiator` / `cw_gas_duct`  
**additional types**: `cw_gas_network`  
**source**: `RadiatorPeripheral.java`

## 概要

ラジエーター（ガスダクト兼用）。ガスの熱交換・冷却・加熱を行う。`cw_radiator` と `cw_gas_duct` の両方の type で利用可能。[GasNetwork 基底](./gas_network.md)のメソッドを継承する。

## メソッド一覧

### 固有メソッド

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getFanType` | — | `string` | ✓ | ファンの種類を返す |
| `getFanRPM` | — | `number` | ✓ | ファンの回転数 (RPM) を返す |
| `getFanCount` | — | `number` | ✓ | 接続ファン数を返す |
| `getFans` | — | `FanInfo[]` | ✓ | 各ファンの詳細情報リストを返す |
| `isActive` | — | `boolean` | ✓ | ラジエーターが動作中かを返す |
| `isCooling` | — | `boolean` | ✓ | 冷却中かを返す |
| `isHeating` | — | `boolean` | ✓ | 加熱中かを返す |
| `getTargetTemp` | — | `number` | ✓ | 目標温度 (K) を返す |
| `getInputTemperature` | — | `number` | ✓ | 入力ガスの温度 (K) を返す |
| `getOutputTemperature` | — | `number` | ✓ | 出力ガスの温度 (K) を返す |
| `getThermalFactor` | — | `number` | ✓ | 熱交換係数を返す |
| `getAtmosphericPressure` | — | `number` | ✓ | 大気圧を返す |
| `getPressureScale` | — | `number` | ✓ | 圧力スケール係数を返す |
| `getThermalPower` | — | `number` | ✓ | 熱輸送電力 (W) を返す |
| `getStatus` | — | `string` | ✓ | 動作ステータス文字列を返す |
| `getConversionRate` | — | `number` | ✓ | ガス変換レートを返す |
| `getConversions` | — | `ConversionInfo[]` | ✓ | ガス変換情報のリストを返す |

### 継承メソッド（GasNetwork 共通）

→ [gas_network.md](./gas_network.md) 参照（`getTemperature`, `getPressure`, `getHeatEnergy`, `getGasMass`, `getPosition`, `getNetworkInfo`）

## 返値テーブル構造

### `getFans` 要素 (`FanInfo`)

```lua
{
  type      = string,  -- ファン種類
  rpm       = number,  -- 回転数
  dir       = string,  -- 回転方向
  dist      = number,  -- 距離
}
```

### `getConversions` 要素 (`ConversionInfo`)

```lua
{
  from   = string,  -- 変換元ガス名
  to     = string,  -- 変換先ガス名
  amount = number,  -- 変換量 (kg/s)
}
```
