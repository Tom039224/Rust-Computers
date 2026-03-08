# RotationSpeedController

**Module:** `create`  
**Peripheral Type:** `create:rotation_speed_controller`

Create Rotation Speed Controller peripheral. Controls the target rotation speed of a rotation speed controller.

## Book-Read Methods

These methods use the book-read pattern. Call `book_next_*` to schedule a request,
`wait_for_next_tick().await` to flush, then `read_last_*` to get the result.

### `book_next_set_target_speed` / `read_last_set_target_speed`

Set the target rotation speed.

```rust
pub fn book_next_set_target_speed(&mut self, speed: i32)
pub fn read_last_set_target_speed(&self) -> Result<(), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `speed` | `i32` | Target rotation speed (RPM) |

### `book_next_get_target_speed` / `read_last_get_target_speed`

Get the current target rotation speed.

```rust
pub fn book_next_get_target_speed(&mut self)
pub fn read_last_get_target_speed(&self) -> Result<f32, PeripheralError>
```

**Returns:** `f32` — current target speed (RPM).

## Immediate Methods

### `get_target_speed_imm`

Get the current target rotation speed immediately (no tick wait required).

```rust
pub fn get_target_speed_imm(&self) -> Result<f32, PeripheralError>
```

**Returns:** `f32` — current target speed (RPM).

## Usage Example

```rust
use rust_computers_api::create::rotation_speed_controller::RotationSpeedController;
use rust_computers_api::peripheral::Peripheral;

let mut controller = RotationSpeedController::wrap(addr);

// Set target speed to 128 RPM
controller.book_next_set_target_speed(128);
wait_for_next_tick().await;
controller.read_last_set_target_speed()?;

// Read target speed using book-read pattern
controller.book_next_get_target_speed();
wait_for_next_tick().await;
let speed = controller.read_last_get_target_speed()?;

// Or read target speed immediately
let speed = controller.get_target_speed_imm()?;
```
