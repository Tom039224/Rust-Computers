# DisplayLink

**Module:** `create`  
**Peripheral Type:** `create:display_link`

Create Display Link peripheral. Allows writing text to Create display targets (e.g. Display Boards).

## Methods

### book/read Methods

These methods use the book-read pattern. Call `book_next_*` to schedule a request,
`wait_for_next_tick().await` to flush, then `read_last_*` to get the result.

#### `book_next_set_cursor_pos` / `read_last_set_cursor_pos`

Set the cursor position.

```rust
pub fn book_next_set_cursor_pos(&mut self, x: u32, y: u32)
pub fn read_last_set_cursor_pos(&self) -> Result<(), PeripheralError>
```

#### `book_next_get_cursor_pos` / `read_last_get_cursor_pos`

Get the current cursor position.

```rust
pub fn book_next_get_cursor_pos(&mut self)
pub fn read_last_get_cursor_pos(&self) -> Result<(u32, u32), PeripheralError>
```

#### `book_next_get_size` / `read_last_get_size`

Get the display size. Note: `mainThread=true`, no imm variant available.

```rust
pub fn book_next_get_size(&mut self)
pub fn read_last_get_size(&self) -> Result<(u32, u32), PeripheralError>
```

#### `book_next_is_color` / `read_last_is_color`

Check if the display supports color.

```rust
pub fn book_next_is_color(&mut self)
pub fn read_last_is_color(&self) -> Result<bool, PeripheralError>
```

#### `book_next_write` / `read_last_write`

Write text at the current cursor position.

```rust
pub fn book_next_write(&mut self, text: &str)
pub fn read_last_write(&self) -> Result<(), PeripheralError>
```

#### `book_next_write_bytes` / `read_last_write_bytes`

Write raw bytes to the display.

```rust
pub fn book_next_write_bytes(&mut self, data: &[u8]) -> Result<(), PeripheralError>
pub fn read_last_write_bytes(&self) -> Result<(), PeripheralError>
```

#### `book_next_clear_line` / `read_last_clear_line`

Clear the current line.

```rust
pub fn book_next_clear_line(&mut self)
pub fn read_last_clear_line(&self) -> Result<(), PeripheralError>
```

#### `book_next_clear` / `read_last_clear`

Clear the entire display.

```rust
pub fn book_next_clear(&mut self)
pub fn read_last_clear(&self) -> Result<(), PeripheralError>
```

#### `book_next_update` / `read_last_update`

Flush updates to the display.

```rust
pub fn book_next_update(&mut self)
pub fn read_last_update(&self) -> Result<(), PeripheralError>
```

### Immediate Methods

#### `get_cursor_pos_imm`

Get the cursor position immediately.

```rust
pub fn get_cursor_pos_imm(&self) -> Result<(u32, u32), PeripheralError>
```

#### `is_color_imm`

Check if color is supported immediately.

```rust
pub fn is_color_imm(&self) -> Result<bool, PeripheralError>
```

## Example

```rust
use rust_computers_api::create::display_link::DisplayLink;
use rust_computers_api::peripheral::Peripheral;

let mut display = DisplayLink::wrap(addr);

// Clear and write text
display.book_next_clear();
wait_for_next_tick().await;
display.read_last_clear()?;

display.book_next_set_cursor_pos(1, 1);
wait_for_next_tick().await;
display.read_last_set_cursor_pos()?;

display.book_next_write("Hello World!");
wait_for_next_tick().await;
display.read_last_write()?;

display.book_next_update();
wait_for_next_tick().await;
display.read_last_update()?;
```
