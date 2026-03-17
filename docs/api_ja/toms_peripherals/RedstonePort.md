# RedstonePort
**モジュール:** `toms_peripherals::redstone_port`  
**ペリフェラルタイプ:** `tm:redstone_port`

Tom's Peripherals のレッドストーンポートペリフェラル。複数の面でデジタル、アナログ、バンドルのレッドストーン入出力を提供します。

## Book-Read メソッド

### `book_next_get_input` / `read_last_get_input`
指定サイドのデジタルレッドストーン入力を取得する。
```rust
pub fn book_next_get_input(&mut self, side: &str)
pub fn read_last_get_input(&self) -> Result<bool, PeripheralError>
```
**パラメータ:** `side` — サイド名（例: `"north"`, `"south"`）。  
**戻り値:** 入力がアクティブなら `true`。

---

### `book_next_get_analog_input` / `read_last_get_analog_input`
指定サイドのアナログレッドストーン入力レベルを取得する。
```rust
pub fn book_next_get_analog_input(&mut self, side: &str)
pub fn read_last_get_analog_input(&self) -> Result<u8, PeripheralError>
```
**パラメータ:** `side` — サイド名。  
**戻り値:** アナログ信号強度 (0–15) を `u8` で返す。

---

### `book_next_get_bundled_input` / `read_last_get_bundled_input`
指定サイドのバンドルレッドストーン入力を取得する。
```rust
pub fn book_next_get_bundled_input(&mut self, side: &str)
pub fn read_last_get_bundled_input(&self) -> Result<u16, PeripheralError>
```
**パラメータ:** `side` — サイド名。  
**戻り値:** バンドル入力ビットマスク (`u16`)。

---

### `book_next_get_output` / `read_last_get_output`
指定サイドのデジタルレッドストーン出力を取得する。
```rust
pub fn book_next_get_output(&mut self, side: &str)
pub fn read_last_get_output(&self) -> Result<bool, PeripheralError>
```
**パラメータ:** `side` — サイド名。  
**戻り値:** 出力がアクティブなら `true`。

---

### `book_next_get_analog_output` / `read_last_get_analog_output`
指定サイドのアナログレッドストーン出力レベルを取得する。
```rust
pub fn book_next_get_analog_output(&mut self, side: &str)
pub fn read_last_get_analog_output(&self) -> Result<u8, PeripheralError>
```
**パラメータ:** `side` — サイド名。  
**戻り値:** アナログ出力レベル (0–15) を `u8` で返す。

---

### `book_next_get_bundled_output` / `read_last_get_bundled_output`
指定サイドのバンドルレッドストーン出力を取得する。
```rust
pub fn book_next_get_bundled_output(&mut self, side: &str)
pub fn read_last_get_bundled_output(&self) -> Result<u16, PeripheralError>
```
**パラメータ:** `side` — サイド名。  
**戻り値:** バンドル出力ビットマスク (`u16`)。

---

### `book_next_set_output` / `read_last_set_output`
指定サイドのデジタルレッドストーン出力を設定する。
```rust
pub fn book_next_set_output(&mut self, side: &str, value: bool)
pub fn read_last_set_output(&self) -> Result<(), PeripheralError>
```
**パラメータ:** `side` — サイド名; `value` — `true` でオン、`false` でオフ。  
**戻り値:** 成功時 `()`。

---

### `book_next_set_analog_output` / `read_last_set_analog_output`
指定サイドのアナログレッドストーン出力レベルを設定する。
```rust
pub fn book_next_set_analog_output(&mut self, side: &str, value: u8)
pub fn read_last_set_analog_output(&self) -> Result<(), PeripheralError>
```
**パラメータ:** `side` — サイド名; `value` — アナログレベル (0–15)。  
**戻り値:** 成功時 `()`。

---

### `book_next_set_bundled_output` / `read_last_set_bundled_output`
指定サイドのバンドルレッドストーン出力を設定する。
```rust
pub fn book_next_set_bundled_output(&mut self, side: &str, mask: u16)
pub fn read_last_set_bundled_output(&self) -> Result<(), PeripheralError>
```
**パラメータ:** `side` — サイド名; `mask` — バンドル出力ビットマスク。  
**戻り値:** 成功時 `()`。

---

### `book_next_test_bundled_input` / `read_last_test_bundled_input`
入力サイドで特定のバンドルレッドストーンカラーがアクティブかテストする。
```rust
pub fn book_next_test_bundled_input(&mut self, side: &str, mask: u16)
pub fn read_last_test_bundled_input(&self) -> Result<bool, PeripheralError>
```
**パラメータ:** `side` — サイド名; `mask` — テストするカラーのビットマスク。  
**戻り値:** テストしたカラーがすべてアクティブなら `true`。

## 実装状況

### ✅ 実装済み

- All book_next_* / read_last_* methods

### 🚧 未実装

- async_* variants for all methods


## Immediate メソッド

以下のメソッドは book-read サイクルなしで即座に呼び出せます:

### `get_sides_imm`
利用可能なサイドのリストを取得する（Immediate 専用 — book-read バリアントなし）。
```rust
pub fn get_sides_imm(&self) -> Result<Vec<String>, PeripheralError>
```
**戻り値:** 利用可能なサイド名の `Vec<String>`。

### `get_output_imm`
```rust
pub fn get_output_imm(&self, side: &str) -> Result<bool, PeripheralError>
```
**パラメータ:** `side` — サイド名。  
**戻り値:** 出力がアクティブなら `true`。

### `get_analog_output_imm`
```rust
pub fn get_analog_output_imm(&self, side: &str) -> Result<u8, PeripheralError>
```
**パラメータ:** `side` — サイド名。  
**戻り値:** アナログ出力レベル (0–15) を `u8` で返す。

### `get_bundled_output_imm`
```rust
pub fn get_bundled_output_imm(&self, side: &str) -> Result<u16, PeripheralError>
```
**パラメータ:** `side` — サイド名。  
**戻り値:** バンドル出力ビットマスク (`u16`)。

## 使用例

```rust
use rust_computers_api::toms_peripherals::redstone_port::RedstonePort;
use rust_computers_api::peripheral::Peripheral;

let mut rp = RedstonePort::find().expect("RedstonePort が見つかりません");

// 利用可能なサイドを取得
let sides = rp.get_sides_imm().unwrap();

// north からアナログ入力を読み取る
rp.book_next_get_analog_input("north");
let level = rp.read_last_get_analog_input().unwrap();

// south に出力を設定
rp.book_next_set_output("south", true);
rp.read_last_set_output().unwrap();

// Immediate — east の現在の出力を確認
let is_on = rp.get_output_imm("east").unwrap();
```
