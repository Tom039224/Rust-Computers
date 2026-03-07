# `AutomataWarping` — AdvancedPeripherals

**モジュールパス / Module path**: `rc::advanced_peripherals::automata_warping::AutomataWarping`

**ソースTOML**: `peripherals/advanced_peripherals/automata_warping.toml`


## 概要 / Overview

AdvancedPeripherals の `AutomataWarping` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `AutomataWarping` peripheral from AdvancedPeripherals.


## コンストラクタ / Constructor

```rust
use rc::advanced_peripherals::automata_warping::AutomataWarping;
use rc::peripheral::Direction;

let p = AutomataWarping::new(Direction::South);
```

## メソッド / Methods (5)

### `save_point(&self, name: &str) -> `bool``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `savePoint` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
let result = p.save_point("" /* name */).await?;
```

### `delete_point(&self, name: &str) -> `bool``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `deletePoint` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
let result = p.delete_point("" /* name */).await?;
```

### `warp_to_point(&self, name: &str) -> `bool``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `warpToPoint` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
let result = p.warp_to_point("" /* name */).await?;
```

### `estimate_warp_cost(&self, name: &str) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `estimateWarpCost` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.estimate_warp_cost("" /* name */).await?;
```

### `distance_to_point(&self, name: &str) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `distanceToPoint` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.distance_to_point("" /* name */).await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
