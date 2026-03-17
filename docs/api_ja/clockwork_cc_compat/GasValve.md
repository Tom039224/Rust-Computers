# GasValve

**モジュール:** `clockwork_cc_compat::gas_valve`  
**ペリフェラルタイプ:** `clockwork:gas_valve`

Clockwork CC Compat ガスバルブ ペリフェラル。バルブの開度、向き、共有ガスネットワーク情報へのアクセスを提供します。

## ブックリードメソッド

以下のすべてのメソッドには `_imm` イミディエイトバリアントがあります。

### `book_next_get_aperture` / `read_last_get_aperture`
現在のバルブ開度を取得します。
```rust
pub fn book_next_get_aperture(&mut self)
pub fn read_last_get_aperture(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

### `book_next_get_facing` / `read_last_get_facing`
向きを取得します。
```rust
pub fn book_next_get_facing(&mut self)
pub fn read_last_get_facing(&self) -> Result<String, PeripheralError>
```
**戻り値:** `String`

---

### ガスネットワーク共通メソッド

[GasNetwork](GasNetwork.md) を参照: `getTemperature`、`getPressure`、`getHeatEnergy`、`getGasMass`、`getPosition`、`getNetworkInfo`。

## 実装状況

### ✅ 実装済み

- All book_next_* / read_last_* methods

### 🚧 未実装

- async_* variants for all methods


## イミディエイトメソッド

- `get_aperture_imm(&self) -> Result<f64, PeripheralError>`
- `get_facing_imm(&self) -> Result<String, PeripheralError>`
- その他すべてのGasNetwork `_imm` バリアント

## 型定義

```rust
pub struct CLPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
```

## 使用例

```rust
use rust_computers_api::clockwork_cc_compat::GasValve;
use rust_computers_api::peripheral::Peripheral;

let mut valve = GasValve::wrap(addr);

loop {
    let aperture = valve.read_last_get_aperture();
    let facing = valve.read_last_get_facing();

    valve.book_next_get_aperture();
    valve.book_next_get_facing();
    wait_for_next_tick().await;
}
```
