# Inventory

**Mod:** CC:Tweaked  
**ペリフェラルタイプ:** `inventory`  
**ソース:** `AbstractInventoryMethods.java`

## 概要

Inventoryペリフェラルは、ブロックインベントリ（チェスト、バレル、かまどなど）への有線ネットワーク経由のアクセスを提供します。インベントリの内容を照会し、詳細なアイテム情報を取得し、接続されたインベントリ間でアイテムを転送できます。

## 3つの関数パターン

Inventory APIは全メソッドで3つの関数パターンを使用します：

1. **`book_next_*`** — 次のティックのリクエストをスケジュール
2. **`read_last_*`** — 前のティックの結果を読み取り
3. **`async_*`** — book、待機、読み取りを1つの呼び出しで行う便利メソッド

### パターン説明

```lua
-- 方法1: book_next / read_last パターン
inventory.book_next_list()
wait_for_next_tick()
local items = inventory.read_last_list()

-- 方法2: async パターン（推奨）
local items = inventory.async_list()
```

## メソッド

### `size()` / `book_next_size()` / `read_last_size()` / `async_size()`

インベントリのスロット数を取得します。

**Lua署名:**
```lua
function size() -> number
```

**Rust署名:**
```rust
pub fn book_next_size(&mut self)
pub fn read_last_size(&self) -> Result<u32, PeripheralError>
pub async fn async_size(&self) -> Result<u32, PeripheralError>
```

**戻り値:** `number` — スロット総数

**例:**
```lua
local inventory = peripheral.find("inventory")
local slot_count = inventory.async_size()
print("Inventory has " .. slot_count .. " slots")
```

---

### `list()` / `book_next_list()` / `read_last_list()` / `async_list()`

インベントリ内の全アイテムをサマリー情報付きでリストアップします。

**Lua署名:**
```lua
function list() -> table
```

**Rust署名:**
```rust
pub fn book_next_list(&mut self)
pub fn read_last_list(&self) -> Result<BTreeMap<u32, SlotInfo>, PeripheralError>
pub async fn async_list(&self) -> Result<BTreeMap<u32, SlotInfo>, PeripheralError>
```

**戻り値:** `table` — スロット番号をキーとするスパーステーブル

**アイテム情報構造:**
```lua
{
  name = "minecraft:diamond",  -- アイテムレジストリ名
  count = 5,                    -- スタックサイズ
}
```

**例:**
```lua
local inventory = peripheral.find("inventory")
local items = inventory.async_list()

for slot, item in pairs(items) do
  print(("Slot %d: %d x %s"):format(slot, item.count, item.name))
end
```

---

### `getItemDetail(slot)` / `book_next_get_item_detail(slot)` / `read_last_get_item_detail()` / `async_get_item_detail(slot)`

特定スロットのアイテムの詳細情報を取得します。

**Lua署名:**
```lua
function getItemDetail(slot: number) -> table | nil
```

**Rust署名:**
```rust
pub fn book_next_get_item_detail(&mut self, slot: u32)
pub fn read_last_get_item_detail(&self) -> Result<Option<ItemDetail>, PeripheralError>
pub async fn async_get_item_detail(&self, slot: u32) -> Result<Option<ItemDetail>, PeripheralError>
```

**パラメータ:**
- `slot: number` — スロットインデックス（1から始まる）

**戻り値:** `table | nil` — アイテム詳細テーブルまたはスロットが空の場合は `nil`

**アイテム詳細構造:**
```lua
{
  name = "minecraft:diamond",      -- アイテムレジストリ名
  count = 5,                        -- スタックサイズ
  displayName = "Diamond",          -- 表示名
  maxCount = 64,                    -- 最大スタックサイズ
  damage = 10,                      -- 耐久値ダメージ（オプション）
  maxDamage = 100,                  -- 最大耐久値（オプション）
  tags = {                          -- アイテムタグ
    ["minecraft:gems"] = true,
  },
}
```

**例:**
```lua
local inventory = peripheral.find("inventory")
local item = inventory.async_get_item_detail(1)

if item then
  print(("Slot 1: %d x %s"):format(item.count, item.displayName))
  if item.damage then
    print(("Durability: %d/%d"):format(item.damage, item.maxDamage))
  end
else
  print("Slot 1 is empty")
end
```

