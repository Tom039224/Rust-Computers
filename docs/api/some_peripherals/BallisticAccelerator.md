# `BallisticAccelerator` — Some-Peripherals

**モジュールパス / Module path**: `rc::some_peripherals::ballistic_accelerator::BallisticAccelerator`

**ソースTOML**: `peripherals/some_peripherals/ballistic_accelerator.toml`


## 概要 / Overview

Some-Peripherals の `BallisticAccelerator` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `BallisticAccelerator` peripheral from Some-Peripherals.


## コンストラクタ / Constructor

```rust
use rc::some_peripherals::ballistic_accelerator::BallisticAccelerator;
use rc::peripheral::Direction;

let p = BallisticAccelerator::new(Direction::South);
```

## メソッド / Methods (1)

### `get_drag(&self, base_drag: f64, dimensional_drag_multiplier: f64) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getDrag` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_drag(0.0 /* base_drag */, 0.0 /* dimensional_drag_multiplier */).await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
