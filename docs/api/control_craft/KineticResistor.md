# `KineticResistor` — Control-Craft

**モジュールパス / Module path**: `rc::control_craft::kinetic_resistor::KineticResistor`

**ソースTOML**: `peripherals/control_craft/kinetic_resistor.toml`


## 概要 / Overview

Control-Craft の `KineticResistor` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `KineticResistor` peripheral from Control-Craft.


## コンストラクタ / Constructor

```rust
use rc::control_craft::kinetic_resistor::KineticResistor;
use rc::peripheral::Direction;

let p = KineticResistor::new(Direction::South);
```

## メソッド / Methods (2)

### `get_ratio(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getRatio` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_ratio().await?;
```

### `set_ratio(&self, ratio: f64) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `setRatio` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.set_ratio(0.0 /* ratio */).await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
