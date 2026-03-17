# BlockReader

**モジュール:** AdvancedPeripherals  
**ペリフェラルタイプ:** `advancedPeripherals:block_reader`  
**ソース:** `BlockReaderPeripheral.java`

## 概要

BlockReaderペリフェラルは、ペリフェラルの正面にあるブロックの詳細情報を読み取ります。ブロックのレジストリ名、NBTデータ、ブロックステートプロパティ、およびタイルエンティティ（ブロックエンティティ）であるかどうかが含まれます。これは、特定のブロックを識別して反応する必要があるオートメーションシステムに役立ちます。

## 3つの関数パターン

BlockReader APIは、すべてのメソッドに対して3つの関数パターンを使用します：

1. **`book_next_*`** - 次のティックのリクエストをスケジュール
2. **`read_last_*`** - 前のティックの結果を読み取り
3. **`async_*`** - 便利なメソッド（book、待機、読み取りを1つの呼び出しで実行）

### パターンの説明

```lua
-- 方法1: book_next / read_last パターン
reader.book_next_get_block_name()
wait_for_next_tick()
local name = reader.read_last_get_block_name()

-- 方法2: async パターン（推奨）
local name = reader.async_get_block_name()
```

## メソッド

### `getBlockName()` / `book_next_get_block_name()` / `read_last_get_block_name()` / `async_get_block_name()`

ペリフェラルの正面にあるブロックのレジストリ名を取得します。

**Lua署名:**
```lua
function getBlockName() -> string
```

**Rust署名:**
```rust
pub fn book_next_get_block_name(&mut self)
pub fn read_last_get_block_name(&self) -> Result<String, PeripheralError>
pub async fn async_get_block_name(&self) -> Result<String, PeripheralError>
```

**戻り値:** `string` — ブロックレジストリ名（例：`"minecraft:stone"`、`"minecraft:chest"`）

**例:**
```lua
local reader = peripheral.find("advancedPeripherals:block_reader")
local name = reader.async_get_block_name()
print("ブロック: " .. name)
```

---

### `getBlockData()` / `book_next_get_block_data()` / `read_last_get_block_data()` / `async_get_block_data()`

ペリフェラルの正面にあるブロックのNBTデータを取得します。

**Lua署名:**
```lua
function getBlockData() -> table
```

**Rust署名:**
```rust
pub fn book_next_get_block_data(&mut self)
pub fn read_last_get_block_data(&self) -> Result<Value, PeripheralError>
pub async fn async_get_block_data(&self) -> Result<Value, PeripheralError>
```

**戻り値:** `table` — 動的テーブルとしてのNBTデータ

戻り値はブロックタイプによって異なります。例えば：
- チェスト: `{Items = {...}, id = "minecraft:chest"}`
- かまど: `{Items = {...}, BurnTime = 0, CookTime = 0, ...}`
- カスタムブロックは追加のプロパティを持つ場合があります

**例:**
```lua
local reader = peripheral.find("advancedPeripherals:block_reader")
local data = reader.async_get_block_data()

-- チェストにアイテムがあるかチェック
if data.Items then
  print("チェストに " .. #data.Items .. " 個のアイテムが見つかりました")
end
```

---

### `getBlockStates()` / `book_next_get_block_states()` / `read_last_get_block_states()` / `async_get_block_states()`

ペリフェラルの正面にあるブロックのブロックステートプロパティを取得します。

**Lua署名:**
```lua
function getBlockStates() -> table
```

**Rust署名:**
```rust
pub fn book_next_get_block_states(&mut self)
pub fn read_last_get_block_states(&self) -> Result<Value, PeripheralError>
pub async fn async_get_block_states(&self) -> Result<Value, PeripheralError>
```

**戻り値:** `table` — ブロックステートプロパティ

ブロックステートはブロックタイプによって異なります。一般的な例：
- ログ: `{axis = "y", natural = true}`
- 階段: `{facing = "north", half = "bottom", shape = "straight"}`
- レッドストーン: `{north = "up", south = "side", east = "none", west = "up", power = 15}`
- ドア: `{facing = "north", half = "lower", hinge = "left", open = false, powered = false}`

**例:**
```lua
local reader = peripheral.find("advancedPeripherals:block_reader")
local states = reader.async_get_block_states()

-- ドアが開いているかチェック
if states.open then
  print("ドアは開いています")
else
  print("ドアは閉じています")
end
```

---

### `isTileEntity()` / `book_next_is_tile_entity()` / `read_last_is_tile_entity()` / `async_is_tile_entity()`

ペリフェラルの正面にあるブロックがタイルエンティティ（ブロックエンティティ）であるかどうかを確認します。

**Lua署名:**
```lua
function isTileEntity() -> boolean
```

**Rust署名:**
```rust
pub fn book_next_is_tile_entity(&mut self)
pub fn read_last_is_tile_entity(&self) -> Result<bool, PeripheralError>
pub async fn async_is_tile_entity(&self) -> Result<bool, PeripheralError>
```

