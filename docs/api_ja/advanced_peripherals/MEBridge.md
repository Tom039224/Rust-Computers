# MEBridge

**モジュール:** AdvancedPeripherals  
**ペリフェラルタイプ:** `advancedPeripherals:me_bridge`  
**ソース:** `MEBridgePeripheral.java`

## 概要

MEBridgeペリフェラルは、Applied Energistics 2（AE2）MEネットワークへの完全なアクセスを提供します。MEシステム内のアイテム、流体、化学物質をクエリおよび操作し、クラフト操作をリクエストし、エネルギーレベルを監視し、ストレージを管理できます。これはAE2ネットワークと相互作用するオートメーションシステムの作成に不可欠です。

## 3つの関数パターン

MEBridge APIは、すべてのメソッドに対して3つの関数パターンを使用します：

1. **`book_next_*`** - 次のティックのリクエストをスケジュール
2. **`read_last_*`** - 前のティックの結果を読み取り
3. **`async_*`** - 便利なメソッド（book、待機、読み取りを1つの呼び出しで実行）

### パターンの説明

```lua
-- 方法1: book_next / read_last パターン
me.book_next_list_items()
wait_for_next_tick()
local items = me.read_last_list_items()

-- 方法2: async パターン（推奨）
local items = me.async_list_items()
```

## メソッド

### アイテム操作

#### `listItems()` / `book_next_list_items()` / `read_last_list_items()` / `async_list_items()`

MEネットワークに現在保存されているすべてのアイテムをリストします。

**Lua署名:**
```lua
function listItems() -> table
```

**Rust署名:**
```rust
pub fn book_next_list_items(&mut self)
pub fn read_last_list_items(&self) -> Result<Vec<MEItemEntry>, PeripheralError>
pub async fn async_list_items(&self) -> Result<Vec<MEItemEntry>, PeripheralError>
```

**戻り値:** `table` — アイテムエントリの配列

**例:**
```lua
local me = peripheral.find("advancedPeripherals:me_bridge")
local items = me.async_list_items()

for _, item in ipairs(items) do
  print(("アイテム: %s、数量: %d"):format(item.displayName, item.count))
end
```

---

#### `getItem(filter)` / `book_next_get_item(filter)` / `read_last_get_item()` / `async_get_item(filter)`

指定されたフィルターに一致する最初のアイテムを取得します。

**Lua署名:**
```lua
function getItem(filter: table) -> table | nil
```

**Rust署名:**
```rust
pub fn book_next_get_item(&mut self, filter: &[u8])
pub fn read_last_get_item(&self) -> Result<MEItemEntry, PeripheralError>
pub async fn async_get_item(&self, filter: &[u8]) -> Result<MEItemEntry, PeripheralError>
```

**パラメータ:**
- `filter: table` — `name`、`displayName`、`tags`などのプロパティを持つフィルターテーブル

**フィルター例:**
```lua
-- 名前で
{name = "minecraft:diamond"}

-- 表示名で
{displayName = "ダイヤモンド"}

-- タグで
{tags = {"minecraft:gems"}}

-- 複数条件
{name = "minecraft:diamond", count = {min = 10}}
```

**戻り値:** `table | nil` — アイテムエントリまたは見つからない場合は`nil`

**例:**
```lua
local me = peripheral.find("advancedPeripherals:me_bridge")
local item = me.async_get_item({name = "minecraft:diamond"})

if item then
  print("ダイヤモンド " .. item.count .. " 個を見つけました")
else
  print("ダイヤモンドが見つかりません")
end
```

---

#### `exportItem(filter, side, amount?)` / `book_next_export_item(...)` / `read_last_export_item()` / `async_export_item(...)`

MEネットワークから隣接するインベントリにアイテムをエクスポートします。

**Lua署名:**
```lua
function exportItem(filter: table, side: string, amount?: number) -> number
```

**Rust署名:**
```rust
pub fn book_next_export_item(&mut self, filter: &[u8], side: &str, amount: Option<u32>)
pub fn read_last_export_item(&self) -> Result<u32, PeripheralError>
pub async fn async_export_item(&self, filter: &[u8], side: &str, amount: Option<u32>) -> Result<u32, PeripheralError>
```

