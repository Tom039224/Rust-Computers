# Inventory

**モジュール:** `computer_craft::inventory`  
**ペリフェラルタイプ:** `inventory`

CC:Tweaked のインベントリペリフェラル。チェストやかまどなどのブロックインベントリとのやり取りに使用します。アイテムの一覧取得、詳細情報の取得、インベントリ間のアイテム転送をサポートしています。

## Book-Read メソッド

### `book_next_size` / `read_last_size`
インベントリのスロット数を取得します。
```rust
pub fn book_next_size(&mut self) { ... }
pub fn read_last_size(&self) -> Result<u32, PeripheralError> { ... }
```
**戻り値:** `u32` — スロット数

---

### `book_next_list` / `read_last_list`
全スロットの簡易情報を一覧取得します。
```rust
pub fn book_next_list(&mut self) { ... }
pub fn read_last_list(&self) -> Result<BTreeMap<u32, SlotInfo>, PeripheralError> { ... }
```
**戻り値:** `BTreeMap<u32, SlotInfo>` — スロットインデックスからスロット情報へのマップ

---

### `book_next_get_item_detail` / `read_last_get_item_detail`
指定スロットのアイテム詳細情報を取得します。
```rust
pub fn book_next_get_item_detail(&mut self, slot: u32) { ... }
pub fn read_last_get_item_detail(&self) -> Result<Option<ItemDetail>, PeripheralError> { ... }
```
**パラメータ:**
- `slot: u32` — スロットインデックス

**戻り値:** `Option<ItemDetail>` — アイテム詳細（スロットが空の場合は `None`）

---

### `book_next_push_items` / `read_last_push_items`
このインベントリから別のインベントリにアイテムを転送します。
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
**パラメータ:**
- `to: &Inventory` — 転送先インベントリ
- `from_slot: u32` — 転送元スロットインデックス
- `limit: Option<u32>` — 転送するアイテムの最大数（省略可）
- `to_slot: Option<u32>` — 転送先スロットインデックス（省略可）

**戻り値:** `u32` — 実際に転送されたアイテム数

---

### `book_next_pull_items` / `read_last_pull_items`
別のインベントリからこのインベントリにアイテムを引き出します。
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
**パラメータ:**
- `from: &Inventory` — 引き出し元インベントリ
- `from_slot: u32` — 引き出し元スロットインデックス
- `limit: Option<u32>` — 転送するアイテムの最大数（省略可）
- `to_slot: Option<u32>` — 転送先スロットインデックス（省略可）

**戻り値:** `u32` — 実際に転送されたアイテム数

## 型定義

### `ItemDetail`
アイテムの詳細情報。
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
スロットの簡易情報。
```rust
pub struct SlotInfo {
    pub name: String,
    pub count: u32,
}
```

## 使用例

```rust
use rust_computers_api::computer_craft::inventory::*;
use rust_computers_api::peripheral::Peripheral;

let mut chest = Inventory::find().unwrap();

// インベントリサイズを取得
chest.book_next_size();
wait_for_next_tick().await;
let size = chest.read_last_size().unwrap();

// 全アイテムを一覧取得
chest.book_next_list();
wait_for_next_tick().await;
let slots = chest.read_last_list().unwrap();

for (slot, info) in &slots {
    // 各スロットの詳細情報を取得
    chest.book_next_get_item_detail(*slot);
    wait_for_next_tick().await;
    if let Ok(Some(detail)) = chest.read_last_get_item_detail() {
        // アイテム詳細を処理
    }
}
```
