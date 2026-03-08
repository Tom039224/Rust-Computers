# TableclothShop

**Module:** `create`  
**Peripheral Type:** `create:tablecloth_shop`

Create Tablecloth Shop peripheral. Manages a shop with price tags, wares, and addresses.

## Methods

### book/read Methods

These methods use the book-read pattern. Call `book_next_*` to schedule a request,
`wait_for_next_tick().await` to flush, then `read_last_*` to get the result.

#### `book_next_is_shop` / `read_last_is_shop`

Check if the tablecloth is functioning as a shop.

```rust
pub fn book_next_is_shop(&mut self)
pub fn read_last_is_shop(&self) -> Result<bool, PeripheralError>
```

#### `book_next_get_address` / `read_last_get_address`

Get the shop address.

```rust
pub fn book_next_get_address(&mut self)
pub fn read_last_get_address(&self) -> Result<String, PeripheralError>
```

#### `book_next_set_address` / `read_last_set_address`

Set the shop address.

```rust
pub fn book_next_set_address(&mut self, address: &str)
pub fn read_last_set_address(&self) -> Result<(), PeripheralError>
```

#### `book_next_get_price_tag_item` / `read_last_get_price_tag_item`

Get the price tag item.

```rust
pub fn book_next_get_price_tag_item(&mut self)
pub fn read_last_get_price_tag_item(&self) -> Result<Option<CRItemDetail>, PeripheralError>
```

#### `book_next_set_price_tag_item` / `read_last_set_price_tag_item`

Set the price tag item by name.

```rust
pub fn book_next_set_price_tag_item(&mut self, item_name: &str)
pub fn read_last_set_price_tag_item(&self) -> Result<(), PeripheralError>
```

#### `book_next_get_price_tag_count` / `read_last_get_price_tag_count`

Get the price tag quantity.

```rust
pub fn book_next_get_price_tag_count(&mut self)
pub fn read_last_get_price_tag_count(&self) -> Result<u32, PeripheralError>
```

#### `book_next_set_price_tag_count` / `read_last_set_price_tag_count`

Set the price tag quantity.

```rust
pub fn book_next_set_price_tag_count(&mut self, count: u32)
pub fn read_last_set_price_tag_count(&self) -> Result<(), PeripheralError>
```

#### `book_next_get_wares` / `read_last_get_wares`

Get the wares (item being sold).

```rust
pub fn book_next_get_wares(&mut self)
pub fn read_last_get_wares(&self) -> Result<Option<CRItemDetail>, PeripheralError>
```

#### `book_next_set_wares` / `read_last_set_wares`

Set the wares by item name.

```rust
pub fn book_next_set_wares(&mut self, item_name: &str)
pub fn read_last_set_wares(&self) -> Result<(), PeripheralError>
```

## Example

```rust
use rust_computers_api::create::tablecloth_shop::TableclothShop;
use rust_computers_api::peripheral::Peripheral;

let mut shop = TableclothShop::wrap(addr);

// Configure shop
shop.book_next_set_price_tag_item("minecraft:diamond");
wait_for_next_tick().await;
shop.read_last_set_price_tag_item()?;

shop.book_next_set_price_tag_count(1);
wait_for_next_tick().await;
shop.read_last_set_price_tag_count()?;

shop.book_next_set_wares("minecraft:iron_ingot");
wait_for_next_tick().await;
shop.read_last_set_wares()?;
```
