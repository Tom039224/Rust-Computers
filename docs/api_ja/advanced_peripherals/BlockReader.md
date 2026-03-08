# BlockReader

**モジュール:** `advanced_peripherals::block_reader`  
**ペリフェラルタイプ:** `advancedPeripherals:block_reader`

AdvancedPeripherals ブロックリーダー ペリフェラル。ペリフェラルの前方にあるブロックの情報（ブロック名、NBTデータ、ブロックステート、タイルエンティティ検出）を読み取ります。

## ブックリードメソッド

### `book_next_get_block_name` / `read_last_get_block_name`
ブロックのリソースID（例: `minecraft:stone`）を取得します。
```rust
pub fn book_next_get_block_name(&mut self)
pub fn read_last_get_block_name(&self) -> Result<String, PeripheralError>
```
**戻り値:** `String`

---

### `book_next_get_block_data` / `read_last_get_block_data`
ブロックのNBTデータを動的テーブルとして取得します。
```rust
pub fn book_next_get_block_data(&mut self)
pub fn read_last_get_block_data(&self) -> Result<Value, PeripheralError>
```
**戻り値:** `Value`（動的テーブル）

---

### `book_next_get_block_states` / `read_last_get_block_states`
ブロックステートのプロパティを取得します。
```rust
pub fn book_next_get_block_states(&mut self)
pub fn read_last_get_block_states(&self) -> Result<Value, PeripheralError>
```
**戻り値:** `Value`（動的テーブル）

---

### `book_next_is_tile_entity` / `read_last_is_tile_entity`
ブロックがタイルエンティティ（ブロックエンティティ）かどうかを確認します。
```rust
pub fn book_next_is_tile_entity(&mut self)
pub fn read_last_is_tile_entity(&self) -> Result<bool, PeripheralError>
```
**戻り値:** `bool`

## イミディエイトメソッド

なし。

## 型定義

なし。

## 使用例

```rust
use rust_computers_api::advanced_peripherals::BlockReader;
use rust_computers_api::peripheral::Peripheral;

let mut reader = BlockReader::wrap(addr);

loop {
    let name = reader.read_last_get_block_name();
    let is_te = reader.read_last_is_tile_entity();

    reader.book_next_get_block_name();
    reader.book_next_is_tile_entity();
    wait_for_next_tick().await;
}
```
