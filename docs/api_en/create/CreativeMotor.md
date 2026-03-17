# CreativeMotor

**Module:** `create`  
**Peripheral Type:** `create:creative_motor`

Create Creative Motor peripheral. Controls the generated rotation speed of a creative motor.

## Book-Read Methods

These methods use the book-read pattern. Call `book_next_*` to schedule a request,
`wait_for_next_tick().await` to flush, then `read_last_*` to get the result.

### `book_next_set_generated_speed` / `read_last_set_generated_speed`

Set the generated rotation speed.

```rust
pub fn book_next_set_generated_speed(&mut self, speed: i32)
pub fn read_last_set_generated_speed(&self) -> Result<(), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `speed` | `i32` | The rotation speed to generate (RPM) |

### `book_next_get_generated_speed` / `read_last_get_generated_speed`

Get the current generated rotation speed.

```rust
pub fn book_next_get_generated_speed(&mut self)
pub fn read_last_get_generated_speed(&self) -> Result<f32, PeripheralError>
```

**Returns:** `f32` — current generated speed (RPM).

## Implementation Status

### ✅ Implemented

- All book_next_* / read_last_* methods

### 🚧 Not Yet Implemented

- async_* variants for all methods


## Immediate Methods

### `get_generated_speed_imm`

Get the current generated rotation speed immediately (no tick wait required).

```rust
pub fn get_generated_speed_imm(&self) -> Result<f32, PeripheralError>
```

**Returns:** `f32` — current generated speed (RPM).

## Usage Example

```rust
use rust_computers_api::create::creative_motor::CreativeMotor;
use rust_computers_api::peripheral::Peripheral;

let mut motor = CreativeMotor::wrap(addr);

// Set speed to 256 RPM
motor.book_next_set_generated_speed(256);
wait_for_next_tick().await;
motor.read_last_set_generated_speed()?;

// Read speed using book-read pattern
motor.book_next_get_generated_speed();
wait_for_next_tick().await;
let speed = motor.read_last_get_generated_speed()?;

// Or read speed immediately
let speed = motor.get_generated_speed_imm()?;
```
