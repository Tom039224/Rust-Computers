# `PropellerController` — Control-Craft

**モジュールパス / Module path**: `rc::control_craft::propeller_controller::PropellerController`

**ソースTOML**: `peripherals/control_craft/propeller_controller.toml`


## 概要 / Overview

Control-Craft の `PropellerController` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `PropellerController` peripheral from Control-Craft.


## コンストラクタ / Constructor

```rust
use rc::control_craft::propeller_controller::PropellerController;
use rc::peripheral::Direction;

let p = PropellerController::new(Direction::South);
```

## メソッド / Methods (2)

### `set_target_speed(&self, speed: f64) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `setTargetSpeed` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.set_target_speed(0.0 /* speed */).await?;
```

### `get_target_speed(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getTargetSpeed` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_target_speed().await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
