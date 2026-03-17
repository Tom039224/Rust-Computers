# PlayerDetector

**モジュール:** AdvancedPeripherals  
**ペリフェラルタイプ:** `advancedPeripherals:player_detector`  
**ソース:** `PlayerDetectorPeripheral.java`

## 概要

PlayerDetectorペリフェラルは、指定された半径内のプレイヤーを検出し、その位置、名前、ゲームモード、ヘルス、飢餓レベルなどの詳細情報を提供します。これはセキュリティシステム、プレイヤートラッキング、インタラクティブシステムの作成に役立ちます。

## 3つの関数パターン

PlayerDetector APIは、すべてのメソッドに対して3つの関数パターンを使用します：

1. **`book_next_*`** - 次のティックのリクエストをスケジュール
2. **`read_last_*`** - 前のティックの結果を読み取り
3. **`async_*`** - 便利なメソッド（book、待機、読み取りを1つの呼び出しで実行）

### パターンの説明

```lua
-- 方法1: book_next / read_last パターン
detector.book_next_get_players()
wait_for_next_tick()
local players = detector.read_last_get_players()

-- 方法2: async パターン（推奨）
local players = detector.async_get_players()
```

## メソッド

### `getPlayers()` / `book_next_get_players()` / `read_last_get_players()` / `async_get_players()`

指定された半径内のすべてのプレイヤーを取得します。

**Lua署名:**
```lua
function getPlayers() -> table
```

**Rust署名:**
```rust
pub fn book_next_get_players(&mut self)
pub fn read_last_get_players(&self) -> Result<Vec<PlayerEntry>, PeripheralError>
pub async fn async_get_players(&self) -> Result<Vec<PlayerEntry>, PeripheralError>
```

**戻り値:** `table` — プレイヤーエントリの配列

**プレイヤーエントリ構造:**
```lua
{
  name: string,           -- プレイヤー名
  x: number,              -- X座標
  y: number,              -- Y座標
  z: number,              -- Z座標
  gamemode: string,       -- ゲームモード（"survival", "creative", "adventure", "spectator"）
  health: number,         -- ヘルスポイント（0-20）
  hunger: number,         -- 飢餓レベル（0-20）
  saturation: number,     -- 飽和度
  dimension: string,      -- ディメンション名
}
```

**例:**
```lua
local detector = peripheral.find("advancedPeripherals:player_detector")
local players = detector.async_get_players()

print("検出されたプレイヤー数: " .. #players)
for _, player in ipairs(players) do
  print(("プレイヤー: %s、位置: (%.1f, %.1f, %.1f)"):format(player.name, player.x, player.y, player.z))
end
```

---

### `getPlayerPos(name)` / `book_next_get_player_pos(name)` / `read_last_get_player_pos()` / `async_get_player_pos(name)`

特定のプレイヤーの位置を取得します。

**Lua署名:**
```lua
function getPlayerPos(name: string) -> table | nil
```

**Rust署名:**
```rust
pub fn book_next_get_player_pos(&mut self, name: &str)
pub fn read_last_get_player_pos(&self) -> Result<PlayerPos, PeripheralError>
pub async fn async_get_player_pos(&self, name: &str) -> Result<PlayerPos, PeripheralError>
```

**パラメータ:**
- `name: string` — プレイヤー名

**戻り値:** `table | nil` — プレイヤー位置またはプレイヤーが見つからない場合は`nil`

**位置構造:**
```lua
{
  x: number,              -- X座標
  y: number,              -- Y座標
  z: number,              -- Z座標
  dimension: string,      -- ディメンション名
}
```

**例:**
```lua
local detector = peripheral.find("advancedPeripherals:player_detector")
local pos = detector.async_get_player_pos("Steve")

if pos then
  print(("Steveの位置: (%.1f, %.1f, %.1f)"):format(pos.x, pos.y, pos.z))
else
  print("Steveが見つかりません")
end
```

---

### `getPlayerData(name)` / `book_next_get_player_data(name)` / `read_last_get_player_data()` / `async_get_player_data(name)`

特定のプレイヤーの詳細データを取得します。

**Lua署名:**
```lua
function getPlayerData(name: string) -> table | nil
```

**Rust署名:**
```rust
pub fn book_next_get_player_data(&mut self, name: &str)
pub fn read_last_get_player_data(&self) -> Result<PlayerData, PeripheralError>
pub async fn async_get_player_data(&self, name: &str) -> Result<PlayerData, PeripheralError>
```

**パラメータ:**
- `name: string` — プレイヤー名

**戻り値:** `table | nil` — プレイヤーデータまたはプレイヤーが見つからない場合は`nil`

