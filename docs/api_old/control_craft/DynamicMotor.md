# `DynamicMotor` — Control-Craft

**モジュールパス / Module path**: `rc::control_craft::dynamic_motor::DynamicMotor`

**ソースTOML**: `peripherals/control_craft/dynamic_motor.toml`


## 概要 / Overview

Control-Craft の `DynamicMotor` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `DynamicMotor` peripheral from Control-Craft.


## コンストラクタ / Constructor

```rust
use rc::control_craft::dynamic_motor::DynamicMotor;
use rc::peripheral::Direction;

let p = DynamicMotor::new(Direction::South);
```

## メソッド / Methods (11)

### `set_p_i_d(&self, p: f64, i: f64, d: f64) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `setPID` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.set_p_i_d(0.0 /* p */, 0.0 /* i */, 0.0 /* d */).await?;
```

### `set_target_value(&self, value: f64) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `setTargetValue` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.set_target_value(0.0 /* value */).await?;
```

### `get_target_value(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getTargetValue` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_target_value().await?;
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

### `get_angular_velocity(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getAngularVelocity` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_angular_velocity().await?;
```

### `get_current_value(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getCurrentValue` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_current_value().await?;
```

### `set_output_torque(&self, torque: f64) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `setOutputTorque` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.set_output_torque(0.0 /* torque */).await?;
```

### `set_is_adjusting_angle(&self, adjusting: bool) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `setIsAdjustingAngle` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.set_is_adjusting_angle(false /* adjusting */).await?;
```

### `lock(&self) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `lock` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.lock().await?;
```

### `unlock(&self) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `unlock` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.unlock().await?;
```

### `is_locked(&self) -> `bool``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `isLocked` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.is_locked().await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
