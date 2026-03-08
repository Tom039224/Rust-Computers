# NixieTube

**Module:** `create`  
**Peripheral Type:** `create:nixie_tube`

Create Nixie Tube peripheral. Displays text and signal information on nixie tube displays.

## Methods

### book/read Methods

These methods use the book-read pattern. Call `book_next_*` to schedule a request,
`wait_for_next_tick().await` to flush, then `read_last_*` to get the result.

#### `book_next_set_text` / `read_last_set_text`

Set the displayed text. Optionally specify a color.

```rust
pub fn book_next_set_text(&mut self, text: &str, colour: Option<&str>)
pub fn read_last_set_text(&self) -> Result<(), PeripheralError>
```

#### `book_next_set_text_colour` / `read_last_set_text_colour`

Set the text color.

```rust
pub fn book_next_set_text_colour(&mut self, colour: &str)
pub fn read_last_set_text_colour(&self) -> Result<(), PeripheralError>
```

#### `book_next_set_signal` / `read_last_set_signal`

Set signal display. `front` is required, `back` is optional.

```rust
pub fn book_next_set_signal(&mut self, front: &CRSignalParams, back: Option<&CRSignalParams>) -> Result<(), PeripheralError>
pub fn read_last_set_signal(&self) -> Result<(), PeripheralError>
```

## Example

```rust
use rust_computers_api::create::nixie_tube::NixieTube;
use rust_computers_api::peripheral::Peripheral;

let mut nixie = NixieTube::wrap(addr);

// Display text
nixie.book_next_set_text("42", None);
wait_for_next_tick().await;
nixie.read_last_set_text()?;

// Set color
nixie.book_next_set_text_colour("red");
wait_for_next_tick().await;
nixie.read_last_set_text_colour()?;
```
