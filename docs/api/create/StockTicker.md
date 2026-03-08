# StockTicker

**Module:** `create`  
**Peripheral Type:** `create:stock_ticker`

Create Stock Ticker peripheral. Monitors stock inventory and sends filtered item requests.

## Methods

### book/read Methods

These methods use the book-read pattern. Call `book_next_*` to schedule a request,
`wait_for_next_tick().await` to flush, then `read_last_*` to get the result.

#### `book_next_stock` / `read_last_stock`

Get stock inventory information.

```rust
pub fn book_next_stock(&mut self)
pub fn read_last_stock(&self) -> Result<Vec<CRSlotInfo>, PeripheralError>
```

#### `book_next_get_stock_item_detail` / `read_last_get_stock_item_detail`

Get details of a specific stock item.

```rust
pub fn book_next_get_stock_item_detail(&mut self, slot: u32)
pub fn read_last_get_stock_item_detail(&self) -> Result<Option<CRItemDetail>, PeripheralError>
```

#### `book_next_request_filtered` / `read_last_request_filtered`

Send a filtered item request.

```rust
pub fn book_next_request_filtered(&mut self, filters: &[CRItemFilter]) -> Result<(), PeripheralError>
pub fn read_last_request_filtered(&self) -> Result<(), PeripheralError>
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

## Example

```rust
use rust_computers_api::create::stock_ticker::StockTicker;
use rust_computers_api::peripheral::Peripheral;

let mut ticker = StockTicker::wrap(addr);

// Check stock
loop {
    let stock = ticker.read_last_stock();
    ticker.book_next_stock();
    wait_for_next_tick().await;
}
```
