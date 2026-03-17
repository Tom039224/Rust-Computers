# ColonyIntegrator

**モジュール:** `advanced_peripherals::colony_integrator`  
**ペリフェラルタイプ:** `advancedPeripherals:colony_integrator`

AdvancedPeripherals コロニーインテグレーター ペリフェラル。MineColoniesのコロニーデータ（コロニー情報、市民、建物、作業指示、リクエスト、攻撃状態）への包括的なアクセスを提供します。

## ブックリードメソッド

### コロニー情報

#### `book_next_is_in_colony` / `read_last_is_in_colony`
ペリフェラルがコロニー内にあるかを確認します。
```rust
pub fn book_next_is_in_colony(&mut self)
pub fn read_last_is_in_colony(&self) -> Result<bool, PeripheralError>
```
**戻り値:** `bool`

---

#### `book_next_get_colony_id` / `read_last_get_colony_id`
コロニーIDを取得します。
```rust
pub fn book_next_get_colony_id(&mut self)
pub fn read_last_get_colony_id(&self) -> Result<i32, PeripheralError>
```
**戻り値:** `i32`

---

#### `book_next_get_colony_name` / `read_last_get_colony_name`
コロニー名を取得します。
```rust
pub fn book_next_get_colony_name(&mut self)
pub fn read_last_get_colony_name(&self) -> Result<String, PeripheralError>
```
**戻り値:** `String`

---

#### `book_next_get_colony_style` / `read_last_get_colony_style`
コロニーの建築スタイルを取得します。
```rust
pub fn book_next_get_colony_style(&mut self)
pub fn read_last_get_colony_style(&self) -> Result<String, PeripheralError>
```
**戻り値:** `String`

---

#### `book_next_is_active` / `read_last_is_active`
コロニーが現在アクティブかを確認します。
```rust
pub fn book_next_is_active(&mut self)
pub fn read_last_is_active(&self) -> Result<bool, PeripheralError>
```
**戻り値:** `bool`

---

#### `book_next_get_amount_of_citizens` / `read_last_get_amount_of_citizens`
現在の市民数を取得します。
```rust
pub fn book_next_get_amount_of_citizens(&mut self)
pub fn read_last_get_amount_of_citizens(&self) -> Result<i32, PeripheralError>
```
**戻り値:** `i32`

---

#### `book_next_get_max_citizens` / `read_last_get_max_citizens`
最大市民数を取得します。
```rust
pub fn book_next_get_max_citizens(&mut self)
pub fn read_last_get_max_citizens(&self) -> Result<i32, PeripheralError>
```
**戻り値:** `i32`

---

#### `book_next_get_happiness` / `read_last_get_happiness`
コロニーの幸福度を取得します。
```rust
pub fn book_next_get_happiness(&mut self)
pub fn read_last_get_happiness(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

#### `book_next_get_position` / `read_last_get_position`
タウンホールの位置を取得します。
```rust
pub fn book_next_get_position(&mut self)
pub fn read_last_get_position(&self) -> Result<ColonyPosition, PeripheralError>
```
**戻り値:** `ColonyPosition`

---

### 市民

#### `book_next_get_citizens` / `read_last_get_citizens`
全市民のリストを取得します。
```rust
pub fn book_next_get_citizens(&mut self)
pub fn read_last_get_citizens(&self) -> Result<Vec<CitizenInfo>, PeripheralError>
```
**戻り値:** `Vec<CitizenInfo>`

---

#### `book_next_get_citizen_info` / `read_last_get_citizen_info`
IDで特定の市民の詳細情報を取得します。
```rust
pub fn book_next_get_citizen_info(&mut self, id: i32)
pub fn read_last_get_citizen_info(&self) -> Result<CitizenInfo, PeripheralError>
```
**パラメータ:**
- `id: i32` — 市民ID

**戻り値:** `CitizenInfo`

---

### 建物

#### `book_next_get_buildings` / `read_last_get_buildings`
全建物のリストを取得します。
```rust
pub fn book_next_get_buildings(&mut self)
pub fn read_last_get_buildings(&self) -> Result<Vec<BuildingInfo>, PeripheralError>
```
**戻り値:** `Vec<BuildingInfo>`

---

#### `book_next_get_building_info` / `read_last_get_building_info`
特定位置の建物情報を取得します。
```rust
pub fn book_next_get_building_info(&mut self, x: f64, y: f64, z: f64)
pub fn read_last_get_building_info(&self) -> Result<BuildingInfo, PeripheralError>
```
**パラメータ:**
- `x: f64`, `y: f64`, `z: f64` — ブロック座標

**戻り値:** `BuildingInfo`

---

### 作業指示・リクエスト

#### `book_next_get_work_orders` / `read_last_get_work_orders`
作業指示のリストを取得します。
```rust
pub fn book_next_get_work_orders(&mut self)
pub fn read_last_get_work_orders(&self) -> Result<Vec<WorkOrderInfo>, PeripheralError>
```
**戻り値:** `Vec<WorkOrderInfo>`

---

#### `book_next_get_requests` / `read_last_get_requests`
未解決のリクエストリストを取得します。
```rust
pub fn book_next_get_requests(&mut self)
pub fn read_last_get_requests(&self) -> Result<Value, PeripheralError>
```
**戻り値:** `Value`（動的テーブル）

---

#### `book_next_get_builder_resources` / `read_last_get_builder_resources`
指定位置のビルダーに必要なリソースリストを取得します。
```rust
pub fn book_next_get_builder_resources(&mut self, x: f64, y: f64, z: f64)
pub fn read_last_get_builder_resources(&self) -> Result<Value, PeripheralError>
```
**パラメータ:**
- `x: f64`, `y: f64`, `z: f64` — ビルダーの位置

**戻り値:** `Value`（動的テーブル）

---

### 攻撃

#### `book_next_is_under_attack` / `read_last_is_under_attack`
コロニーが攻撃を受けているかを確認します。
```rust
pub fn book_next_is_under_attack(&mut self)
pub fn read_last_is_under_attack(&self) -> Result<bool, PeripheralError>
```
**戻り値:** `bool`

## 実装状況

### ✅ 実装済み

- All book_next_* / read_last_* methods

### 🚧 未実装

- async_* variants for all methods


## イミディエイトメソッド

なし。

## 型定義

```rust
pub struct ColonyPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct CitizenInfo {
    pub id: i32,
    pub name: String,
    pub job: Option<String>,
    pub level: Option<i32>,
    pub health: Option<f64>,
    pub max_health: Option<f64>,   // serde: "maxHealth"
    pub happiness: Option<f64>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub z: Option<f64>,
    pub bed_pos: Option<ColonyPosition>, // serde: "bedPos"
}

pub struct BuildingInfo {
    pub building_type: Option<String>, // serde: "type"
    pub location: Option<ColonyPosition>,
    pub level: Option<i32>,
    pub max_level: Option<i32>,        // serde: "maxLevel"
    pub style: Option<String>,
}

pub struct WorkOrderInfo {
    pub id: i32,
    pub order_type: Option<String>,    // serde: "type"
    pub builder: Option<ColonyPosition>,
    pub location: Option<ColonyPosition>,
    pub priority: Option<i32>,
}
```

## 使用例

```rust
use rust_computers_api::advanced_peripherals::ColonyIntegrator;
use rust_computers_api::peripheral::Peripheral;

let mut colony = ColonyIntegrator::wrap(addr);

loop {
    let citizens = colony.read_last_get_citizens();
    let under_attack = colony.read_last_is_under_attack();

    colony.book_next_get_citizens();
    colony.book_next_is_under_attack();
    wait_for_next_tick().await;
}
```
