# RSBridge

**モジュール:** `advanced_peripherals::rs_bridge`  
**ペリフェラルタイプ:** `advancedPeripherals:rs_bridge`

AdvancedPeripherals RS Bridge ペリフェラル。Refined Storageネットワークへの完全なアクセスを提供します。MEBridgeとほぼ同一のインターフェースを持ち、アイテム、流体、化学物質のオペレーション、クラフティング、ストレージモニタリングをサポートします。

## ブックリードメソッド

### アイテムオペレーション

#### `book_next_list_items` / `read_last_list_items`
RSネットワーク内の全アイテムをリストします。
```rust
pub fn book_next_list_items(&mut self)
pub fn read_last_list_items(&self) -> Result<Vec<MEItemEntry>, PeripheralError>
```
**戻り値:** `Vec<MEItemEntry>`

---

#### `book_next_get_item` / `read_last_get_item`
フィルターに一致する最初のアイテムを取得します。
```rust
pub fn book_next_get_item(&mut self, filter: &[u8])
pub fn read_last_get_item(&self) -> Result<MEItemEntry, PeripheralError>
```

---

#### `book_next_export_item` / `read_last_export_item`
隣接するインベントリにアイテムをエクスポートします。
```rust
pub fn book_next_export_item(&mut self, filter: &[u8], side: &str)
pub fn read_last_export_item(&self) -> Result<i64, PeripheralError>
```

---

#### `book_next_import_item` / `read_last_import_item`
隣接するインベントリからアイテムをインポートします。
```rust
pub fn book_next_import_item(&mut self, filter: &[u8], side: &str)
pub fn read_last_import_item(&self) -> Result<i64, PeripheralError>
```

---

#### `book_next_export_item_to_peripheral` / `read_last_export_item_to_peripheral`
名前付きペリフェラルにアイテムをエクスポートします。
```rust
pub fn book_next_export_item_to_peripheral(&mut self, filter: &[u8], target_name: &str)
pub fn read_last_export_item_to_peripheral(&self) -> Result<i64, PeripheralError>
```

---

#### `book_next_import_item_from_peripheral` / `read_last_import_item_from_peripheral`
名前付きペリフェラルからアイテムをインポートします。
```rust
pub fn book_next_import_item_from_peripheral(&mut self, filter: &[u8], target_name: &str)
pub fn read_last_import_item_from_peripheral(&self) -> Result<i64, PeripheralError>
```

---

### 流体オペレーション

#### `book_next_list_fluids` / `read_last_list_fluids`
RSネットワーク内の全流体をリストします。
```rust
pub fn book_next_list_fluids(&mut self)
pub fn read_last_list_fluids(&self) -> Result<Vec<MEFluidEntry>, PeripheralError>
```

---

#### `book_next_get_fluid` / `read_last_get_fluid`
フィルターに一致する最初の流体を取得します。
```rust
pub fn book_next_get_fluid(&mut self, filter: &[u8])
pub fn read_last_get_fluid(&self) -> Result<MEFluidEntry, PeripheralError>
```

---

#### `book_next_export_fluid` / `read_last_export_fluid`
隣接するタンクに流体をエクスポートします。
```rust
pub fn book_next_export_fluid(&mut self, filter: &[u8], side: &str)
pub fn read_last_export_fluid(&self) -> Result<i64, PeripheralError>
```

---

#### `book_next_import_fluid` / `read_last_import_fluid`
隣接するタンクから流体をインポートします。
```rust
pub fn book_next_import_fluid(&mut self, filter: &[u8], side: &str)
pub fn read_last_import_fluid(&self) -> Result<i64, PeripheralError>
```

---

#### `book_next_export_fluid_to_peripheral` / `read_last_export_fluid_to_peripheral`
名前付きペリフェラルに流体をエクスポートします。
```rust
pub fn book_next_export_fluid_to_peripheral(&mut self, filter: &[u8], target_name: &str)
pub fn read_last_export_fluid_to_peripheral(&self) -> Result<i64, PeripheralError>
```

---

#### `book_next_import_fluid_from_peripheral` / `read_last_import_fluid_from_peripheral`
名前付きペリフェラルから流体をインポートします。
```rust
pub fn book_next_import_fluid_from_peripheral(&mut self, filter: &[u8], target_name: &str)
pub fn read_last_import_fluid_from_peripheral(&self) -> Result<i64, PeripheralError>
```

---

### 化学物質オペレーション

#### `book_next_list_chemicals` / `read_last_list_chemicals`
RSネットワーク内の全化学物質をリストします。
```rust
pub fn book_next_list_chemicals(&mut self)
pub fn read_last_list_chemicals(&self) -> Result<Vec<MEChemicalEntry>, PeripheralError>
```

---

#### `book_next_get_chemical` / `read_last_get_chemical`
フィルターに一致する最初の化学物質を取得します。
```rust
pub fn book_next_get_chemical(&mut self, filter: &[u8])
pub fn read_last_get_chemical(&self) -> Result<MEChemicalEntry, PeripheralError>
```

---

#### `book_next_export_chemical` / `read_last_export_chemical`
化学物質をエクスポートします。
```rust
pub fn book_next_export_chemical(&mut self, filter: &[u8], side: &str)
pub fn read_last_export_chemical(&self) -> Result<i64, PeripheralError>
```

---

