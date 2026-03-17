# InventoryManager

**モジュール:** `advanced_peripherals::inventory_manager`  
**ペリフェラルタイプ:** `advancedPeripherals:inventory_manager`

AdvancedPeripherals の InventoryManager ペリフェラル。バインドされたプレイヤーのインベントリを管理します。アイテムの一覧表示、プレイヤーとの間のアイテム転送、装備状態の確認、チェストインベントリのクエリをサポートしています。

## Book-Read メソッド

### `book_next_get_owner` / `read_last_get_owner`
バインドされたプレイヤー（オーナー）の名前を取得します。
```rust
pub fn book_next_get_owner(&mut self) { ... }
pub fn read_last_get_owner(&self) -> Result<String, PeripheralError> { ... }
```
**戻り値:** `String`

---

### `book_next_add_item_to_player` / `read_last_add_item_to_player`
隣接インベントリからバインドされたプレイヤーのインベントリにアイテムを追加します。
```rust
pub fn book_next_add_item_to_player(&mut self, slot: u32, count: Option<u32>) { ... }
pub fn read_last_add_item_to_player(&self) -> Result<u32, PeripheralError> { ... }
```
**パラメータ:**
- `slot: u32` — 隣接インベントリのスロット番号
- `count: Option<u32>` — 転送するアイテム数（省略可）

**戻り値:** `u32` — 転送されたアイテム数

---

### `book_next_remove_item_from_player` / `read_last_remove_item_from_player`
バインドされたプレイヤーのインベントリから隣接インベントリにアイテムを除去します。
```rust
pub fn book_next_remove_item_from_player(&mut self, slot: u32, count: Option<u32>) { ... }
pub fn read_last_remove_item_from_player(&self) -> Result<u32, PeripheralError> { ... }
```
**パラメータ:**
- `slot: u32` — プレイヤーインベントリのスロット番号
- `count: Option<u32>` — 転送するアイテム数（省略可）

**戻り値:** `u32` — 転送されたアイテム数

---

### `book_next_list` / `read_last_list`
プレイヤーのインベントリ内の全アイテムを一覧取得します。
```rust
pub fn book_next_list(&mut self) { ... }
pub fn read_last_list(&self) -> Result<Vec<ADItemEntry>, PeripheralError> { ... }
```
**戻り値:** `Vec<ADItemEntry>`

---

### `book_next_get_armor` / `read_last_get_armor`
プレイヤーが装備している防具を一覧取得します。
```rust
pub fn book_next_get_armor(&mut self) { ... }
pub fn read_last_get_armor(&self) -> Result<Vec<ADItemEntry>, PeripheralError> { ... }
```
**戻り値:** `Vec<ADItemEntry>`

---

### `book_next_is_player_equipped` / `read_last_is_player_equipped`
プレイヤーが装備品を持っているかどうかを確認します。
```rust
pub fn book_next_is_player_equipped(&mut self) { ... }
pub fn read_last_is_player_equipped(&self) -> Result<bool, PeripheralError> { ... }
```
**戻り値:** `bool`

---

### `book_next_is_wearing` / `read_last_is_wearing`
プレイヤーが指定スロットに装備しているかどうかを確認します。
```rust
pub fn book_next_is_wearing(&mut self, slot: u32) { ... }
pub fn read_last_is_wearing(&self) -> Result<bool, PeripheralError> { ... }
```
**パラメータ:**
- `slot: u32` — 防具スロットインデックス

**戻り値:** `bool`

---

### `book_next_get_item_in_hand` / `read_last_get_item_in_hand`
プレイヤーのメインハンドのアイテムを取得します。
```rust
pub fn book_next_get_item_in_hand(&mut self) { ... }
pub fn read_last_get_item_in_hand(&self) -> Result<ADItemEntry, PeripheralError> { ... }
```
**戻り値:** `ADItemEntry`

---

### `book_next_get_item_in_off_hand` / `read_last_get_item_in_off_hand`
プレイヤーのオフハンドのアイテムを取得します。
```rust
pub fn book_next_get_item_in_off_hand(&mut self) { ... }
pub fn read_last_get_item_in_off_hand(&self) -> Result<ADItemEntry, PeripheralError> { ... }
```
**戻り値:** `ADItemEntry`

---

### `book_next_get_empty_space` / `read_last_get_empty_space`
インベントリの空きスロット数を取得します。
```rust
pub fn book_next_get_empty_space(&mut self) { ... }
pub fn read_last_get_empty_space(&self) -> Result<u32, PeripheralError> { ... }
```
**戻り値:** `u32`

---

### `book_next_is_space_available` / `read_last_is_space_available`
インベントリに空きがあるかどうかを確認します。
```rust
pub fn book_next_is_space_available(&mut self) { ... }
pub fn read_last_is_space_available(&self) -> Result<bool, PeripheralError> { ... }
```
**戻り値:** `bool`

---

### `book_next_get_free_slot` / `read_last_get_free_slot`
最初の空きスロットのインデックスを取得します。空きスロットがない場合は -1 を返します。
```rust
pub fn book_next_get_free_slot(&mut self) { ... }
pub fn read_last_get_free_slot(&self) -> Result<i32, PeripheralError> { ... }
```
**戻り値:** `i32`

---

### `book_next_list_chest` / `read_last_list_chest`
隣接するチェストインベントリのアイテムを一覧取得します。
```rust
pub fn book_next_list_chest(&mut self) { ... }
pub fn read_last_list_chest(&self) -> Result<Vec<ADItemEntry>, PeripheralError> { ... }
```
**戻り値:** `Vec<ADItemEntry>`

## 実装状況

### ✅ 実装済み

- All book_next_* / read_last_* / async_* methods

## 即時メソッド

### `get_owner_imm`
バインドされたプレイヤーの名前を即時取得します。
```rust
pub fn get_owner_imm(&self) -> Result<String, PeripheralError> { ... }
```

## 型定義

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

## 使用例

```rust
use rust_computers_api::advanced_peripherals::inventory_manager::*;
use rust_computers_api::peripheral::Peripheral;

let mut inv = InventoryManager::find().unwrap();

// バインドされたプレイヤー名を取得
let owner = inv.get_owner_imm().unwrap();

// プレイヤーインベントリを一覧取得
inv.book_next_list();
wait_for_next_tick().await;
let items = inv.read_last_list().unwrap();

// メインハンドのアイテムを確認
inv.book_next_get_item_in_hand();
wait_for_next_tick().await;
let hand_item = inv.read_last_get_item_in_hand().unwrap();

// スロット0からプレイヤーにアイテムを転送
inv.book_next_add_item_to_player(0, Some(16));
wait_for_next_tick().await;
let transferred = inv.read_last_add_item_to_player().unwrap();
```
