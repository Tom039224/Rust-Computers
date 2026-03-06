# `PowahThermo` — AdvancedPeripherals

**モジュールパス / Module path**: `rc::advanced_peripherals::powah_thermo::PowahThermo`

**ソースTOML**: `peripherals/advanced_peripherals/powah_thermo.toml`


## 概要 / Overview

AdvancedPeripherals の `PowahThermo` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `PowahThermo` peripheral from AdvancedPeripherals.


## コンストラクタ / Constructor

```rust
use rc::advanced_peripherals::powah_thermo::PowahThermo;
use rc::peripheral::Direction;

let p = PowahThermo::new(Direction::South);
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
