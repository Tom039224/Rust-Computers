# Sticker

**Module:** `create`  
**Peripheral Type:** `create:sticker`

Create Sticker peripheral. Controls and monitors a sticky mechanical piston (Sticker).

## Methods

### book/read Methods

These methods use the book-read pattern. Call `book_next_*` to schedule a request,
`wait_for_next_tick().await` to flush, then `read_last_*` to get the result.

#### `book_next_is_extended` / `read_last_is_extended`

Check if the sticker is extended.

```rust
pub fn book_next_is_extended(&mut self)
pub fn read_last_is_extended(&self) -> Result<bool, PeripheralError>
```

#### `book_next_is_attached_to_block` / `read_last_is_attached_to_block`

Check if the sticker is attached to a block.

```rust
pub fn book_next_is_attached_to_block(&mut self)
pub fn read_last_is_attached_to_block(&self) -> Result<bool, PeripheralError>
```

#### `book_next_extend` / `read_last_extend`

Extend the sticker. Returns whether the operation succeeded.

```rust
pub fn book_next_extend(&mut self)
pub fn read_last_extend(&self) -> Result<bool, PeripheralError>
```

#### `book_next_retract` / `read_last_retract`

Retract the sticker. Returns whether the operation succeeded.

```rust
pub fn book_next_retract(&mut self)
pub fn read_last_retract(&self) -> Result<bool, PeripheralError>
```

#### `book_next_toggle` / `read_last_toggle`

Toggle the sticker extension/retraction. Returns whether the operation succeeded.

```rust
pub fn book_next_toggle(&mut self)
pub fn read_last_toggle(&self) -> Result<bool, PeripheralError>
```

### Immediate Methods

#### `is_extended_imm`

```rust
pub fn is_extended_imm(&self) -> Result<bool, PeripheralError>
```

#### `is_attached_to_block_imm`

```rust
pub fn is_attached_to_block_imm(&self) -> Result<bool, PeripheralError>
```

## Example

```rust
use rust_computers_api::create::sticker::Sticker;
use rust_computers_api::peripheral::Peripheral;

let mut sticker = Sticker::wrap(addr);

// Toggle the sticker
sticker.book_next_toggle();
wait_for_next_tick().await;
let success = sticker.read_last_toggle()?;

// Check state
let extended = sticker.is_extended_imm()?;
```
