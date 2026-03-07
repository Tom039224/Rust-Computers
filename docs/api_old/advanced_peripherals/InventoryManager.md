# `InventoryManager` — AdvancedPeripherals

**モジュールパス / Module path**: `rc::advanced_peripherals::inventory_manager::InventoryManager`

**ソースTOML**: `peripherals/advanced_peripherals/inventory_manager.toml`


## 概要 / Overview

AdvancedPeripherals の `InventoryManager` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `InventoryManager` peripheral from AdvancedPeripherals.


## コンストラクタ / Constructor

```rust
use rc::advanced_peripherals::inventory_manager::InventoryManager;
use rc::peripheral::Direction;

let p = InventoryManager::new(Direction::South);
```

## メソッド / Methods (6)

### `get_owner(&self) -> `String``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getOwner` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_owner().await?;
```

### `is_player_equipped(&self) -> `bool``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `isPlayerEquipped` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.is_player_equipped().await?;
```

### `is_wearing(&self, index: i32) -> `bool``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `isWearing` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.is_wearing(0 /* index */).await?;
```

### `get_empty_space(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getEmptySpace` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_empty_space().await?;
```

### `is_space_available(&self) -> `bool``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `isSpaceAvailable` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.is_space_available().await?;
```

### `get_free_slot(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getFreeSlot` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_free_slot().await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
