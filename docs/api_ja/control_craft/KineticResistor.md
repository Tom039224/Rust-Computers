# KineticResistor

**モジュール:** `control_craft::kinetic_resistor`  
**ペリフェラルタイプ:** `controlcraft:kinetic_resistor_peripheral`

Control-Craft の KineticResistor ペリフェラル。キネティック抵抗デバイスの抵抗比率を制御します。

## Book-Read メソッド

### プロパティ取得（imm 対応）

以下のメソッドは全て `book_read_imm!` マクロパターンを使用しており、`book_next_*`、`read_last_*`、`*_imm` の3バリアントを提供します。

| Book メソッド | Read メソッド | Imm メソッド | 戻り値 |
|---|---|---|---|
| `book_next_get_ratio` | `read_last_get_ratio` | `get_ratio_imm` | `f64` |

---

### セッター

#### `book_next_set_ratio` / `read_last_set_ratio`
抵抗比率を設定します（mainThread で実行）。
```rust
pub fn book_next_set_ratio(&mut self, ratio: f64) { ... }
pub fn read_last_set_ratio(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:** `ratio: f64` — 抵抗比率

## 使用例

```rust
use rust_computers_api::control_craft::kinetic_resistor::*;
use rust_computers_api::peripheral::Peripheral;

let mut resistor = KineticResistor::find().unwrap();

// 抵抗比率を設定
resistor.book_next_set_ratio(0.5);
wait_for_next_tick().await;
let _ = resistor.read_last_set_ratio();

// 現在の比率を即時取得
let ratio = resistor.get_ratio_imm().unwrap();
```
