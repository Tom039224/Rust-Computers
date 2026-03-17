```markdown
# SequencedGearshift

**Module:** `create`  
**Peripheral Type:** `create:sequenced_gearshift`

Create Sequenced Gearshift peripheral. Controls sequenced rotation and linear movement with optional speed modifiers.

## Book-Read Methods

These methods use the book-read pattern. Call `book_next_*` to schedule a request,
`wait_for_next_tick().await` to flush, then `read_last_*` to get the result.

### `book_next_rotate` / `read_last_rotate`

Rotate by the specified amount.

```rust
pub fn book_next_rotate(&mut self, amount: i32, speed_modifier: Option<i32>)
pub fn read_last_rotate(&self) -> Result<(), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `amount` | `i32` | The rotation amount |
| `speed_modifier` | `Option<i32>` | Optional speed modifier |

### `book_next_move_by` / `read_last_move_by`

Move by the specified distance.

```rust
pub fn book_next_move_by(&mut self, distance: i32, speed_modifier: Option<i32>)
pub fn read_last_move_by(&self) -> Result<(), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `distance` | `i32` | The movement distance |
| `speed_modifier` | `Option<i32>` | Optional speed modifier |

### `book_next_is_running` / `read_last_is_running`

Get whether the gearshift is currently running.

```rust
pub fn book_next_is_running(&mut self)
pub fn read_last_is_running(&self) -> Result<bool, PeripheralError>
```

**Returns:** `bool` — `true` if currently running.

## Implementation Status

### ✅ Implemented

- All book_next_* / read_last_* methods

### 🚧 Not Yet Implemented

- async_* variants for all methods


## Immediate Methods

### `is_running_imm`

Get whether the gearshift is currently running immediately (no tick wait required).

```rust
pub fn is_running_imm(&self) -> Result<bool, PeripheralError>
```

**Returns:** `bool` — `true` if currently running.

## Usage Example

```rust
use rust_computers_api::create::sequenced_gearshift::SequencedGearshift;
use rust_computers_api::peripheral::Peripheral;

let mut gearshift = SequencedGearshift::wrap(addr);

// Rotate by 90 degrees with speed modifier
gearshift.book_next_rotate(90, Some(2));
wait_for_next_tick().await;
gearshift.read_last_rotate()?;

// Move by 5 blocks
gearshift.book_next_move_by(5, None);
wait_for_next_tick().await;
gearshift.read_last_move_by()?;

// Check if running immediately
let running = gearshift.is_running_imm()?;
```

```
