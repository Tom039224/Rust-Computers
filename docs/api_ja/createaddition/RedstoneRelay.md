# RedstoneRelay

**モジュール:** `createaddition::redstone_relay`  
**ペリフェラルタイプ:** `createaddition:redstone_relay`

Create Additions レッドストーンリレー ペリフェラル。レッドストーン制御のエネルギーリレー。転送率、スループット、通電状態へのアクセスを提供します。

## ブックリードメソッド

### `book_next_get_max_insert` / `read_last_get_max_insert`
最大入力レート（FE/t）を取得します。
```rust
pub fn book_next_get_max_insert(&mut self)
pub fn read_last_get_max_insert(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

### `book_next_get_max_extract` / `read_last_get_max_extract`
最大出力レート（FE/t）を取得します。
```rust
pub fn book_next_get_max_extract(&mut self)
pub fn read_last_get_max_extract(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

### `book_next_get_throughput` / `read_last_get_throughput`
現在のエネルギースループット（FE/t）を取得します。
```rust
pub fn book_next_get_throughput(&mut self)
pub fn read_last_get_throughput(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

### `book_next_is_powered` / `read_last_is_powered`
リレーがレッドストーン信号を受信しているかを確認します。
```rust
pub fn book_next_is_powered(&mut self)
pub fn read_last_is_powered(&self) -> Result<bool, PeripheralError>
```
**戻り値:** `bool`

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
use rust_computers_api::createaddition::RedstoneRelay;
use rust_computers_api::peripheral::Peripheral;

let mut relay = RedstoneRelay::wrap(addr);

loop {
    let powered = relay.read_last_is_powered();
    let throughput = relay.read_last_get_throughput();

    relay.book_next_is_powered();
    relay.book_next_get_throughput();
    wait_for_next_tick().await;
}
```
