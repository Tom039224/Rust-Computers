# GeoScanner

**モジュール:** AdvancedPeripherals  
**ペリフェラルタイプ:** `advancedPeripherals:geo_scanner`  
**ソース:** `GeoScannerPeripheral.java`

## 概要

GeoScannerペリフェラルは、指定された半径内の周囲のブロックをスキャンし、現在のチャンク内の鉱石分布を分析します。ブロックタイプ、その位置、タグに関する詳細情報を提供します。これはマイニングオートメーション、リソース位置特定、地形分析システムに役立ちます。

## 3つの関数パターン

GeoScanner APIは、すべてのメソッドに対して3つの関数パターンを使用します：

1. **`book_next_*`** - 次のティックのリクエストをスケジュール
2. **`read_last_*`** - 前のティックの結果を読み取り
3. **`async_*`** - 便利なメソッド（book、待機、読み取りを1つの呼び出しで実行）

### パターンの説明

```lua
-- 方法1: book_next / read_last パターン
scanner.book_next_scan(16)
wait_for_next_tick()
local blocks = scanner.read_last_scan()

-- 方法2: async パターン（推奨）
local blocks = scanner.async_scan(16)
```

## メソッド

### `cost(radius)` / `book_next_cost(radius)` / `read_last_cost()` / `cost_imm(radius)`

指定された半径でのスキャンの燃料コストを取得します。

**Lua署名:**
```lua
function cost(radius: number) -> number
```

**Rust署名:**
```rust
pub fn book_next_cost(&mut self, radius: f64)
pub fn read_last_cost(&self) -> Result<f64, PeripheralError>
pub fn cost_imm(&self, radius: f64) -> Result<f64, PeripheralError>
```

**パラメータ:**
- `radius: number` — ブロック単位のスキャン半径

**戻り値:** `number` — 燃料コスト（AEユニットまたは同様）

**注記:** `cost_imm`メソッドは次のティックを待たずに即座に結果を返します。

**例:**
```lua
local scanner = peripheral.find("advancedPeripherals:geo_scanner")

-- スキャン前にコストをチェック
local cost = scanner.cost_imm(16)
print("スキャンコスト: " .. cost)

if cost < 100 then
  local blocks = scanner.async_scan(16)
end
```

---

### `scan(radius)` / `book_next_scan(radius)` / `read_last_scan()` / `async_scan(radius)`

指定された半径内のブロックをスキャンします。

**Lua署名:**
```lua
function scan(radius: number) -> table
```

**Rust署名:**
```rust
pub fn book_next_scan(&mut self, radius: f64)
pub fn read_last_scan(&self) -> Result<Vec<GeoBlockEntry>, PeripheralError>
pub async fn async_scan(&self, radius: f64) -> Result<Vec<GeoBlockEntry>, PeripheralError>
```

**パラメータ:**
- `radius: number` — ブロック単位のスキャン半径

**戻り値:** `table` — ブロックエントリの配列

**ブロックエントリ構造:**
```lua
{
  x: number,           -- X座標（スキャナーからの相対位置）
  y: number,           -- Y座標（スキャナーからの相対位置）
  z: number,           -- Z座標（スキャナーからの相対位置）
  name: string,        -- ブロックレジストリ名（例："minecraft:diamond_ore"）
  tags: table,         -- ブロックタグ（例：{"minecraft:ores", "minecraft:mineable/pickaxe"}）
}
```

**例:**
```lua
local scanner = peripheral.find("advancedPeripherals:geo_scanner")

local blocks = scanner.async_scan(16)
print("見つかったブロック数: " .. #blocks)

for _, block in ipairs(blocks) do
  print(("ブロック位置 (%d, %d, %d): %s"):format(block.x, block.y, block.z, block.name))
end
```

---

### `chunkAnalyze()` / `book_next_chunk_analyze()` / `read_last_chunk_analyze()` / `async_chunk_analyze()`

現在のチャンク内の鉱石分布を分析します。

**Lua署名:**
```lua
function chunkAnalyze() -> table
```

**Rust署名:**
```rust
pub fn book_next_chunk_analyze(&mut self)
pub fn read_last_chunk_analyze(&self) -> Result<Value, PeripheralError>
pub async fn async_chunk_analyze(&self) -> Result<Value, PeripheralError>
```

**戻り値:** `table` — 鉱石名から数へのマップ

**戻り値構造:**
```lua
{
  ["minecraft:diamond_ore"] = 5,
  ["minecraft:iron_ore"] = 12,
  ["minecraft:gold_ore"] = 3,
  ["minecraft:coal_ore"] = 8,
  -- ... その他の鉱石
}
```

**例:**
```lua
local scanner = peripheral.find("advancedPeripherals:geo_scanner")

local ores = scanner.async_chunk_analyze()
print("チャンク内の鉱石分布:")

for ore_name, count in pairs(ores) do
  print(("  %s: %d"):format(ore_name, count))
end
```

---

## イベント

GeoScannerペリフェラルはイベントを生成しません。

---

## 使用例

### 例1: 特定の鉱石を検索

```lua
local scanner = peripheral.find("advancedPeripherals:geo_scanner")

local function find_ore(ore_name, radius)
  local blocks = scanner.async_scan(radius)
  
  for _, block in ipairs(blocks) do
    if block.name == ore_name then
      return block
    end
  end
  
  return nil
end

local diamond = find_ore("minecraft:diamond_ore", 32)
if diamond then
  print(("ダイヤモンド鉱石を見つけました位置 (%d, %d, %d)"):format(diamond.x, diamond.y, diamond.z))
else
  print("ダイヤモンド鉱石が見つかりません")
end
```

