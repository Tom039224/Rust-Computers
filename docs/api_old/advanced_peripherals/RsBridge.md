# `RsBridge` — AdvancedPeripherals

**モジュールパス / Module path**: `rc::advanced_peripherals::rs_bridge::RsBridge`

**ソースTOML**: `peripherals/advanced_peripherals/rs_bridge.toml`


## 概要 / Overview

AdvancedPeripherals の `RsBridge` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `RsBridge` peripheral from AdvancedPeripherals.


## コンストラクタ / Constructor

```rust
use rc::advanced_peripherals::rs_bridge::RsBridge;
use rc::peripheral::Direction;

let p = RsBridge::new(Direction::South);
```

## メソッド / Methods (6)

### `is_connected(&self) -> `bool``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `isConnected` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.is_connected().await?;
```

### `is_online(&self) -> `bool``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `isOnline` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.is_online().await?;
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

### `get_energy_capacity(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getEnergyCapacity` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_energy_capacity().await?;
```

### `get_energy_usage(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getEnergyUsage` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_energy_usage().await?;
```

### `get_average_energy_input(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getAverageEnergyInput` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_average_energy_input().await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
