# Speedometer

**Module:** `create`  
**Peripheral Type:** `create:speedometer`

Create Speedometer peripheral. Reads the current rotation speed and listens for speed change events.

## Methods

### book/read Methods

These methods use the book-read pattern. Call `book_next_*` to schedule a request,
`wait_for_next_tick().await` to flush, then `read_last_*` to get the result.

#### `book_next_get_speed` / `read_last_get_speed`

Get the current rotation speed.

```rust
pub fn book_next_get_speed(&mut self)
pub fn read_last_get_speed(&self) -> Result<f32, PeripheralError>
```

### Immediate Methods

#### `get_speed_imm`

Get the current speed immediately.

```rust
pub fn get_speed_imm(&self) -> Result<f32, PeripheralError>
```

### Async Methods (Event)

#### `book_next_try_pull_speed_change` / `read_last_try_pull_speed_change`

Try to pull a speed change event (non-blocking). Returns the new speed.

```rust
pub fn book_next_try_pull_speed_change(&mut self)
pub fn read_last_try_pull_speed_change(&self) -> Result<Option<f32>, PeripheralError>
```

#### `pull_speed_change`

Wait for a speed change event (blocking async loop). Returns the new speed.

```rust
pub async fn pull_speed_change(&self) -> Result<f32, PeripheralError>
```

## Example

```rust
use rust_computers_api::create::speedometer::Speedometer;
use rust_computers_api::peripheral::Peripheral;

let mut speedo = Speedometer::wrap(addr);

// Read current speed immediately
let speed = speedo.get_speed_imm()?;

// Wait for speed to change
let new_speed = speedo.pull_speed_change().await?;
```
