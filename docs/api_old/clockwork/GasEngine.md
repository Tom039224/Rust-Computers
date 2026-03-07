# `GasEngine` — Clockwork CC Compat

**モジュールパス / Module path**: `rc::clockwork::gas_engine::GasEngine`

**ソースTOML**: `peripherals/clockwork/gas_engine.toml`


## 概要 / Overview

Clockwork CC Compat の `GasEngine` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `GasEngine` peripheral from Clockwork CC Compat.


## コンストラクタ / Constructor

```rust
use rc::clockwork::gas_engine::GasEngine;
use rc::peripheral::Direction;

let p = GasEngine::new(Direction::South);
```

## メソッド / Methods (2)

### `get_attached_engines(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getAttachedEngines` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_attached_engines().await?;
```

### `get_total_efficiency(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getTotalEfficiency` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_total_efficiency().await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