**パラメータ:**
- `filter: table` — アイテムフィルター
- `side: string` — 方向（"north"、"south"、"east"、"west"、"up"、"down"）
- `amount?: number` — エクスポートする最大アイテム数（オプション）

**戻り値:** `number` — 実際にエクスポートされたアイテム数

**例:**
```lua
local me = peripheral.find("advancedPeripherals:me_bridge")
local exported = me.async_export_item({name = "minecraft:diamond"}, "north", 64)
print("ダイヤモンド " .. exported .. " 個をエクスポートしました")
```

---

#### `importItem(filter, side, amount?)` / `book_next_import_item(...)` / `read_last_import_item()` / `async_import_item(...)`

隣接するインベントリからMEネットワークにアイテムをインポートします。

**Lua署名:**
```lua
function importItem(filter: table, side: string, amount?: number) -> number
```

**Rust署名:**
```rust
pub fn book_next_import_item(&mut self, filter: &[u8], side: &str, amount: Option<u32>)
pub fn read_last_import_item(&self) -> Result<u32, PeripheralError>
pub async fn async_import_item(&self, filter: &[u8], side: &str, amount: Option<u32>) -> Result<u32, PeripheralError>
```

**パラメータ/戻り値:** `exportItem`と同じ

**例:**
```lua
local me = peripheral.find("advancedPeripherals:me_bridge")
local imported = me.async_import_item({}, "north", 64)
print("アイテム " .. imported .. " 個をインポートしました")
```

---

### 流体操作

#### `listFluids()` / `book_next_list_fluids()` / `read_last_list_fluids()` / `async_list_fluids()`

MEネットワークに現在保存されているすべての流体をリストします。

**Lua署名:**
```lua
function listFluids() -> table
```

**Rust署名:**
```rust
pub fn book_next_list_fluids(&mut self)
pub fn read_last_list_fluids(&self) -> Result<Vec<MEFluidEntry>, PeripheralError>
pub async fn async_list_fluids(&self) -> Result<Vec<MEFluidEntry>, PeripheralError>
```

**戻り値:** `table` — 流体エントリの配列

**例:**
```lua
local me = peripheral.find("advancedPeripherals:me_bridge")
local fluids = me.async_list_fluids()

for _, fluid in ipairs(fluids) do
  print(("流体: %s、量: %d mB"):format(fluid.displayName, fluid.count))
end
```

---

#### `getFluid(filter)` / `book_next_get_fluid(filter)` / `read_last_get_fluid()` / `async_get_fluid(filter)`

指定されたフィルターに一致する最初の流体を取得します。

**Lua署名:**
```lua
function getFluid(filter: table) -> table | nil
```

**Rust署名:**
```rust
pub fn book_next_get_fluid(&mut self, filter: &[u8])
pub fn read_last_get_fluid(&self) -> Result<MEFluidEntry, PeripheralError>
pub async fn async_get_fluid(&self, filter: &[u8]) -> Result<MEFluidEntry, PeripheralError>
```

**パラメータ/戻り値:** `getItem`と同じ

---

#### `exportFluid(filter, side, amount?)` / `book_next_export_fluid(...)` / `read_last_export_fluid()` / `async_export_fluid(...)`

MEから隣接するタンクに流体をエクスポートします。

**Lua署名:**
```lua
function exportFluid(filter: table, side: string, amount?: number) -> number
```

**Rust署名:**
```rust
pub fn book_next_export_fluid(&mut self, filter: &[u8], side: &str, amount: Option<u32>)
pub fn read_last_export_fluid(&self) -> Result<u32, PeripheralError>
pub async fn async_export_fluid(&self, filter: &[u8], side: &str, amount: Option<u32>) -> Result<u32, PeripheralError>
```

**パラメータ/戻り値:** `exportItem`と同じ（量はmB単位）

---

#### `importFluid(filter, side, amount?)` / `book_next_import_fluid(...)` / `read_last_import_fluid()` / `async_import_fluid(...)`

隣接するタンクからMEに流体をインポートします。

**Lua署名:**
```lua
function importFluid(filter: table, side: string, amount?: number) -> number
```

