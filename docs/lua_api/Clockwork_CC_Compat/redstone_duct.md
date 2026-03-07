# RedstoneDuct

**mod**: Clockwork CC Compat  
**peripheral type**: `cw_redstone_duct`  
**additional types**: `cw_gas_network`  
**source**: `RedstoneDuctPeripheral.java`

## 概要

レッドストーンダクト。レッドストーン信号に基づいてガスフローを制御する。[GasNetwork 基底](./gas_network.md)のメソッドを継承する。

## メソッド一覧

### 固有メソッド

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getPower` | — | `number` | ✓ | 受信しているレッドストーン信号強度 (0〜15) を返す |
| `getConditional` | — | `ConditionalInfo` | ✓ | ダクトの条件設定テーブルを返す |

### 継承メソッド（GasNetwork 共通）

→ [gas_network.md](./gas_network.md) 参照（`getTemperature`, `getPressure`, `getHeatEnergy`, `getGasMass`, `getPosition`, `getNetworkInfo`）

## 返値テーブル構造

### `getConditional` (`ConditionalInfo`)

```lua
{
  moreThan         = boolean,  -- true: 比較値より大 で開弁, false: 小で開弁
  comparisonValue  = number,   -- 比較に使用する値
  filterBlacklist  = boolean,  -- true: ブラックリストフィルター
}
```
