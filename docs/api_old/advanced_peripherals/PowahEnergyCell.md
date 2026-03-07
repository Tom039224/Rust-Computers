# `PowahEnergyCell` — AdvancedPeripherals

**モジュールパス / Module path**: `rc::advanced_peripherals::powah_energy_cell::PowahEnergyCell`

**ソースTOML**: `peripherals/advanced_peripherals/powah_energy_cell.toml`


## 概要 / Overview

AdvancedPeripherals の `PowahEnergyCell` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `PowahEnergyCell` peripheral from AdvancedPeripherals.


## コンストラクタ / Constructor

```rust
use rc::advanced_peripherals::powah_energy_cell::PowahEnergyCell;
use rc::peripheral::Direction;

let p = PowahEnergyCell::new(Direction::South);
```

## メソッド / Methods (2)

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


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
