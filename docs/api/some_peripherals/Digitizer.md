# `Digitizer` — Some-Peripherals

**モジュールパス / Module path**: `rc::some_peripherals::digitizer::Digitizer`

**ソースTOML**: `peripherals/some_peripherals/digitizer.toml`


## 概要 / Overview

Some-Peripherals の `Digitizer` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `Digitizer` peripheral from Some-Peripherals.


## コンストラクタ / Constructor

```rust
use rc::some_peripherals::digitizer::Digitizer;
use rc::peripheral::Direction;

let p = Digitizer::new(Direction::South);
```

## メソッド / Methods (5)

### `digitize_amount(&self, amount: i32) -> `String``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `digitizeAmount` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
let result = p.digitize_amount(0 /* amount */).await?;
```

### `rematerialize_amount(&self, uuid: &str, amount: i32) -> `bool``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `rematerializeAmount` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
let result = p.rematerialize_amount("" /* uuid */, 0 /* amount */).await?;
```

### `merge_digital_items(&self, into_uuid: &str, from_uuid: &str, amount: i32) -> `bool``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `mergeDigitalItems` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
let result = p.merge_digital_items("" /* into_uuid */, "" /* from_uuid */, 0 /* amount */).await?;
```

### `separate_digital_item(&self, from_uuid: &str, amount: i32) -> `String``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `separateDigitalItem` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
let result = p.separate_digital_item("" /* from_uuid */, 0 /* amount */).await?;
```

### `get_item_limit_in_slot(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getItemLimitInSlot` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_item_limit_in_slot().await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