**戻り値:** `boolean` — ブロックがタイルエンティティの場合は`true`、そうでない場合は`false`

タイルエンティティには以下が含まれます：
- チェスト、樽、ホッパー
- かまど、溶鉱炉、燻製器
- 醸造スタンド、大釜
- エンチャントテーブル、金床
- ビーコン、コンジット
- タイルエンティティを持つカスタムmodブロック

**例:**
```lua
local reader = peripheral.find("advancedPeripherals:block_reader")
local is_te = reader.async_is_tile_entity()

if is_te then
  print("ブロックはタイルエンティティデータを持っています")
  local data = reader.async_get_block_data()
  print("NBT: " .. textutils.serialize(data))
else
  print("ブロックはシンプルなブロックです")
end
```

---

## イベント

BlockReaderペリフェラルはイベントを生成しません。

---

## 使用例

### 例1: ブロックタイプの識別

```lua
local reader = peripheral.find("advancedPeripherals:block_reader")

local name = reader.async_get_block_name()
local states = reader.async_get_block_states()

print("ブロック: " .. name)
print("ステート: " .. textutils.serialize(states))
```

### 例2: チェストの内容をチェック

```lua
local reader = peripheral.find("advancedPeripherals:block_reader")

local name = reader.async_get_block_name()
if name == "minecraft:chest" then
  local data = reader.async_get_block_data()
  if data.Items then
    print("チェストに " .. #data.Items .. " スタック含まれています")
    for i, item in ipairs(data.Items) do
      print(("  スロット %d: %d x %s"):format(item.Slot, item.Count, item.id))
    end
  else
    print("チェストは空です")
  end
else
  print("チェストではありません: " .. name)
end
```

### 例3: ブロックステート変化の監視

```lua
local reader = peripheral.find("advancedPeripherals:block_reader")

local last_state = nil

while true do
  local states = reader.async_get_block_states()
  
  if last_state and last_state.open ~= states.open then
    if states.open then
      print("ドアが開きました！")
    else
      print("ドアが閉じました！")
    end
  end
  
  last_state = states
  sleep(0.5)
end
```

### 例4: レッドストーン信号の検出

```lua
local reader = peripheral.find("advancedPeripherals:block_reader")

local name = reader.async_get_block_name()
if name == "minecraft:redstone_wire" then
  local states = reader.async_get_block_states()
  local power = tonumber(states.power) or 0
  print("レッドストーン電力レベル: " .. power)
end
```

### 例5: インベントリ監視

```lua
local reader = peripheral.find("advancedPeripherals:block_reader")

local function get_inventory_size()
  local data = reader.async_get_block_data()
  if data.Items then
    local max_slot = 0
    for _, item in ipairs(data.Items) do
      if item.Slot > max_slot then
        max_slot = item.Slot
      end
    end
    return max_slot
  end
  return 0
end

local size = get_inventory_size()
print("インベントリは " .. size .. " スロットを持っています")
```

---

## エラーハンドリング

すべてのメソッドは以下の場合にエラーをスローする可能性があります：

- **正面にブロックがない**: ペリフェラルがブロックに面していない（例：空気に面している）
- **ペリフェラル切断**: ペリフェラルにアクセスできなくなった
- **無効なステート**: ブロックステートを読み取ることができない

**エラーハンドリングの例:**
```lua
local reader = peripheral.find("advancedPeripherals:block_reader")
if not reader then
  error("BlockReaderが見つかりません")
end

local success, result = pcall(function()
  return reader.async_get_block_name()
end)

if not success then
  print("エラー: " .. result)
else
  print("ブロック: " .. result)
end
```

---

## 型定義

### BlockState
```lua
{
  [key: string]: string | number | boolean,
  -- 例:
  -- facing = "north"
  -- half = "bottom"
  -- open = false
  -- power = 15
}
```

### NBTData
```lua
{
  -- ブロックタイプによって異なります
  -- 一般的なプロパティ:
  id?: string,           -- ブロックID
  Items?: table,         -- インベントリアイテム（該当する場合）
  BurnTime?: number,     -- かまど燃焼時間（該当する場合）
  CookTime?: number,     -- かまど調理時間（該当する場合）
  -- ... その他のブロック固有プロパティ
}
```

---

## 注記

- BlockReaderはそれが向いている方向の正面のブロックを読み取ります
- ブロックステートは常に文字列または数値です（ブール値のような値でも）
- NBTデータ構造はブロックタイプに依存します
- タイルエンティティはシンプルなブロックを超えた追加のNBTデータを持ちます
- 3つの関数パターンは効率的なバッチ操作を可能にします

---

## 関連

- [GeoScanner](./GeoScanner.md) — 領域内の複数ブロックをスキャン
- [AdvancedPeripherals ドキュメント](https://advancedperipherals.readthedocs.io/) — 公式ドキュメント