**エラーハンドリング:**
- スロット番号が範囲外の場合、エラーをスロー

---

### `getItemLimit(slot)` / `book_next_get_item_limit(slot)` / `read_last_get_item_limit()` / `async_get_item_limit(slot)`

スロットに保存できるアイテムの最大数を取得します。

**Lua署名:**
```lua
function getItemLimit(slot: number) -> number
```

**Rust署名:**
```rust
pub fn book_next_get_item_limit(&mut self, slot: u32)
pub fn read_last_get_item_limit(&self) -> Result<u32, PeripheralError>
pub async fn async_get_item_limit(&self, slot: u32) -> Result<u32, PeripheralError>
```

**パラメータ:**
- `slot: number` — スロットインデックス（1から始まる）

**戻り値:** `number` — このスロットの最大アイテム数（通常は64、特殊なインベントリではより大きい）

**例:**
```lua
local inventory = peripheral.find("inventory")
local limit = inventory.async_get_item_limit(1)
print("Slot 1 can hold up to " .. limit .. " items")
```

---

### `pushItems(toName, fromSlot, limit?, toSlot?)` / `book_next_push_items(...)` / `read_last_push_items()` / `async_push_items(...)`

このインベントリから別の接続されたインベントリにアイテムを転送します。

**Lua署名:**
```lua
function pushItems(toName: string, fromSlot: number, limit?: number, toSlot?: number) -> number
```

**Rust署名:**
```rust
pub fn book_next_push_items(&mut self, to_name: &str, from_slot: u32, limit: Option<u32>, to_slot: Option<u32>)
pub fn read_last_push_items(&self) -> Result<u32, PeripheralError>
pub async fn async_push_items(&self, to_name: &str, from_slot: u32, limit: Option<u32>, to_slot: Option<u32>) -> Result<u32, PeripheralError>
```

**パラメータ:**
- `toName: string` — 転送先インベントリの名前（`peripheral.getNamesRemote()` から取得）
- `fromSlot: number` — 転送元スロットインデックス（1から始まる）
- `limit?: number` — 転送する最大アイテム数（オプション、デフォルトはスタック上限）
- `toSlot?: number` — 転送先スロットインデックス（オプション、省略時は自動選択）

**戻り値:** `number` — 実際に転送されたアイテム数

**要件:**
- 両方のインベントリが有線モデムとネットワークケーブルで接続されている必要があります
- 転送先インベントリが存在し、アクセス可能である必要があります

**例:**
```lua
local source = peripheral.find("minecraft:chest_0")
local dest_name = "minecraft:chest_1"

-- スロット1からダイヤモンド32個を転送先に移動
local moved = source.async_push_items(dest_name, 1, 32)
print("Moved " .. moved .. " items")
```

**エラーハンドリング:**
- 転送先インベントリが存在しない場合、エラーをスロー
- スロットが範囲外の場合、エラーをスロー

---

### `pullItems(fromName, fromSlot, limit?, toSlot?)` / `book_next_pull_items(...)` / `read_last_pull_items()` / `async_pull_items(...)`

別の接続されたインベントリからこのインベントリにアイテムを転送します。

**Lua署名:**
```lua
function pullItems(fromName: string, fromSlot: number, limit?: number, toSlot?: number) -> number
```

**Rust署名:**
```rust
pub fn book_next_pull_items(&mut self, from_name: &str, from_slot: u32, limit: Option<u32>, to_slot: Option<u32>)
pub fn read_last_pull_items(&self) -> Result<u32, PeripheralError>
pub async fn async_pull_items(&self, from_name: &str, from_slot: u32, limit: Option<u32>, to_slot: Option<u32>) -> Result<u32, PeripheralError>
```

**パラメータ:**
- `fromName: string` — 転送元インベントリの名前（`peripheral.getNamesRemote()` から取得）
- `fromSlot: number` — 転送元スロットインデックス（1から始まる）
- `limit?: number` — 転送する最大アイテム数（オプション、デフォルトはスタック上限）
- `toSlot?: number` — 転送先スロットインデックス（オプション、省略時は自動選択）

**戻り値:** `number` — 実際に転送されたアイテム数

**要件:**
- 両方のインベントリが有線モデムとネットワークケーブルで接続されている必要があります
- 転送元インベントリが存在し、アクセス可能である必要があります

