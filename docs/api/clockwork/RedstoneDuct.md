# `RedstoneDuct` — Clockwork CC Compat

**モジュールパス / Module path**: `rc::clockwork::redstone_duct::RedstoneDuct`

**ソースTOML**: `peripherals/clockwork/redstone_duct.toml`


## 概要 / Overview

Clockwork CC Compat の `RedstoneDuct` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `RedstoneDuct` peripheral from Clockwork CC Compat.


## コンストラクタ / Constructor

```rust
use rc::clockwork::redstone_duct::RedstoneDuct;
use rc::peripheral::Direction;

let p = RedstoneDuct::new(Direction::South);
```

## メソッド / Methods (1)

### `get_power(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getPower` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_power().await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
