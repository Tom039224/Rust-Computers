# ModularAccumulator

**モジュール:** `createaddition::modular_accumulator`  
**ペリフェラルタイプ:** `createaddition:modular_accumulator`

Create Additions モジュラーアキュムレーター ペリフェラル。スケーラブルなマルチブロックエネルギーストレージ。エネルギーレベル、容量、充電率、転送率、物理的寸法へのアクセスを提供します。

## ブックリードメソッド

### `book_next_get_energy` / `read_last_get_energy`
現在の蓄積エネルギー（FE）を取得します。
```rust
pub fn book_next_get_energy(&mut self)
pub fn read_last_get_energy(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

### `book_next_get_capacity` / `read_last_get_capacity`
最大エネルギー容量（FE）を取得します。
```rust
pub fn book_next_get_capacity(&mut self)
pub fn read_last_get_capacity(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

### `book_next_get_percent` / `read_last_get_percent`
充電率（0.0–100.0%）を取得します。
```rust
pub fn book_next_get_percent(&mut self)
pub fn read_last_get_percent(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

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

---

### `book_next_get_height` / `read_last_get_height`
マルチブロックの高さ（ブロック）を取得します。
```rust
pub fn book_next_get_height(&mut self)
pub fn read_last_get_height(&self) -> Result<i32, PeripheralError>
```
**戻り値:** `i32`

---

### `book_next_get_width` / `read_last_get_width`
マルチブロックの幅（ブロック）を取得します。
```rust
pub fn book_next_get_width(&mut self)
pub fn read_last_get_width(&self) -> Result<i32, PeripheralError>
```
**戻り値:** `i32`

## イミディエイトメソッド

なし。

## 型定義

なし。

## 使用例

```rust
use rust_computers_api::createaddition::ModularAccumulator;
use rust_computers_api::peripheral::Peripheral;

let mut acc = ModularAccumulator::wrap(addr);

loop {
    let energy = acc.read_last_get_energy();
    let percent = acc.read_last_get_percent();

    acc.book_next_get_energy();
    acc.book_next_get_percent();
    wait_for_next_tick().await;
}
```
