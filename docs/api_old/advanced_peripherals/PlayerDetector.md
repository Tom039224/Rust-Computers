# `PlayerDetector` — AdvancedPeripherals

**モジュールパス / Module path**: `rc::advanced_peripherals::player_detector::PlayerDetector`

**ソースTOML**: `peripherals/advanced_peripherals/player_detector.toml`


## 概要 / Overview

AdvancedPeripherals の `PlayerDetector` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `PlayerDetector` peripheral from AdvancedPeripherals.


## コンストラクタ / Constructor

```rust
use rc::advanced_peripherals::player_detector::PlayerDetector;
use rc::peripheral::Direction;

let p = PlayerDetector::new(Direction::South);
```

## メソッド / Methods (4)

### `is_players_in_cubic(&self, x: i32, y: i32, z: i32) -> `bool``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `isPlayersInCubic` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.is_players_in_cubic(0 /* x */, 0 /* y */, 0 /* z */).await?;
```

### `is_players_in_range(&self, range: i32) -> `bool``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `isPlayersInRange` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.is_players_in_range(0 /* range */).await?;
```

### `is_player_in_cubic(&self, x: i32, y: i32, z: i32, username: &str) -> `bool``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `isPlayerInCubic` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.is_player_in_cubic(0 /* x */, 0 /* y */, 0 /* z */, "" /* username */).await?;
```

### `is_player_in_range(&self, range: i32, username: &str) -> `bool``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `isPlayerInRange` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.is_player_in_range(0 /* range */, "" /* username */).await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
