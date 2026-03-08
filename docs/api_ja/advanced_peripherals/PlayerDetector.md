# PlayerDetector

**モジュール:** `advanced_peripherals::player_detector`  
**ペリフェラルタイプ:** `advancedPeripherals:player_detector`

AdvancedPeripherals の PlayerDetector ペリフェラル。半径、座標範囲、立方体範囲など様々な範囲設定でプレイヤーの検出やクエリを行います。

## Book-Read メソッド

### `book_next_get_online_players` / `read_last_get_online_players`
オンラインプレイヤーの名前一覧を取得します。
```rust
pub fn book_next_get_online_players(&mut self) { ... }
pub fn read_last_get_online_players(&self) -> Result<Vec<String>, PeripheralError> { ... }
```
**戻り値:** `Vec<String>`

---

### `book_next_get_players_in_range` / `read_last_get_players_in_range`
球状の半径内のプレイヤーを取得します。
```rust
pub fn book_next_get_players_in_range(&mut self, radius: f64) { ... }
pub fn read_last_get_players_in_range(&self) -> Result<Vec<String>, PeripheralError> { ... }
```
**パラメータ:**
- `radius: f64` — 検索半径

**戻り値:** `Vec<String>`

---

### `book_next_get_players_in_coords` / `read_last_get_players_in_coords`
座標バウンディングボックス内のプレイヤーを取得します。
```rust
pub fn book_next_get_players_in_coords(
    &mut self, x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64,
) { ... }
pub fn read_last_get_players_in_coords(&self) -> Result<Vec<String>, PeripheralError> { ... }
```
**パラメータ:**
- `x1, y1, z1` — 第1コーナーの座標
- `x2, y2, z2` — 第2コーナーの座標

**戻り値:** `Vec<String>`

---

### `book_next_get_players_in_cubic` / `read_last_get_players_in_cubic`
ディテクターを中心とした立方体範囲内のプレイヤーを取得します。
```rust
pub fn book_next_get_players_in_cubic(&mut self, dx: f64, dy: f64, dz: f64) { ... }
pub fn read_last_get_players_in_cubic(&self) -> Result<Vec<String>, PeripheralError> { ... }
```
**パラメータ:**
- `dx, dy, dz: f64` — 立方体範囲の半径

**戻り値:** `Vec<String>`

---

### `book_next_is_players_in_range` / `read_last_is_players_in_range`
半径内にプレイヤーがいるかどうかを確認します。
```rust
pub fn book_next_is_players_in_range(&mut self, radius: f64) { ... }
pub fn read_last_is_players_in_range(&self) -> Result<bool, PeripheralError> { ... }
```
**戻り値:** `bool`

---

### `book_next_is_players_in_coords` / `read_last_is_players_in_coords`
座標範囲内にプレイヤーがいるかどうかを確認します。
```rust
pub fn book_next_is_players_in_coords(
    &mut self, x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64,
) { ... }
pub fn read_last_is_players_in_coords(&self) -> Result<bool, PeripheralError> { ... }
```
**戻り値:** `bool`

---

### `book_next_is_players_in_cubic` / `read_last_is_players_in_cubic`
立方体範囲内にプレイヤーがいるかどうかを確認します。
```rust
pub fn book_next_is_players_in_cubic(&mut self, dx: f64, dy: f64, dz: f64) { ... }
pub fn read_last_is_players_in_cubic(&self) -> Result<bool, PeripheralError> { ... }
```
**戻り値:** `bool`

---

### `book_next_is_player_in_range` / `read_last_is_player_in_range`
特定のプレイヤーが半径内にいるかどうかを確認します。
```rust
pub fn book_next_is_player_in_range(&mut self, player: &str, radius: f64) { ... }
pub fn read_last_is_player_in_range(&self) -> Result<bool, PeripheralError> { ... }
```
**パラメータ:**
- `player: &str` — プレイヤー名
- `radius: f64` — 検索半径

**戻り値:** `bool`

---

### `book_next_is_player_in_coords` / `read_last_is_player_in_coords`
特定のプレイヤーが座標範囲内にいるかどうかを確認します。
```rust
pub fn book_next_is_player_in_coords(
    &mut self, player: &str, x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64,
) { ... }
pub fn read_last_is_player_in_coords(&self) -> Result<bool, PeripheralError> { ... }
```
**戻り値:** `bool`

---

### `book_next_is_player_in_cubic` / `read_last_is_player_in_cubic`
特定のプレイヤーが立方体範囲内にいるかどうかを確認します。
```rust
pub fn book_next_is_player_in_cubic(&mut self, player: &str, dx: f64, dy: f64, dz: f64) { ... }
pub fn read_last_is_player_in_cubic(&self) -> Result<bool, PeripheralError> { ... }
```
**戻り値:** `bool`

---

### `book_next_get_player_pos` / `read_last_get_player_pos`
プレイヤーの位置を取得します。
```rust
pub fn book_next_get_player_pos(&mut self, player: &str, decimals: Option<u32>) { ... }
pub fn read_last_get_player_pos(&self) -> Result<ADPlayerInfo, PeripheralError> { ... }
```
**パラメータ:**
- `player: &str` — プレイヤー名
- `decimals: Option<u32>` — 小数点精度（省略可）

**戻り値:** `ADPlayerInfo`

---

### `book_next_get_player` / `read_last_get_player`
プレイヤーの詳細情報を取得します。
```rust
pub fn book_next_get_player(&mut self, player: &str, decimals: Option<u32>) { ... }
pub fn read_last_get_player(&self) -> Result<ADPlayerInfo, PeripheralError> { ... }
```
**パラメータ:**
- `player: &str` — プレイヤー名
- `decimals: Option<u32>` — 小数点精度（省略可）

**戻り値:** `ADPlayerInfo`

## 型定義

### `ADPlayerInfo`
```rust
pub struct ADPlayerInfo {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub name: Option<String>,
    pub uuid: Option<String>,
    pub health: Option<f64>,
    pub max_health: Option<f64>,
    pub is_flying: Option<bool>,
    pub is_sprinting: Option<bool>,
    pub is_sneaking: Option<bool>,
    pub game_mode: Option<String>,
    pub experience: Option<u32>,
    pub level: Option<u32>,
    pub pitch: Option<f64>,
}
```

## 使用例

```rust
use rust_computers_api::advanced_peripherals::player_detector::*;
use rust_computers_api::peripheral::Peripheral;

let mut detector = PlayerDetector::find().unwrap();

// オンラインプレイヤーを取得
detector.book_next_get_online_players();
wait_for_next_tick().await;
let players = detector.read_last_get_online_players().unwrap();

// 近くにプレイヤーがいるか確認
detector.book_next_is_players_in_range(50.0);
wait_for_next_tick().await;
let nearby = detector.read_last_is_players_in_range().unwrap();

// 特定プレイヤーの詳細情報を取得
if let Some(name) = players.first() {
    detector.book_next_get_player(name, Some(2));
    wait_for_next_tick().await;
    let info = detector.read_last_get_player().unwrap();
}
```