**Rust署名:**
```rust
pub fn book_next_import_fluid(&mut self, filter: &[u8], side: &str, amount: Option<u32>)
pub fn read_last_import_fluid(&self) -> Result<u32, PeripheralError>
pub async fn async_import_fluid(&self, filter: &[u8], side: &str, amount: Option<u32>) -> Result<u32, PeripheralError>
```

**パラメータ/戻り値:** `importItem`と同じ

---

### クラフト操作

#### `craftItem(filter, amount?)` / `book_next_craft_item(...)` / `read_last_craft_item()` / `async_craft_item(...)`

MEネットワークからアイテムのクラフトをリクエストします。

**Lua署名:**
```lua
function craftItem(filter: table, amount?: number) -> boolean
```

**Rust署名:**
```rust
pub fn book_next_craft_item(&mut self, filter: &[u8], amount: Option<u32>)
pub fn read_last_craft_item(&self) -> Result<bool, PeripheralError>
pub async fn async_craft_item(&self, filter: &[u8], amount: Option<u32>) -> Result<bool, PeripheralError>
```

**パラメータ:**
- `filter: table` — アイテムフィルター
- `amount?: number` — クラフト量

**戻り値:** `boolean` — クラフトリクエストが受け入れられた場合は`true`

**例:**
```lua
local me = peripheral.find("advancedPeripherals:me_bridge")
local ok = me.async_craft_item({name = "minecraft:diamond_pickaxe"})
print("クラフトリクエスト: " .. tostring(ok))
```

---

#### `isItemCrafting(filter)` / `book_next_is_item_crafting(filter)` / `read_last_is_item_crafting()` / `async_is_item_crafting(filter)`

アイテムが現在クラフト中かどうかを確認します。

**Lua署名:**
```lua
function isItemCrafting(filter: table) -> boolean
```

**Rust署名:**
```rust
pub fn book_next_is_item_crafting(&mut self, filter: &[u8])
pub fn read_last_is_item_crafting(&self) -> Result<bool, PeripheralError>
pub async fn async_is_item_crafting(&self, filter: &[u8]) -> Result<bool, PeripheralError>
```

**パラメータ/戻り値:** `craftItem`と同じ

---

### ストレージ＆エネルギー監視

#### `getEnergyStorage()` / `book_next_get_energy_storage()` / `read_last_get_energy_storage()` / `async_get_energy_storage()`

MEネットワークに現在保存されているエネルギーを取得します。

**Lua署名:**
```lua
function getEnergyStorage() -> number
```

**Rust署名:**
```rust
pub fn book_next_get_energy_storage(&mut self)
pub fn read_last_get_energy_storage(&self) -> Result<f64, PeripheralError>
pub async fn async_get_energy_storage(&self) -> Result<f64, PeripheralError>
```

**戻り値:** `number` — AEユニット単位のエネルギー

**例:**
```lua
local me = peripheral.find("advancedPeripherals:me_bridge")
local energy = me.async_get_energy_storage()
print("エネルギー: " .. energy .. " AE")
```

---

#### `getMaxEnergyStorage()` / `book_next_get_max_energy_storage()` / `read_last_get_max_energy_storage()` / `async_get_max_energy_storage()`

MEネットワークの最大エネルギー容量を取得します。

**Lua署名:**
```lua
function getMaxEnergyStorage() -> number
```

**Rust署名:**
```rust
pub fn book_next_get_max_energy_storage(&mut self)
pub fn read_last_get_max_energy_storage(&self) -> Result<f64, PeripheralError>
pub async fn async_get_max_energy_storage(&self) -> Result<f64, PeripheralError>
```

**戻り値:** `number` — 最大エネルギー容量

---

#### `getTotalItemStorage()` / `book_next_get_total_item_storage()` / `read_last_get_total_item_storage()` / `async_get_total_item_storage()`

総アイテムストレージ容量を取得します。

**Lua署名:**
```lua
function getTotalItemStorage() -> number
```

**Rust署名:**
```rust
pub fn book_next_get_total_item_storage(&mut self)
pub fn read_last_get_total_item_storage(&self) -> Result<i64, PeripheralError>
pub async fn async_get_total_item_storage(&self) -> Result<i64, PeripheralError>
```

