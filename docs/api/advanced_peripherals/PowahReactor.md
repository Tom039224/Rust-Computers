# `PowahReactor` — AdvancedPeripherals

**モジュールパス / Module path**: `rc::advanced_peripherals::powah_reactor::PowahReactor`

**ソースTOML**: `peripherals/advanced_peripherals/powah_reactor.toml`


## 概要 / Overview

AdvancedPeripherals の `PowahReactor` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `PowahReactor` peripheral from AdvancedPeripherals.


## コンストラクタ / Constructor

```rust
use rc::advanced_peripherals::powah_reactor::PowahReactor;
use rc::peripheral::Direction;

let p = PowahReactor::new(Direction::South);
```

## メソッド / Methods (7)

### `is_running(&self) -> `bool``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `isRunning` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.is_running().await?;
```

### `get_fuel(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getFuel` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_fuel().await?;
```

### `get_carbon(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getCarbon` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_carbon().await?;
```

### `get_redstone(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getRedstone` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_redstone().await?;
```

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

### `get_temperature(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getTemperature` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_temperature().await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
