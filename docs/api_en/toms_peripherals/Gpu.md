# GPU

**Module:** `toms_peripherals`  
**Peripheral Type:** `tm:gpu`

Toms-Peripherals GPU peripheral for pixel-level rendering on monitors.

## Types

```rust
pub struct TMImage {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u32>,
}

pub struct TMWindow {
    pub x: f64,
    pub y: f64,
    pub width: u32,
    pub height: u32,
}
```

## Implementation Status

### ✅ Implemented

- All book_next_* / read_last_* methods

### 🚧 Not Yet Implemented

- async_* variants for all methods


## Methods

### Async Action Methods

#### `set_size`

Set the pixel size.

```rust
pub async fn set_size(&self, pixels: u32) -> Result<(), PeripheralError>
```

#### `refresh_size`

Refresh size information.

```rust
pub async fn refresh_size(&self) -> Result<(), PeripheralError>
```

#### `fill`

Fill the screen with an RGBA color.

```rust
pub async fn fill(&self, r: f32, g: f32, b: f32, a: f32) -> Result<(), PeripheralError>
```

#### `sync`

Synchronize the screen (flush drawing commands).

```rust
pub async fn sync(&self) -> Result<(), PeripheralError>
```

#### `filled_rectangle`

Draw a filled rectangle with an RGBA color.

```rust
pub async fn filled_rectangle(&self, x: u32, y: u32, w: u32, h: u32, r: f32, g: f32, b: f32, a: f32) -> Result<(), PeripheralError>
```

#### `draw_image`

Draw an image at a position.

```rust
pub async fn draw_image(&self, image: &TMImage, x: u32, y: u32) -> Result<(), PeripheralError>
```

#### `draw_text`

Draw text at a position with an RGBA color.

```rust
pub async fn draw_text(&self, text: &str, x: u32, y: u32, r: f32, g: f32, b: f32, a: f32) -> Result<(), PeripheralError>
```

#### `draw_char`

Draw a single character at a position with an RGBA color.

```rust
pub async fn draw_char(&self, ch: char, x: u32, y: u32, r: f32, g: f32, b: f32, a: f32) -> Result<(), PeripheralError>
```

#### `set_font`

Set the font.

```rust
pub async fn set_font(&self, font_name: &str) -> Result<(), PeripheralError>
```

#### `clear_chars`

Clear custom characters.

```rust
pub async fn clear_chars(&self) -> Result<(), PeripheralError>
```

#### `add_new_char`

Add a custom character.

```rust
pub async fn add_new_char(&self, codepoint: u32, data: &[u8]) -> Result<(), PeripheralError>
```

#### `decode_image`

Decode an image from a Base64 string.

```rust
pub async fn decode_image(&self, data: &str) -> Result<TMImage, PeripheralError>
```

### Async Methods (with imm variants)

#### `get_size` / `get_size_imm`

Get size info. Returns `(pixel_width, height, monitor_cols, rows, pixel_size)`.

```rust
pub async fn get_size(&self) -> Result<(u32, u32, u32, u32, u32), PeripheralError>
pub fn get_size_imm(&self) -> Result<(u32, u32, u32, u32, u32), PeripheralError>
```

#### `get_text_length` / `get_text_length_imm`

Get the rendered length of a text string in pixels.

```rust
pub async fn get_text_length(&self, text: &str) -> Result<u32, PeripheralError>
pub fn get_text_length_imm(&self, text: &str) -> Result<u32, PeripheralError>
```

#### `create_window` / `create_window_imm`

Create a rendering window.

```rust
pub async fn create_window(&self, x: u32, y: u32, w: u32, h: u32) -> Result<TMWindow, PeripheralError>
pub fn create_window_imm(&self, x: u32, y: u32, w: u32, h: u32) -> Result<TMWindow, PeripheralError>
```

#### `new_image` / `new_image_imm`

Create a new empty image.

```rust
pub async fn new_image(&self, w: u32, h: u32) -> Result<TMImage, PeripheralError>
pub fn new_image_imm(&self, w: u32, h: u32) -> Result<TMImage, PeripheralError>
```

## Example

```rust
use rust_computers_api::toms_peripherals::gpu::GPU;
use rust_computers_api::peripheral::Peripheral;

let gpu = GPU::wrap(addr);

// Clear screen to black
gpu.fill(0.0, 0.0, 0.0, 1.0).await?;

// Draw a red rectangle
gpu.filled_rectangle(10, 10, 100, 50, 1.0, 0.0, 0.0, 1.0).await?;

// Draw text
gpu.draw_text("Hello!", 10, 70, 1.0, 1.0, 1.0, 1.0).await?;

// Synchronize (flush to screen)
gpu.sync().await?;
```
