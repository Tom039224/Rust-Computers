# `SpeedGauge` — Create

**モジュールパス / Module path**: `rc::create::speed_gauge::SpeedGauge`

**ソースTOML**: `peripherals/create/speed_gauge.toml`


## 概要 / Overview

Create の `SpeedGauge` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `SpeedGauge` peripheral from Create.


## コンストラクタ / Constructor

```rust
use rc::create::speed_gauge::SpeedGauge;
use rc::peripheral::Direction;

let p = SpeedGauge::new(Direction::South);
```

## メソッド / Methods (1)

### `get_speed(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getSpeed` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_speed().await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
