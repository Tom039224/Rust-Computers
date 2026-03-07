# `BlockReader` — AdvancedPeripherals

**モジュールパス / Module path**: `rc::advanced_peripherals::block_reader::BlockReader`

**ソースTOML**: `peripherals/advanced_peripherals/block_reader.toml`


## 概要 / Overview

AdvancedPeripherals の `BlockReader` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `BlockReader` peripheral from AdvancedPeripherals.


## コンストラクタ / Constructor

```rust
use rc::advanced_peripherals::block_reader::BlockReader;
use rc::peripheral::Direction;

let p = BlockReader::new(Direction::South);
```

## メソッド / Methods (2)

### `get_block_name(&self) -> `String``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getBlockName` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_block_name().await?;
```

### `is_tile_entity(&self) -> `bool``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `isTileEntity` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.is_tile_entity().await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
