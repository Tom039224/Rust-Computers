```markdown
# StockTicker

**Module:** `create`  
**Peripheral Type:** `create:stock_ticker`

Create Stock Ticker peripheral. Monitors stock inventory, provides item details, and supports filtered requests.

## Book-Read Methods

These methods use the book-read pattern. Call `book_next_*` to schedule a request,
`wait_for_next_tick().await` to flush, then `read_last_*` to get the result.

### `book_next_stock` / `read_last_stock`

Get the stock inventory information.

```rust
pub fn book_next_stock(&mut self)
pub fn read_last_stock(&self) -> Result<Vec<CRSlotInfo>, PeripheralError>
```

**Returns:** `Vec<CRSlotInfo>` — list of stock slot information.

### `book_next_get_stock_item_detail` / `read_last_get_stock_item_detail`

Get detailed information about a stock item at a specific slot.

```rust
pub fn book_next_get_stock_item_detail(&mut self, slot: u32)
pub fn read_last_get_stock_item_detail(&self) -> Result<Option<CRItemDetail>, PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `slot` | `u32` | The slot index |

**Returns:** `Option<CRItemDetail>` — item details, or `None` if slot is empty.

### `book_next_request_filtered` / `read_last_request_filtered`

Send a filtered item request.

```rust
pub fn book_next_request_filtered(&mut self, filters: &[CRItemFilter]) -> Result<(), PeripheralError>
pub fn read_last_request_filtered(&self) -> Result<(), PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `filters` | `&[CRItemFilter]` | Array of item filters |

### `book_next_list` / `read_last_list`

List all slot information in the inventory.

```rust
pub fn book_next_list(&mut self)
pub fn read_last_list(&self) -> Result<Vec<CRSlotInfo>, PeripheralError>
```

**Returns:** `Vec<CRSlotInfo>` — list of slot information.

### `book_next_get_item_detail` / `read_last_get_item_detail`

Get detailed information about an item at a specific slot.

```rust
pub fn book_next_get_item_detail(&mut self, slot: u32)
pub fn read_last_get_item_detail(&self) -> Result<Option<CRItemDetail>, PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `slot` | `u32` | The slot index |

**Returns:** `Option<CRItemDetail>` — item details, or `None` if slot is empty.

## Implementation Status

### ✅ Implemented

- All book_next_* / read_last_* methods

### 🚧 Not Yet Implemented

- async_* variants for all methods


## Types

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

### `CRItemFilter`

```rust
pub struct CRItemFilter {
    pub name: Option<String>,
    pub request_count: Option<u32>,
}
```

## Usage Example

```rust
use rust_computers_api::create::stock_ticker::StockTicker;
use rust_computers_api::peripheral::Peripheral;

let mut ticker = StockTicker::wrap(addr);

// Get stock list
ticker.book_next_stock();
wait_for_next_tick().await;
let stock = ticker.read_last_stock()?;

// Get detail for slot 0
ticker.book_next_get_stock_item_detail(0);
wait_for_next_tick().await;
let detail = ticker.read_last_get_stock_item_detail()?;

// List inventory
ticker.book_next_list();
wait_for_next_tick().await;
let items = ticker.read_last_list()?;
```

```
