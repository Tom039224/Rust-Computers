# `GasValve` — Clockwork CC Compat

**モジュールパス / Module path**: `rc::clockwork::gas_valve::GasValve`

**ソースTOML**: `peripherals/clockwork/gas_valve.toml`


## 概要 / Overview

Clockwork CC Compat の `GasValve` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `GasValve` peripheral from Clockwork CC Compat.


## コンストラクタ / Constructor

```rust
use rc::clockwork::gas_valve::GasValve;
use rc::peripheral::Direction;

let p = GasValve::new(Direction::South);
```

## メソッド / Methods (2)

### `get_aperture(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getAperture` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_aperture().await?;
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