**戻り値:** `number` — 総アイテムスロット数

---

#### `getUsedItemStorage()` / `book_next_get_used_item_storage()` / `read_last_get_used_item_storage()` / `async_get_used_item_storage()`

使用されているアイテムストレージを取得します。

**Lua署名:**
```lua
function getUsedItemStorage() -> number
```

**Rust署名:**
```rust
pub fn book_next_get_used_item_storage(&mut self)
pub fn read_last_get_used_item_storage(&self) -> Result<i64, PeripheralError>
pub async fn async_get_used_item_storage(&self) -> Result<i64, PeripheralError>
```

**戻り値:** `number` — 使用されているアイテムスロット数

---

#### `getAvailableItemStorage()` / `book_next_get_available_item_storage()` / `read_last_get_available_item_storage()` / `async_get_available_item_storage()`

利用可能なアイテムストレージスペースを取得します。

**Lua署名:**
```lua
function getAvailableItemStorage() -> number
```

**Rust署名:**
```rust
pub fn book_next_get_available_item_storage(&mut self)
pub fn read_last_get_available_item_storage(&self) -> Result<i64, PeripheralError>
pub async fn async_get_available_item_storage(&self) -> Result<i64, PeripheralError>
```

**戻り値:** `number` — 利用可能なアイテムスロット数

---

#### `getTotalFluidStorage()` / `book_next_get_total_fluid_storage()` / `read_last_get_total_fluid_storage()` / `async_get_total_fluid_storage()`

総流体ストレージ容量をmB単位で取得します。

**Lua署名:**
```lua
function getTotalFluidStorage() -> number
```

**Rust署名:**
```rust
pub fn book_next_get_total_fluid_storage(&mut self)
pub fn read_last_get_total_fluid_storage(&self) -> Result<i64, PeripheralError>
pub async fn async_get_total_fluid_storage(&self) -> Result<i64, PeripheralError>
```

**戻り値:** `number` — mB単位の総流体容量

---

#### `getUsedFluidStorage()` / `book_next_get_used_fluid_storage()` / `read_last_get_used_fluid_storage()` / `async_get_used_fluid_storage()`

使用されている流体ストレージを取得します。

**Lua署名:**
```lua
function getUsedFluidStorage() -> number
```

**Rust署名:**
```rust
pub fn book_next_get_used_fluid_storage(&mut self)
pub fn read_last_get_used_fluid_storage(&self) -> Result<i64, PeripheralError>
pub async fn async_get_used_fluid_storage(&self) -> Result<i64, PeripheralError>
```

**戻り値:** `number` — mB単位で使用されている流体ストレージ

---

#### `getAvailableFluidStorage()` / `book_next_get_available_fluid_storage()` / `read_last_get_available_fluid_storage()` / `async_get_available_fluid_storage()`

利用可能な流体ストレージスペースを取得します。

**Lua署名:**
```lua
function getAvailableFluidStorage() -> number
```

**Rust署名:**
```rust
pub fn book_next_get_available_fluid_storage(&mut self)
pub fn read_last_get_available_fluid_storage(&self) -> Result<i64, PeripheralError>
pub async fn async_get_available_fluid_storage(&self) -> Result<i64, PeripheralError>
```

**戻り値:** `number` — mB単位で利用可能な流体ストレージ

---

## イベント

MEBridgeペリフェラルはイベントを生成しません。

---

## 使用例

### 例1: すべてのアイテムと流体をリスト

```lua
local me = peripheral.find("advancedPeripherals:me_bridge")

local items = me.async_list_items()
print("MEのアイテム:")
for _, item in ipairs(items) do
  print(("  %s: %d"):format(item.displayName, item.count))
end

local fluids = me.async_list_fluids()
print("MEの流体:")
for _, fluid in ipairs(fluids) do
  print(("  %s: %d mB"):format(fluid.displayName, fluid.count))
end
```

### 例2: 隣接するインベントリにアイテムをエクスポート

```lua
local me = peripheral.find("advancedPeripherals:me_bridge")

-- 北にダイヤモンド64個をエクスポート
local exported = me.async_export_item({name = "minecraft:diamond"}, "north", 64)
print("ダイヤモンド " .. exported .. " 個をエクスポートしました")
```

