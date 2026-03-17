# Monitor

**モジュール:** CC:Tweaked  
**ペリフェラルタイプ:** `monitor`  
**ソース:** `MonitorPeripheral.java`, `TermMethods.java`

## 概要

Monitorペリフェラルは、ワールドに配置できるターミナルディスプレイを提供します。テキストレンダリング、色操作、タッチ入力（高度なモニターの場合）をサポートします。モニターはTerminal APIのすべてのメソッドを継承し、テキストスケーリング用のモニター固有のメソッドを追加します。

## 3つの関数パターン

Monitor APIは、すべてのメソッドに対して3つの関数パターンを使用します：

1. **`book_next_*`** - 次のティックのリクエストをスケジュール
2. **`read_last_*`** - 前のティックの結果を読み取り
3. **`async_*`** - 便利なメソッド（book、待機、読み取りを1つの呼び出しで実行）

### パターンの説明

```lua
-- 方法1: book_next / read_last パターン
monitor.book_next_clear()
wait_for_next_tick()
monitor.read_last_clear()

-- 方法2: async パターン（推奨）
monitor.async_clear()
```

## モニターの種類

- **通常モニター** — モノクロディスプレイ
- **高度なモニター** — カラーディスプレイとタッチサポート

## メソッド

### `setTextScale(scale)` / `book_next_set_text_scale(scale)` / `read_last_set_text_scale()` / `async_set_text_scale(scale)`

モニターのテキストスケールを設定します。

**Lua署名:**
```lua
function setTextScale(scale: number) -> nil
```

**Rust署名:**
```rust
pub fn book_next_set_text_scale(&mut self, scale: f32)
pub fn read_last_set_text_scale(&self) -> Result<(), PeripheralError>
pub async fn async_set_text_scale(&self, scale: f32) -> Result<(), PeripheralError>
```

**パラメータ:**
- `scale: number` — テキストスケール（0.5–5.0、0.5刻み）

**戻り値:** `nil`

**注記:**
- より大きなスケールはテキストを大きくしますが、解像度を低下させます
- 有効な値: 0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0

**例:**
```lua
local monitor = peripheral.find("monitor")
monitor.async_set_text_scale(2.0)
```

---

### `getTextScale()` / `book_next_get_text_scale()` / `read_last_get_text_scale()` / `async_get_text_scale()`

現在のテキストスケールを取得します。

**Lua署名:**
```lua
function getTextScale() -> number
```

**Rust署名:**
```rust
pub fn book_next_get_text_scale(&mut self)
pub fn read_last_get_text_scale(&self) -> Result<f32, PeripheralError>
pub async fn async_get_text_scale(&self) -> Result<f32, PeripheralError>
```

**戻り値:** `number` — 現在のテキストスケール

**例:**
```lua
local monitor = peripheral.find("monitor")
local scale = monitor.async_get_text_scale()
print("現在のスケール: " .. scale)
```

---

### `write(text)` / `book_next_write(text)` / `read_last_write()` / `async_write(text)`

現在のカーソル位置にテキストを書き込みます。

**Lua署名:**
```lua
function write(text: string) -> nil
```

**Rust署名:**
```rust
pub fn book_next_write(&mut self, text: &str)
pub fn read_last_write(&self) -> Result<(), PeripheralError>
pub async fn async_write(&self, text: &str) -> Result<(), PeripheralError>
```

**パラメータ:**
- `text: string` — 書き込むテキスト

**戻り値:** `nil`

**例:**
```lua
local monitor = peripheral.find("monitor")
monitor.async_write("こんにちは、世界！")
```

---

### `clear()` / `book_next_clear()` / `read_last_clear()` / `async_clear()`

モニター画面全体をクリアします。

**Lua署名:**
```lua
function clear() -> nil
```

**Rust署名:**
```rust
pub fn book_next_clear(&mut self)
pub fn read_last_clear(&self) -> Result<(), PeripheralError>
pub async fn async_clear(&self) -> Result<(), PeripheralError>
```

**戻り値:** `nil`

**例:**
```lua
local monitor = peripheral.find("monitor")
monitor.async_clear()
```

---

### `clearLine()` / `book_next_clear_line()` / `read_last_clear_line()` / `async_clear_line()`

現在の行をクリアします。

**Lua署名:**
```lua
function clearLine() -> nil
```

**Rust署名:**
```rust
pub fn book_next_clear_line(&mut self)
pub fn read_last_clear_line(&self) -> Result<(), PeripheralError>
pub async fn async_clear_line(&self) -> Result<(), PeripheralError>
```

