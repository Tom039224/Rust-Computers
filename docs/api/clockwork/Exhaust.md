# `Exhaust` — Clockwork CC Compat

**モジュールパス / Module path**: `rc::clockwork::exhaust::Exhaust`

**ソースTOML**: `peripherals/clockwork/exhaust.toml`


## 概要 / Overview

Clockwork CC Compat の `Exhaust` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `Exhaust` peripheral from Clockwork CC Compat.


## コンストラクタ / Constructor

```rust
use rc::clockwork::exhaust::Exhaust;
use rc::peripheral::Direction;

let p = Exhaust::new(Direction::South);
```

## メソッド / Methods (1)

### `get_facing(&self) -> `String``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getFacing` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_facing().await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
