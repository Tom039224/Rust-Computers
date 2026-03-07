# `LinkBridge` — Control-Craft

**モジュールパス / Module path**: `rc::control_craft::link_bridge::LinkBridge`

**ソースTOML**: `peripherals/control_craft/link_bridge.toml`


## 概要 / Overview

Control-Craft の `LinkBridge` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `LinkBridge` peripheral from Control-Craft.


## コンストラクタ / Constructor

```rust
use rc::control_craft::link_bridge::LinkBridge;
use rc::peripheral::Direction;

let p = LinkBridge::new(Direction::South);
```

## メソッド / Methods (2)

### `set_input(&self, index: i32, value: f64) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `setInput` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.set_input(0 /* index */, 0.0 /* value */).await?;
```

### `get_output(&self, index: i32) -> `f64``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getOutput` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_output(0 /* index */).await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
