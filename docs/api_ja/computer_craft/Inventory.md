# Inventory

**モジュール:** CC:Tweaked  
**ペリフェラルタイプ:** `inventory`  
**ソース:** `AbstractInventoryMethods.java`

## 概要

Inventoryペリフェラルは、ワイヤードネットワークを通じてブロックインベントリ（チェスト、樽、かまど等）にアクセスできます。インベントリの内容を照会し、詳細なアイテム情報を取得し、接続されたインベントリ間でアイテムを転送できます。

## 3つの関数パターン

Inventory APIは、すべてのメソッドに対して3つの関数パターンを使用します：

1. **`book_next_*`** - 次のティックのリクエストをスケジュール
2. **`read_last_*`** - 前のティックの結果を読み取り
3. **`async_*`** - 便利なメソッド（book、待機、読み取りを1つの呼び出しで実行）

### パターンの説明

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

**戻り値:** `number` — スロットの総数

**例:**
```lua
local inventory = peripheral.find("inventory")
local slot_count = inventory.async_size()
print("インベントリは " .. slot_count .. " スロットあります")
```

---

### `list()` / `book_next_list()` / `read_last_list()` / `async_list()`

インベントリ内のすべてのアイテムを概要情報とともにリストアップします。

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

**戻り値:** `table` — スロット番号をアイテム情報にマッピングするスパーステーブル

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
  print(("スロット %d: %d x %s"):format(slot, item.count, item.name))
end
```

---

### `getItemDetail(slot)` / `book_next_get_item_detail(slot)` / `read_last_get_item_detail()` / `async_get_item_detail(slot)`

特定のスロット内のアイテムの詳細情報を取得します。

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
- `slot: number` — スロットインデックス（1ベース）

**戻り値:** `table | nil` — アイテム詳細テーブルまたはスロットが空の場合は`nil`

**アイテム詳細構造:**
```lua
{
  name = "minecraft:diamond",      -- アイテムレジストリ名
  count = 5,                        -- スタックサイズ
  displayName = "Diamond",          -- 表示名
  maxCount = 64,                    -- 最大スタックサイズ
  damage = 10,                      -- 耐久度ダメージ（オプション）
  maxDamage = 100,                  -- 最大耐久度（オプション）
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
  print(("スロット 1: %d x %s"):format(item.count, item.displayName))
  if item.damage then
    print(("耐久度: %d/%d"):format(item.damage, item.maxDamage))
  end
else
  print("スロット 1 は空です")
end
```

**エラーハンドリング:**
- スロット番号が範囲外の場合、エラーをスロー

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
- `toName: string` — 宛先インベントリの名前（`peripheral.getNamesRemote()`から）
- `fromSlot: number` — ソーススロットインデックス（1ベース）
- `limit?: number` — 転送する最大アイテム数（オプション、デフォルトはスタック制限）
- `toSlot?: number` — 宛先スロットインデックス（オプション、省略時は自動選択）

**戻り値:** `number` — 実際に転送されたアイテム数

**要件:**
- 両方のインベントリはワイヤードモデムとネットワークケーブルで接続されている必要があります
- 宛先インベントリが存在し、アクセス可能である必要があります

**例:**
```lua
local source = peripheral.find("minecraft:chest_0")
local dest_name = "minecraft:chest_1"

-- スロット 1 から 32 個のダイヤモンドを転送
local moved = source.async_push_items(dest_name, 1, 32)
print("移動したアイテム数: " .. moved)
```

**エラーハンドリング:**
- 宛先インベントリが存在しない場合、エラーをスロー
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
- `fromName: string` — ソースインベントリの名前（`peripheral.getNamesRemote()`から）
- `fromSlot: number` — ソーススロットインデックス（1ベース）
- `limit?: number` — 転送する最大アイテム数（オプション、デフォルトはスタック制限）
- `toSlot?: number` — 宛先スロットインデックス（オプション、省略時は自動選択）

**戻り値:** `number` — 実際に転送されたアイテム数

**要件:**
- 両方のインベントリはワイヤードモデムとネットワークケーブルで接続されている必要があります
- ソースインベントリが存在し、アクセス可能である必要があります

**例:**
```lua
local dest = peripheral.find("minecraft:chest_0")
local source_name = "minecraft:chest_1"

-- ソーススロット 1 から 32 個のダイヤモンドを引き出す
local moved = dest.async_pull_items(source_name, 1, 32)
print("引き出したアイテム数: " .. moved)
```

**エラーハンドリング:**
- ソースインベントリが存在しない場合、エラーをスロー
- スロットが範囲外の場合、エラーをスロー

---

## イベント

Inventoryペリフェラルはイベントを生成しません。

---

## 使用例

### 例1: すべてのアイテムをリストアップ

```lua
local inventory = peripheral.find("minecraft:chest")

local items = inventory.async_list()
for slot, item in pairs(items) do
  print(("スロット %d: %d x %s"):format(slot, item.count, item.name))
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
  print(("スロット %d にダイヤモンド %d 個を発見"):format(slot, item.count))
end
```

### 例3: チェスト間でアイテムを転送

```lua
local source = peripheral.find("minecraft:chest_0")
local dest_name = "minecraft:chest_1"

-- ソースから宛先にすべてのアイテムを転送
local items = source.async_list()
for slot, item in pairs(items) do
  local moved = source.async_push_items(dest_name, slot)
  print(("スロット %d から %d 個のアイテムを移動"):format(slot, moved))
end
```

### 例4: 総容量を計算

```lua
local inventory = peripheral.find("minecraft:chest")

local total_capacity = 0
for i = 1, inventory.async_size() do
  total_capacity = total_capacity + inventory.async_get_item_limit(i)
end

print("総容量: " .. total_capacity)
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
  print(("スロット %d: %s"):format(entry.slot, entry.item.name))
end
```

---

## エラーハンドリング

すべてのメソッドは以下の場合にエラーをスローする可能性があります：

- **インベントリが見つからない**: ペリフェラルが切断されているか、アクセスできない
- **無効なスロット**: スロット番号が範囲外（1 から size）
- **ネットワークエラー**: ワイヤードネットワーク接続が切断されている
- **宛先が見つからない**: ターゲットインベントリが存在しないか、アクセスできない

**エラーハンドリングの例:**
```lua
local inventory = peripheral.find("minecraft:chest")
if not inventory then
  error("インベントリが見つかりません")
end

local success, result = pcall(function()
  return inventory.async_list()
end)

if not success then
  print("エラー: " .. result)
else
  print("取得したアイテム数: " .. #result)
end
```

---

## 型定義

### ItemDetail
```lua
{
  name: string,           -- レジストリ名（例："minecraft:diamond"）
  count: number,          -- スタックサイズ
  displayName: string,    -- 表示名
  maxCount: number,       -- 最大スタックサイズ
  damage?: number,        -- 耐久度ダメージ（オプション）
  maxDamage?: number,     -- 最大耐久度（オプション）
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

- すべてのスロットインデックスは1ベース（最初のスロットは0ではなく1）
- `pushItems` と `pullItems` はワイヤードネットワーク接続が必要です
- インベントリ名は `peripheral.getNamesRemote()` でワイヤードモデムから取得します
- 空のスロットは `list()` の結果に含まれません（スパーステーブル）
- 3つの関数パターンは効率的なバッチ操作を可能にします

---

## 関連

- [Modem](./Modem.md) — ワイヤードネットワーク通信に必要
- [CC:Tweaked ドキュメント](https://tweaked.cc/) — 公式ドキュメント
