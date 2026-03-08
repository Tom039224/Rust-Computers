# Repackager

**Module:** `create`  
**Peripheral Type:** `create:repackager`

Create Repackager peripheral. Creates packages, manages addresses, and listens for package repackaging events.

## Methods

### book/read Methods

These methods use the book-read pattern. Call `book_next_*` to schedule a request,
`wait_for_next_tick().await` to flush, then `read_last_*` to get the result.

#### `book_next_make_package` / `read_last_make_package`

Create a package from inventory contents.

```rust
pub fn book_next_make_package(&mut self)
pub fn read_last_make_package(&self) -> Result<(), PeripheralError>
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

#### `book_next_get_address` / `read_last_get_address`

Get the repackager address.

```rust
pub fn book_next_get_address(&mut self)
pub fn read_last_get_address(&self) -> Result<String, PeripheralError>
```

#### `book_next_set_address` / `read_last_set_address`

Set the repackager address.

```rust
pub fn book_next_set_address(&mut self, address: &str)
pub fn read_last_set_address(&self) -> Result<(), PeripheralError>
```

#### `book_next_get_package` / `read_last_get_package`

Get the current package information.

```rust
pub fn book_next_get_package(&mut self)
pub fn read_last_get_package(&self) -> Result<Option<CRPackage>, PeripheralError>
```

### Async Methods (Event)

#### `book_next_try_pull_package_repackaged` / `read_last_try_pull_package_repackaged`

Try to pull a package repackaged event (non-blocking). Returns `(CRPackage, count)`.

```rust
pub fn book_next_try_pull_package_repackaged(&mut self)
pub fn read_last_try_pull_package_repackaged(&self) -> Result<Option<(CRPackage, u32)>, PeripheralError>
```

#### `pull_package_repackaged`

Wait for a package repackaged event (blocking async loop).

```rust
pub async fn pull_package_repackaged(&self) -> Result<(CRPackage, u32), PeripheralError>
```

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
use rust_computers_api::create::repackager::Repackager;
use rust_computers_api::peripheral::Peripheral;

let mut repackager = Repackager::wrap(addr);

// Make a package
repackager.book_next_make_package();
wait_for_next_tick().await;
repackager.read_last_make_package()?;

// Wait for repackaging event
let (package, count) = repackager.pull_package_repackaged().await?;
```
