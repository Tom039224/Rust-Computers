# `GasPump` — Clockwork CC Compat

**モジュールパス / Module path**: `rc::clockwork::gas_pump::GasPump`

**ソースTOML**: `peripherals/clockwork/gas_pump.toml`


## 概要 / Overview

Clockwork CC Compat の `GasPump` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `GasPump` peripheral from Clockwork CC Compat.


## コンストラクタ / Constructor

```rust
use rc::clockwork::gas_pump::GasPump;
use rc::peripheral::Direction;

let p = GasPump::new(Direction::South);
```

## メソッド / Methods (3)

### `get_pump_pressure(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getPumpPressure` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_pump_pressure().await?;
```

### `get_speed(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getSpeed` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_speed().await?;
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
