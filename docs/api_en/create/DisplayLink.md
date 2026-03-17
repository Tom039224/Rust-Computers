# DisplayLink

**Module:** `create`  
**Peripheral Type:** `create:display_link`

Create Display Link peripheral. Controls a display linked to Create's display boards, supporting cursor positioning, text writing, and display management.

## Book-Read Methods

These methods use the book-read pattern. Call `book_next_*` to schedule a request,
`wait_for_next_tick().await` to flush, then `read_last_*` to get the result.

### `book_next_set_cursor_pos` / `read_last_set_cursor_pos`

Set the cursor position on the display.

```rust
pub fn book_next_set_cursor_pos(&mut self, x: u32, y: u32)
pub fn read_last_set_cursor_pos(&self) -> Result<(), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `x` | `u32` | Horizontal cursor position |
| `y` | `u32` | Vertical cursor position |

### `book_next_get_cursor_pos` / `read_last_get_cursor_pos`

Get the current cursor position.

```rust
pub fn book_next_get_cursor_pos(&mut self)
pub fn read_last_get_cursor_pos(&self) -> Result<(u32, u32), PeripheralError>
```

**Returns:** `(u32, u32)` — `(x, y)` cursor position.

### `book_next_get_size` / `read_last_get_size`

Get the display size. (mainThread=true, no imm variant available.)

```rust
pub fn book_next_get_size(&mut self)
pub fn read_last_get_size(&self) -> Result<(u32, u32), PeripheralError>
```

**Returns:** `(u32, u32)` — `(width, height)` of the display.

### `book_next_is_color` / `read_last_is_color`

Check whether the display supports colour.

```rust
pub fn book_next_is_color(&mut self)
pub fn read_last_is_color(&self) -> Result<bool, PeripheralError>
```

**Returns:** `bool` — `true` if the display supports colour.

### `book_next_write` / `read_last_write`

Write text at the current cursor position.

```rust
pub fn book_next_write(&mut self, text: &str)
pub fn read_last_write(&self) -> Result<(), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `text` | `&str` | Text to write |

### `book_next_write_bytes` / `read_last_write_bytes`

Write raw bytes to the display.

```rust
pub fn book_next_write_bytes(&mut self, data: &[u8]) -> Result<(), PeripheralError>
pub fn read_last_write_bytes(&self) -> Result<(), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `data` | `&[u8]` | Byte data to write |

### `book_next_clear_line` / `read_last_clear_line`

Clear the current line.

```rust
pub fn book_next_clear_line(&mut self)
pub fn read_last_clear_line(&self) -> Result<(), PeripheralError>
```

### `book_next_clear` / `read_last_clear`

Clear the entire display.

```rust
pub fn book_next_clear(&mut self)
pub fn read_last_clear(&self) -> Result<(), PeripheralError>
```

### `book_next_update` / `read_last_update`

Flush and update the display.

```rust
pub fn book_next_update(&mut self)
pub fn read_last_update(&self) -> Result<(), PeripheralError>
```

## Implementation Status

### ✅ Implemented

- All book_next_* / read_last_* methods

### 🚧 Not Yet Implemented

- async_* variants for all methods


## Immediate Methods

### `get_cursor_pos_imm`

Get the current cursor position immediately.

```rust
pub fn get_cursor_pos_imm(&self) -> Result<(u32, u32), PeripheralError>
```

**Returns:** `(u32, u32)` — `(x, y)` cursor position.

### `is_color_imm`

Check whether the display supports colour immediately.

```rust
pub fn is_color_imm(&self) -> Result<bool, PeripheralError>
```

**Returns:** `bool` — `true` if the display supports colour.

## Usage Example

```rust
use rust_computers_api::create::display_link::DisplayLink;
use rust_computers_api::peripheral::Peripheral;

let mut display = DisplayLink::wrap(addr);

// Clear display and write text
display.book_next_clear();
display.book_next_set_cursor_pos(1, 1);
display.book_next_write("Hello, Create!");
display.book_next_update();
wait_for_next_tick().await;
display.read_last_clear()?;
display.read_last_set_cursor_pos()?;
display.read_last_write()?;
display.read_last_update()?;

// Read display size
display.book_next_get_size();
wait_for_next_tick().await;
let (w, h) = display.read_last_get_size()?;
```
