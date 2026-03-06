# `AirCompressor` — Clockwork CC Compat

**モジュールパス / Module path**: `rc::clockwork::air_compressor::AirCompressor`

**ソースTOML**: `peripherals/clockwork/air_compressor.toml`


## 概要 / Overview

Clockwork CC Compat の `AirCompressor` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `AirCompressor` peripheral from Clockwork CC Compat.


## コンストラクタ / Constructor

```rust
use rc::clockwork::air_compressor::AirCompressor;
use rc::peripheral::Direction;

let p = AirCompressor::new(Direction::South);
```

## メソッド / Methods (3)

### `get_status(&self) -> `String``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getStatus` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_status().await?;
```

### `get_speed(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getSpeed` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_speed().await?;
```

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
