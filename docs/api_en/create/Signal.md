```markdown
# Signal

**Module:** `create`  
**Peripheral Type:** `create:signal`

Create Signal peripheral. Monitors and controls train signal state, blocking trains, and signal type.

## Book-Read Methods

These methods use the book-read pattern. Call `book_next_*` to schedule a request,
`wait_for_next_tick().await` to flush, then `read_last_*` to get the result.

### `book_next_get_state` / `read_last_get_state`

Get the current signal state.

```rust
pub fn book_next_get_state(&mut self)
pub fn read_last_get_state(&self) -> Result<String, PeripheralError>
```

**Returns:** `String` — the current signal state (e.g. `"green"`, `"yellow"`, `"red"`).

### `book_next_is_forced_red` / `read_last_is_forced_red`

Get whether the signal is forced to red.

```rust
pub fn book_next_is_forced_red(&mut self)
pub fn read_last_is_forced_red(&self) -> Result<bool, PeripheralError>
```

**Returns:** `bool` — `true` if forced red.

### `book_next_set_forced_red` / `read_last_set_forced_red`

Set the forced red state of the signal.

```rust
pub fn book_next_set_forced_red(&mut self, powered: bool)
pub fn read_last_set_forced_red(&self) -> Result<(), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `powered` | `bool` | `true` to force the signal to red |

### `book_next_list_blocking_train_names` / `read_last_list_blocking_train_names`

Get the list of train names blocking this signal's section.

```rust
pub fn book_next_list_blocking_train_names(&mut self)
pub fn read_last_list_blocking_train_names(&self) -> Result<Vec<String>, PeripheralError>
```

**Returns:** `Vec<String>` — list of blocking train names.

### `book_next_get_signal_type` / `read_last_get_signal_type`

Get the signal type.

```rust
pub fn book_next_get_signal_type(&mut self)
pub fn read_last_get_signal_type(&self) -> Result<String, PeripheralError>
```

**Returns:** `String` — the signal type.

### `book_next_cycle_signal_type` / `read_last_cycle_signal_type`

Cycle to the next signal type.

```rust
pub fn book_next_cycle_signal_type(&mut self)
pub fn read_last_cycle_signal_type(&self) -> Result<(), PeripheralError>
```

## Immediate Methods

### `get_state_imm`

Get the current signal state immediately (no tick wait required).

```rust
pub fn get_state_imm(&self) -> Result<String, PeripheralError>
```

**Returns:** `String` — the current signal state.

### `is_forced_red_imm`

Get whether the signal is forced to red immediately.

```rust
pub fn is_forced_red_imm(&self) -> Result<bool, PeripheralError>
```

**Returns:** `bool` — `true` if forced red.

### `list_blocking_train_names_imm`

Get the list of blocking train names immediately.

```rust
pub fn list_blocking_train_names_imm(&self) -> Result<Vec<String>, PeripheralError>
```

**Returns:** `Vec<String>` — list of blocking train names.

### `get_signal_type_imm`

Get the signal type immediately.

```rust
pub fn get_signal_type_imm(&self) -> Result<String, PeripheralError>
```

**Returns:** `String` — the signal type.

## Event-Wait Methods

### `book_next_try_pull_train_signal_state_change` / `read_last_try_pull_train_signal_state_change`

Wait 1 tick for a signal state change event. Returns `None` if no event occurred.

```rust
pub fn book_next_try_pull_train_signal_state_change(&mut self)
pub fn read_last_try_pull_train_signal_state_change(&self) -> Result<Option<()>, PeripheralError>
```

### `pull_train_signal_state_change`

Async method that waits until a signal state change event is received.

```rust
pub async fn pull_train_signal_state_change(&self) -> Result<(), PeripheralError>
```

## Usage Example

```rust
use rust_computers_api::create::signal::Signal;
use rust_computers_api::peripheral::Peripheral;

let mut signal = Signal::wrap(addr);

// Get signal state immediately
let state = signal.get_state_imm()?;

// Force signal to red
signal.book_next_set_forced_red(true);
wait_for_next_tick().await;
signal.read_last_set_forced_red()?;

// List blocking trains
let trains = signal.list_blocking_train_names_imm()?;

// Wait for signal state change
signal.pull_train_signal_state_change().await?;
```

```