**プレイヤーデータ構造:**
```lua
{
  name: string,           -- プレイヤー名
  x: number,              -- X座標
  y: number,              -- Y座標
  z: number,              -- Z座標
  gamemode: string,       -- ゲームモード
  health: number,         -- ヘルスポイント
  hunger: number,         -- 飢餓レベル
  saturation: number,     -- 飽和度
  dimension: string,      -- ディメンション名
  uuid: string,           -- プレイヤーUUID
  level: number,          -- 経験値レベル
  experience: number,     -- 経験値
}
```

**例:**
```lua
local detector = peripheral.find("advancedPeripherals:player_detector")
local data = detector.async_get_player_data("Steve")

if data then
  print(("プレイヤー: %s"):format(data.name))
  print(("ヘルス: %.1f/20"):format(data.health))
  print(("飢餓: %.1f/20"):format(data.hunger))
  print(("レベル: %d"):format(data.level))
else
  print("プレイヤーが見つかりません")
end
```

---

### `isPlayerNear(name, radius)` / `book_next_is_player_near(name, radius)` / `read_last_is_player_near()` / `async_is_player_near(name, radius)`

プレイヤーが指定された半径内にいるかどうかを確認します。

**Lua署名:**
```lua
function isPlayerNear(name: string, radius: number) -> boolean
```

**Rust署名:**
```rust
pub fn book_next_is_player_near(&mut self, name: &str, radius: f64)
pub fn read_last_is_player_near(&self) -> Result<bool, PeripheralError>
pub async fn async_is_player_near(&self, name: &str, radius: f64) -> Result<bool, PeripheralError>
```

**パラメータ:**
- `name: string` — プレイヤー名
- `radius: number` — ブロック単位の半径

**戻り値:** `boolean` — プレイヤーが半径内にいる場合は`true`

**例:**
```lua
local detector = peripheral.find("advancedPeripherals:player_detector")
local near = detector.async_is_player_near("Steve", 32)

if near then
  print("Steveが近くにいます")
else
  print("Steveが遠くにいます")
end
```

---

### `getPlayersInRadius(radius)` / `book_next_get_players_in_radius(radius)` / `read_last_get_players_in_radius()` / `async_get_players_in_radius(radius)`

指定された半径内のすべてのプレイヤーを取得します。

**Lua署名:**
```lua
function getPlayersInRadius(radius: number) -> table
```

**Rust署名:**
```rust
pub fn book_next_get_players_in_radius(&mut self, radius: f64)
pub fn read_last_get_players_in_radius(&self) -> Result<Vec<PlayerEntry>, PeripheralError>
pub async fn async_get_players_in_radius(&self, radius: f64) -> Result<Vec<PlayerEntry>, PeripheralError>
```

**パラメータ:**
- `radius: number` — ブロック単位の半径

**戻り値:** `table` — プレイヤーエントリの配列

**例:**
```lua
local detector = peripheral.find("advancedPeripherals:player_detector")
local players = detector.async_get_players_in_radius(50)

print("50ブロック以内のプレイヤー数: " .. #players)
for _, player in ipairs(players) do
  print("  - " .. player.name)
end
```

---

### `getOnlinePlayers()` / `book_next_get_online_players()` / `read_last_get_online_players()` / `async_get_online_players()`

サーバー上のすべてのオンラインプレイヤーを取得します。

**Lua署名:**
```lua
function getOnlinePlayers() -> table
```

**Rust署名:**
```rust
pub fn book_next_get_online_players(&mut self)
pub fn read_last_get_online_players(&self) -> Result<Vec<String>, PeripheralError>
pub async fn async_get_online_players(&self) -> Result<Vec<String>, PeripheralError>
```

**戻り値:** `table` — プレイヤー名の配列

**例:**
```lua
local detector = peripheral.find("advancedPeripherals:player_detector")
local players = detector.async_get_online_players()

print("オンラインプレイヤー:")
for _, name in ipairs(players) do
  print("  - " .. name)
end
```

---

## イベント

PlayerDetectorペリフェラルは以下のイベントを生成します：

### `player_join`

プレイヤーがサーバーに参加したときに発生します。

**イベントデータ:**
```lua
{
  name: string,           -- プレイヤー名
  uuid: string,           -- プレイヤーUUID
}
```

**例:**
```lua
local detector = peripheral.find("advancedPeripherals:player_detector")

while true do
  local event, name, uuid = os.pullEvent("player_join")
  print("プレイヤーが参加しました: " .. name)
end
```

---

### `player_leave`

プレイヤーがサーバーから退出したときに発生します。

**イベントデータ:**
```lua
{
  name: string,           -- プレイヤー名
  uuid: string,           -- プレイヤーUUID
}
```

---

## 使用例

### 例1: プレイヤーの検出と追跡

