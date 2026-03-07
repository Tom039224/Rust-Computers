# `PowahEnderCell` — AdvancedPeripherals

**モジュールパス / Module path**: `rc::advanced_peripherals::powah_ender_cell::PowahEnderCell`

**ソースTOML**: `peripherals/advanced_peripherals/powah_ender_cell.toml`


## 概要 / Overview

AdvancedPeripherals の `PowahEnderCell` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `PowahEnderCell` peripheral from AdvancedPeripherals.


## コンストラクタ / Constructor

```rust
use rc::advanced_peripherals::powah_ender_cell::PowahEnderCell;
use rc::peripheral::Direction;

let p = PowahEnderCell::new(Direction::South);
```

## メソッド / Methods (5)

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

### `get_channel(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getChannel` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_channel().await?;
```

### `set_channel(&self, channel: i32) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `setChannel` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.set_channel(0 /* channel */).await?;
```

### `get_max_channels(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getMaxChannels` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_max_channels().await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
