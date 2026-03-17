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

```rust
// Rust example to be added
```
## 実装状況

### ✅ 実装済み

- All book_next_* / read_last_* methods

### 🚧 未実装

- async_* variants for all methods
- playerJoin / playerLeave events


## メソッド

### `getPlayers()` / `book_next_get_players()` / `read_last_get_players()` / `async_get_players()`

指定された半径内のすべてのプレイヤーを取得します。

**Lua署名:**
```rust
// Rust example to be added
```
**Rust署名:**
```rust
pub fn book_next_get_players(&mut self)
pub fn read_last_get_players(&self) -> Result<Vec<PlayerEntry>, PeripheralError>
pub async fn async_get_players(&self) -> Result<Vec<PlayerEntry>, PeripheralError>
```

**戻り値:** `table` — プレイヤーエントリの配列

**プレイヤーエントリ構造:**
```rust
// Rust example to be added
```
**例:**
```rust
// Rust example to be added
```
---

### `getPlayerPos(name)` / `book_next_get_player_pos(name)` / `read_last_get_player_pos()` / `async_get_player_pos(name)`

特定のプレイヤーの位置を取得します。

**Lua署名:**
```rust
// Rust example to be added
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
```rust
// Rust example to be added
```
**例:**
```rust
// Rust example to be added
```
---

### `getPlayerData(name)` / `book_next_get_player_data(name)` / `read_last_get_player_data()` / `async_get_player_data(name)`

特定のプレイヤーの詳細データを取得します。

**Lua署名:**
```rust
// Rust example to be added
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
```rust
// Rust example to be added
```
**例:**
```rust
// Rust example to be added
```
---

### `isPlayerNear(name, radius)` / `book_next_is_player_near(name, radius)` / `read_last_is_player_near()` / `async_is_player_near(name, radius)`

プレイヤーが指定された半径内にいるかどうかを確認します。

**Lua署名:**
```rust
// Rust example to be added
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
```rust
// Rust example to be added
```
---

### `getPlayersInRadius(radius)` / `book_next_get_players_in_radius(radius)` / `read_last_get_players_in_radius()` / `async_get_players_in_radius(radius)`

指定された半径内のすべてのプレイヤーを取得します。

**Lua署名:**
```rust
// Rust example to be added
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
```rust
// Rust example to be added
```
---

### `getOnlinePlayers()` / `book_next_get_online_players()` / `read_last_get_online_players()` / `async_get_online_players()`

サーバー上のすべてのオンラインプレイヤーを取得します。

**Lua署名:**
```rust
// Rust example to be added
```
**Rust署名:**
```rust
pub fn book_next_get_online_players(&mut self)
pub fn read_last_get_online_players(&self) -> Result<Vec<String>, PeripheralError>
pub async fn async_get_online_players(&self) -> Result<Vec<String>, PeripheralError>
```

**戻り値:** `table` — プレイヤー名の配列

**例:**
```rust
// Rust example to be added
```
---

## イベント

PlayerDetectorペリフェラルは以下のイベントを生成します：

### `player_join`

プレイヤーがサーバーに参加したときに発生します。

**イベントデータ:**
```rust
// Rust example to be added
```
**例:**
```rust
// Rust example to be added
```
---

### `player_leave`

プレイヤーがサーバーから退出したときに発生します。

**イベントデータ:**
```rust
// Rust example to be added
```
---

## 使用例

### 例1: プレイヤーの検出と追跡

```rust
// Rust example to be added
```
### 例2: プレイヤーのヘルス監視

```rust
// Rust example to be added
```
### 例3: セキュリティシステム

```rust
// Rust example to be added
```
### 例4: プレイヤー参加/退出の監視

```rust
// Rust example to be added
```
### 例5: 近くのプレイヤーの検出

```rust
// Rust example to be added
```
---

## エラーハンドリング

すべてのメソッドは以下の場合にエラーをスローする可能性があります：

- **プレイヤーが見つからない**: 指定されたプレイヤーがオンラインではない
- **ペリフェラル切断**: PlayerDetectorにアクセスできなくなった
- **無効な半径**: 半径が負またはマキシマムを超えている

**エラーハンドリングの例:**
```rust
// Rust example to be added
```
---

## 型定義

### PlayerEntry
```rust
// Rust example to be added
```
### PlayerPos
```rust
// Rust example to be added
```
### PlayerData
```rust
// Rust example to be added
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
