# `EnvironmentDetector` — AdvancedPeripherals

**モジュールパス / Module path**: `rc::advanced_peripherals::environment_detector::EnvironmentDetector`

**ソースTOML**: `peripherals/advanced_peripherals/environment_detector.toml`


## 概要 / Overview

AdvancedPeripherals の `EnvironmentDetector` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `EnvironmentDetector` peripheral from AdvancedPeripherals.


## コンストラクタ / Constructor

```rust
use rc::advanced_peripherals::environment_detector::EnvironmentDetector;
use rc::peripheral::Direction;

let p = EnvironmentDetector::new(Direction::South);
```

## メソッド / Methods (16)

### `get_biome(&self) -> `String``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getBiome` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_biome().await?;
```

### `get_sky_light_level(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getSkyLightLevel` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_sky_light_level().await?;
```

### `get_block_light_level(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getBlockLightLevel` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_block_light_level().await?;
```

### `get_day_light_level(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getDayLightLevel` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_day_light_level().await?;
```

### `is_slime_chunk(&self) -> `bool``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `isSlimeChunk` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.is_slime_chunk().await?;
```

### `get_dimension(&self) -> `String``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getDimension` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_dimension().await?;
```

### `is_dimension(&self, dimension: &str) -> `bool``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `isDimension` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.is_dimension("" /* dimension */).await?;
```

### `get_moon_id(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getMoonId` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_moon_id().await?;
```

### `is_moon(&self, phase: i32) -> `bool``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `isMoon` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.is_moon(0 /* phase */).await?;
```

### `get_moon_name(&self) -> `String``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getMoonName` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_moon_name().await?;
```

### `is_raining(&self) -> `bool``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `isRaining` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.is_raining().await?;
```

### `is_thunder(&self) -> `bool``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `isThunder` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.is_thunder().await?;
```

### `is_sunny(&self) -> `bool``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `isSunny` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.is_sunny().await?;
```

### `scan_cost(&self, radius: i32) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `scanCost` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.scan_cost(0 /* radius */).await?;
```

### `can_sleep_here(&self) -> `bool``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `canSleepHere` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.can_sleep_here().await?;
```

### `can_sleep_player(&self, playername: &str) -> `bool``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `canSleepPlayer` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.can_sleep_player("" /* playername */).await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
