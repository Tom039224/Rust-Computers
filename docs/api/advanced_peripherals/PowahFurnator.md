# `PowahFurnator` — AdvancedPeripherals

**モジュールパス / Module path**: `rc::advanced_peripherals::powah_furnator::PowahFurnator`

**ソースTOML**: `peripherals/advanced_peripherals/powah_furnator.toml`


## 概要 / Overview

AdvancedPeripherals の `PowahFurnator` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `PowahFurnator` peripheral from AdvancedPeripherals.


## コンストラクタ / Constructor

```rust
use rc::advanced_peripherals::powah_furnator::PowahFurnator;
use rc::peripheral::Direction;

let p = PowahFurnator::new(Direction::South);
```

## メソッド / Methods (4)

### `is_burning(&self) -> `bool``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `isBurning` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.is_burning().await?;
```

### `get_stored_energy(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getStoredEnergy` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_stored_energy().await?;
```

### `get_max_energy(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getMaxEnergy` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_max_energy().await?;
```

### `get_carbon(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getCarbon` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_carbon().await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
