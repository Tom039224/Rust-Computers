# Signal

**Module:** `create`  
**Peripheral Type:** `create:signal`

Create Signal peripheral. Monitors and controls train signal state.

## Methods

### book/read Methods

These methods use the book-read pattern. Call `book_next_*` to schedule a request,
`wait_for_next_tick().await` to flush, then `read_last_*` to get the result.

#### `book_next_get_state` / `read_last_get_state`

Get the current signal state.

```rust
pub fn book_next_get_state(&mut self)
pub fn read_last_get_state(&self) -> Result<String, PeripheralError>
```

#### `book_next_is_forced_red` / `read_last_is_forced_red`

Check if the signal is forced red.

```rust
pub fn book_next_is_forced_red(&mut self)
pub fn read_last_is_forced_red(&self) -> Result<bool, PeripheralError>
```

#### `book_next_set_forced_red` / `read_last_set_forced_red`

Set or unset forced red state.

```rust
pub fn book_next_set_forced_red(&mut self, powered: bool)
pub fn read_last_set_forced_red(&self) -> Result<(), PeripheralError>
```

#### `book_next_list_blocking_train_names` / `read_last_list_blocking_train_names`

List the names of trains blocking this signal.

```rust
pub fn book_next_list_blocking_train_names(&mut self)
pub fn read_last_list_blocking_train_names(&self) -> Result<Vec<String>, PeripheralError>
```

#### `book_next_get_signal_type` / `read_last_get_signal_type`

Get the signal type.

```rust
pub fn book_next_get_signal_type(&mut self)
pub fn read_last_get_signal_type(&self) -> Result<String, PeripheralError>
```

#### `book_next_cycle_signal_type` / `read_last_cycle_signal_type`

Cycle through signal types.

```rust
pub fn book_next_cycle_signal_type(&mut self)
pub fn read_last_cycle_signal_type(&self) -> Result<(), PeripheralError>
```

### Immediate Methods

#### `get_state_imm`

```rust
pub fn get_state_imm(&self) -> Result<String, PeripheralError>
```

#### `is_forced_red_imm`

```rust
pub fn is_forced_red_imm(&self) -> Result<bool, PeripheralError>
```

#### `list_blocking_train_names_imm`

```rust
pub fn list_blocking_train_names_imm(&self) -> Result<Vec<String>, PeripheralError>
```

#### `get_signal_type_imm`

```rust
pub fn get_signal_type_imm(&self) -> Result<String, PeripheralError>
```

### Async Methods (Event)

#### `book_next_try_pull_train_signal_state_change` / `read_last_try_pull_train_signal_state_change`

Try to pull a signal state change event (non-blocking).

```rust
pub fn book_next_try_pull_train_signal_state_change(&mut self)
pub fn read_last_try_pull_train_signal_state_change(&self) -> Result<Option<()>, PeripheralError>
```

#### `pull_train_signal_state_change`

Wait for a signal state change event (blocking async loop).

```rust
pub async fn pull_train_signal_state_change(&self) -> Result<(), PeripheralError>
```

## Example

```rust
use rust_computers_api::create::signal::Signal;
use rust_computers_api::peripheral::Peripheral;

let mut signal = Signal::wrap(addr);

// Force red
signal.book_next_set_forced_red(true);
wait_for_next_tick().await;
signal.read_last_set_forced_red()?;

// Wait for state change
signal.pull_train_signal_state_change().await?;
let state = signal.get_state_imm()?;
```
