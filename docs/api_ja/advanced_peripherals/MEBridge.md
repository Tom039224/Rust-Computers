# MEBridge

**モジュール:** `advanced_peripherals::me_bridge`  
**ペリフェラルタイプ:** `advancedPeripherals:me_bridge`

AdvancedPeripherals ME Bridge ペリフェラル。Applied Energistics 2 MEネットワークへの完全なアクセスを提供します。アイテム、流体、化学物質のオペレーション、クラフティング、ストレージモニタリングをサポートします。

## ブックリードメソッド

### アイテムオペレーション

#### `book_next_list_items` / `read_last_list_items`
MEネットワーク内の全アイテムをリストします。
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
**パラメータ:**
- `filter: &[u8]` — MsgPackエンコードされたフィルターテーブル

**戻り値:** `MEItemEntry`

---

#### `book_next_export_item` / `read_last_export_item`
MEから隣接するインベントリにアイテムをエクスポートします。
```rust
pub fn book_next_export_item(&mut self, filter: &[u8], side: &str)
pub fn read_last_export_item(&self) -> Result<i64, PeripheralError>
```
**パラメータ:**
- `filter: &[u8]` — MsgPackエンコードされたフィルターテーブル
- `side: &str` — 方向（例: `"north"`、`"up"`）

**戻り値:** `i64` — 転送されたアイテム数

---

#### `book_next_import_item` / `read_last_import_item`
隣接するインベントリからMEにアイテムをインポートします。
```rust
pub fn book_next_import_item(&mut self, filter: &[u8], side: &str)
pub fn read_last_import_item(&self) -> Result<i64, PeripheralError>
```
**パラメータ / 戻り値:** `export_item` と同じ

---

#### `book_next_export_item_to_peripheral` / `read_last_export_item_to_peripheral`
MEから名前付きペリフェラルにアイテムをエクスポートします。
```rust
pub fn book_next_export_item_to_peripheral(&mut self, filter: &[u8], target_name: &str)
pub fn read_last_export_item_to_peripheral(&self) -> Result<i64, PeripheralError>
```

---

#### `book_next_import_item_from_peripheral` / `read_last_import_item_from_peripheral`
名前付きペリフェラルからMEにアイテムをインポートします。
```rust
pub fn book_next_import_item_from_peripheral(&mut self, filter: &[u8], target_name: &str)
pub fn read_last_import_item_from_peripheral(&self) -> Result<i64, PeripheralError>
```

---

### 流体オペレーション

#### `book_next_list_fluids` / `read_last_list_fluids`
MEネットワーク内の全流体をリストします。
```rust
pub fn book_next_list_fluids(&mut self)
pub fn read_last_list_fluids(&self) -> Result<Vec<MEFluidEntry>, PeripheralError>
```
**戻り値:** `Vec<MEFluidEntry>`

---

#### `book_next_get_fluid` / `read_last_get_fluid`
フィルターに一致する最初の流体を取得します。
```rust
pub fn book_next_get_fluid(&mut self, filter: &[u8])
pub fn read_last_get_fluid(&self) -> Result<MEFluidEntry, PeripheralError>
```

---

#### `book_next_export_fluid` / `read_last_export_fluid`
MEから隣接するタンクに流体をエクスポートします。
```rust
pub fn book_next_export_fluid(&mut self, filter: &[u8], side: &str)
pub fn read_last_export_fluid(&self) -> Result<i64, PeripheralError>
```

---

#### `book_next_import_fluid` / `read_last_import_fluid`
隣接するタンクからMEに流体をインポートします。
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
MEネットワーク内の全化学物質をリストします。
```rust
pub fn book_next_list_chemicals(&mut self)
pub fn read_last_list_chemicals(&self) -> Result<Vec<MEChemicalEntry>, PeripheralError>
```
**戻り値:** `Vec<MEChemicalEntry>`

---

#### `book_next_get_chemical` / `read_last_get_chemical`
フィルターに一致する最初の化学物質を取得します。
```rust
pub fn book_next_get_chemical(&mut self, filter: &[u8])
pub fn read_last_get_chemical(&self) -> Result<MEChemicalEntry, PeripheralError>
```

---

#### `book_next_export_chemical` / `read_last_export_chemical`
MEから化学物質をエクスポートします。
```rust
pub fn book_next_export_chemical(&mut self, filter: &[u8], side: &str)
pub fn read_last_export_chemical(&self) -> Result<i64, PeripheralError>
```

---

#### `book_next_import_chemical` / `read_last_import_chemical`
MEに化学物質をインポートします。
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
**戻り値:** `bool`

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
MEネットワーク内の蓄積エネルギーを取得します。
```rust
pub fn book_next_get_energy_storage(&mut self)
pub fn read_last_get_energy_storage(&self) -> Result<f64, PeripheralError>
```
**戻り値:** `f64`

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

## イミディエイトメソッド

なし。

## 型定義

```rust
pub struct MEItemEntry {
    pub name: String,
    pub tags: Vec<String>,
    pub count: i64,
    pub display_name: String,       // serde: "displayName"
    pub max_stack_size: Option<i32>, // serde: "maxStackSize"
    pub components: Value,
    pub fingerprint: String,
}

pub struct MEFluidEntry {
    pub name: String,
    pub tags: Vec<String>,
    pub count: i64,
    pub display_name: String,       // serde: "displayName"
    pub fluid_type: Value,          // serde: "fluidType"
    pub components: Value,
    pub fingerprint: String,
}

pub struct MEChemicalEntry {
    pub name: String,
    pub tags: Vec<String>,
    pub is_gaseous: Option<bool>,   // serde: "isGaseous"
    pub radioactivity: Option<f64>,
    pub count: i64,
    pub display_name: String,       // serde: "displayName"
    pub fingerprint: String,
}
```

## 使用例

```rust
use rust_computers_api::advanced_peripherals::MEBridge;
use rust_computers_api::peripheral::Peripheral;

let mut me = MEBridge::wrap(addr);

loop {
    let items = me.read_last_list_items();
    let energy = me.read_last_get_energy_storage();

    me.book_next_list_items();
    me.book_next_get_energy_storage();
    wait_for_next_tick().await;
}
```
