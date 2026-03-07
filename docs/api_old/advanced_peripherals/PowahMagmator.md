# `PowahMagmator` — AdvancedPeripherals

**モジュールパス / Module path**: `rc::advanced_peripherals::powah_magmator::PowahMagmator`

**ソースTOML**: `peripherals/advanced_peripherals/powah_magmator.toml`


## 概要 / Overview

AdvancedPeripherals の `PowahMagmator` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `PowahMagmator` peripheral from AdvancedPeripherals.


## コンストラクタ / Constructor

```rust
use rc::advanced_peripherals::powah_magmator::PowahMagmator;
use rc::peripheral::Direction;

let p = PowahMagmator::new(Direction::South);
```

## メソッド / Methods (3)

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
