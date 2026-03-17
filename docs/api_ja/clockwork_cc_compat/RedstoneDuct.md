# RedstoneDuct

**モジュール:** `clockwork_cc_compat::redstone_duct`  
**ペリフェラルタイプ:** `clockwork:redstone_duct`

Clockwork CC Compat レッドストーンダクト ペリフェラル。レッドストーン信号レベル、条件設定、共有ガスネットワーク情報へのアクセスを提供します。

## ブックリードメソッド

以下のすべてのメソッドには `_imm` イミディエイトバリアントがあります。

### `book_next_get_power` / `read_last_get_power`
現在のレッドストーン信号レベルを取得します。
```rust
pub fn book_next_get_power(&mut self)
pub fn read_last_get_power(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

---

### `book_next_get_conditional` / `read_last_get_conditional`
レッドストーンダクトの条件設定を取得します。
```rust
pub fn book_next_get_conditional(&mut self)
pub fn read_last_get_conditional(&self) -> Result<ConditionalInfo, PeripheralError>
```
**戻り値:** `ConditionalInfo`

---

### ガスネットワーク共通メソッド

[GasNetwork](GasNetwork.md) を参照: `getTemperature`、`getPressure`、`getHeatEnergy`、`getGasMass`、`getPosition`、`getNetworkInfo`。

## 実装状況

### ✅ 実装済み

- All book_next_* / read_last_* / async_* methods

## イミディエイトメソッド

- `get_power_imm(&self) -> Result<f64, PeripheralError>`
- `get_conditional_imm(&self) -> Result<ConditionalInfo, PeripheralError>`
- その他すべてのGasNetwork `_imm` バリアント

## 型定義

```rust
pub struct ConditionalInfo {
    pub more_than: bool,        // serde: "moreThan"
    pub comparison_value: f64,  // serde: "comparisonValue"
    pub filter_blacklist: bool, // serde: "filterBlacklist"
}

pub struct CLPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
```

## 使用例

```rust
use rust_computers_api::clockwork_cc_compat::RedstoneDuct;
use rust_computers_api::peripheral::Peripheral;

let mut duct = RedstoneDuct::wrap(addr);

loop {
    let power = duct.read_last_get_power();
    let cond = duct.read_last_get_conditional();

    duct.book_next_get_power();
    duct.book_next_get_conditional();
    wait_for_next_tick().await;
}
```
