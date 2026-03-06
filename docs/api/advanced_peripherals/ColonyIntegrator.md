# `ColonyIntegrator` — AdvancedPeripherals

**モジュールパス / Module path**: `rc::advanced_peripherals::colony_integrator::ColonyIntegrator`

**ソースTOML**: `peripherals/advanced_peripherals/colony_integrator.toml`


## 概要 / Overview

AdvancedPeripherals の `ColonyIntegrator` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `ColonyIntegrator` peripheral from AdvancedPeripherals.


## コンストラクタ / Constructor

```rust
use rc::advanced_peripherals::colony_integrator::ColonyIntegrator;
use rc::peripheral::Direction;

let p = ColonyIntegrator::new(Direction::South);
```

## メソッド / Methods (12)

### `is_in_colony(&self) -> `bool``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `isInColony` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.is_in_colony().await?;
```

### `amount_of_construction_sites(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `amountOfConstructionSites` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.amount_of_construction_sites().await?;
```

### `get_colony_i_d(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getColonyID` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_colony_i_d().await?;
```

### `get_colony_name(&self) -> `String``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getColonyName` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_colony_name().await?;
```

### `get_colony_style(&self) -> `String``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getColonyStyle` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_colony_style().await?;
```

### `is_active(&self) -> `bool``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `isActive` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.is_active().await?;
```

### `get_happiness(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getHappiness` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_happiness().await?;
```

### `is_under_attack(&self) -> `bool``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `isUnderAttack` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.is_under_attack().await?;
```

### `is_under_raid(&self) -> `bool``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `isUnderRaid` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.is_under_raid().await?;
```

### `amount_of_citizens(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `amountOfCitizens` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.amount_of_citizens().await?;
```

### `max_of_citizens(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `maxOfCitizens` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.max_of_citizens().await?;
```

### `amount_of_graves(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `amountOfGraves` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.amount_of_graves().await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
