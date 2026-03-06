# `Raycaster` — Some-Peripherals

**モジュールパス / Module path**: `rc::some_peripherals::raycaster::Raycaster`

**ソースTOML**: `peripherals/some_peripherals/raycaster.toml`


## 概要 / Overview

Some-Peripherals の `Raycaster` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `Raycaster` peripheral from Some-Peripherals.


## コンストラクタ / Constructor

```rust
use rc::some_peripherals::raycaster::Raycaster;
use rc::peripheral::Direction;

let p = Raycaster::new(Direction::South);
```

## メソッド / Methods (2)

### `get_facing_direction(&self) -> `String``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getFacingDirection` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_facing_direction().await?;
```

### `add_stickers(&self, state: bool) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `addStickers` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.add_stickers(false /* state */).await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
