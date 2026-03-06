# `Radar` — Some-Peripherals

**モジュールパス / Module path**: `rc::some_peripherals::radar::Radar`

**ソースTOML**: `peripherals/some_peripherals/radar.toml`


## 概要 / Overview

Some-Peripherals の `Radar` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `Radar` peripheral from Some-Peripherals.


## コンストラクタ / Constructor

```rust
use rc::some_peripherals::radar::Radar;
use rc::peripheral::Direction;

let p = Radar::new(Direction::South);
```

## メソッド / Methods (5)

### `scan(&self, radius: f64) -> `Vec<u8>` (raw MessagePack)`

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `scan` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.scan(0.0 /* radius */).await?;
```

### `scan_for_entities(&self, radius: f64) -> `Vec<u8>` (raw MessagePack)`

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `scanForEntities` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.scan_for_entities(0.0 /* radius */).await?;
```

### `scan_for_ships(&self, radius: f64) -> `Vec<u8>` (raw MessagePack)`

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `scanForShips` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.scan_for_ships(0.0 /* radius */).await?;
```

### `scan_for_players(&self, radius: f64) -> `Vec<u8>` (raw MessagePack)`

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `scanForPlayers` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.scan_for_players(0.0 /* radius */).await?;
```

### `get_config_info(&self) -> `Vec<u8>` (raw MessagePack)`

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getConfigInfo` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_config_info().await?;
```

---

## `bytes` 戻り値型について / About `bytes` return type

`ret = "bytes"` のメソッドは Lua が返す `table` (List / Map) を
生の MessagePack バイト列 (`Vec<u8>`) として受け取ります。

Methods with `ret = "bytes"` receive the Lua `table` (List/Map)
as raw MessagePack bytes (`Vec<u8>`).

解析には MessagePack パーサーを使用してください。
Decode with a MessagePack parser of your choice.


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