```lua
local detector = peripheral.find("advancedPeripherals:player_detector")

while true do
  local players = detector.async_get_players()
  
  if #players > 0 then
    print("検出されたプレイヤー:")
    for _, player in ipairs(players) do
      local distance = math.sqrt(player.x^2 + player.y^2 + player.z^2)
      print(("  %s - 距離: %.1f ブロック"):format(player.name, distance))
    end
  else
    print("プレイヤーが検出されません")
  end
  
  sleep(1)
end
```

### 例2: プレイヤーのヘルス監視

```lua
local detector = peripheral.find("advancedPeripherals:player_detector")

local function check_player_health(name)
  local data = detector.async_get_player_data(name)
  
  if data then
    print(("プレイヤー: %s"):format(data.name))
    print(("ヘルス: %.1f/20"):format(data.health))
    
    if data.health < 5 then
      print("警告: プレイヤーのヘルスが低い！")
    end
  else
    print("プレイヤーが見つかりません")
  end
end

check_player_health("Steve")
```

### 例3: セキュリティシステム

```lua
local detector = peripheral.find("advancedPeripherals:player_detector")
local allowed_players = {"Steve", "Alex"}

while true do
  local players = detector.async_get_players()
  
  for _, player in ipairs(players) do
    local is_allowed = false
    for _, allowed in ipairs(allowed_players) do
      if player.name == allowed then
        is_allowed = true
        break
      end
    end
    
    if not is_allowed then
      print("警告: 許可されていないプレイヤーが検出されました: " .. player.name)
    end
  end
  
  sleep(1)
end
```

### 例4: プレイヤー参加/退出の監視

```lua
local detector = peripheral.find("advancedPeripherals:player_detector")

while true do
  local event, name, uuid = os.pullEvent()
  
  if event == "player_join" then
    print("プレイヤーが参加しました: " .. name)
  elseif event == "player_leave" then
    print("プレイヤーが退出しました: " .. name)
  end
end
```

### 例5: 近くのプレイヤーの検出

```lua
local detector = peripheral.find("advancedPeripherals:player_detector")

local function find_nearby_players(radius)
  local players = detector.async_get_players_in_radius(radius)
  
  if #players > 0 then
    print(radius .. " ブロック以内のプレイヤー:")
    for _, player in ipairs(players) do
      print(("  %s - ゲームモード: %s"):format(player.name, player.gamemode))
    end
  else
    print("近くにプレイヤーがいません")
  end
end

find_nearby_players(50)
```

---

## エラーハンドリング

すべてのメソッドは以下の場合にエラーをスローする可能性があります：

- **プレイヤーが見つからない**: 指定されたプレイヤーがオンラインではない
- **ペリフェラル切断**: PlayerDetectorにアクセスできなくなった
- **無効な半径**: 半径が負またはマキシマムを超えている

**エラーハンドリングの例:**
```lua
local detector = peripheral.find("advancedPeripherals:player_detector")
if not detector then
  error("PlayerDetectorが見つかりません")
end

local success, result = pcall(function()
  return detector.async_get_players()
end)

if not success then
  print("エラー: " .. result)
else
  print("検出されたプレイヤー数: " .. #result)
end
```

---

## 型定義

### PlayerEntry
```lua
{
  name: string,           -- プレイヤー名
  x: number,              -- X座標
  y: number,              -- Y座標
  z: number,              -- Z座標
  gamemode: string,       -- ゲームモード
  health: number,         -- ヘルスポイント
  hunger: number,         -- 飢餓レベル
  saturation: number,     -- 飽和度
  dimension: string,      -- ディメンション名
}
```

### PlayerPos
```lua
{
  x: number,              -- X座標
  y: number,              -- Y座標
  z: number,              -- Z座標
  dimension: string,      -- ディメンション名
}
```

### PlayerData
```lua
{
  name: string,           -- プレイヤー名
  x: number,              -- X座標
  y: number,              -- Y座標
  z: number,              -- Z座標
  gamemode: string,       -- ゲームモード
  health: number,         -- ヘルスポイント
  hunger: number,         -- 飢餓レベル
  saturation: number,     -- 飽和度
  dimension: string,      -- ディメンション名
  uuid: string,           -- プレイヤーUUID
  level: number,          -- 経験値レベル
  experience: number,     -- 経験値
}
```

---

## 注記

- すべての座標はワールド座標です
- ゲームモードは"survival"、"creative"、"adventure"、"spectator"のいずれかです
- ヘルスと飢餓は0-20の範囲です
- 3つの関数パターンは効率的なバッチ操作を可能にします
- イベントはペリフェラルが監視を開始した後に発生します

---

## 関連

- [GeoScanner](./GeoScanner.md) — ブロックスキャン用
- [BlockReader](./BlockReader.md) — ブロック情報読取用
- [AdvancedPeripherals ドキュメント](https://advancedperipherals.readthedocs.io/) — 公式ドキュメント
