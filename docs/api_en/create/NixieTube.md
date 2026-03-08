# NixieTube

**Module:** `create`  
**Peripheral Type:** `create:nixie_tube`

Create Nixie Tube peripheral. Controls the text, colour, and signal display of nixie tubes.

## Book-Read Methods

These methods use the book-read pattern. Call `book_next_*` to schedule a request,
`wait_for_next_tick().await` to flush, then `read_last_*` to get the result.

### `book_next_set_text` / `read_last_set_text`

Set the displayed text. Optionally specify a colour.

```rust
pub fn book_next_set_text(&mut self, text: &str, colour: Option<&str>)
pub fn read_last_set_text(&self) -> Result<(), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `text` | `&str` | Text to display |
| `colour` | `Option<&str>` | Optional colour name (e.g. `"red"`, `"blue"`) |

### `book_next_set_text_colour` / `read_last_set_text_colour`

Set the text colour.

```rust
pub fn book_next_set_text_colour(&mut self, colour: &str)
pub fn read_last_set_text_colour(&self) -> Result<(), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `colour` | `&str` | Colour name to set |

### `book_next_set_signal` / `read_last_set_signal`

Set the signal display parameters. Front signal is required; back signal is optional.

```rust
pub fn book_next_set_signal(&mut self, front: &CRSignalParams, back: Option<&CRSignalParams>) -> Result<(), PeripheralError>
pub fn read_last_set_signal(&self) -> Result<(), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `front` | `&CRSignalParams` | Front signal parameters |
| `back` | `Option<&CRSignalParams>` | Optional back signal parameters |

## Types

### `CRSignalParams`

```rust
pub struct CRSignalParams {
    pub r: Option<u8>,
    pub g: Option<u8>,
    pub b: Option<u8>,
    pub glow_width: Option<u8>,
    pub glow_height: Option<u8>,
    pub blink_period: Option<u8>,
    pub blink_off_time: Option<u8>,
}
```

## Usage Example

```rust
use rust_computers_api::create::nixie_tube::NixieTube;
use rust_computers_api::create::common::CRSignalParams;
use rust_computers_api::peripheral::Peripheral;

let mut nixie = NixieTube::wrap(addr);

// Set text with colour
nixie.book_next_set_text("42", Some("orange"));
wait_for_next_tick().await;
nixie.read_last_set_text()?;

// Set signal display
let front = CRSignalParams {
    r: Some(255), g: Some(128), b: Some(0),
    glow_width: None, glow_height: None,
    blink_period: None, blink_off_time: None,
};
nixie.book_next_set_signal(&front, None)?;
wait_for_next_tick().await;
nixie.read_last_set_signal()?;
```
