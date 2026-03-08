# RedstoneRequester

**Module:** `create`  
**Peripheral Type:** `create:redstone_requester`

Create Redstone Requester peripheral. Sends item requests through the Create logistics network.

## Methods

### book/read Methods

These methods use the book-read pattern. Call `book_next_*` to schedule a request,
`wait_for_next_tick().await` to flush, then `read_last_*` to get the result.

#### `book_next_request` / `read_last_request`

Send a request for items.

```rust
pub fn book_next_request(&mut self, items: &[CROrderItem]) -> Result<(), PeripheralError>
pub fn read_last_request(&self) -> Result<(), PeripheralError>
```

#### `book_next_set_request` / `read_last_set_request`

Set a request in a specific slot.

```rust
pub fn book_next_set_request(&mut self, slot: u32, item: &CROrderItem) -> Result<(), PeripheralError>
pub fn read_last_set_request(&self) -> Result<(), PeripheralError>
```

#### `book_next_set_crafting_request` / `read_last_set_crafting_request`

Set a crafting request in a specific slot.

```rust
pub fn book_next_set_crafting_request(&mut self, slot: u32, item: &CROrderItem) -> Result<(), PeripheralError>
pub fn read_last_set_crafting_request(&self) -> Result<(), PeripheralError>
```

#### `book_next_get_request` / `read_last_get_request`

Get the request at a specific slot.

```rust
pub fn book_next_get_request(&mut self, slot: u32)
pub fn read_last_get_request(&self) -> Result<Option<CRItemFilter>, PeripheralError>
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

#### `book_next_set_address` / `read_last_set_address`

Set the address.

```rust
pub fn book_next_set_address(&mut self, address: &str)
pub fn read_last_set_address(&self) -> Result<(), PeripheralError>
```

#### `book_next_get_address` / `read_last_get_address`

Get the address.

```rust
pub fn book_next_get_address(&mut self)
pub fn read_last_get_address(&self) -> Result<String, PeripheralError>
```

## Example

```rust
use rust_computers_api::create::redstone_requester::RedstoneRequester;
use rust_computers_api::peripheral::Peripheral;

let mut requester = RedstoneRequester::wrap(addr);

// Get current request
requester.book_next_get_request(0);
wait_for_next_tick().await;
let request = requester.read_last_get_request()?;
```
