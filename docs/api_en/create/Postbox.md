# Postbox

**Module:** `create`  
**Peripheral Type:** `create:postbox`

Create Postbox peripheral. Manages addresses, configurations, inventory, and package send/receive events.

## Book-Read Methods

These methods use the book-read pattern. Call `book_next_*` to schedule a request,
`wait_for_next_tick().await` to flush, then `read_last_*` to get the result.

### `book_next_set_address` / `read_last_set_address`

Set the postbox address.

```rust
pub fn book_next_set_address(&mut self, address: &str)
pub fn read_last_set_address(&self) -> Result<(), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `address` | `&str` | The address to set |

### `book_next_get_address` / `read_last_get_address`

Get the postbox address.

```rust
pub fn book_next_get_address(&mut self)
pub fn read_last_get_address(&self) -> Result<String, PeripheralError>
```

**Returns:** `String` — the current address.

### `book_next_list` / `read_last_list`

List inventory slot contents.

```rust
pub fn book_next_list(&mut self)
pub fn read_last_list(&self) -> Result<Vec<CRSlotInfo>, PeripheralError>
```

**Returns:** `Vec<CRSlotInfo>` — list of slots with item name and count.

### `book_next_get_item_detail` / `read_last_get_item_detail`

Get detailed item information for a specific slot.

```rust
pub fn book_next_get_item_detail(&mut self, slot: u32)
pub fn read_last_get_item_detail(&self) -> Result<Option<CRItemDetail>, PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `slot` | `u32` | Slot index to inspect |

**Returns:** `Option<CRItemDetail>` — item details, or `None` if slot is empty.

### `book_next_get_configuration` / `read_last_get_configuration`

Get the current configuration (package routing info).

```rust
pub fn book_next_get_configuration(&mut self)
pub fn read_last_get_configuration(&self) -> Result<CRPackage, PeripheralError>
```

**Returns:** `CRPackage` — the current configuration.

### `book_next_set_configuration` / `read_last_set_configuration`

Set the configuration.

```rust
pub fn book_next_set_configuration(&mut self, config: &CRPackage) -> Result<(), PeripheralError>
pub fn read_last_set_configuration(&self) -> Result<(), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `config` | `&CRPackage` | Configuration to set |

## Implementation Status

### ✅ Implemented

- All book_next_* / read_last_* methods

### 🚧 Not Yet Implemented

- async_* variants for all methods


## Event Methods

### `book_next_try_pull_package_received` / `read_last_try_pull_package_received`

Try to pull a package received event (non-blocking, waits 1 tick). Returns `None` if no event.

```rust
pub fn book_next_try_pull_package_received(&mut self)
pub fn read_last_try_pull_package_received(&self) -> Result<Option<CRPackage>, PeripheralError>
```

### `pull_package_received`

Wait for a package received event (blocking async loop).

```rust
pub async fn pull_package_received(&self) -> Result<CRPackage, PeripheralError>
```

### `book_next_try_pull_package_sent` / `read_last_try_pull_package_sent`

Try to pull a package sent event (non-blocking, waits 1 tick). Returns `None` if no event.

```rust
pub fn book_next_try_pull_package_sent(&mut self)
pub fn read_last_try_pull_package_sent(&self) -> Result<Option<CRPackage>, PeripheralError>
```

### `pull_package_sent`

Wait for a package sent event (blocking async loop).

```rust
pub async fn pull_package_sent(&self) -> Result<CRPackage, PeripheralError>
```

## Types

### `CRPackage`

```rust
pub struct CRPackage {
    pub address: String,
}
```

### `CRSlotInfo`

```rust
pub struct CRSlotInfo {
    pub name: String,
    pub count: u32,
}
```

### `CRItemDetail`

```rust
pub struct CRItemDetail {
    pub name: String,
    pub count: u32,
    pub display_name: String,
    pub tags: BTreeMap<String, bool>,
}
```

## Usage Example

```rust
use rust_computers_api::create::postbox::Postbox;
use rust_computers_api::peripheral::Peripheral;

let mut postbox = Postbox::wrap(addr);

// Set address
postbox.book_next_set_address("main_hub");
wait_for_next_tick().await;
postbox.read_last_set_address()?;

// Check inventory
postbox.book_next_list();
wait_for_next_tick().await;
let slots = postbox.read_last_list()?;

// Wait for a package to arrive
let package = postbox.pull_package_received().await?;
```
