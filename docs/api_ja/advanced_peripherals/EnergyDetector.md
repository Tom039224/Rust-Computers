# EnergyDetector

**モジュール:** `advanced_peripherals::energy_detector`  
**ペリフェラルタイプ:** `advancedPeripherals:energy_detector`

AdvancedPeripherals エネルギー検出器 ペリフェラル。エネルギー転送率を監視し、転送率制限の設定を可能にします。

## ブックリードメソッド

### `book_next_get_transfer_rate` / `read_last_get_transfer_rate`
現在のエネルギー転送率（FE/t）を取得します。
```rust
pub fn book_next_get_transfer_rate(&mut self)
pub fn read_last_get_transfer_rate(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64` — 転送率（FE/t）

---

### `book_next_get_transfer_rate_limit` / `read_last_get_transfer_rate_limit`
現在の転送率制限（FE/t）を取得します。
```rust
pub fn book_next_get_transfer_rate_limit(&mut self)
pub fn read_last_get_transfer_rate_limit(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64` — レート制限（FE/t）

---

### `book_next_set_transfer_rate_limit` / `read_last_set_transfer_rate_limit`
最大転送率制限（FE/t）を設定します。
```rust
pub fn book_next_set_transfer_rate_limit(&mut self, rate: f64)
pub fn read_last_set_transfer_rate_limit(&self) -> Result<(), PeripheralError>
```
**パラメータ:**
- `rate: f64` — 最大転送率（FE/t）

**戻り値:** `()`

## イミディエイトメソッド

なし。

## 型定義

なし。

## 使用例

```rust
use rust_computers_api::advanced_peripherals::EnergyDetector;
use rust_computers_api::peripheral::Peripheral;

let mut detector = EnergyDetector::wrap(addr);

// レート制限を設定
detector.book_next_set_transfer_rate_limit(10000.0);
wait_for_next_tick().await;

loop {
    let rate = detector.read_last_get_transfer_rate();

    detector.book_next_get_transfer_rate();
    wait_for_next_tick().await;
}
```
