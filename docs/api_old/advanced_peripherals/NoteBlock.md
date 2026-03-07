# `NoteBlock` — AdvancedPeripherals

**モジュールパス / Module path**: `rc::advanced_peripherals::note_block::NoteBlock`

**ソースTOML**: `peripherals/advanced_peripherals/note_block.toml`


## 概要 / Overview

AdvancedPeripherals の `NoteBlock` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `NoteBlock` peripheral from AdvancedPeripherals.


## コンストラクタ / Constructor

```rust
use rc::advanced_peripherals::note_block::NoteBlock;
use rc::peripheral::Direction;

let p = NoteBlock::new(Direction::South);
```

## メソッド / Methods (4)

### `change_note(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `changeNote` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
let result = p.change_note().await?;
```

### `change_note_by(&self, note: i32) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `changeNoteBy` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
let result = p.change_note_by(0 /* note */).await?;
```

### `get_note(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getNote` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_note().await?;
```

### `play_note(&self) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `playNote` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.play_note().await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
