# NBTStorage

**モジュール:** `advanced_peripherals::nbt_storage`  
**ペリフェラルタイプ:** `advancedPeripherals:nbt_storage`

AdvancedPeripherals NBTストレージ ペリフェラル。読み書き可能な永続的NBTデータストレージを提供します。SNBT文字列とテーブルベースの書き込み操作をサポートします。

## ブックリードメソッド

### `book_next_read` / `read_last_read`
保存されているNBTデータを読み取ります。
```rust
pub fn book_next_read(&mut self)
pub fn read_last_read(&self) -> Result<Value, PeripheralError>
```
**戻り値:** `Value`（動的テーブル）

---

### `book_next_write_json` / `read_last_write_json`
SNBT文字列をパースしてNBTデータとして保存します。
```rust
pub fn book_next_write_json(&mut self, snbt: &str)
pub fn read_last_write_json(&self) -> Result<bool, PeripheralError>
```
**パラメータ:**
- `snbt: &str` — SNBT（Stringified NBT）文字列

**戻り値:** `bool` — 成功ステータス

---

### `book_next_write_table` / `read_last_write_table`
MsgPackエンコードされたテーブルをNBTに変換して保存します。
```rust
pub fn book_next_write_table(&mut self, table_data: &[u8])
pub fn read_last_write_table(&self) -> Result<bool, PeripheralError>
```
**パラメータ:**
- `table_data: &[u8]` — MsgPackエンコードされたテーブルデータ

**戻り値:** `bool` — 成功ステータス

## 実装状況

### ✅ 実装済み

- All book_next_* / read_last_* methods

### 🚧 未実装

- async_* variants for all methods


## イミディエイトメソッド

なし。

## 型定義

なし。

## 使用例

```rust
use rust_computers_api::advanced_peripherals::NbtStorage;
use rust_computers_api::peripheral::Peripheral;

let mut storage = NbtStorage::wrap(addr);

// SNBTを書き込み
storage.book_next_write_json(r#"{myKey: "hello"}"#);
wait_for_next_tick().await;
let ok = storage.read_last_write_json();

// 読み戻し
storage.book_next_read();
wait_for_next_tick().await;
let data = storage.read_last_read();
```
