# Keyboard
**モジュール:** `toms_peripherals::keyboard`  
**ペリフェラルタイプ:** `tm:keyboard`

Tom's Peripherals のキーボードペリフェラル。ネイティブキーボードイベント発火の設定が可能です。

## Book-Read メソッド

### `book_next_set_fire_native_events` / `read_last_set_fire_native_events`
ネイティブキーボードイベントの発火を有効または無効にする。
```rust
pub fn book_next_set_fire_native_events(&mut self, enabled: bool)
pub fn read_last_set_fire_native_events(&self) -> Result<(), PeripheralError>
```
**パラメータ:** `enabled` — `true` でネイティブイベント発火を有効化、`false` で無効化。  
**戻り値:** 成功時 `()`。

## 実装状況

### ✅ 実装済み

- All book_next_* / read_last_* methods

### 🚧 未実装

- async_* variants for all methods


## 使用例

```rust
use rust_computers_api::toms_peripherals::keyboard::Keyboard;
use rust_computers_api::peripheral::Peripheral;

let mut kb = Keyboard::find().expect("Keyboard が見つかりません");

// ネイティブイベントを無効化
kb.book_next_set_fire_native_events(false);
kb.read_last_set_fire_native_events().unwrap();
```
