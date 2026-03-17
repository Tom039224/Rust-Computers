# Drag

**モジュール:** `cc_vs::drag`  
**ペリフェラルタイプ:** `vs_drag`

CC-VS の Drag API。船の抗力、揚力、風の効果を制御するために使用します。Valkyrien Skies の船上に配置されたコンピュータから呼び出します。

## Book-Read メソッド

### `book_next_get_drag_force` / `read_last_get_drag_force`
現在の抗力ベクトルを取得します。
```rust
pub fn book_next_get_drag_force(&mut self) { ... }
pub fn read_last_get_drag_force(&self) -> Result<Option<VSVector3>, PeripheralError> { ... }
```
**戻り値:** `Option<VSVector3>`

---

### `book_next_get_lift_force` / `read_last_get_lift_force`
現在の揚力ベクトルを取得します。
```rust
pub fn book_next_get_lift_force(&mut self) { ... }
pub fn read_last_get_lift_force(&self) -> Result<Option<VSVector3>, PeripheralError> { ... }
```
**戻り値:** `Option<VSVector3>`

---

### `book_next_enable_drag` / `read_last_enable_drag`
抗力シミュレーションを有効化します。
```rust
pub fn book_next_enable_drag(&mut self) { ... }
pub fn read_last_enable_drag(&self) -> Result<(), PeripheralError> { ... }
```
**戻り値:** `()`

---

### `book_next_disable_drag` / `read_last_disable_drag`
抗力シミュレーションを無効化します。
```rust
pub fn book_next_disable_drag(&mut self) { ... }
pub fn read_last_disable_drag(&self) -> Result<(), PeripheralError> { ... }
```
**戻り値:** `()`

---

### `book_next_enable_lift` / `read_last_enable_lift`
揚力シミュレーションを有効化します。
```rust
pub fn book_next_enable_lift(&mut self) { ... }
pub fn read_last_enable_lift(&self) -> Result<(), PeripheralError> { ... }
```
**戻り値:** `()`

---

### `book_next_disable_lift` / `read_last_disable_lift`
揚力シミュレーションを無効化します。
```rust
pub fn book_next_disable_lift(&mut self) { ... }
pub fn read_last_disable_lift(&self) -> Result<(), PeripheralError> { ... }
```
**戻り値:** `()`

---

### `book_next_enable_rot_drag` / `read_last_enable_rot_drag`
回転ドラッグを有効化します。
```rust
pub fn book_next_enable_rot_drag(&mut self) { ... }
pub fn read_last_enable_rot_drag(&self) -> Result<(), PeripheralError> { ... }
```
**戻り値:** `()`

---

### `book_next_disable_rot_drag` / `read_last_disable_rot_drag`
回転ドラッグを無効化します。
```rust
pub fn book_next_disable_rot_drag(&mut self) { ... }
pub fn read_last_disable_rot_drag(&self) -> Result<(), PeripheralError> { ... }
```
**戻り値:** `()`

---

### `book_next_set_wind_direction` / `read_last_set_wind_direction`
風向ベクトルを設定します。
```rust
pub fn book_next_set_wind_direction(&mut self, x: f64, y: f64, z: f64) { ... }
pub fn read_last_set_wind_direction(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:**
- `x, y, z: f64` — 風向ベクトルの成分

**戻り値:** `()`

---

### `book_next_set_wind_speed` / `read_last_set_wind_speed`
風速を設定します。
```rust
pub fn book_next_set_wind_speed(&mut self, speed: f64) { ... }
pub fn read_last_set_wind_speed(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:**
- `speed: f64` — 風速

**戻り値:** `()`

---

### `book_next_apply_wind_impulse` / `read_last_apply_wind_impulse`
風インパルスを印加します。
```rust
pub fn book_next_apply_wind_impulse(&mut self, x: f64, y: f64, z: f64, speed: f64) { ... }
pub fn read_last_apply_wind_impulse(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:**
- `x, y, z: f64` — 風インパルスの方向
- `speed: f64` — インパルスの速度

**戻り値:** `()`

## 実装状況

### ✅ 実装済み

- All book_next_* / read_last_* methods

### 🚧 未実装

- async_* variants for all methods


## 即時メソッド

### `get_drag_force_imm`
抗力ベクトルを即時取得します。
```rust
pub fn get_drag_force_imm(&self) -> Result<Option<VSVector3>, PeripheralError> { ... }
```

### `get_lift_force_imm`
揚力ベクトルを即時取得します。
```rust
pub fn get_lift_force_imm(&self) -> Result<Option<VSVector3>, PeripheralError> { ... }
```

## 型定義

### `VSVector3`
（`cc_vs::ship` からインポート）
```rust
pub struct VSVector3 { pub x: f64, pub y: f64, pub z: f64 }
```

## 使用例

```rust
use rust_computers_api::cc_vs::drag::*;
use rust_computers_api::peripheral::Peripheral;

let mut drag = Drag::find().unwrap();

// 抗力と揚力を有効化
drag.book_next_enable_drag();
wait_for_next_tick().await;
let _ = drag.read_last_enable_drag();

drag.book_next_enable_lift();
wait_for_next_tick().await;
let _ = drag.read_last_enable_lift();

// 風を設定
drag.book_next_set_wind_direction(1.0, 0.0, 0.0);
wait_for_next_tick().await;
let _ = drag.read_last_set_wind_direction();

drag.book_next_set_wind_speed(10.0);
wait_for_next_tick().await;
let _ = drag.read_last_set_wind_speed();

// 抗力を取得
let force = drag.get_drag_force_imm().unwrap();
```
