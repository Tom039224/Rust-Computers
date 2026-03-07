# `NbtStorage` — AdvancedPeripherals

**モジュールパス / Module path**: `rc::advanced_peripherals::nbt_storage::NbtStorage`

**ソースTOML**: `peripherals/advanced_peripherals/nbt_storage.toml`


## 概要 / Overview

AdvancedPeripherals の `NbtStorage` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `NbtStorage` peripheral from AdvancedPeripherals.


## コンストラクタ / Constructor

```rust
use rc::advanced_peripherals::nbt_storage::NbtStorage;
use rc::peripheral::Direction;

let p = NbtStorage::new(Direction::South);
```

## メソッド / Methods (1)

### `write_json(&self, json_data: &str) -> `bool``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `writeJson` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
let result = p.write_json("" /* json_data */).await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
