# Inventory

**Module:** `computer_craft`
**Peripheral Type:** `inventory` (the NAME constant)

CC:Tweaked Inventory peripheral. Provides access to block inventory contents with item inspection and transfer capabilities.

## Methods

### Book/Read Methods

These methods use the book-read pattern. Call `book_next_*` to schedule a request,
`wait_for_next_tick().await` to flush, then `read_last_*` to get the result.

#### `book_next_size` / `read_last_size`

Get the number of slots in the inventory.

```rust
pub fn book_next_size(&mut self)
pub fn read_last_size(&self) -> Result<u32, PeripheralError>
```

#### `book_next_list` / `read_last_list`

List all slots with their item summaries. Returns a map from slot index to `SlotInfo`.

```rust
pub fn book_next_list(&mut self)
pub fn read_last_list(&self) -> Result<BTreeMap<u32, SlotInfo>, PeripheralError>
```

#### `book_next_get_item_detail` / `read_last_get_item_detail`

Get detailed item information for a specific slot. Returns `None` if the slot is empty.

```rust
pub fn book_next_get_item_detail(&mut self, slot: u32)
pub fn read_last_get_item_detail(&self) -> Result<Option<ItemDetail>, PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| slot | `u32` | The slot index to inspect |

#### `book_next_push_items` / `read_last_push_items`

Push items from this inventory to another inventory. Returns the number of items transferred.

```rust
pub fn book_next_push_items(
    &mut self,
    to: &Inventory,
    from_slot: u32,
    limit: Option<u32>,
    to_slot: Option<u32>,
)
pub fn read_last_push_items(&self) -> Result<u32, PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| to | `&Inventory` | The destination inventory |
| from_slot | `u32` | The source slot index |
| limit | `Option<u32>` | Maximum number of items to transfer (optional) |
| to_slot | `Option<u32>` | The destination slot index (optional) |

#### `book_next_pull_items` / `read_last_pull_items`

Pull items from another inventory into this inventory. Returns the number of items transferred.

```rust
pub fn book_next_pull_items(
    &mut self,
    from: &Inventory,
    from_slot: u32,
    limit: Option<u32>,
    to_slot: Option<u32>,
)
pub fn read_last_pull_items(&self) -> Result<u32, PeripheralError>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| from | `&Inventory` | The source inventory |
| from_slot | `u32` | The source slot index |
| limit | `Option<u32>` | Maximum number of items to transfer (optional) |
| to_slot | `Option<u32>` | The destination slot index (optional) |

## Example

```rust
// List all items in the inventory
inventory.book_next_list();
wait_for_next_tick().await;
let items = inventory.read_last_list().unwrap();

// Get detail for slot 1
inventory.book_next_get_item_detail(1);
wait_for_next_tick().await;
if let Some(detail) = inventory.read_last_get_item_detail().unwrap() {
    // Use detail.name, detail.count, etc.
}

// Transfer items between inventories
inventory_a.book_next_push_items(&inventory_b, 1, Some(32), None);
wait_for_next_tick().await;
let transferred = inventory_a.read_last_push_items().unwrap();
```

## Types

### ItemDetail

Detailed item information.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemDetail {
    pub name: String,
    pub count: u32,
    pub max_count: u32,
    pub display_name: String,
    pub damage: Option<u32>,
    pub max_damage: Option<u32>,
    pub tags: BTreeMap<String, bool>,
}
```

| Field | Type | Description |
|-------|------|-------------|
| name | `String` | Item registry name (e.g. `"minecraft:diamond"`) |
| count | `u32` | Current stack count |
| max_count | `u32` | Maximum stack size |
| display_name | `String` | Human-readable display name |
| damage | `Option<u32>` | Current damage value (if applicable) |
| max_damage | `Option<u32>` | Maximum damage value (if applicable) |
| tags | `BTreeMap<String, bool>` | Item tags |

### SlotInfo

Slot summary information.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotInfo {
    pub name: String,
    pub count: u32,
}
```

| Field | Type | Description |
|-------|------|-------------|
| name | `String` | Item registry name |
| count | `u32` | Stack count |
