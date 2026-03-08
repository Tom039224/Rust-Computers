# LinkBridge

**モジュール:** `control_craft::link_bridge`  
**ペリフェラルタイプ:** `controlcraft:link_bridge_peripheral`

Control-Craft の LinkBridge ペリフェラル。インデックス指定で入力値の設定と出力値の読み取りを行います。リンクされたコンポーネント間で数値データを受け渡すブリッジインターフェースを提供します。

## Book-Read メソッド

### 入力

#### `book_next_set_input` / `read_last_set_input`
指定インデックスの入力値を設定します。
```rust
pub fn book_next_set_input(&mut self, index: f64, value: f64) { ... }
pub fn read_last_set_input(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:** `index: f64` — 入力インデックス、`value: f64` — 設定する値

---

### 出力（imm 対応）

#### `book_next_get_output` / `read_last_get_output`
指定インデックスの出力値を取得します（book-read パターン）。
```rust
pub fn book_next_get_output(&mut self, index: f64) { ... }
pub fn read_last_get_output(&self) -> Result<f64, PeripheralError> { ... }
```
**パラメータ:** `index: f64` — 出力インデックス  
**戻り値:** `f64`

#### `get_output_imm`
指定インデックスの出力値を即時取得します。
```rust
pub fn get_output_imm(&self, index: f64) -> Result<f64, PeripheralError> { ... }
```
**パラメータ:** `index: f64` — 出力インデックス  
**戻り値:** `f64`

## 使用例

```rust
use rust_computers_api::control_craft::link_bridge::*;
use rust_computers_api::peripheral::Peripheral;

let mut bridge = LinkBridge::find().unwrap();

// インデックス0の入力値を設定
bridge.book_next_set_input(0.0, 42.0);
wait_for_next_tick().await;
let _ = bridge.read_last_set_input();

// インデックス1の出力値を即時取得
let output = bridge.get_output_imm(1.0).unwrap();
```
