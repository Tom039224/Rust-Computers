# SequencedGearshift

**Module:** `create`  
**Peripheral Type:** `create:sequenced_gearshift`

Create Sequenced Gearshift peripheral. Controls rotation and linear movement sequences.

## Methods

### book/read Methods

These methods use the book-read pattern. Call `book_next_*` to schedule a request,
`wait_for_next_tick().await` to flush, then `read_last_*` to get the result.

#### `book_next_rotate` / `read_last_rotate`

Rotate by a specified amount. Optionally specify a speed modifier.

```rust
pub fn book_next_rotate(&mut self, amount: i32, speed_modifier: Option<i32>)
pub fn read_last_rotate(&self) -> Result<(), PeripheralError>
```

#### `book_next_move_by` / `read_last_move_by`

Move a specified distance. Optionally specify a speed modifier.

```rust
pub fn book_next_move_by(&mut self, distance: i32, speed_modifier: Option<i32>)
pub fn read_last_move_by(&self) -> Result<(), PeripheralError>
```

#### `book_next_is_running` / `read_last_is_running`

Check if the gearshift is currently running a sequence.

```rust
pub fn book_next_is_running(&mut self)
pub fn read_last_is_running(&self) -> Result<bool, PeripheralError>
```

### Immediate Methods

#### `is_running_imm`

Check if running immediately.

```rust
pub fn is_running_imm(&self) -> Result<bool, PeripheralError>
```

## Example

```rust
use rust_computers_api::create::sequenced_gearshift::SequencedGearshift;
use rust_computers_api::peripheral::Peripheral;

let mut gearshift = SequencedGearshift::wrap(addr);

// Rotate 90 degrees
gearshift.book_next_rotate(90, None);
wait_for_next_tick().await;
gearshift.read_last_rotate()?;

// Wait until done
loop {
    let running = gearshift.read_last_is_running();
    gearshift.book_next_is_running();
    wait_for_next_tick().await;

    if let Ok(false) = running {
        break;
    }
}
```
