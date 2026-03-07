# `Compass` — AdvancedPeripherals

**モジュールパス / Module path**: `rc::advanced_peripherals::compass::Compass`

**ソースTOML**: `peripherals/advanced_peripherals/compass.toml`


## 概要 / Overview

AdvancedPeripherals の `Compass` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `Compass` peripheral from AdvancedPeripherals.


## コンストラクタ / Constructor

```rust
use rc::advanced_peripherals::compass::Compass;
use rc::peripheral::Direction;

let p = Compass::new(Direction::South);
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
