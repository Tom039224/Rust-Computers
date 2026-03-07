# `Spinalyzer` — Control-Craft

**モジュールパス / Module path**: `rc::control_craft::spinalyzer::Spinalyzer`

**ソースTOML**: `peripherals/control_craft/spinalyzer.toml`


## 概要 / Overview

Control-Craft の `Spinalyzer` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `Spinalyzer` peripheral from Control-Craft.


## コンストラクタ / Constructor

```rust
use rc::control_craft::spinalyzer::Spinalyzer;
use rc::peripheral::Direction;

let p = Spinalyzer::new(Direction::South);
```

## メソッド / Methods (4)

### `apply_invariant_force(&self, x: f64, y: f64, z: f64) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `applyInvariantForce` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.apply_invariant_force(0.0 /* x */, 0.0 /* y */, 0.0 /* z */).await?;
```

### `apply_invariant_torque(&self, x: f64, y: f64, z: f64) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `applyInvariantTorque` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.apply_invariant_torque(0.0 /* x */, 0.0 /* y */, 0.0 /* z */).await?;
```

### `apply_rot_dependent_force(&self, x: f64, y: f64, z: f64) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `applyRotDependentForce` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.apply_rot_dependent_force(0.0 /* x */, 0.0 /* y */, 0.0 /* z */).await?;
```

### `apply_rot_dependent_torque(&self, x: f64, y: f64, z: f64) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `applyRotDependentTorque` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.apply_rot_dependent_torque(0.0 /* x */, 0.0 /* y */, 0.0 /* z */).await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
