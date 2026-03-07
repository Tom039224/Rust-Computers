# `Camera` — Control-Craft

**モジュールパス / Module path**: `rc::control_craft::camera::Camera`

**ソースTOML**: `peripherals/control_craft/camera.toml`


## 概要 / Overview

Control-Craft の `Camera` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `Camera` peripheral from Control-Craft.


## コンストラクタ / Constructor

```rust
use rc::control_craft::camera::Camera;
use rc::peripheral::Direction;

let p = Camera::new(Direction::South);
```

## メソッド / Methods (8)

### `is_being_used(&self) -> `bool``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `isBeingUsed` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.is_being_used().await?;
```

### `get_direction(&self) -> `String``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getDirection` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_direction().await?;
```

### `get_pitch(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getPitch` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_pitch().await?;
```

### `get_yaw(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getYaw` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_yaw().await?;
```

### `set_pitch(&self, pitch: f64) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `setPitch` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.set_pitch(0.0 /* pitch */).await?;
```

### `set_yaw(&self, yaw: f64) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `setYaw` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.set_yaw(0.0 /* yaw */).await?;
```

### `set_cone_angle(&self, angle: f64) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `setConeAngle` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.set_cone_angle(0.0 /* angle */).await?;
```

### `set_clip_range(&self, range: f64) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `setClipRange` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.set_clip_range(0.0 /* range */).await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
