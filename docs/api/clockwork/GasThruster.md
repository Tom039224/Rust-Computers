# `GasThruster` — Clockwork CC Compat

**モジュールパス / Module path**: `rc::clockwork::gas_thruster::GasThruster`

**ソースTOML**: `peripherals/clockwork/gas_thruster.toml`


## 概要 / Overview

Clockwork CC Compat の `GasThruster` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `GasThruster` peripheral from Clockwork CC Compat.


## コンストラクタ / Constructor

```rust
use rc::clockwork::gas_thruster::GasThruster;
use rc::peripheral::Direction;

let p = GasThruster::new(Direction::South);
```

## メソッド / Methods (3)

### `get_thrust(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getThrust` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_thrust().await?;
```

### `get_flow_rate(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getFlowRate` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_flow_rate().await?;
```

### `get_facing(&self) -> `String``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getFacing` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_facing().await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
