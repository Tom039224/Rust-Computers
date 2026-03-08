```markdown
# Sticker

**Module:** `create`  
**Peripheral Type:** `create:sticker`

Create Sticker peripheral. Controls the extension and retraction state of a sticker (piston-like block).

## Book-Read Methods

These methods use the book-read pattern. Call `book_next_*` to schedule a request,
`wait_for_next_tick().await` to flush, then `read_last_*` to get the result.

### `book_next_is_extended` / `read_last_is_extended`

Get whether the sticker is in the extended state.

```rust
pub fn book_next_is_extended(&mut self)
pub fn read_last_is_extended(&self) -> Result<bool, PeripheralError>
```

**Returns:** `bool` — `true` if extended.

### `book_next_is_attached_to_block` / `read_last_is_attached_to_block`

Get whether the sticker is attached to a block.

```rust
pub fn book_next_is_attached_to_block(&mut self)
pub fn read_last_is_attached_to_block(&self) -> Result<bool, PeripheralError>
```

**Returns:** `bool` — `true` if attached to a block.

### `book_next_extend` / `read_last_extend`

Extend the sticker. Returns whether the operation succeeded.

```rust
pub fn book_next_extend(&mut self)
pub fn read_last_extend(&self) -> Result<bool, PeripheralError>
```

**Returns:** `bool` — `true` if the extension succeeded.

### `book_next_retract` / `read_last_retract`

Retract the sticker. Returns whether the operation succeeded.

```rust
pub fn book_next_retract(&mut self)
pub fn read_last_retract(&self) -> Result<bool, PeripheralError>
```

**Returns:** `bool` — `true` if the retraction succeeded.

### `book_next_toggle` / `read_last_toggle`

Toggle the sticker between extended and retracted. Returns whether the operation succeeded.

```rust
pub fn book_next_toggle(&mut self)
pub fn read_last_toggle(&self) -> Result<bool, PeripheralError>
```

**Returns:** `bool` — `true` if the toggle succeeded.

## Immediate Methods

### `is_extended_imm`

Get whether the sticker is extended immediately (no tick wait required).

```rust
pub fn is_extended_imm(&self) -> Result<bool, PeripheralError>
```

**Returns:** `bool` — `true` if extended.

### `is_attached_to_block_imm`

Get whether the sticker is attached to a block immediately.

```rust
pub fn is_attached_to_block_imm(&self) -> Result<bool, PeripheralError>
```

**Returns:** `bool` — `true` if attached to a block.

## Usage Example

```rust
use rust_computers_api::create::sticker::Sticker;
use rust_computers_api::peripheral::Peripheral;

let mut sticker = Sticker::wrap(addr);

// Check if extended
let extended = sticker.is_extended_imm()?;

// Extend the sticker
sticker.book_next_extend();
wait_for_next_tick().await;
let success = sticker.read_last_extend()?;

// Toggle state
sticker.book_next_toggle();
wait_for_next_tick().await;
let success = sticker.read_last_toggle()?;
```

```
