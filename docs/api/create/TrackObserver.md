# TrackObserver

**Module:** `create`  
**Peripheral Type:** `create:track_observer`

Create Track Observer peripheral. Monitors train passing events on a track segment.

## Methods

### book/read Methods

These methods use the book-read pattern. Call `book_next_*` to schedule a request,
`wait_for_next_tick().await` to flush, then `read_last_*` to get the result.

#### `book_next_is_train_passing` / `read_last_is_train_passing`

Check if a train is currently passing.

```rust
pub fn book_next_is_train_passing(&mut self)
pub fn read_last_is_train_passing(&self) -> Result<bool, PeripheralError>
```

#### `book_next_get_passing_train_name` / `read_last_get_passing_train_name`

Get the name of the passing train. Returns `None` if no train is passing.

```rust
pub fn book_next_get_passing_train_name(&mut self)
pub fn read_last_get_passing_train_name(&self) -> Result<Option<String>, PeripheralError>
```

### Immediate Methods

#### `is_train_passing_imm`

```rust
pub fn is_train_passing_imm(&self) -> Result<bool, PeripheralError>
```

#### `get_passing_train_name_imm`

```rust
pub fn get_passing_train_name_imm(&self) -> Result<Option<String>, PeripheralError>
```

### Async Methods (Event)

#### `book_next_try_pull_train_passing` / `read_last_try_pull_train_passing`

Try to pull a train passing start event (non-blocking).

```rust
pub fn book_next_try_pull_train_passing(&mut self)
pub fn read_last_try_pull_train_passing(&self) -> Result<Option<()>, PeripheralError>
```

#### `pull_train_passing`

Wait for a train passing start event (blocking async loop).

```rust
pub async fn pull_train_passing(&self) -> Result<(), PeripheralError>
```

#### `book_next_try_pull_train_passed` / `read_last_try_pull_train_passed`

Try to pull a train passed (finished passing) event (non-blocking).

```rust
pub fn book_next_try_pull_train_passed(&mut self)
pub fn read_last_try_pull_train_passed(&self) -> Result<Option<()>, PeripheralError>
```

#### `pull_train_passed`

Wait for a train passed event (blocking async loop).

```rust
pub async fn pull_train_passed(&self) -> Result<(), PeripheralError>
```

## Example

```rust
use rust_computers_api::create::track_observer::TrackObserver;
use rust_computers_api::peripheral::Peripheral;

let observer = TrackObserver::wrap(addr);

// Wait for a train to pass
observer.pull_train_passing().await?;
let name = observer.get_passing_train_name_imm()?;

// Wait for the train to finish passing
observer.pull_train_passed().await?;
```
