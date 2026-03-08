```markdown
# Speedometer

**Module:** `create`  
**Peripheral Type:** `create:speedometer`

Create Speedometer peripheral. Reads the current rotation speed and provides speed change events.

## Book-Read Methods

These methods use the book-read pattern. Call `book_next_*` to schedule a request,
`wait_for_next_tick().await` to flush, then `read_last_*` to get the result.

### `book_next_get_speed` / `read_last_get_speed`

Get the current rotation speed.

```rust
pub fn book_next_get_speed(&mut self)
pub fn read_last_get_speed(&self) -> Result<f32, PeripheralError>
```

**Returns:** `f32` — current rotation speed (RPM).

## Immediate Methods

### `get_speed_imm`

Get the current rotation speed immediately (no tick wait required).

```rust
pub fn get_speed_imm(&self) -> Result<f32, PeripheralError>
```

**Returns:** `f32` — current rotation speed (RPM).

## Event-Wait Methods

### `book_next_try_pull_speed_change` / `read_last_try_pull_speed_change`

Wait 1 tick for a speed change event. Returns `None` if no event occurred.

```rust
pub fn book_next_try_pull_speed_change(&mut self)
pub fn read_last_try_pull_speed_change(&self) -> Result<Option<f32>, PeripheralError>
```

**Returns:** `Option<f32>` — the new speed if an event occurred, or `None`.

### `pull_speed_change`

Async method that waits until a speed change event is received.

```rust
pub async fn pull_speed_change(&self) -> Result<f32, PeripheralError>
```

**Returns:** `f32` — the new speed.

## Usage Example

```rust
use rust_computers_api::create::speedometer::Speedometer;
use rust_computers_api::peripheral::Peripheral;

let mut speedometer = Speedometer::wrap(addr);

// Read speed immediately
let speed = speedometer.get_speed_imm()?;

// Or use book-read pattern
speedometer.book_next_get_speed();
wait_for_next_tick().await;
let speed = speedometer.read_last_get_speed()?;

// Wait for speed change event
let new_speed = speedometer.pull_speed_change().await?;
```

```
