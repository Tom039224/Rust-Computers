# `Boiler` — Clockwork CC Compat

**モジュールパス / Module path**: `rc::clockwork::boiler::Boiler`

**ソースTOML**: `peripherals/clockwork/boiler.toml`


## 概要 / Overview

Clockwork CC Compat の `Boiler` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `Boiler` peripheral from Clockwork CC Compat.


## コンストラクタ / Constructor

```rust
use rc::clockwork::boiler::Boiler;
use rc::peripheral::Direction;

let p = Boiler::new(Direction::South);
```

## メソッド / Methods (14)

### `is_active(&self) -> `bool``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `isActive` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.is_active().await?;
```

### `get_heat_level(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getHeatLevel` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_heat_level().await?;
```

### `get_active_heat(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getActiveHeat` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_active_heat().await?;
```

### `is_passive_heat(&self) -> `bool``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `isPassiveHeat` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.is_passive_heat().await?;
```

### `get_water_supply(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getWaterSupply` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_water_supply().await?;
```

### `get_attached_engines(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getAttachedEngines` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_attached_engines().await?;
```

### `get_attached_whistles(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getAttachedWhistles` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_attached_whistles().await?;
```

### `get_engine_efficiency(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getEngineEfficiency` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_engine_efficiency().await?;
```

### `get_boiler_size(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getBoilerSize` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_boiler_size().await?;
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

### `get_height(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getHeight` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_height().await?;
```

### `get_max_heat_for_size(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getMaxHeatForSize` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_max_heat_for_size().await?;
```

### `get_max_heat_for_water(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getMaxHeatForWater` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_max_heat_for_water().await?;
```

### `get_fill_state(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getFillState` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_fill_state().await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
