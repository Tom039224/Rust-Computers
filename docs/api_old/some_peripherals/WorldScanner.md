# `WorldScanner` — Some-Peripherals

**モジュールパス / Module path**: `rc::some_peripherals::world_scanner::WorldScanner`

**ソースTOML**: `peripherals/some_peripherals/world_scanner.toml`


## 概要 / Overview

Some-Peripherals の `WorldScanner` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `WorldScanner` peripheral from Some-Peripherals.


## コンストラクタ / Constructor

```rust
use rc::some_peripherals::world_scanner::WorldScanner;
use rc::peripheral::Direction;

let p = WorldScanner::new(Direction::South);
```

## メソッド / Methods (1)

### `get_block_at(&self, x: f64, y: f64, z: f64, in_shipyard: bool) -> `Vec<u8>` (raw MessagePack)`

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getBlockAt` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_block_at(0.0 /* x */, 0.0 /* y */, 0.0 /* z */, false /* in_shipyard */).await?;
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
