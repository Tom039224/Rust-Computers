# Monitor

**Module:** `computer_craft`
**Peripheral Type:** `monitor` (the NAME constant)

CC:Tweaked Monitor peripheral (unified for normal and advanced). Provides text-based display output with cursor control, color management, and scrolling.

## Methods

### Book/Read Methods

These methods use the book-read pattern. Call `book_next_*` to schedule a request,
`wait_for_next_tick().await` to flush, then `read_last_*` to get the result.

#### `book_next_set_text_scale` / `read_last_set_text_scale`

Set the text scale of the monitor.

```rust
pub fn book_next_set_text_scale(&mut self, scale: MonitorTextScale)
pub fn read_last_set_text_scale(&self) -> Result<(), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| scale | `MonitorTextScale` | The text scale (0.5–5.0, in 0.5 increments) |

#### `book_next_get_text_scale` / `read_last_get_text_scale`

Get the current text scale of the monitor.

```rust
pub fn book_next_get_text_scale(&mut self)
pub fn read_last_get_text_scale(&self) -> Result<MonitorTextScale, PeripheralError>
```

#### `book_next_write` / `read_last_write`

Write text at the current cursor position.

```rust
pub fn book_next_write(&mut self, text: &str)
pub fn read_last_write(&self) -> Result<(), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| text | `&str` | The text to write |

#### `book_next_scroll` / `read_last_scroll`

Scroll the screen content.

```rust
pub fn book_next_scroll(&mut self, y: u32)
pub fn read_last_scroll(&self) -> Result<(), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| y | `u32` | The number of lines to scroll |

#### `book_next_get_cursor_pos` / `read_last_get_cursor_pos`

Get the current cursor position.

```rust
pub fn book_next_get_cursor_pos(&mut self)
pub fn read_last_get_cursor_pos(&self) -> Result<MonitorPosition, PeripheralError>
```

#### `book_next_set_cursor_pos` / `read_last_set_cursor_pos`

Set the cursor position.

```rust
pub fn book_next_set_cursor_pos(&mut self, pos: MonitorPosition)
pub fn read_last_set_cursor_pos(&self) -> Result<(), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| pos | `MonitorPosition` | The target cursor position |

#### `book_next_get_cursor_blink` / `read_last_get_cursor_blink`

Get the cursor blink state.

```rust
pub fn book_next_get_cursor_blink(&mut self)
pub fn read_last_get_cursor_blink(&self) -> Result<bool, PeripheralError>
```

#### `book_next_set_cursor_blink` / `read_last_set_cursor_blink`

Set the cursor blink state.

```rust
pub fn book_next_set_cursor_blink(&mut self, blink: bool)
pub fn read_last_set_cursor_blink(&self) -> Result<(), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| blink | `bool` | Whether the cursor should blink |

#### `book_next_get_size` / `read_last_get_size`

Get the monitor size in characters.

```rust
pub fn book_next_get_size(&mut self)
pub fn read_last_get_size(&self) -> Result<MonitorSize, PeripheralError>
```

#### `book_next_clear` / `read_last_clear`

Clear the entire screen.

```rust
pub fn book_next_clear(&mut self)
pub fn read_last_clear(&self) -> Result<(), PeripheralError>
```

#### `book_next_clear_line` / `read_last_clear_line`

Clear the current line.

```rust
pub fn book_next_clear_line(&mut self)
pub fn read_last_clear_line(&self) -> Result<(), PeripheralError>
```

#### `book_next_get_text_color` / `read_last_get_text_color`

Get the current text color.

```rust
pub fn book_next_get_text_color(&mut self)
pub fn read_last_get_text_color(&self) -> Result<MonitorColor, PeripheralError>
```

#### `book_next_set_text_color` / `read_last_set_text_color`

Set the text color.

```rust
pub fn book_next_set_text_color(&mut self, color: MonitorColor)
pub fn read_last_set_text_color(&self) -> Result<(), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| color | `MonitorColor` | The text color (RRGGBB format) |

#### `book_next_get_background_color` / `read_last_get_background_color`

Get the current background color.