**戻り値:** `nil`

---

### `scroll(lines)` / `book_next_scroll(lines)` / `read_last_scroll()` / `async_scroll(lines)`

画面を垂直にスクロールします。

**Lua署名:**
```lua
function scroll(lines: number) -> nil
```

**Rust署名:**
```rust
pub fn book_next_scroll(&mut self, lines: u32)
pub fn read_last_scroll(&self) -> Result<(), PeripheralError>
pub async fn async_scroll(&self, lines: u32) -> Result<(), PeripheralError>
```

**パラメータ:**
- `lines: number` — スクロールする行数（正 = 下、負 = 上）

**戻り値:** `nil`

---

### `getCursorPos()` / `book_next_get_cursor_pos()` / `read_last_get_cursor_pos()` / `async_get_cursor_pos()`

現在のカーソル位置を取得します。

**Lua署名:**
```lua
function getCursorPos() -> number, number
```

**Rust署名:**
```rust
pub fn book_next_get_cursor_pos(&mut self)
pub fn read_last_get_cursor_pos(&self) -> Result<(u32, u32), PeripheralError>
pub async fn async_get_cursor_pos(&self) -> Result<(u32, u32), PeripheralError>
```

**戻り値:** `number, number` — X および Y 座標（1ベース）

**例:**
```lua
local monitor = peripheral.find("monitor")
local x, y = monitor.async_get_cursor_pos()
print(("カーソル位置: (%d, %d)"):format(x, y))
```

---

### `setCursorPos(x, y)` / `book_next_set_cursor_pos(x, y)` / `read_last_set_cursor_pos()` / `async_set_cursor_pos(x, y)`

カーソル位置を設定します。

**Lua署名:**
```lua
function setCursorPos(x: number, y: number) -> nil
```

**Rust署名:**
```rust
pub fn book_next_set_cursor_pos(&mut self, x: u32, y: u32)
pub fn read_last_set_cursor_pos(&self) -> Result<(), PeripheralError>
pub async fn async_set_cursor_pos(&self, x: u32, y: u32) -> Result<(), PeripheralError>
```

**パラメータ:**
- `x: number` — X 座標（1ベース）
- `y: number` — Y 座標（1ベース）

**戻り値:** `nil`

**例:**
```lua
local monitor = peripheral.find("monitor")
monitor.async_set_cursor_pos(1, 1)
```

---

### `getCursorBlink()` / `book_next_get_cursor_blink()` / `read_last_get_cursor_blink()` / `async_get_cursor_blink()`

カーソルの点滅状態を取得します。

**Lua署名:**
```lua
function getCursorBlink() -> boolean
```

**Rust署名:**
```rust
pub fn book_next_get_cursor_blink(&mut self)
pub fn read_last_get_cursor_blink(&self) -> Result<bool, PeripheralError>
pub async fn async_get_cursor_blink(&self) -> Result<bool, PeripheralError>
```

**戻り値:** `boolean` — 点滅している場合は`true`、そうでない場合は`false`

---

### `setCursorBlink(blink)` / `book_next_set_cursor_blink(blink)` / `read_last_set_cursor_blink()` / `async_set_cursor_blink(blink)`

カーソルの点滅状態を設定します。

**Lua署名:**
```lua
function setCursorBlink(blink: boolean) -> nil
```

**Rust署名:**
```rust
pub fn book_next_set_cursor_blink(&mut self, blink: bool)
pub fn read_last_set_cursor_blink(&self) -> Result<(), PeripheralError>
pub async fn async_set_cursor_blink(&self, blink: bool) -> Result<(), PeripheralError>
```

**パラメータ:**
- `blink: boolean` — カーソルが点滅するかどうか

**戻り値:** `nil`

---

### `getSize()` / `book_next_get_size()` / `read_last_get_size()` / `async_get_size()`

モニターのサイズを文字単位で取得します。

**Lua署名:**
```lua
function getSize() -> number, number
```

**Rust署名:**
```rust
pub fn book_next_get_size(&mut self)
pub fn read_last_get_size(&self) -> Result<(u32, u32), PeripheralError>
pub async fn async_get_size(&self) -> Result<(u32, u32), PeripheralError>
```

**戻り値:** `number, number` — 幅と高さ（文字単位）

**例:**
```lua
local monitor = peripheral.find("monitor")
local width, height = monitor.async_get_size()
print(("モニターサイズ: %d x %d"):format(width, height))
```

