# `ModularAccumulator` — Create Additions

**モジュールパス / Module path**: `rc::createaddition::modular_accumulator::ModularAccumulator`

**ソースTOML**: `peripherals/createaddition/modular_accumulator.toml`


## 概要 / Overview

Create Additions の `ModularAccumulator` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `ModularAccumulator` peripheral from Create Additions.


## コンストラクタ / Constructor

```rust
use rc::createaddition::modular_accumulator::ModularAccumulator;
use rc::peripheral::Direction;

let p = ModularAccumulator::new(Direction::South);
```

## メソッド / Methods (7)

### `get_energy(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getEnergy` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_energy().await?;
```

### `get_capacity(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getCapacity` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_capacity().await?;
```

### `get_percent(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getPercent` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_percent().await?;
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

### `get_height(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getHeight` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_height().await?;
```

### `get_width(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getWidth` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_width().await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
