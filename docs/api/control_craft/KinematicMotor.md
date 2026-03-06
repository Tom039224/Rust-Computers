# `KinematicMotor` — Control-Craft

**モジュールパス / Module path**: `rc::control_craft::kinematic_motor::KinematicMotor`

**ソースTOML**: `peripherals/control_craft/kinematic_motor.toml`


## 概要 / Overview

Control-Craft の `KinematicMotor` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `KinematicMotor` peripheral from Control-Craft.


## コンストラクタ / Constructor

```rust
use rc::control_craft::kinematic_motor::KinematicMotor;
use rc::peripheral::Direction;

let p = KinematicMotor::new(Direction::South);
```

## メソッド / Methods (6)

### `set_target_angle(&self, angle: f64) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `setTargetAngle` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.set_target_angle(0.0 /* angle */).await?;
```

### `get_target_angle(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getTargetAngle` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_target_angle().await?;
```

### `get_control_target(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getControlTarget` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_control_target().await?;
```

### `set_control_target(&self, target: f64) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `setControlTarget` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.set_control_target(0.0 /* target */).await?;
```

### `get_angle(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getAngle` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_angle().await?;
```

### `set_is_forcing_angle(&self, forcing: bool) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `setIsForcingAngle` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.set_is_forcing_angle(false /* forcing */).await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