---

### `setTextColor(color)` / `book_next_set_text_color(color)` / `read_last_set_text_color()` / `async_set_text_color(color)`

テキスト色を設定します（高度なモニターのみ）。

**Lua署名:**
```lua
function setTextColor(color: number) -> nil
```

**Rust署名:**
```rust
pub fn book_next_set_text_color(&mut self, color: u32)
pub fn read_last_set_text_color(&self) -> Result<(), PeripheralError>
pub async fn async_set_text_color(&self, color: u32) -> Result<(), PeripheralError>
```

**パラメータ:**
- `color: number` — 色値（0x000000–0xFFFFFF）

**戻り値:** `nil`

**定義済みの色:**
- `0xFFFFFF` — 白
- `0xFF0000` — 赤
- `0x00FF00` — 緑
- `0x0000FF` — 青
- `0xFFFF00` — 黄
- `0xFF00FF` — マゼンタ
- `0x00FFFF` — シアン
- `0x000000` — 黒

---

### `getTextColor()` / `book_next_get_text_color()` / `read_last_get_text_color()` / `async_get_text_color()`

現在のテキスト色を取得します。

**Lua署名:**
```lua
function getTextColor() -> number
```

**Rust署名:**
```rust
pub fn book_next_get_text_color(&mut self)
pub fn read_last_get_text_color(&self) -> Result<u32, PeripheralError>
pub async fn async_get_text_color(&self) -> Result<u32, PeripheralError>
```

**戻り値:** `number` — 現在のテキスト色

---

### `setBackgroundColor(color)` / `book_next_set_background_color(color)` / `read_last_set_background_color()` / `async_set_background_color(color)`

背景色を設定します（高度なモニターのみ）。

**Lua署名:**
```lua
function setBackgroundColor(color: number) -> nil
```

**Rust署名:**
```rust
pub fn book_next_set_background_color(&mut self, color: u32)
pub fn read_last_set_background_color(&self) -> Result<(), PeripheralError>
pub async fn async_set_background_color(&self, color: u32) -> Result<(), PeripheralError>
```

**パラメータ:**
- `color: number` — 色値（0x000000–0xFFFFFF）

**戻り値:** `nil`

---

### `getBackgroundColor()` / `book_next_get_background_color()` / `read_last_get_background_color()` / `async_get_background_color()`

現在の背景色を取得します。

**Lua署名:**
```lua
function getBackgroundColor() -> number
```

**Rust署名:**
```rust
pub fn book_next_get_background_color(&mut self)
pub fn read_last_get_background_color(&self) -> Result<u32, PeripheralError>
pub async fn async_get_background_color(&self) -> Result<u32, PeripheralError>
```

**戻り値:** `number` — 現在の背景色

---

### `blit(text, textColor, backgroundColor)` / `book_next_blit(...)` / `read_last_blit()` / `async_blit(...)`

指定された色を使用してテキストを書き込みます。

**Lua署名:**
```lua
function blit(text: string, textColor: string, backgroundColor: string) -> nil
```

**Rust署名:**
```rust
pub fn book_next_blit(&mut self, text: &str, text_color: &str, bg_color: &str)
pub fn read_last_blit(&self) -> Result<(), PeripheralError>
pub async fn async_blit(&self, text: &str, text_color: &str, bg_color: &str) -> Result<(), PeripheralError>
```

**パラメータ:**
- `text: string` — 書き込むテキスト
- `textColor: string` — テキスト色コード（文字ごとに1つ）
- `backgroundColor: string` — 背景色コード（文字ごとに1つ）

**戻り値:** `nil`

**色コード:**
- `0` — 黒, `1` — 青, `2` — 緑, `3` — シアン
- `4` — 赤, `5` — マゼンタ, `6` — 黄, `7` — 白
- `8` — ライトグレー, `9` — ライトブルー, `a` — ライム, `b` — ライトシアン
- `c` — ライトレッド, `d` — ピンク, `e` — ライトイエロー, `f` — ライトグレー

---

## イベント

### `monitor_resize`

モニターのサイズが変更されたときに発生します。

**イベントパラメータ:**
1. `string` — イベント名（`"monitor_resize"`）
2. `string` — モニターの側またはネットワークID

**例:**
```lua
local monitor = peripheral.find("monitor")

while true do
  local event, side = os.pullEvent("monitor_resize")
  local width, height = monitor.async_get_size()
  print(("モニターが %d x %d にリサイズされました"):format(width, height))
end
```

