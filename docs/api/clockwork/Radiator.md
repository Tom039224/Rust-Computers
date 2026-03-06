# `Radiator` — Clockwork CC Compat

**モジュールパス / Module path**: `rc::clockwork::radiator::Radiator`

**ソースTOML**: `peripherals/clockwork/radiator.toml`


## 概要 / Overview

Clockwork CC Compat の `Radiator` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `Radiator` peripheral from Clockwork CC Compat.


## コンストラクタ / Constructor

```rust
use rc::clockwork::radiator::Radiator;
use rc::peripheral::Direction;

let p = Radiator::new(Direction::South);
```

## メソッド / Methods (14)

### `get_fan_type(&self) -> `String``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getFanType` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_fan_type().await?;
```

### `get_fan_r_p_m(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getFanRPM` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_fan_r_p_m().await?;
```

### `get_fan_count(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getFanCount` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_fan_count().await?;
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

### `is_cooling(&self) -> `bool``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `isCooling` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.is_cooling().await?;
```

### `is_heating(&self) -> `bool``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `isHeating` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.is_heating().await?;
```

### `get_target_temp(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getTargetTemp` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_target_temp().await?;
```

### `get_input_temperature(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getInputTemperature` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_input_temperature().await?;
```

### `get_output_temperature(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getOutputTemperature` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_output_temperature().await?;
```

### `get_thermal_factor(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getThermalFactor` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_thermal_factor().await?;
```

### `get_atmospheric_pressure(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getAtmosphericPressure` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_atmospheric_pressure().await?;
```

### `get_pressure_scale(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getPressureScale` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_pressure_scale().await?;
```

### `get_thermal_power(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getThermalPower` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_thermal_power().await?;
```

### `get_conversion_rate(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getConversionRate` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_conversion_rate().await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
