# `Station` — Create

**モジュールパス / Module path**: `rc::create::station::Station`

**ソースTOML**: `peripherals/create/station.toml`


## 概要 / Overview

Create の `Station` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `Station` peripheral from Create.


## コンストラクタ / Constructor

```rust
use rc::create::station::Station;
use rc::peripheral::Direction;

let p = Station::new(Direction::South);
```

## メソッド / Methods (10)

### `assemble(&self) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `assemble` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.assemble().await?;
```

### `disassemble(&self) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `disassemble` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.disassemble().await?;
```

### `is_train_present(&self) -> `bool``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `isTrainPresent` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.is_train_present().await?;
```

### `get_station_name(&self) -> `String``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getStationName` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_station_name().await?;
```

### `set_station_name(&self, name: &str) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `setStationName` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.set_station_name("" /* name */).await?;
```

### `get_train_name(&self) -> `String``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getTrainName` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_train_name().await?;
```

### `set_train_name(&self, name: &str) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `setTrainName` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.set_train_name("" /* name */).await?;
```

### `has_schedule(&self) -> `bool``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `hasSchedule` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.has_schedule().await?;
```

### `is_in_assembly_mode(&self) -> `bool``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `isInAssemblyMode` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.is_in_assembly_mode().await?;
```

### `set_assembly_mode(&self, enabled: bool) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `setAssemblyMode` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.set_assembly_mode(false /* enabled */).await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
