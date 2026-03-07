# `GasNozzle` — Clockwork CC Compat

**モジュールパス / Module path**: `rc::clockwork::gas_nozzle::GasNozzle`

**ソースTOML**: `peripherals/clockwork/gas_nozzle.toml`


## 概要 / Overview

Clockwork CC Compat の `GasNozzle` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `GasNozzle` peripheral from Clockwork CC Compat.


## コンストラクタ / Constructor

```rust
use rc::clockwork::gas_nozzle::GasNozzle;
use rc::peripheral::Direction;

let p = GasNozzle::new(Direction::South);
```

## メソッド / Methods (18)

### `set_pointer(&self, value: f64) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `setPointer` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.set_pointer(0.0 /* value */).await?;
```

### `has_balloon(&self) -> `bool``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `hasBalloon` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.has_balloon().await?;
```

### `get_pointer(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getPointer` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_pointer().await?;
```

### `get_pointer_speed(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getPointerSpeed` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_pointer_speed().await?;
```

### `get_pocket_temperature(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getPocketTemperature` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_pocket_temperature().await?;
```

### `get_duct_temperature(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getDuctTemperature` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_duct_temperature().await?;
```

### `get_target_temperature(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getTargetTemperature` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_target_temperature().await?;
```

### `get_balloon_volume(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getBalloonVolume` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_balloon_volume().await?;
```

### `get_leaks(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getLeaks` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_leaks().await?;
```

### `get_balloon_pressure(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getBalloonPressure` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_balloon_pressure().await?;
```

### `get_loss_rate(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getLossRate` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_loss_rate().await?;
```

### `get_inflow_rate(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getInflowRate` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_inflow_rate().await?;
```

### `get_buoyancy_force(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getBuoyancyForce` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_buoyancy_force().await?;
```

### `get_total_gas_mass(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getTotalGasMass` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_total_gas_mass().await?;
```

### `get_leak_integrity(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getLeakIntegrity` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_leak_integrity().await?;
```

### `get_max_leaks(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getMaxLeaks` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_max_leaks().await?;
```

### `get_internal_density(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getInternalDensity` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_internal_density().await?;
```

### `get_temperature_delta(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getTemperatureDelta` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_temperature_delta().await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
