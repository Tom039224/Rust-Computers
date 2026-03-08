# WatchDogTimer
**モジュール:** `toms_peripherals::watchdog_timer`  
**ペリフェラルタイプ:** `tm:watchdog_timer`

Tom's Peripherals のウォッチドッグタイマーペリフェラル。定期的にリセットされないとトリガーされる設定可能なハードウェアタイマーで、ハングしたプログラムの検出に有用です。

## Book-Read メソッド

### `book_next_is_enabled` / `read_last_is_enabled`
ウォッチドッグタイマーが現在有効かどうかを確認する。
```rust
pub fn book_next_is_enabled(&mut self)
pub fn read_last_is_enabled(&self) -> Result<bool, PeripheralError>
```
**パラメータ:** なし。  
**戻り値:** タイマーが有効なら `true`。

---

### `book_next_get_timeout` / `read_last_get_timeout`
現在のタイムアウト値を tick 単位で取得する。
```rust
pub fn book_next_get_timeout(&mut self)
pub fn read_last_get_timeout(&self) -> Result<u32, PeripheralError>
```
**パラメータ:** なし。  
**戻り値:** タイムアウト値 (tick) を `u32` で返す。

---

### `book_next_set_enabled` / `read_last_set_enabled`
ウォッチドッグタイマーを有効または無効にする。
```rust
pub fn book_next_set_enabled(&mut self, enabled: bool)
pub fn read_last_set_enabled(&self) -> Result<(), PeripheralError>
```
**パラメータ:** `enabled` — `true` で有効化、`false` で無効化。  
**戻り値:** 成功時 `()`。

---

### `book_next_set_timeout` / `read_last_set_timeout`
タイムアウト値を tick 単位で設定する。
```rust
pub fn book_next_set_timeout(&mut self, ticks: u32)
pub fn read_last_set_timeout(&self) -> Result<(), PeripheralError>
```
**パラメータ:** `ticks` — ゲーム tick 単位のタイムアウト時間。  
**戻り値:** 成功時 `()`。

---

### `book_next_reset` / `read_last_reset`
ウォッチドッグタイマーをリセットし、トリガーを防止する。
```rust
pub fn book_next_reset(&mut self)
pub fn read_last_reset(&self) -> Result<(), PeripheralError>
```
**パラメータ:** なし。  
**戻り値:** 成功時 `()`。

## Immediate メソッド

### `is_enabled_imm`
```rust
pub fn is_enabled_imm(&self) -> Result<bool, PeripheralError>
```
**戻り値:** タイマーが有効なら `true`。

### `get_timeout_imm`
```rust
pub fn get_timeout_imm(&self) -> Result<u32, PeripheralError>
```
**戻り値:** タイムアウト値 (tick) を `u32` で返す。

## 使用例

```rust
use rust_computers_api::toms_peripherals::watchdog_timer::WatchDogTimer;
use rust_computers_api::peripheral::Peripheral;

let mut wdt = WatchDogTimer::find().expect("WatchDogTimer が見つかりません");

// 設定: 100 tick のタイムアウト、有効化
wdt.book_next_set_timeout(100);
wdt.read_last_set_timeout().unwrap();

wdt.book_next_set_enabled(true);
wdt.read_last_set_enabled().unwrap();

// 定期的にリセットしてトリガーを防止
wdt.book_next_reset();
wdt.read_last_reset().unwrap();

// Immediate — ステータスを確認
let enabled = wdt.is_enabled_imm().unwrap();
let timeout = wdt.get_timeout_imm().unwrap();
```