### 例3: エネルギーレベルを監視

```lua
local me = peripheral.find("advancedPeripherals:me_bridge")

while true do
  local energy = me.async_get_energy_storage()
  local max_energy = me.async_get_max_energy_storage()
  local percent = (energy / max_energy) * 100
  
  print(("エネルギー: %.1f%%"):format(percent))
  
  if percent < 10 then
    print("警告: エネルギーが低い！")
  end
  
  sleep(1)
end
```

### 例4: 必要に応じてアイテムをクラフト

```lua
local me = peripheral.find("advancedPeripherals:me_bridge")

local function craft_if_needed(item_name, min_count)
  local item = me.async_get_item({name = item_name})
  
  if not item or item.count < min_count then
    print("クラフト中: " .. item_name)
    me.async_craft_item({name = item_name}, min_count)
  end
end

craft_if_needed("minecraft:diamond_pickaxe", 1)
```

### 例5: ストレージ監視

```lua
local me = peripheral.find("advancedPeripherals:me_bridge")

local function print_storage_status()
  local item_used = me.async_get_used_item_storage()
  local item_total = me.async_get_total_item_storage()
  local item_percent = (item_used / item_total) * 100
  
  local fluid_used = me.async_get_used_fluid_storage()
  local fluid_total = me.async_get_total_fluid_storage()
  local fluid_percent = (fluid_used / fluid_total) * 100
  
  print(("アイテムストレージ: %.1f%%"):format(item_percent))
  print(("流体ストレージ: %.1f%%"):format(fluid_percent))
end

print_storage_status()
```

---

## エラーハンドリング

すべてのメソッドは以下の場合にエラーをスローする可能性があります：

- **MEネットワークが見つからない**: MEBridgeがMEネットワークに接続されていない
- **ペリフェラル切断**: MEBridgeにアクセスできなくなった
- **無効なフィルター**: フィルターテーブルが不正
- **クラフト不可**: アイテムをクラフトできない（レシピなし）
- **ストレージ不足**: アイテムをインポートするスペースが不足

**エラーハンドリングの例:**
```lua
local me = peripheral.find("advancedPeripherals:me_bridge")
if not me then
  error("MEBridgeが見つかりません")
end

local success, result = pcall(function()
  return me.async_list_items()
end)

if not success then
  print("エラー: " .. result)
else
  print("見つかったアイテムタイプ: " .. #result)
end
```

---

## 型定義

### MEItemEntry
```lua
{
  name: string,              -- レジストリ名（例："minecraft:diamond"）
  displayName: string,       -- 表示名
  count: number,             -- スタックサイズ
  maxStackSize: number,      -- 最大スタックサイズ
  tags: table,               -- アイテムタグ
  components: table,         -- アイテムコンポーネント（オプション）
  fingerprint: string,       -- ユニークフィンガープリント
}
```

### MEFluidEntry
```lua
{
  name: string,              -- レジストリ名
  displayName: string,       -- 表示名
  count: number,             -- mB単位の量
  tags: table,               -- 流体タグ
  fluidType: table,          -- 流体タイプ情報
  components: table,         -- 流体コンポーネント（オプション）
  fingerprint: string,       -- ユニークフィンガープリント
}
```

### ItemFilter
```lua
{
  name?: string,             -- レジストリ名
  displayName?: string,      -- 表示名
  tags?: table,              -- マッチするタグ
  count?: {min: number, max: number},  -- 数量範囲
}
```

---

## 注記

- すべての量はアイテムの場合はアイテム（スタック）単位、流体の場合はmB（ミリバケット）単位です
- フィルターは名前と表示名の部分一致を使用します
- クラフトリクエストは非同期で、完了に時間がかかる場合があります
- 3つの関数パターンは効率的なバッチ操作を可能にします
- MEBridgeはMEコントローラーとネットワークが機能する必要があります

---

## 関連

- [BlockReader](./BlockReader.md) — ブロック情報読取用
- [PlayerDetector](./PlayerDetector.md) — プレイヤー検出用
- [AdvancedPeripherals ドキュメント](https://advancedperipherals.readthedocs.io/) — 公式ドキュメント
