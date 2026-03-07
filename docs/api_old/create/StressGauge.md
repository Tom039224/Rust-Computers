# `StressGauge` — Create

**モジュールパス / Module path**: `rc::create::stress_gauge::StressGauge`

**ソースTOML**: `peripherals/create/stress_gauge.toml`


## 概要 / Overview

Create の `StressGauge` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `StressGauge` peripheral from Create.


## コンストラクタ / Constructor

```rust
use rc::create::stress_gauge::StressGauge;
use rc::peripheral::Direction;

let p = StressGauge::new(Direction::South);
```

## メソッド / Methods (2)

### `get_stress(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getStress` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_stress().await?;
```

### `get_stress_capacity(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getStressCapacity` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_stress_capacity().await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
