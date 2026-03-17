```markdown
# Stressometer

**Module:** `create`  
**Peripheral Type:** `create:stressometer`

Create Stressometer peripheral. Monitors kinetic stress and capacity with overstress and stress change events.

## Book-Read Methods

These methods use the book-read pattern. Call `book_next_*` to schedule a request,
`wait_for_next_tick().await` to flush, then `read_last_*` to get the result.

### `book_next_get_stress` / `read_last_get_stress`

Get the current stress value.

```rust
pub fn book_next_get_stress(&mut self)
pub fn read_last_get_stress(&self) -> Result<f32, PeripheralError>
```

**Returns:** `f32` — current stress value (SU).

### `book_next_get_stress_capacity` / `read_last_get_stress_capacity`

Get the stress capacity.

```rust
pub fn book_next_get_stress_capacity(&mut self)
pub fn read_last_get_stress_capacity(&self) -> Result<f32, PeripheralError>
```

**Returns:** `f32` — stress capacity (SU).

## Implementation Status

### ✅ Implemented

- All book_next_* / read_last_* methods

### 🚧 Not Yet Implemented

- async_* variants for all methods


## Immediate Methods

### `get_stress_imm`

Get the current stress value immediately (no tick wait required).

```rust
pub fn get_stress_imm(&self) -> Result<f32, PeripheralError>
```

**Returns:** `f32` — current stress value (SU).

### `get_stress_capacity_imm`

Get the stress capacity immediately.

```rust
pub fn get_stress_capacity_imm(&self) -> Result<f32, PeripheralError>
```

**Returns:** `f32` — stress capacity (SU).

## Event-Wait Methods

### `book_next_try_pull_overstressed` / `read_last_try_pull_overstressed`

Wait 1 tick for an overstressed event. Returns `None` if no event occurred.

```rust
pub fn book_next_try_pull_overstressed(&mut self)
pub fn read_last_try_pull_overstressed(&self) -> Result<Option<()>, PeripheralError>
```

### `pull_overstressed`

Async method that waits until an overstressed event is received.

```rust
pub async fn pull_overstressed(&self) -> Result<(), PeripheralError>
```

### `book_next_try_pull_stress_change` / `read_last_try_pull_stress_change`

Wait 1 tick for a stress change event. Returns `None` if no event occurred.

```rust
pub fn book_next_try_pull_stress_change(&mut self)
pub fn read_last_try_pull_stress_change(&self) -> Result<Option<(f32, f32)>, PeripheralError>
```

**Returns:** `Option<(f32, f32)>` — tuple of (stress, capacity) if an event occurred, or `None`.

### `pull_stress_change`

Async method that waits until a stress change event is received.

```rust
pub async fn pull_stress_change(&self) -> Result<(f32, f32), PeripheralError>
```

**Returns:** `(f32, f32)` — tuple of (stress, capacity).

## Usage Example

```rust
use rust_computers_api::create::stressometer::Stressometer;
use rust_computers_api::peripheral::Peripheral;

let mut stressometer = Stressometer::wrap(addr);

// Read stress immediately
let stress = stressometer.get_stress_imm()?;
let capacity = stressometer.get_stress_capacity_imm()?;

// Wait for stress change
let (new_stress, new_capacity) = stressometer.pull_stress_change().await?;

// Wait for overstress event
stressometer.pull_overstressed().await?;
```

```
