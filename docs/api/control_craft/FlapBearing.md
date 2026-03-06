# `FlapBearing` — Control-Craft

**モジュールパス / Module path**: `rc::control_craft::flap_bearing::FlapBearing`

**ソースTOML**: `peripherals/control_craft/flap_bearing.toml`


## 概要 / Overview

Control-Craft の `FlapBearing` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `FlapBearing` peripheral from Control-Craft.


## コンストラクタ / Constructor

```rust
use rc::control_craft::flap_bearing::FlapBearing;
use rc::peripheral::Direction;

let p = FlapBearing::new(Direction::South);
```

## メソッド / Methods (4)

### `get_angle(&self) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getAngle` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_angle().await?;
```

### `assemble_next_tick(&self) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `assembleNextTick` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.assemble_next_tick().await?;
```

### `disassemble_next_tick(&self) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `disassembleNextTick` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.disassemble_next_tick().await?;
```

### `set_angle(&self, angle: f64) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `setAngle` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.set_angle(0.0 /* angle */).await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
