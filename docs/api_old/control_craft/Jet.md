# `Jet` — Control-Craft

**モジュールパス / Module path**: `rc::control_craft::jet::Jet`

**ソースTOML**: `peripherals/control_craft/jet.toml`


## 概要 / Overview

Control-Craft の `Jet` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `Jet` peripheral from Control-Craft.


## コンストラクタ / Constructor

```rust
use rc::control_craft::jet::Jet;
use rc::peripheral::Direction;

let p = Jet::new(Direction::South);
```

## メソッド / Methods (3)

### `set_output_thrust(&self, thrust: f64) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `setOutputThrust` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.set_output_thrust(0.0 /* thrust */).await?;
```

### `set_horizontal_tilt(&self, tilt: f64) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `setHorizontalTilt` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.set_horizontal_tilt(0.0 /* tilt */).await?;
```

### `set_vertical_tilt(&self, tilt: f64) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `setVerticalTilt` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.set_vertical_tilt(0.0 /* tilt */).await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
