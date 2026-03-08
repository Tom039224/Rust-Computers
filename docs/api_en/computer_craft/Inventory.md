# Inventory

**Module:** `computer_craft::inventory`  
**Peripheral Type:** `inventory`

CC:Tweaked Inventory peripheral for interacting with block inventories (chests, furnaces, etc.). Supports listing items, getting details, and transferring items between inventories.

## Book-Read Methods

### `book_next_size` / `read_last_size`
Get the number of slots in the inventory.
```rust
pub fn book_next_size(&mut self) { ... }
pub fn read_last_size(&self) -> Result<u32, PeripheralError> { ... }
```
**Returns:** `u32` — Number of slots

---

### `book_next_list` / `read_last_list`
List all slots with their summary information.
```rust
pub fn book_next_list(&mut self) { ... }
pub fn read_last_list(&self) -> Result<BTreeMap<u32, SlotInfo>, PeripheralError> { ... }
```
**Returns:** `BTreeMap<u32, SlotInfo>` — Map of slot index to slot info

---

### `book_next_get_item_detail` / `read_last_get_item_detail`
Get detailed item information for a specific slot.
```rust
pub fn book_next_get_item_detail(&mut self, slot: u32) { ... }
pub fn read_last_get_item_detail(&self) -> Result<Option<ItemDetail>, PeripheralError> { ... }
```
**Parameters:**
- `slot: u32` — Slot index

**Returns:** `Option<ItemDetail>` — Item detail or `None` if slot is empty

---

### `book_next_push_items` / `read_last_push_items`
Push items from this inventory to another inventory.
```rust
pub fn book_next_push_items(
    &mut self,
    to: &Inventory,
    from_slot: u32,
    limit: Option<u32>,
    to_slot: Option<u32>,
) { ... }
pub fn read_last_push_items(&self) -> Result<u32, PeripheralError> { ... }
```
**Parameters:**
- `to: &Inventory` — Destination inventory
- `from_slot: u32` — Source slot index
- `limit: Option<u32>` — Maximum number of items to transfer (optional)
- `to_slot: Option<u32>` — Destination slot index (optional)

**Returns:** `u32` — Number of items actually transferred

---

### `book_next_pull_items` / `read_last_pull_items`
Pull items from another inventory into this one.
```rust
pub fn book_next_pull_items(
    &mut self,
    from: &Inventory,
    from_slot: u32,
    limit: Option<u32>,
    to_slot: Option<u32>,
) { ... }
pub fn read_last_pull_items(&self) -> Result<u32, PeripheralError> { ... }
```
**Parameters:**
- `from: &Inventory` — Source inventory
- `from_slot: u32` — Source slot index
- `limit: Option<u32>` — Maximum number of items to transfer (optional)
- `to_slot: Option<u32>` — Destination slot index (optional)

**Returns:** `u32` — Number of items actually transferred

## Types

### `ItemDetail`
Detailed item information.
```rust
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

### `SlotInfo`
Slot summary information.
```rust
pub struct SlotInfo {
    pub name: String,
    pub count: u32,
}
```

## Usage Example

```rust
use rust_computers_api::computer_craft::inventory::*;
use rust_computers_api::peripheral::Peripheral;

let mut chest = Inventory::find().unwrap();

// Get inventory size
chest.book_next_size();
wait_for_next_tick().await;
let size = chest.read_last_size().unwrap();

// List all items
chest.book_next_list();
wait_for_next_tick().await;
let slots = chest.read_last_list().unwrap();

for (slot, info) in &slots {
    // Get detailed info for each non-empty slot
    chest.book_next_get_item_detail(*slot);
    wait_for_next_tick().await;
    if let Ok(Some(detail)) = chest.read_last_get_item_detail() {
        // Process item detail
    }
}
```