#### `book_next_import_chemical` / `read_last_import_chemical`
化学物質をインポートします。
```rust
pub fn book_next_import_chemical(&mut self, filter: &[u8], side: &str)
pub fn read_last_import_chemical(&self) -> Result<i64, PeripheralError>
```

---

#### `book_next_export_chemical_to_peripheral` / `read_last_export_chemical_to_peripheral`
名前付きペリフェラルに化学物質をエクスポートします。
```rust
pub fn book_next_export_chemical_to_peripheral(&mut self, filter: &[u8], target_name: &str)
pub fn read_last_export_chemical_to_peripheral(&self) -> Result<i64, PeripheralError>
```

---

#### `book_next_import_chemical_from_peripheral` / `read_last_import_chemical_from_peripheral`
名前付きペリフェラルから化学物質をインポートします。
```rust
pub fn book_next_import_chemical_from_peripheral(&mut self, filter: &[u8], target_name: &str)
pub fn read_last_import_chemical_from_peripheral(&self) -> Result<i64, PeripheralError>
```

---

### クラフティング

#### `book_next_craft_item` / `read_last_craft_item`
アイテムのクラフトをリクエストします。
```rust
pub fn book_next_craft_item(&mut self, filter: &[u8])
pub fn read_last_craft_item(&self) -> Result<bool, PeripheralError>
```

---

#### `book_next_craft_fluid` / `read_last_craft_fluid`
流体のクラフトをリクエストします。
```rust
pub fn book_next_craft_fluid(&mut self, filter: &[u8])
pub fn read_last_craft_fluid(&self) -> Result<bool, PeripheralError>
```

---

#### `book_next_craft_chemical` / `read_last_craft_chemical`
化学物質のクラフトをリクエストします。
```rust
pub fn book_next_craft_chemical(&mut self, filter: &[u8])
pub fn read_last_craft_chemical(&self) -> Result<bool, PeripheralError>
```

---

#### `book_next_is_item_crafting` / `read_last_is_item_crafting`
アイテムが現在クラフト中かを確認します。
```rust
pub fn book_next_is_item_crafting(&mut self, filter: &[u8])
pub fn read_last_is_item_crafting(&self) -> Result<bool, PeripheralError>
```

---

#### `book_next_is_fluid_crafting` / `read_last_is_fluid_crafting`
流体が現在クラフト中かを確認します。
```rust
pub fn book_next_is_fluid_crafting(&mut self, filter: &[u8])
pub fn read_last_is_fluid_crafting(&self) -> Result<bool, PeripheralError>
```

---

### ストレージ・エネルギー

#### `book_next_get_energy_storage` / `read_last_get_energy_storage`
蓄積エネルギーを取得します。
```rust
pub fn book_next_get_energy_storage(&mut self)
pub fn read_last_get_energy_storage(&self) -> Result<f64, PeripheralError>
```

---

#### `book_next_get_max_energy_storage` / `read_last_get_max_energy_storage`
最大エネルギー容量を取得します。
```rust
pub fn book_next_get_max_energy_storage(&mut self)
pub fn read_last_get_max_energy_storage(&self) -> Result<f64, PeripheralError>
```

---

#### `book_next_get_avg_power_usage` / `read_last_get_avg_power_usage`
平均電力使用量を取得します。
```rust
pub fn book_next_get_avg_power_usage(&mut self)
pub fn read_last_get_avg_power_usage(&self) -> Result<f64, PeripheralError>
```

---

#### `book_next_get_avg_power_injection` / `read_last_get_avg_power_injection`
平均電力注入量を取得します。
```rust
pub fn book_next_get_avg_power_injection(&mut self)
pub fn read_last_get_avg_power_injection(&self) -> Result<f64, PeripheralError>
```

---

#### ストレージ容量メソッド
- `book_next_get_total_item_storage` / `read_last_get_total_item_storage` → `i64`
- `book_next_get_used_item_storage` / `read_last_get_used_item_storage` → `i64`
- `book_next_get_available_item_storage` / `read_last_get_available_item_storage` → `i64`
- `book_next_get_total_fluid_storage` / `read_last_get_total_fluid_storage` → `i64`
- `book_next_get_used_fluid_storage` / `read_last_get_used_fluid_storage` → `i64`
- `book_next_get_available_fluid_storage` / `read_last_get_available_fluid_storage` → `i64`
- `book_next_get_total_chemical_storage` / `read_last_get_total_chemical_storage` → `i64`
- `book_next_get_used_chemical_storage` / `read_last_get_used_chemical_storage` → `i64`
- `book_next_get_available_chemical_storage` / `read_last_get_available_chemical_storage` → `i64`

## 実装状況

### ✅ 実装済み

- All book_next_* / read_last_* methods

### 🚧 未実装

- async_* variants for all methods


## イミディエイトメソッド

なし。

## 型定義

型は `me_bridge` から再エクスポートされています：

```rust
pub use super::me_bridge::{MEItemEntry, MEFluidEntry, MEChemicalEntry};
```

型定義の詳細は [MEBridge](MEBridge.md) を参照してください。

## 使用例

```rust
use rust_computers_api::advanced_peripherals::RSBridge;
use rust_computers_api::peripheral::Peripheral;

let mut rs = RSBridge::wrap(addr);

loop {
    let items = rs.read_last_list_items();
    let energy = rs.read_last_get_energy_storage();

    rs.book_next_list_items();
    rs.book_next_get_energy_storage();
    wait_for_next_tick().await;
}
```
