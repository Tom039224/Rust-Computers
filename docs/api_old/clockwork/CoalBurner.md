# `CoalBurner` — Clockwork CC Compat

**モジュールパス / Module path**: `rc::clockwork::coal_burner::CoalBurner`

**ソースTOML**: `peripherals/clockwork/coal_burner.toml`


## 概要 / Overview

Clockwork CC Compat の `CoalBurner` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `CoalBurner` peripheral from Clockwork CC Compat.


## コンストラクタ / Constructor

```rust
use rc::clockwork::coal_burner::CoalBurner;
use rc::peripheral::Direction;

let p = CoalBurner::new(Direction::South);
```

## メソッド / Methods (3)

### `get_fuel_ticks(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getFuelTicks` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_fuel_ticks().await?;
```

### `get_max_burn_time(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getMaxBurnTime` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_max_burn_time().await?;
```

### `is_burning(&self) -> `bool``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `isBurning` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.is_burning().await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
