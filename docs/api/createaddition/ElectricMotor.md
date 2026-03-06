# `ElectricMotor` — Create Additions

**モジュールパス / Module path**: `rc::createaddition::electric_motor::ElectricMotor`

**ソースTOML**: `peripherals/createaddition/electric_motor.toml`


## 概要 / Overview

Create Additions の `ElectricMotor` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `ElectricMotor` peripheral from Create Additions.


## コンストラクタ / Constructor

```rust
use rc::createaddition::electric_motor::ElectricMotor;
use rc::peripheral::Direction;

let p = ElectricMotor::new(Direction::South);
```

## メソッド / Methods (8)

### `get_type(&self) -> `String``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getType` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_type().await?;
```

### `set_speed(&self, speed: f64) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `setSpeed` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.set_speed(0.0 /* speed */).await?;
```

### `stop(&self) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `stop` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.stop().await?;
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

### `get_stress_capacity(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getStressCapacity` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_stress_capacity().await?;
```

### `get_energy_consumption(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getEnergyConsumption` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_energy_consumption().await?;
```

### `get_max_insert(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getMaxInsert` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_max_insert().await?;
```

### `get_max_extract(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getMaxExtract` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_max_extract().await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
