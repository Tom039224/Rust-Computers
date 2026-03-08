# DisplayLink

**モジュール:** `create`  
**ペリフェラルタイプ:** `create:display_link`

Create Display Link ペリフェラル。Create のディスプレイボードに接続し、カーソル操作、テキスト書き込み、ディスプレイ管理を行います。

## Book-Read メソッド

book-read パターンを使用します。`book_next_*` でリクエストを予約し、
`wait_for_next_tick().await` でフラッシュした後、`read_last_*` で結果を取得します。

### `book_next_set_cursor_pos` / `read_last_set_cursor_pos`

カーソル位置を設定します。

```rust
pub fn book_next_set_cursor_pos(&mut self, x: u32, y: u32)
pub fn read_last_set_cursor_pos(&self) -> Result<(), PeripheralError>
```

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `x` | `u32` | 水平カーソル位置 |
| `y` | `u32` | 垂直カーソル位置 |

### `book_next_get_cursor_pos` / `read_last_get_cursor_pos`

現在のカーソル位置を取得します。

```rust
pub fn book_next_get_cursor_pos(&mut self)
pub fn read_last_get_cursor_pos(&self) -> Result<(u32, u32), PeripheralError>
```

**戻り値:** `(u32, u32)` — `(x, y)` カーソル位置。

### `book_next_get_size` / `read_last_get_size`

ディスプレイのサイズを取得します。（mainThread=true のため imm 非対応）

```rust
pub fn book_next_get_size(&mut self)
pub fn read_last_get_size(&self) -> Result<(u32, u32), PeripheralError>
```

**戻り値:** `(u32, u32)` — ディスプレイの `(幅, 高さ)`。

### `book_next_is_color` / `read_last_is_color`

カラー対応かどうかを取得します。

```rust
pub fn book_next_is_color(&mut self)
pub fn read_last_is_color(&self) -> Result<bool, PeripheralError>
```

**戻り値:** `bool` — カラー対応なら `true`。

### `book_next_write` / `read_last_write`

現在のカーソル位置にテキストを書き込みます。

```rust
pub fn book_next_write(&mut self, text: &str)
pub fn read_last_write(&self) -> Result<(), PeripheralError>
```

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `text` | `&str` | 書き込むテキスト |

### `book_next_write_bytes` / `read_last_write_bytes`

バイト列をディスプレイに書き込みます。

```rust
pub fn book_next_write_bytes(&mut self, data: &[u8]) -> Result<(), PeripheralError>
pub fn read_last_write_bytes(&self) -> Result<(), PeripheralError>
```

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `data` | `&[u8]` | 書き込むバイトデータ |

### `book_next_clear_line` / `read_last_clear_line`

現在の行をクリアします。

```rust
pub fn book_next_clear_line(&mut self)
pub fn read_last_clear_line(&self) -> Result<(), PeripheralError>
```

### `book_next_clear` / `read_last_clear`

ディスプレイ全体をクリアします。

```rust
pub fn book_next_clear(&mut self)
pub fn read_last_clear(&self) -> Result<(), PeripheralError>
```

### `book_next_update` / `read_last_update`

ディスプレイを更新（フラッシュ）します。

```rust
pub fn book_next_update(&mut self)
pub fn read_last_update(&self) -> Result<(), PeripheralError>
```

## 即時メソッド (Immediate)

### `get_cursor_pos_imm`

現在のカーソル位置を即時取得します。

```rust
pub fn get_cursor_pos_imm(&self) -> Result<(u32, u32), PeripheralError>
```

**戻り値:** `(u32, u32)` — `(x, y)` カーソル位置。

### `is_color_imm`

カラー対応かどうかを即時取得します。

```rust
pub fn is_color_imm(&self) -> Result<bool, PeripheralError>
```

**戻り値:** `bool` — カラー対応なら `true`。

## 使用例

```rust
use rust_computers_api::create::display_link::DisplayLink;
use rust_computers_api::peripheral::Peripheral;

let mut display = DisplayLink::wrap(addr);

// ディスプレイをクリアしてテキストを書き込む
display.book_next_clear();
display.book_next_set_cursor_pos(1, 1);
display.book_next_write("Hello, Create!");
display.book_next_update();
wait_for_next_tick().await;
display.read_last_clear()?;
display.read_last_set_cursor_pos()?;
display.read_last_write()?;
display.read_last_update()?;

// ディスプレイサイズを取得
display.book_next_get_size();
wait_for_next_tick().await;
let (w, h) = display.read_last_get_size()?;
```