### 例2: タグでスキャンしてフィルター

```lua
local scanner = peripheral.find("advancedPeripherals:geo_scanner")

local function find_ores(radius)
  local blocks = scanner.async_scan(radius)
  local ores = {}
  
  for _, block in ipairs(blocks) do
    -- ブロックが"minecraft:ores"タグを持つかチェック
    for _, tag in ipairs(block.tags) do
      if tag == "minecraft:ores" then
        table.insert(ores, block)
        break
      end
    end
  end
  
  return ores
end

local ores = find_ores(16)
print("見つかった鉱石ブロック数: " .. #ores)
```

### 例3: チャンク分析レポート

```lua
local scanner = peripheral.find("advancedPeripherals:geo_scanner")

local function analyze_chunk()
  local ores = scanner.async_chunk_analyze()
  
  local total = 0
  for _, count in pairs(ores) do
    total = total + count
  end
  
  print("チャンク分析レポート:")
  print("総鉱石ブロック数: " .. total)
  print("")
  
  -- 数でソート
  local sorted = {}
  for ore_name, count in pairs(ores) do
    table.insert(sorted, {name = ore_name, count = count})
  end
  
  table.sort(sorted, function(a, b)
    return a.count > b.count
  end)
  
  for _, entry in ipairs(sorted) do
    print(("  %s: %d"):format(entry.name, entry.count))
  end
end

analyze_chunk()
```

### 例4: コスト認識スキャン

```lua
local scanner = peripheral.find("advancedPeripherals:geo_scanner")

local function smart_scan(max_cost)
  -- 余裕のある最大半径を見つける
  local radius = 1
  while true do
    local cost = scanner.cost_imm(radius + 1)
    if cost > max_cost then
      break
    end
    radius = radius + 1
  end
  
  print("スキャン半径: " .. radius)
  local blocks = scanner.async_scan(radius)
  
  return blocks
end

local blocks = smart_scan(500)
print("見つかったブロック数: " .. #blocks)
```

### 例5: マイニングオートメーション

```lua
local scanner = peripheral.find("advancedPeripherals:geo_scanner")
local robot = peripheral.find("robot")  -- 仮想ロボットペリフェラル

local function mine_nearest_ore(ore_name, radius)
  local blocks = scanner.async_scan(radius)
  
  -- 最も近い鉱石を見つける
  local nearest = nil
  local min_distance = math.huge
  
  for _, block in ipairs(blocks) do
    if block.name == ore_name then
      local distance = math.sqrt(block.x^2 + block.y^2 + block.z^2)
      if distance < min_distance then
        min_distance = distance
        nearest = block
      end
    end
  end
  
  if nearest then
    print(("マイニング中 %s 位置 (%d, %d, %d)"):format(ore_name, nearest.x, nearest.y, nearest.z))
    -- ロボットを鉱石に移動してマイニング
    robot.move_to(nearest.x, nearest.y, nearest.z)
    robot.mine()
  else
    print("鉱石が見つかりません")
  end
end

mine_nearest_ore("minecraft:diamond_ore", 32)
```

---

## エラーハンドリング

すべてのメソッドは以下の場合にエラーをスローする可能性があります：

- **燃料不足**: スキャンを実行するのに十分なエネルギーがない
- **ペリフェラル切断**: GeoScannerにアクセスできなくなった
- **無効な半径**: 半径が負またはマキシマムを超えている

**エラーハンドリングの例:**
```lua
local scanner = peripheral.find("advancedPeripherals:geo_scanner")
if not scanner then
  error("GeoScannerが見つかりません")
end

local success, result = pcall(function()
  return scanner.async_scan(16)
end)

if not success then
  print("エラー: " .. result)
else
  print("見つかったブロック数: " .. #result)
end
```

---

## 型定義

### GeoBlockEntry
```lua
{
  x: number,           -- X座標（スキャナーからの相対位置）
  y: number,           -- Y座標（スキャナーからの相対位置）
  z: number,           -- Z座標（スキャナーからの相対位置）
  name: string,        -- ブロックレジストリ名
  tags: table,         -- ブロックタグの配列
}
```

### ChunkAnalysisResult
```lua
{
  [ore_name: string]: number,  -- 鉱石名から数へのマッピング
  -- 例:
  -- ["minecraft:diamond_ore"] = 5
  -- ["minecraft:iron_ore"] = 12
}
```

---

## 注記

- `scan()`で返される座標はスキャナーの位置からの相対位置です
- `cost_imm()`メソッドは唯一のイミディエイトメソッド（待機なしで返す）です
- チャンク分析はスキャン半径ではなくチャンク全体をスキャンします
- ブロックタグは結果をフィルターするために使用できます（例："minecraft:ores"、"minecraft:mineable/pickaxe"）
- 3つの関数パターンは効率的なバッチ操作を可能にします
- より大きなスキャン半径はより多くの燃料/エネルギーを消費します

---

## 関連

- [BlockReader](./BlockReader.md) — 詳細なブロック情報読取用
- [PlayerDetector](./PlayerDetector.md) — プレイヤー検出用
- [AdvancedPeripherals ドキュメント](https://advancedperipherals.readthedocs.io/) — 公式ドキュメント
