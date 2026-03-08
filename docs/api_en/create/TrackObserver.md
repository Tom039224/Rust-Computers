```markdown
# TrackObserver

**Module:** `create`  
**Peripheral Type:** `create:track_observer`

Create Track Observer peripheral. Monitors train passage on a track with real-time events.

## Book-Read Methods

These methods use the book-read pattern. Call `book_next_*` to schedule a request,
`wait_for_next_tick().await` to flush, then `read_last_*` to get the result.

### `book_next_is_train_passing` / `read_last_is_train_passing`

Get whether a train is currently passing.

```rust
pub fn book_next_is_train_passing(&mut self)
pub fn read_last_is_train_passing(&self) -> Result<bool, PeripheralError>
```

**Returns:** `bool` — `true` if a train is currently passing.

### `book_next_get_passing_train_name` / `read_last_get_passing_train_name`

Get the name of the currently passing train.

```rust
pub fn book_next_get_passing_train_name(&mut self)
pub fn read_last_get_passing_train_name(&self) -> Result<Option<String>, PeripheralError>
```

**Returns:** `Option<String>` — the train name, or `None` if no train is passing.

## Immediate Methods

### `is_train_passing_imm`

Get whether a train is currently passing immediately (no tick wait required).

```rust
pub fn is_train_passing_imm(&self) -> Result<bool, PeripheralError>
```

**Returns:** `bool` — `true` if a train is currently passing.

### `get_passing_train_name_imm`

Get the name of the currently passing train immediately.

```rust
pub fn get_passing_train_name_imm(&self) -> Result<Option<String>, PeripheralError>
```

**Returns:** `Option<String>` — the train name, or `None` if no train is passing.

## Event-Wait Methods

### `book_next_try_pull_train_passing` / `read_last_try_pull_train_passing`

Wait 1 tick for a train passing (start) event. Returns `None` if no event occurred.

```rust
pub fn book_next_try_pull_train_passing(&mut self)
pub fn read_last_try_pull_train_passing(&self) -> Result<Option<()>, PeripheralError>
```

### `pull_train_passing`

Async method that waits until a train passing (start) event is received.

```rust
pub async fn pull_train_passing(&self) -> Result<(), PeripheralError>
```

### `book_next_try_pull_train_passed` / `read_last_try_pull_train_passed`

Wait 1 tick for a train passed (finish) event. Returns `None` if no event occurred.

```rust
pub fn book_next_try_pull_train_passed(&mut self)
pub fn read_last_try_pull_train_passed(&self) -> Result<Option<()>, PeripheralError>
```

### `pull_train_passed`

Async method that waits until a train passed (finish) event is received.

```rust
pub async fn pull_train_passed(&self) -> Result<(), PeripheralError>
```

## Usage Example

```rust
use rust_computers_api::create::track_observer::TrackObserver;
use rust_computers_api::peripheral::Peripheral;

let mut observer = TrackObserver::wrap(addr);

// Check if a train is passing
let passing = observer.is_train_passing_imm()?;

// Get the passing train name
let name = observer.get_passing_train_name_imm()?;

// Wait for a train to start passing
observer.pull_train_passing().await?;

// Wait for the train to finish passing
observer.pull_train_passed().await?;
```

```