---

### `monitor_touch`

高度なモニターが右クリックされたときに発生します。

**イベントパラメータ:**
1. `string` — イベント名（`"monitor_touch"`）
2. `string` — モニターの側またはネットワークID
3. `number` — X 座標（1ベース）
4. `number` — Y 座標（1ベース）

**例:**
```lua
local monitor = peripheral.find("monitor")

while true do
  local event, side, x, y = os.pullEvent("monitor_touch")
  print(("タッチ位置: (%d, %d)"):format(x, y))
end
```

---

## 使用例

### 例1: 基本的なテキスト表示

```lua
local monitor = peripheral.find("monitor")

monitor.async_clear()
monitor.async_set_cursor_pos(1, 1)
monitor.async_write("こんにちは、世界！")
```

### 例2: カラーテキスト（高度なモニター）

```lua
local monitor = peripheral.find("monitor")

monitor.async_clear()
monitor.async_set_cursor_pos(1, 1)

monitor.async_set_text_color(0xFF0000)  -- 赤
monitor.async_write("赤いテキスト")

monitor.async_set_cursor_pos(1, 2)
monitor.async_set_text_color(0x00FF00)  -- 緑
monitor.async_write("緑のテキスト")
```

### 例3: ターミナルリダイレクト

```lua
local monitor = peripheral.find("monitor")

-- すべての出力をモニターにリダイレクト
term.redirect(monitor)

print("これはモニターに表示されます")
print("これも表示されます！")

-- 通常の出力に戻す
term.restore()
```

### 例4: タッチボタン

```lua
local monitor = peripheral.find("monitor")

local function draw_button(x, y, text, color)
  monitor.async_set_cursor_pos(x, y)
  monitor.async_set_background_color(color)
  monitor.async_write(" " .. text .. " ")
  monitor.async_set_background_color(0x000000)
end

monitor.async_clear()
draw_button(2, 2, "ボタン 1", 0xFF0000)
draw_button(2, 4, "ボタン 2", 0x00FF00)

while true do
  local event, side, x, y = os.pullEvent("monitor_touch")
  
  if y == 2 then
    print("ボタン 1 が押されました")
  elseif y == 4 then
    print("ボタン 2 が押されました")
  end
end
```

### 例5: 動的スケーリング

```lua
local monitor = peripheral.find("monitor")
local scale = 0.5

while true do
  monitor.async_set_text_scale(scale)
  monitor.async_clear()
  monitor.async_set_cursor_pos(1, 1)
  monitor.async_write("スケール: " .. scale)
  
  local w, h = monitor.async_get_size()
  monitor.async_set_cursor_pos(1, 2)
  monitor.async_write(("サイズ: %d x %d"):format(w, h))
  
  scale = scale + 0.5
  if scale > 5.0 then scale = 0.5 end
  
  sleep(2)
end
```

---

## エラーハンドリング

すべてのメソッドは以下の場合にエラーをスローする可能性があります：

- **モニターが見つからない**: ペリフェラルが切断されている
- **無効な色**: 色値が範囲外
- **無効なスケール**: テキストスケールが有効な範囲内にない
- **無効な位置**: カーソル位置が範囲外

**エラーハンドリングの例:**
```lua
local monitor = peripheral.find("monitor")
if not monitor then
  error("モニターが見つかりません")
end

local success, result = pcall(function()
  monitor.async_set_text_scale(2.0)
end)

if not success then
  print("エラー: " .. result)
else
  print("スケールが正常に設定されました")
end
```

---

## 型定義

### Color
```lua
-- RGB色値（0x000000 から 0xFFFFFF）
type Color = number
```

### Position
```lua
-- (x, y) 座標（1ベース）
type Position = (number, number)
```

### Size
```lua
-- (幅, 高さ)（文字単位）
type Size = (number, number)
```

---

## 注記

- すべての座標は1ベース（最初の位置は0ではなく1）
- テキストスケールは表示サイズと解像度の両方に影響します
- 色サポートには高度なモニターが必要です
- タッチイベントは高度なモニターでのみ機能します
- 複数のモニターを1つの大きなディスプレイに組み合わせることができます
- 3つの関数パターンは効率的なバッチ操作を可能にします

---

## 関連

- [Speaker](./Speaker.md) — オーディオ出力ペリフェラル
- [CC:Tweaked ドキュメント](https://tweaked.cc/) — 公式ドキュメント
- `term` API — ターミナル操作API
