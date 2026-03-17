```markdown
# TableclothShop

**Module:** `create`  
**Peripheral Type:** `create:tablecloth_shop`

Create Tablecloth Shop peripheral. Manages a shop block including address, price tag, and wares configuration.

## Book-Read Methods

These methods use the book-read pattern. Call `book_next_*` to schedule a request,
`wait_for_next_tick().await` to flush, then `read_last_*` to get the result.

### `book_next_is_shop` / `read_last_is_shop`

Get whether this block is functioning as a shop.

```rust
pub fn book_next_is_shop(&mut self)
pub fn read_last_is_shop(&self) -> Result<bool, PeripheralError>
```

**Returns:** `bool` — `true` if functioning as a shop.

### `book_next_get_address` / `read_last_get_address`

Get the shop address.

```rust
pub fn book_next_get_address(&mut self)
pub fn read_last_get_address(&self) -> Result<String, PeripheralError>
```

**Returns:** `String` — the shop address.

### `book_next_set_address` / `read_last_set_address`

Set the shop address.

```rust
pub fn book_next_set_address(&mut self, address: &str)
pub fn read_last_set_address(&self) -> Result<(), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `address` | `&str` | The new shop address |

### `book_next_get_price_tag_item` / `read_last_get_price_tag_item`

Get the price tag item.

```rust
pub fn book_next_get_price_tag_item(&mut self)
pub fn read_last_get_price_tag_item(&self) -> Result<Option<CRItemDetail>, PeripheralError>
```

**Returns:** `Option<CRItemDetail>` — price tag item details, or `None` if not set.

### `book_next_set_price_tag_item` / `read_last_set_price_tag_item`

Set the price tag item by name.

```rust
pub fn book_next_set_price_tag_item(&mut self, item_name: &str)
pub fn read_last_set_price_tag_item(&self) -> Result<(), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `item_name` | `&str` | The item name for the price tag |

### `book_next_get_price_tag_count` / `read_last_get_price_tag_count`

Get the price tag count.

```rust
pub fn book_next_get_price_tag_count(&mut self)
pub fn read_last_get_price_tag_count(&self) -> Result<u32, PeripheralError>
```

**Returns:** `u32` — the price tag count.

### `book_next_set_price_tag_count` / `read_last_set_price_tag_count`

Set the price tag count.

```rust
pub fn book_next_set_price_tag_count(&mut self, count: u32)
pub fn read_last_set_price_tag_count(&self) -> Result<(), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `count` | `u32` | The price tag count |

### `book_next_get_wares` / `read_last_get_wares`

Get the wares (merchandise) information.

```rust
pub fn book_next_get_wares(&mut self)
pub fn read_last_get_wares(&self) -> Result<Option<CRItemDetail>, PeripheralError>
```

**Returns:** `Option<CRItemDetail>` — wares item details, or `None` if not set.

### `book_next_set_wares` / `read_last_set_wares`

Set the wares by item name.

```rust
pub fn book_next_set_wares(&mut self, item_name: &str)
pub fn read_last_set_wares(&self) -> Result<(), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `item_name` | `&str` | The item name for the wares |

## Implementation Status

### ✅ Implemented

- All book_next_* / read_last_* methods

### 🚧 Not Yet Implemented

- async_* variants for all methods


## Types

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
use rust_computers_api::create::tablecloth_shop::TableclothShop;
use rust_computers_api::peripheral::Peripheral;

let mut shop = TableclothShop::wrap(addr);

// Check if functioning as a shop
shop.book_next_is_shop();
wait_for_next_tick().await;
let is_shop = shop.read_last_is_shop()?;

// Set address
shop.book_next_set_address("Market Street 1");
wait_for_next_tick().await;
shop.read_last_set_address()?;

// Set price
shop.book_next_set_price_tag_item("minecraft:diamond");
wait_for_next_tick().await;
shop.read_last_set_price_tag_item()?;

shop.book_next_set_price_tag_count(5);
wait_for_next_tick().await;
shop.read_last_set_price_tag_count()?;

// Get wares
shop.book_next_get_wares();
wait_for_next_tick().await;
let wares = shop.read_last_get_wares()?;
```

```
