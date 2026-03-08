# Stressometer

**Module:** `create`  
**Peripheral Type:** `create:stressometer`

Create Stressometer peripheral. Monitors kinetic stress and capacity, with events for overstress and changes.

## Methods

### book/read Methods

These methods use the book-read pattern. Call `book_next_*` to schedule a request,
`wait_for_next_tick().await` to flush, then `read_last_*` to get the result.

#### `book_next_get_stress` / `read_last_get_stress`

Get the current stress value.

```rust
pub fn book_next_get_stress(&mut self)
pub fn read_last_get_stress(&self) -> Result<f32, PeripheralError>
```

#### `book_next_get_stress_capacity` / `read_last_get_stress_capacity`

Get the stress capacity.

```rust
pub fn book_next_get_stress_capacity(&mut self)
pub fn read_last_get_stress_capacity(&self) -> Result<f32, PeripheralError>
```

### Immediate Methods

#### `get_stress_imm`

```rust
pub fn get_stress_imm(&self) -> Result<f32, PeripheralError>
```

#### `get_stress_capacity_imm`

```rust
pub fn get_stress_capacity_imm(&self) -> Result<f32, PeripheralError>
```

### Async Methods (Event)

#### `book_next_try_pull_overstressed` / `read_last_try_pull_overstressed`

Try to pull an overstress event (non-blocking).

```rust
pub fn book_next_try_pull_overstressed(&mut self)
pub fn read_last_try_pull_overstressed(&self) -> Result<Option<()>, PeripheralError>
```

#### `pull_overstressed`

Wait for an overstress event (blocking async loop).

```rust
pub async fn pull_overstressed(&self) -> Result<(), PeripheralError>
```

#### `book_next_try_pull_stress_change` / `read_last_try_pull_stress_change`

Try to pull a stress change event (non-blocking). Returns `(stress, capacity)`.

```rust
pub fn book_next_try_pull_stress_change(&mut self)
pub fn read_last_try_pull_stress_change(&self) -> Result<Option<(f32, f32)>, PeripheralError>
```

#### `pull_stress_change`

Wait for a stress change event (blocking async loop). Returns `(stress, capacity)`.

```rust
pub async fn pull_stress_change(&self) -> Result<(f32, f32), PeripheralError>
```

## Example

```rust
use rust_computers_api::create::stressometer::Stressometer;
use rust_computers_api::peripheral::Peripheral;

let mut stress = Stressometer::wrap(addr);

// Read stress immediately
let current = stress.get_stress_imm()?;
let capacity = stress.get_stress_capacity_imm()?;

// Wait for stress change
let (new_stress, new_cap) = stress.pull_stress_change().await?;
```
