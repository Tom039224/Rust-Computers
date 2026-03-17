# Inventory

**mod**: CC:Tweaked  
**peripheral type**: `inventory` (additional type)  
**source**: `AbstractInventoryMethods.java`

## 概要

GenericPeripheralとして実装されており、任意のインベントリブロック（チェスト、バレル等）に動的に付与されるAPI。有線ネットワーク上のインベントリ間でアイテムを直接移動できる。

## メソッド一覧

| メソッド名 | 引数 | 戻り値 | 説明 |
|---|---|---|---|
| `size` | — | `number` | インベントリのスロット数を返す |
| `list` | — | `table` | 全スロットのアイテム情報テーブルを返す（空スロットは `nil`、スパーステーブル） |
| `getItemDetail` | `slot: number` | `table \| nil` | 指定スロットのアイテム詳細情報テーブルを返す。空スロットなら `nil` |
| `getItemLimit` | `slot: number` | `number` | 指定スロットの最大収納数を返す（通常64） |
| `pushItems` | `toName: string, fromSlot: number, limit?: number, toSlot?: number` | `number` | 有線ネットワーク上の別インベントリにアイテムを移動し、実際の移動数を返す |
| `pullItems` | `fromName: string, fromSlot: number, limit?: number, toSlot?: number` | `number` | 有線ネットワーク上の別インベントリからアイテムを引き出し、実際の移動数を返す |

## メソッド詳細

### size()

インベントリのスロット数を取得する。

**引数:**
- なし

**戻り値:**
- `number` - スロット数

**例:**
```lua
local chest = peripheral.find("minecraft:chest")
print("Chest has " .. chest.size() .. " slots")
```

---

### list()

インベントリ内の全アイテムをリストアップする。スパーステーブルを返し、空スロットは `nil` となる。

各アイテムは以下の情報を含むテーブルで表される：
- `name` - アイテムの登録名（例: `"minecraft:diamond"`）
- `count` - 個数
- `nbt` - NBTデータのハッシュ（オプション、同一アイテムの区別に使用）

**引数:**
- なし

**戻り値:**
- `table` - スロット番号をキーとするスパーステーブル

**例:**
```lua
local chest = peripheral.find("minecraft:chest")
for slot, item in pairs(chest.list()) do
  print(("%d x %s in slot %d"):format(item.count, item.name, slot))
end
```

---

### getItemDetail(slot)

指定スロットのアイテムの詳細情報を取得する。

**引数:**
- `slot: number` - スロット番号（1から始まる）

**戻り値:**
- `table | nil` - アイテム詳細情報テーブル、または空スロットの場合は `nil`

**アイテム詳細テーブル構造:**
```lua
{
  name = "minecraft:diamond",      -- アイテムの登録名
  count = 5,                        -- 個数
  displayName = "Diamond",          -- 表示名
  maxCount = 64,                    -- スタック上限
  damage = 10,                      -- 耐久値（ツール等、オプション）
  maxDamage = 100,                  -- 最大耐久値（オプション）
  tags = {                          -- 適用中のタグ
    ["minecraft:gems"] = true,
  },
  -- modによって追加フィールドあり
}
```

**例:**
```lua
local chest = peripheral.find("minecraft:chest")
local item = chest.getItemDetail(1)
if not item then 
  print("No item") 
  return 
end

print(("%s (%s)"):format(item.displayName, item.name))
print(("Count: %d/%d"):format(item.count, item.maxCount))

if item.damage then
  print(("Damage: %d/%d"):format(item.damage, item.maxDamage))
end
```

**エラー:**
- スロット番号が範囲外の場合、LuaExceptionをスロー

---

### getItemLimit(slot)

指定スロットに収納できる最大アイテム数を取得する。

通常は64だが、バレルやキャッシュなどの特殊なインベントリでは数百〜数千になることがある。

**引数:**
- `slot: number` - スロット番号（1から始まる）

**戻り値:**
- `number` - 最大収納数

**例:**
```lua
local chest = peripheral.find("minecraft:chest")
local total = 0
for i = 1, chest.size() do
  total = total + chest.getItemLimit(i)
end
print("Total capacity: " .. total)
```

