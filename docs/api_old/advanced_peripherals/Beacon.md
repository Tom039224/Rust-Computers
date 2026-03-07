# `Beacon` — AdvancedPeripherals

**モジュールパス / Module path**: `rc::advanced_peripherals::beacon::Beacon`

**ソースTOML**: `peripherals/advanced_peripherals/beacon.toml`


## 概要 / Overview

AdvancedPeripherals の `Beacon` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `Beacon` peripheral from AdvancedPeripherals.


## コンストラクタ / Constructor

```rust
use rc::advanced_peripherals::beacon::Beacon;
use rc::peripheral::Direction;

let p = Beacon::new(Direction::South);
```

## メソッド / Methods (3)

### `get_level(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getLevel` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_level().await?;
```

### `get_primary_effect(&self) -> `String``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getPrimaryEffect` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_primary_effect().await?;
```

### `get_secondary_effect(&self) -> `String``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getSecondaryEffect` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_secondary_effect().await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