```rust
pub fn book_next_get_background_color(&mut self)
pub fn read_last_get_background_color(&self) -> Result<MonitorColor, PeripheralError>
```

#### `book_next_set_background_color` / `read_last_set_background_color`

Set the background color.

```rust
pub fn book_next_set_background_color(&mut self, color: MonitorColor)
pub fn read_last_set_background_color(&self) -> Result<(), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| color | `MonitorColor` | The background color (RRGGBB format) |

#### `book_next_blit` / `read_last_blit`

Draw a string with specified text and background colors using blit.

```rust
pub fn book_next_blit(&mut self, text: &str, text_color: MonitorColor, background_color: MonitorColor)
pub fn read_last_blit(&self) -> Result<(), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| text | `&str` | The text to draw |
| text_color | `MonitorColor` | The text color |
| background_color | `MonitorColor` | The background color |

### Immediate Methods

These methods execute synchronously without needing book/read.

#### `get_text_scale_imm`

Get the text scale immediately.

```rust
pub fn get_text_scale_imm(&self) -> Result<MonitorTextScale, PeripheralError>
```

#### `get_cursor_pos_imm`

Get the cursor position immediately.

```rust
pub fn get_cursor_pos_imm(&self) -> Result<MonitorPosition, PeripheralError>
```

#### `get_cursor_blink_imm`

Get the cursor blink state immediately.

```rust
pub fn get_cursor_blink_imm(&self) -> Result<bool, PeripheralError>
```

#### `get_size_imm`

Get the monitor size immediately.

```rust
pub fn get_size_imm(&self) -> Result<MonitorSize, PeripheralError>
```

#### `get_text_color_imm`

Get the text color immediately.

```rust
pub fn get_text_color_imm(&self) -> Result<MonitorColor, PeripheralError>
```

#### `get_background_color_imm`

Get the background color immediately.

```rust
pub fn get_background_color_imm(&self) -> Result<MonitorColor, PeripheralError>
```

## Example

```rust
// Write "Hello" at position (1, 1) with white text on black background
monitor.book_next_set_text_color(MonitorColor::WHITE);
monitor.book_next_set_background_color(MonitorColor::BLACK);
monitor.book_next_set_cursor_pos(MonitorPosition { x: 1, y: 1 });
monitor.book_next_clear();
monitor.book_next_write("Hello");
wait_for_next_tick().await;

// Read monitor size immediately
let size = monitor.get_size_imm().unwrap();
```

## Types

### MonitorColor

Monitor color in RRGGBB format.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MonitorColor(pub u32);
```

**Predefined constants:**

| Constant | Value |
|----------|-------|
| `WHITE` | `0xF0F0F0` |
| `ORANGE` | `0xF2B233` |
| `MAGENTA` | `0xE57FD8` |
| `LIGHT_BLUE` | `0x99B2F2` |
| `YELLOW` | `0xDEDE6C` |
| `LIME` | `0x7FCC19` |
| `PINK` | `0xF2B2CC` |
| `GRAY` | `0x4C4C4C` |
| `LIGHT_GRAY` | `0x999999` |
| `CYAN` | `0x4C99B2` |
| `PURPLE` | `0xB266E5` |
| `BLUE` | `0x3366CC` |
| `BROWN` | `0x7F664C` |
| `GREEN` | `0x57A64E` |
| `RED` | `0xCC4C4C` |
| `BLACK` | `0x111111` |

**Helper method:**

```rust
pub fn rgb(r: u8, g: u8, b: u8) -> Self
```

### MonitorTextScale

Monitor text scale (0.5–5.0, in 0.5 increments).

```rust
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MonitorTextScale(pub f32);
```

**Predefined constants:** `SIZE_0_5`, `SIZE_1_0`, `SIZE_1_5`, `SIZE_2_0`, `SIZE_2_5`, `SIZE_3_0`, `SIZE_3_5`, `SIZE_4_0`, `SIZE_4_5`, `SIZE_5_0`

### MonitorPosition

Monitor cursor position.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MonitorPosition {
    pub x: u32,
    pub y: u32,
}
```

### MonitorSize

Monitor size in characters.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MonitorSize {
    pub x: u32,
    pub y: u32,
}
```
