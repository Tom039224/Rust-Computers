# `GasNetwork` — Clockwork CC Compat

**モジュールパス / Module path**: `rc::clockwork::gas_network::GasNetwork`

**ソースTOML**: `peripherals/clockwork/gas_network.toml`


## 概要 / Overview

Clockwork CC Compat の `GasNetwork` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `GasNetwork` peripheral from Clockwork CC Compat.


## コンストラクタ / Constructor

```rust
use rc::clockwork::gas_network::GasNetwork;
use rc::peripheral::Direction;

let p = GasNetwork::new(Direction::South);
```

## メソッド / Methods (3)

### `get_temperature(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getTemperature` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_temperature().await?;
```

### `get_pressure(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getPressure` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_pressure().await?;
```

### `get_heat_energy(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getHeatEnergy` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_heat_energy().await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
