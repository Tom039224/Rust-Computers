# `NixieTube` — Create

**モジュールパス / Module path**: `rc::create::nixie_tube::NixieTube`

**ソースTOML**: `peripherals/create/nixie_tube.toml`


## 概要 / Overview

Create の `NixieTube` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `NixieTube` peripheral from Create.


## コンストラクタ / Constructor

```rust
use rc::create::nixie_tube::NixieTube;
use rc::peripheral::Direction;

let p = NixieTube::new(Direction::South);
```

## メソッド / Methods (2)

### `set_text(&self, text: &str) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `setText` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.set_text("" /* text */).await?;
```

### `set_text_colour(&self, colour: &str) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `setTextColour` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.set_text_colour("" /* colour */).await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
