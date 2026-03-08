# RotationSpeedController

**Module:** `create`  
**Peripheral Type:** `create:rotation_speed_controller`

Create Rotation Speed Controller peripheral. Controls the target rotation speed.

## Methods

### book/read Methods

These methods use the book-read pattern. Call `book_next_*` to schedule a request,
`wait_for_next_tick().await` to flush, then `read_last_*` to get the result.

#### `book_next_set_target_speed` / `read_last_set_target_speed`

Set the target rotation speed.

```rust
pub fn book_next_set_target_speed(&mut self, speed: i32)
pub fn read_last_set_target_speed(&self) -> Result<(), PeripheralError>
```

#### `book_next_get_target_speed` / `read_last_get_target_speed`

Get the current target speed.

```rust
pub fn book_next_get_target_speed(&mut self)
pub fn read_last_get_target_speed(&self) -> Result<f32, PeripheralError>
```

### Immediate Methods

#### `get_target_speed_imm`

Get the current target speed immediately.

```rust
pub fn get_target_speed_imm(&self) -> Result<f32, PeripheralError>
```

## Example

```rust
use rust_computers_api::create::rotation_speed_controller::RotationSpeedController;
use rust_computers_api::peripheral::Peripheral;

let mut ctrl = RotationSpeedController::wrap(addr);

// Set target speed
ctrl.book_next_set_target_speed(128);
wait_for_next_tick().await;
ctrl.read_last_set_target_speed()?;

// Read target speed
loop {
    let speed = ctrl.read_last_get_target_speed();
    ctrl.book_next_get_target_speed();
    wait_for_next_tick().await;
}
```
