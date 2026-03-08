# Postbox

**Module:** `create`  
**Peripheral Type:** `create:postbox`

Create Postbox peripheral. Manages package sending/receiving with address, configuration, and inventory access.

## Methods

### book/read Methods

These methods use the book-read pattern. Call `book_next_*` to schedule a request,
`wait_for_next_tick().await` to flush, then `read_last_*` to get the result.

#### `book_next_set_address` / `read_last_set_address`

Set the postbox address.

```rust
pub fn book_next_set_address(&mut self, address: &str)
pub fn read_last_set_address(&self) -> Result<(), PeripheralError>
```

#### `book_next_get_address` / `read_last_get_address`

Get the postbox address.

```rust
pub fn book_next_get_address(&mut self)
pub fn read_last_get_address(&self) -> Result<String, PeripheralError>
```

#### `book_next_list` / `read_last_list`

List inventory slots.

```rust
pub fn book_next_list(&mut self)
pub fn read_last_list(&self) -> Result<Vec<CRSlotInfo>, PeripheralError>
```

#### `book_next_get_item_detail` / `read_last_get_item_detail`

Get item details for a given slot.

```rust
pub fn book_next_get_item_detail(&mut self, slot: u32)
pub fn read_last_get_item_detail(&self) -> Result<Option<CRItemDetail>, PeripheralError>
```

#### `book_next_get_configuration` / `read_last_get_configuration`

Get the configuration.

```rust
pub fn book_next_get_configuration(&mut self)
pub fn read_last_get_configuration(&self) -> Result<CRPackage, PeripheralError>
```

#### `book_next_set_configuration` / `read_last_set_configuration`

Set the configuration.

```rust
pub fn book_next_set_configuration(&mut self, config: &CRPackage) -> Result<(), PeripheralError>
pub fn read_last_set_configuration(&self) -> Result<(), PeripheralError>
```

### Async Methods (Event)

#### `book_next_try_pull_package_received` / `read_last_try_pull_package_received`

Try to pull a package received event (non-blocking).

```rust
pub fn book_next_try_pull_package_received(&mut self)
pub fn read_last_try_pull_package_received(&self) -> Result<Option<CRPackage>, PeripheralError>
```

#### `pull_package_received`

Wait for a package received event (blocking async loop).

```rust
pub async fn pull_package_received(&self) -> Result<CRPackage, PeripheralError>
```

#### `book_next_try_pull_package_sent` / `read_last_try_pull_package_sent`

Try to pull a package sent event (non-blocking).

```rust
pub fn book_next_try_pull_package_sent(&mut self)
pub fn read_last_try_pull_package_sent(&self) -> Result<Option<CRPackage>, PeripheralError>
```

#### `pull_package_sent`

Wait for a package sent event (blocking async loop).

```rust
pub async fn pull_package_sent(&self) -> Result<CRPackage, PeripheralError>
```

## Example

```rust
use rust_computers_api::create::postbox::Postbox;
use rust_computers_api::peripheral::Peripheral;

let mut postbox = Postbox::wrap(addr);

// Configure address
postbox.book_next_set_address("home_base");
wait_for_next_tick().await;
postbox.read_last_set_address()?;

// Wait for incoming package
let package = postbox.pull_package_received().await?;
```
