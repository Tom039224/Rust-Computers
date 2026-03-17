# ElectricMotor

**モジュール:** `createaddition::electric_motor`  
**ペリフェラルタイプ:** `createaddition:electric_motor`

Create Additions の電気モーターペリフェラル。Create mod の回転コンポーネントを制御するために使用します。速度制御、回転・並進コマンド、エネルギー監視をサポートしています。

## Book-Read メソッド

### `book_next_get_type` / `read_last_get_type`
ペリフェラルの型名を取得します。
```rust
pub fn book_next_get_type(&mut self) { ... }
pub fn read_last_get_type(&self) -> Result<String, PeripheralError> { ... }
```
**戻り値:** `String`

---

### `book_next_set_speed` / `read_last_set_speed`
モーターの RPM を設定します。符号で回転方向を制御します。
```rust
pub fn book_next_set_speed(&mut self, speed: f64) { ... }
pub fn read_last_set_speed(&self) -> Result<(), PeripheralError> { ... }
```
**パラメータ:**
- `speed: f64` — RPM 値（正 = 正方向、負 = 逆方向）

**戻り値:** `()`

---

### `book_next_stop` / `read_last_stop`
モーターを停止します。
```rust
pub fn book_next_stop(&mut self) { ... }
pub fn read_last_stop(&self) -> Result<(), PeripheralError> { ... }
```
**戻り値:** `()`

---

### `book_next_get_speed` / `read_last_get_speed`
現在の速度を取得します。
```rust
pub fn book_next_get_speed(&mut self) { ... }
pub fn read_last_get_speed(&self) -> Result<f64, PeripheralError> { ... }
```
**戻り値:** `f64` — 現在の RPM

---

### `book_next_get_stress_capacity` / `read_last_get_stress_capacity`
ストレス容量を取得します。
```rust
pub fn book_next_get_stress_capacity(&mut self) { ... }
pub fn read_last_get_stress_capacity(&self) -> Result<f64, PeripheralError> { ... }
```
**戻り値:** `f64`

---

### `book_next_get_energy_consumption` / `read_last_get_energy_consumption`
現在のエネルギー消費量を取得します。
```rust
pub fn book_next_get_energy_consumption(&mut self) { ... }
pub fn read_last_get_energy_consumption(&self) -> Result<f64, PeripheralError> { ... }
```
**戻り値:** `f64`

---

### `book_next_rotate` / `read_last_rotate`
指定した角度だけ回転します。所要時間（秒）を返します。
```rust
pub fn book_next_rotate(&mut self, degrees: f64, rpm: Option<f64>) { ... }
pub fn read_last_rotate(&self) -> Result<f64, PeripheralError> { ... }
```
**パラメータ:**
- `degrees: f64` — 回転角度（度）
- `rpm: Option<f64>` — 回転に使用する RPM（省略可）

**戻り値:** `f64` — 所要時間（秒）

---

### `book_next_translate` / `read_last_translate`
指定した距離だけ移動します。所要時間（秒）を返します。
```rust
pub fn book_next_translate(&mut self, distance: f64, rpm: Option<f64>) { ... }
pub fn read_last_translate(&self) -> Result<f64, PeripheralError> { ... }
```
**パラメータ:**
- `distance: f64` — 移動距離
- `rpm: Option<f64>` — 使用する RPM（省略可）

**戻り値:** `f64` — 所要時間（秒）

---

### `book_next_get_max_insert` / `read_last_get_max_insert`
最大エネルギー挿入レートを取得します。
```rust
pub fn book_next_get_max_insert(&mut self) { ... }
pub fn read_last_get_max_insert(&self) -> Result<f64, PeripheralError> { ... }
```
**戻り値:** `f64`

---

### `book_next_get_max_extract` / `read_last_get_max_extract`
最大エネルギー抽出レートを取得します。
```rust
pub fn book_next_get_max_extract(&mut self) { ... }
pub fn read_last_get_max_extract(&self) -> Result<f64, PeripheralError> { ... }
```
**戻り値:** `f64`

## 実装状況

### ✅ 実装済み

- All book_next_* / read_last_* methods

### 🚧 未実装

- async_* variants for all methods


## 即時メソッド

### `get_type_imm`
ペリフェラル型名を即時取得します。
```rust
pub fn get_type_imm(&self) -> Result<String, PeripheralError> { ... }
```

## 使用例

```rust
use rust_computers_api::createaddition::electric_motor::*;
use rust_computers_api::peripheral::Peripheral;

let mut motor = ElectricMotor::find().unwrap();

// 速度を 64 RPM に設定
motor.book_next_set_speed(64.0);
wait_for_next_tick().await;
let _ = motor.read_last_set_speed();

// 90度回転
motor.book_next_rotate(90.0, Some(32.0));
wait_for_next_tick().await;
let duration = motor.read_last_rotate().unwrap();

// エネルギー消費量を確認
motor.book_next_get_energy_consumption();
wait_for_next_tick().await;
let consumption = motor.read_last_get_energy_consumption().unwrap();

// 停止
motor.book_next_stop();
wait_for_next_tick().await;
let _ = motor.read_last_stop();
```
