# RedstoneRequester

**Module:** `create`  
**Peripheral Type:** `create:redstone_requester`

Create Redstone Requester peripheral. Sends item requests, manages request slots and crafting requests, and handles address/configuration management.

## Book-Read Methods

These methods use the book-read pattern. Call `book_next_*` to schedule a request,
`wait_for_next_tick().await` to flush, then `read_last_*` to get the result.

### `book_next_request` / `read_last_request`

Send a batch request for items.

```rust
pub fn book_next_request(&mut self, items: &[CROrderItem]) -> Result<(), PeripheralError>
pub fn read_last_request(&self) -> Result<(), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `items` | `&[CROrderItem]` | Array of items to request |

### `book_next_set_request` / `read_last_set_request`

Set a request in a specific slot.

```rust
pub fn book_next_set_request(&mut self, slot: u32, item: &CROrderItem) -> Result<(), PeripheralError>
pub fn read_last_set_request(&self) -> Result<(), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `slot` | `u32` | Slot index to configure |
| `item` | `&CROrderItem` | Item and count to request |

### `book_next_set_crafting_request` / `read_last_set_crafting_request`

Set a crafting request in a specific slot.

```rust
pub fn book_next_set_crafting_request(&mut self, slot: u32, item: &CROrderItem) -> Result<(), PeripheralError>
pub fn read_last_set_crafting_request(&self) -> Result<(), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `slot` | `u32` | Slot index to configure |
| `item` | `&CROrderItem` | Item and count for crafting request |

### `book_next_get_request` / `read_last_get_request`

Get the request information for a specific slot.

```rust
pub fn book_next_get_request(&mut self, slot: u32)
pub fn read_last_get_request(&self) -> Result<Option<CRItemFilter>, PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `slot` | `u32` | Slot index to query |

**Returns:** `Option<CRItemFilter>` — request filter info, or `None` if slot is empty.

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

### `book_next_set_address` / `read_last_set_address`

Set the requester address.

```rust
pub fn book_next_set_address(&mut self, address: &str)
pub fn read_last_set_address(&self) -> Result<(), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `address` | `&str` | The address to set |

### `book_next_get_address` / `read_last_get_address`

Get the requester address.

```rust
pub fn book_next_get_address(&mut self)
pub fn read_last_get_address(&self) -> Result<String, PeripheralError>
```

**Returns:** `String` — the current address.

## Implementation Status

### ✅ Implemented

- All book_next_* / read_last_* methods

### 🚧 Not Yet Implemented

- async_* variants for all methods


## Types

### `CROrderItem`

```rust
pub struct CROrderItem {
    pub name: String,
    pub count: u32,
}
```

### `CRItemFilter`

```rust
pub struct CRItemFilter {
    pub name: Option<String>,
    pub request_count: Option<u32>,
}
```

### `CRPackage`

```rust
pub struct CRPackage {
    pub address: String,
}
```

## Usage Example

```rust
use rust_computers_api::create::redstone_requester::RedstoneRequester;
use rust_computers_api::create::common::CROrderItem;
use rust_computers_api::peripheral::Peripheral;

let mut requester = RedstoneRequester::wrap(addr);

// Set address
requester.book_next_set_address("storage");
wait_for_next_tick().await;
requester.read_last_set_address()?;

// Set a request in slot 0
let item = CROrderItem { name: "minecraft:iron_ingot".into(), count: 64 };
requester.book_next_set_request(0, &item)?;
wait_for_next_tick().await;
requester.read_last_set_request()?;

// Send a batch request
let items = vec![
    CROrderItem { name: "minecraft:iron_ingot".into(), count: 64 },
    CROrderItem { name: "minecraft:gold_ingot".into(), count: 32 },
];
requester.book_next_request(&items)?;
wait_for_next_tick().await;
requester.read_last_request()?;
```
