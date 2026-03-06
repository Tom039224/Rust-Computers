# `SpeedController` — Create

**モジュールパス / Module path**: `rc::create::speed_controller::SpeedController`

**ソースTOML**: `peripherals/create/speed_controller.toml`


## 概要 / Overview

Create の `SpeedController` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `SpeedController` peripheral from Create.


## コンストラクタ / Constructor

```rust
use rc::create::speed_controller::SpeedController;
use rc::peripheral::Direction;

let p = SpeedController::new(Direction::South);
```

## メソッド / Methods (2)

### `set_target_speed(&self, speed: i32) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `setTargetSpeed` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.set_target_speed(0 /* speed */).await?;
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
