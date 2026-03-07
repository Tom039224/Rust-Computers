# `GeoScanner` — AdvancedPeripherals

**モジュールパス / Module path**: `rc::advanced_peripherals::geo_scanner::GeoScanner`

**ソースTOML**: `peripherals/advanced_peripherals/geo_scanner.toml`


## 概要 / Overview

AdvancedPeripherals の `GeoScanner` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `GeoScanner` peripheral from AdvancedPeripherals.


## コンストラクタ / Constructor

```rust
use rc::advanced_peripherals::geo_scanner::GeoScanner;
use rc::peripheral::Direction;

let p = GeoScanner::new(Direction::South);
```

## メソッド / Methods (1)

### `cost(&self, radius: i32) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `cost` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.cost(0 /* radius */).await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
