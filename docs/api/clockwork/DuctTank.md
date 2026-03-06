# `DuctTank` — Clockwork CC Compat

**モジュールパス / Module path**: `rc::clockwork::duct_tank::DuctTank`

**ソースTOML**: `peripherals/clockwork/duct_tank.toml`


## 概要 / Overview

Clockwork CC Compat の `DuctTank` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `DuctTank` peripheral from Clockwork CC Compat.


## コンストラクタ / Constructor

```rust
use rc::clockwork::duct_tank::DuctTank;
use rc::peripheral::Direction;

let p = DuctTank::new(Direction::South);
```

## メソッド / Methods (2)

### `get_height(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getHeight` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_height().await?;
```

### `get_width(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getWidth` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_width().await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