**エラー:**
- スロット番号が範囲外の場合、LuaExceptionをスロー

---

### pushItems(toName, fromSlot, limit?, toSlot?)

有線ネットワーク上の別インベントリにアイテムを移動する。

両方のインベントリが有線モデムに接続され、ケーブルで接続されている必要がある。

**引数:**
- `toName: string` - 移動先ペリフェラル名（`peripheral.wrap` で使用する名前）
- `fromSlot: number` - 移動元スロット番号
- `limit?: number` - 移動する最大個数（省略時は現在のスタック上限）
- `toSlot?: number` - 移動先スロット番号（省略時は空きスロットに挿入）

**戻り値:**
- `number` - 実際に移動したアイテム数

**例:**
```lua
local chest_a = peripheral.wrap("minecraft:chest_0")
local chest_b = peripheral.wrap("minecraft:chest_1")

-- chest_aのスロット1からchest_bへ32個移動
local moved = chest_a.pushItems(peripheral.getName(chest_b), 1, 32)
print("Moved " .. moved .. " items")
```

**エラー:**
- 移動先ペリフェラルが存在しないか、インベントリでない場合
- スロット番号が範囲外の場合

---

### pullItems(fromName, fromSlot, limit?, toSlot?)

有線ネットワーク上の別インベントリからアイテムを引き出す。

両方のインベントリが有線モデムに接続され、ケーブルで接続されている必要がある。

**引数:**
- `fromName: string` - 移動元ペリフェラル名（`peripheral.wrap` で使用する名前）
- `fromSlot: number` - 移動元スロット番号
- `limit?: number` - 移動する最大個数（省略時は現在のスタック上限）
- `toSlot?: number` - 移動先スロット番号（省略時は空きスロットに挿入）

**戻り値:**
- `number` - 実際に移動したアイテム数

**例:**
```lua
local chest_a = peripheral.wrap("minecraft:chest_0")
local chest_b = peripheral.wrap("minecraft:chest_1")

-- chest_bのスロット1からchest_aへ引き出す
local moved = chest_a.pullItems(peripheral.getName(chest_b), 1)
print("Pulled " .. moved .. " items")
```

**エラー:**
- 移動元ペリフェラルが存在しないか、インベントリでない場合
- スロット番号が範囲外の場合

---

## イベント

Inventoryペリフェラル自体はイベントを発生させない。

---

## 備考

- `pushItems` / `pullItems` は有線ネットワーク（WiredModem経由）でのみ使用可能
- `toName` / `fromName` は `peripheral.getNamesRemote()` で取得できる名前を使用する
- すべてのメソッドは `mainThread = true` フラグが設定されており、メインスレッドで実行される

---

## 使用例

### 例1: チェスト内のアイテムを検索

```lua
local chest = peripheral.find("minecraft:chest")

-- ダイヤモンドを探す
for slot, item in pairs(chest.list()) do
  if item.name == "minecraft:diamond" then
    print("Found " .. item.count .. " diamonds in slot " .. slot)
  end
end
```

### 例2: アイテムの自動整理

```lua
local source = peripheral.wrap("minecraft:chest_0")
local target = peripheral.wrap("minecraft:chest_1")

-- sourceからtargetへ全アイテムを移動
for slot, item in pairs(source.list()) do
  local moved = source.pushItems(peripheral.getName(target), slot)
  print("Moved " .. moved .. " items from slot " .. slot)
end
```

### 例3: インベントリの容量計算

```lua
local chest = peripheral.find("minecraft:chest")

local totalCapacity = 0
local usedSlots = 0

for i = 1, chest.size() do
  totalCapacity = totalCapacity + chest.getItemLimit(i)
  if chest.getItemDetail(i) then
    usedSlots = usedSlots + 1
  end
end

print("Total capacity: " .. totalCapacity)
print("Used slots: " .. usedSlots .. "/" .. chest.size())
```

---

## 関連項目

- [Modem](./Modem.md) - 有線ネットワーク接続に必要
- [CC:Tweaked Documentation](https://tweaked.cc/) - 公式ドキュメント
