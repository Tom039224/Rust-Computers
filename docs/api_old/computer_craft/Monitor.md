# `Monitor` — CC:Tweaked

**モジュールパス / Module path**: `rc::computer_craft::monitor::Monitor`

**ソースTOML**: `peripherals/computer_craft/monitor.toml`


## 概要 / Overview

CC:Tweaked の `Monitor` ペリフェラルに対応した型付きラッパー。
Typed Rust wrapper for the `Monitor` peripheral from CC:Tweaked.


## コンストラクタ / Constructor

```rust
use rc::computer_craft::monitor::Monitor;
use rc::peripheral::Direction;

let p = Monitor::new(Direction::South);
```

## メソッド / Methods (18)

### `get_type(&self) -> `String``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getType` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | なし (即時) |
| 即時バリアント | `get_type_imm()` も生成される |

```rust
let result = p.get_type().await?;
```

### `get_size(&self) -> (`i32`, `i32`)`

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getSize` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | なし (即時) |
| 即時バリアント | `get_size_imm()` も生成される |

```rust
let result = p.get_size().await?;
```

### `is_advanced(&self) -> `bool``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `isAdvanced` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | なし (即時) |
| 即時バリアント | `is_advanced_imm()` も生成される |

```rust
let result = p.is_advanced().await?;
```

### `get_text_scale(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getTextScale` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | なし (即時) |
| 即時バリアント | `get_text_scale_imm()` も生成される |

```rust
let result = p.get_text_scale().await?;
```

### `clear(&self) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `clear` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.clear().await?;
```

### `clear_line(&self) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `clearLine` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.clear_line().await?;
```

### `write(&self, text: &str) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `write` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.write("" /* text */).await?;
```

### `blit(&self, text: &str, text_color: &str, bg_color: &str) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `blit` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.blit("" /* text */, "" /* text_color */, "" /* bg_color */).await?;
```

### `get_cursor_pos(&self) -> (`i32`, `i32`)`

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getCursorPos` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | なし (即時) |
| 即時バリアント | `get_cursor_pos_imm()` も生成される |

```rust
let result = p.get_cursor_pos().await?;
```

### `set_cursor_pos(&self, x: i32, y: i32) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `setCursorPos` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.set_cursor_pos(0 /* x */, 0 /* y */).await?;
```

### `get_cursor_blink(&self) -> `bool``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getCursorBlink` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | なし (即時) |
| 即時バリアント | `get_cursor_blink_imm()` も生成される |

```rust
let result = p.get_cursor_blink().await?;
```

### `set_cursor_blink(&self, blink: bool) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `setCursorBlink` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.set_cursor_blink(false /* blink */).await?;
```

### `set_text_color(&self, color: i32) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `setTextColor` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.set_text_color(0 /* color */).await?;
```

### `set_background_color(&self, color: i32) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `setBackgroundColor` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.set_background_color(0 /* color */).await?;
```

### `get_text_color(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getTextColor` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_text_color().await?;
```

### `get_background_color(&self) -> `i32``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `getBackgroundColor` |
| 種別 | 情報取得 (request_info) |
| 1tick 遅延 | あり |

```rust
let result = p.get_background_color().await?;
```

### `scroll(&self, lines: i32) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `scroll` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.scroll(0 /* lines */).await?;
```

### `set_text_scale(&self, scale_x10: i32) -> `()``

| 項目 | 値 |
|------|-----|
| Lua メソッド名 | `setTextScale` |
| 種別 | ワールド干渉系アクション (do_action) |
| 1tick 遅延 | あり |

```rust
p.set_text_scale(0 /* scale_x10 */).await?;
```


---
*このファイルは TOML マニフェストから自動生成されました / Auto-generated from TOML manifest.*
