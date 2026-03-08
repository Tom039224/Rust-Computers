# PortableEnergyInterface

**モジュール:** `createaddition::portable_energy_interface`  
**ペリフェラルタイプ:** `createaddition:portable_energy_interface`

Create Additions ポータブルエネルギーインターフェース ペリフェラル。Createのコントラプションとの間でエネルギーを転送します。バッファエネルギーレベル、容量、接続状態、転送率へのアクセスを提供します。

## ブックリードメソッド

### `book_next_get_energy` / `read_last_get_energy`
バッファのエネルギーレベル（FE）を取得します。
```rust
pub fn book_next_get_energy(&mut self)
pub fn read_last_get_energy(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

### `book_next_get_capacity` / `read_last_get_capacity`
バッファの最大容量（FE）を取得します。
```rust
pub fn book_next_get_capacity(&mut self)
pub fn read_last_get_capacity(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

### `book_next_is_connected` / `read_last_is_connected`
コントラプションが現在接続されているかを確認します。
```rust
pub fn book_next_is_connected(&mut self)
pub fn read_last_is_connected(&self) -> Result<bool, PeripheralError>
```
**戻り値:** `bool`

---

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

## イミディエイトメソッド

なし。

## 型定義

なし。

## 使用例

```rust
use rust_computers_api::createaddition::PortableEnergyInterface;
use rust_computers_api::peripheral::Peripheral;

let mut pei = PortableEnergyInterface::wrap(addr);

loop {
    let connected = pei.read_last_is_connected();
    let energy = pei.read_last_get_energy();

    pei.book_next_is_connected();
    pei.book_next_get_energy();
    wait_for_next_tick().await;
}
```
