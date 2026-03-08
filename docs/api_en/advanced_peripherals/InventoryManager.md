# InventoryManager

**Module:** `advanced_peripherals::inventory_manager`  
**Peripheral Type:** `advancedPeripherals:inventory_manager`

AdvancedPeripherals InventoryManager peripheral for managing a bound player's inventory. Supports listing items, transferring items to/from the player, checking equipment status, and querying chest inventories.

## Book-Read Methods

### `book_next_get_owner` / `read_last_get_owner`
Get the name of the bound player (owner).
```rust
pub fn book_next_get_owner(&mut self) { ... }
pub fn read_last_get_owner(&self) -> Result<String, PeripheralError> { ... }
```
**Returns:** `String`

---

### `book_next_add_item_to_player` / `read_last_add_item_to_player`
Add items to the bound player's inventory from the adjacent inventory.
```rust
pub fn book_next_add_item_to_player(&mut self, slot: u32, count: Option<u32>) { ... }
pub fn read_last_add_item_to_player(&self) -> Result<u32, PeripheralError> { ... }
```
**Parameters:**
- `slot: u32` — Source slot in the adjacent inventory
- `count: Option<u32>` — Number of items to transfer (optional)

**Returns:** `u32` — Number of items transferred

---

### `book_next_remove_item_from_player` / `read_last_remove_item_from_player`
Remove items from the bound player's inventory to the adjacent inventory.
```rust
pub fn book_next_remove_item_from_player(&mut self, slot: u32, count: Option<u32>) { ... }
pub fn read_last_remove_item_from_player(&self) -> Result<u32, PeripheralError> { ... }
```
**Parameters:**
- `slot: u32` — Source slot in the player's inventory
- `count: Option<u32>` — Number of items to transfer (optional)

**Returns:** `u32` — Number of items transferred

---

### `book_next_list` / `read_last_list`
List all items in the player's inventory.
```rust
pub fn book_next_list(&mut self) { ... }
pub fn read_last_list(&self) -> Result<Vec<ADItemEntry>, PeripheralError> { ... }
```
**Returns:** `Vec<ADItemEntry>`

---

### `book_next_get_armor` / `read_last_get_armor`
List the player's equipped armor.
```rust
pub fn book_next_get_armor(&mut self) { ... }
pub fn read_last_get_armor(&self) -> Result<Vec<ADItemEntry>, PeripheralError> { ... }
```
**Returns:** `Vec<ADItemEntry>`

---

### `book_next_is_player_equipped` / `read_last_is_player_equipped`
Check if the player has any equipment.
```rust
pub fn book_next_is_player_equipped(&mut self) { ... }
pub fn read_last_is_player_equipped(&self) -> Result<bool, PeripheralError> { ... }
```
**Returns:** `bool`

---

### `book_next_is_wearing` / `read_last_is_wearing`
Check if the player is wearing equipment in a specific slot.
```rust
pub fn book_next_is_wearing(&mut self, slot: u32) { ... }
pub fn read_last_is_wearing(&self) -> Result<bool, PeripheralError> { ... }
```
**Parameters:**
- `slot: u32` — Armor slot index

**Returns:** `bool`

---

### `book_next_get_item_in_hand` / `read_last_get_item_in_hand`
Get the item in the player's main hand.
```rust
pub fn book_next_get_item_in_hand(&mut self) { ... }
pub fn read_last_get_item_in_hand(&self) -> Result<ADItemEntry, PeripheralError> { ... }
```
**Returns:** `ADItemEntry`

---

### `book_next_get_item_in_off_hand` / `read_last_get_item_in_off_hand`
Get the item in the player's off hand.
```rust
pub fn book_next_get_item_in_off_hand(&mut self) { ... }
pub fn read_last_get_item_in_off_hand(&self) -> Result<ADItemEntry, PeripheralError> { ... }
```
**Returns:** `ADItemEntry`

---

### `book_next_get_empty_space` / `read_last_get_empty_space`
Get the number of empty inventory slots.
```rust
pub fn book_next_get_empty_space(&mut self) { ... }
pub fn read_last_get_empty_space(&self) -> Result<u32, PeripheralError> { ... }
```
**Returns:** `u32`

---

### `book_next_is_space_available` / `read_last_is_space_available`
Check if there is any empty space in the inventory.
```rust
pub fn book_next_is_space_available(&mut self) { ... }
pub fn read_last_is_space_available(&self) -> Result<bool, PeripheralError> { ... }
```
**Returns:** `bool`

---

### `book_next_get_free_slot` / `read_last_get_free_slot`
Get the index of the first free slot. Returns -1 if no free slot exists.
```rust
pub fn book_next_get_free_slot(&mut self) { ... }
pub fn read_last_get_free_slot(&self) -> Result<i32, PeripheralError> { ... }
```
**Returns:** `i32`

---

### `book_next_list_chest` / `read_last_list_chest`
List items in the adjacent chest inventory.
```rust
pub fn book_next_list_chest(&mut self) { ... }
pub fn read_last_list_chest(&self) -> Result<Vec<ADItemEntry>, PeripheralError> { ... }
```
**Returns:** `Vec<ADItemEntry>`

## Immediate Methods

### `get_owner_imm`
Immediately get the bound player's name.
```rust
pub fn get_owner_imm(&self) -> Result<String, PeripheralError> { ... }
```

## Types

### `ADItemEntry`
```rust
pub struct ADItemEntry {
    pub name: String,
    pub tags: Vec<String>,
    pub count: u32,
    pub display_name: String,
    pub max_stack_size: u32,
    pub components: Value,
    pub fingerprint: String,
    pub slot: Option<u32>,
}
```

## Usage Example

```rust
use rust_computers_api::advanced_peripherals::inventory_manager::*;
use rust_computers_api::peripheral::Peripheral;

let mut inv = InventoryManager::find().unwrap();

// Get bound player name
let owner = inv.get_owner_imm().unwrap();

// List player inventory
inv.book_next_list();
wait_for_next_tick().await;
let items = inv.read_last_list().unwrap();

// Check what's in the main hand
inv.book_next_get_item_in_hand();
wait_for_next_tick().await;
let hand_item = inv.read_last_get_item_in_hand().unwrap();

// Transfer item from slot 0 to player
inv.book_next_add_item_to_player(0, Some(16));
wait_for_next_tick().await;
let transferred = inv.read_last_add_item_to_player().unwrap();
```