**例:**
```lua
local dest = peripheral.find("minecraft:chest_0")
local source_name = "minecraft:chest_1"

-- 転送元スロット1からダイヤモンド32個を引き出す
local moved = dest.async_pull_items(source_name, 1, 32)
print("Pulled " .. moved .. " items")
```

**エラーハンドリング:**
- 転送元インベントリが存在しない場合、エラーをスロー
- スロットが範囲外の場合、エラーをスロー

---

## イベント

Inventoryペリフェラルはイベントを生成しません。

---

## 使用例

### 例1: 全アイテムをリストアップ

```lua
local inventory = peripheral.find("minecraft:chest")

local items = inventory.async_list()
for slot, item in pairs(items) do
  print(("Slot %d: %d x %s"):format(slot, item.count, item.name))
end
```

### 例2: 名前でアイテムを検索

```lua
local inventory = peripheral.find("minecraft:chest")

local function find_item(name)
  local items = inventory.async_list()
  for slot, item in pairs(items) do
    if item.name == name then
      return slot, item
    end
  end
  return nil
end

local slot, item = find_item("minecraft:diamond")
if slot then
  print(("Found %d diamonds in slot %d"):format(item.count, slot))
end
```

### 例3: チェスト間でアイテムを転送

```lua
local source = peripheral.find("minecraft:chest_0")
local dest_name = "minecraft:chest_1"

-- 転送元から転送先へ全アイテムを転送
local items = source.async_list()
for slot, item in pairs(items) do
  local moved = source.async_push_items(dest_name, slot)
  print(("Moved %d items from slot %d"):format(moved, slot))
end
```

### 例4: 総容量を計算

```lua
local inventory = peripheral.find("minecraft:chest")

local total_capacity = 0
for i = 1, inventory.async_size() do
  total_capacity = total_capacity + inventory.async_get_item_limit(i)
end

print("Total capacity: " .. total_capacity)
```

### 例5: インベントリをソート

```lua
local inventory = peripheral.find("minecraft:chest")

-- アイテムを名前でソート
local items = inventory.async_list()
local sorted = {}
for slot, item in pairs(items) do
  table.insert(sorted, {slot = slot, item = item})
end

table.sort(sorted, function(a, b)
  return a.item.name < b.item.name
end)

for _, entry in ipairs(sorted) do
  print(("Slot %d: %s"):format(entry.slot, entry.item.name))
end
```

---

## エラーハンドリング

全メソッドは以下の場合にエラーをスロー可能です：

- **Inventoryが見つからない**: ペリフェラルが切断されているか、アクセス不可
- **無効なスロット**: スロット番号が範囲外（1からサイズまで）
- **ネットワークエラー**: 有線ネットワーク接続が切断されている
- **転送先が見つからない**: ターゲットインベントリが存在しないか、アクセス不可

**エラーハンドリング例:**
```lua
local inventory = peripheral.find("minecraft:chest")
if not inventory then
  error("No inventory found")
end

local success, result = pcall(function()
  return inventory.async_list()
end)

if not success then
  print("Error: " .. result)
else
  print("Got " .. #result .. " items")
end
```

---

## 型定義

### ItemDetail
```lua
{
  name: string,           -- レジストリ名（例: "minecraft:diamond"）
  count: number,          -- スタックサイズ
  displayName: string,    -- 表示名
  maxCount: number,       -- 最大スタックサイズ
  damage?: number,        -- 耐久値ダメージ（オプション）
  maxDamage?: number,     -- 最大耐久値（オプション）
  tags?: table,           -- アイテムタグ（オプション）
}
```

### SlotInfo
```lua
{
  name: string,   -- レジストリ名
  count: number,  -- スタックサイズ
}
```

---

## 注記

- 全スロットインデックスは1から始まります（最初のスロットは0ではなく1）
- `pushItems` と `pullItems` は有線ネットワーク接続が必要です
- インベントリ名は有線モデムの `peripheral.getNamesRemote()` から取得します
- 空のスロットは `list()` の結果に含まれません（スパーステーブル）
- 3つの関数パターンは効率的なバッチ操作を可能にします

---

## 関連

- [Modem](./Modem.md) — 有線ネットワーク通信に必要
- [CC:Tweaked Documentation](https://tweaked.cc/) — 公式ドキュメント
