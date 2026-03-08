# Slider

**モジュール:** `control_craft::slider`  
**ペリフェラルタイプ:** `controlcraft:slider_peripheral`

Control-Craft の Slider ペリフェラル。PID制御、力出力、目標値管理、ロック/アンロック機能を備えた直線運動制御を行います。

## Book-Read メソッド

### プロパティ取得（imm 対応）

以下のメソッドは全て `book_read_imm!` マクロパターンを使用しており、`book_next_*`、`read_last_*`、`*_imm` の3バリアントを提供します。

| Book メソッド | Read メソッド | Imm メソッド | 戻り値 |
|---|---|---|---|
| `book_next_get_distance` | `read_last_get_distance` | `get_distance_imm` | `f64` |
| `book_next_get_current_value` | `read_last_get_current_value` | `get_current_value_imm` | `f64` |
| `book_next_get_target_value` | `read_last_get_target_value` | `get_target_value_imm` | `f64` |
| `book_next_get_physics` | `read_last_get_physics` | `get_physics_imm` | `Value` |
| `book_next_is_locked` | `read_last_is_locked` | `is_locked_imm` | `bool` |

---

### セッター

#### `book_next_set_output_force` / `read_last_set_output_force`
出力力スケールを設定します。
```rust
pub fn book_next_set_output_force(&mut self, scale: f64) { ... }
pub fn read_last_set_output_force(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:** `scale: f64` — 力スケール係数

#### `book_next_set_pid` / `read_last_set_pid`
スライダー制御用の PID ゲインを設定します。
```rust
pub fn book_next_set_pid(&mut self, p: f64, i: f64, d: f64) { ... }
pub fn read_last_set_pid(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:** `p: f64` — 比例ゲイン、`i: f64` — 積分ゲイン、`d: f64` — 微分ゲイン

#### `book_next_set_target_value` / `read_last_set_target_value`
スライダー位置の目標値を設定します。
```rust
pub fn book_next_set_target_value(&mut self, target: f64) { ... }
pub fn read_last_set_target_value(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:** `target: f64` — 目標位置の値

---

### ロック制御

#### `book_next_lock` / `read_last_lock`
スライダーをロック（固定）します。
```rust
pub fn book_next_lock(&mut self) { ... }
pub fn read_last_lock(&self) -> Result<(), PeripheralError> { ... }
```

#### `book_next_unlock` / `read_last_unlock`
スライダーのロックを解除します。
```rust
pub fn book_next_unlock(&mut self) { ... }
pub fn read_last_unlock(&self) -> Result<(), PeripheralError> { ... }
```

## 使用例

```rust
use rust_computers_api::control_craft::slider::*;
use rust_computers_api::peripheral::Peripheral;

let mut slider = Slider::find().unwrap();

// PID と目標値を設定
slider.book_next_set_pid(1.0, 0.1, 0.05);
slider.book_next_set_target_value(5.0);
wait_for_next_tick().await;
let _ = slider.read_last_set_pid();
let _ = slider.read_last_set_target_value();

// 現在の距離を即時取得
let dist = slider.get_distance_imm().unwrap();

// スライダーをロック
slider.book_next_lock();
wait_for_next_tick().await;
let _ = slider.read_last_lock();
```
