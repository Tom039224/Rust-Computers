# Monitor

**mod**: CC:Tweaked  
**peripheral type**: `monitor`  
**source**: `MonitorPeripheral.java`, `TermMethods.java`

## 概要

モニターブロックへの接続ペリフェラル。ターミナルリダイレクトとして機能し、ワールド内でGUIを開かずに情報を表示・操作できる。

モニターは `TermMethods` の全メソッドを継承し、テキスト描画・カラー操作などのターミナルAPIを提供する。加えてモニター固有の `setTextScale` / `getTextScale` メソッドを持つ。

## モニターの種類

- **通常モニター (Monitor)**: カラー非対応
- **アドバンスドモニター (Advanced Monitor)**: カラー対応、右クリックで `monitor_touch` イベントを発生

## 継承関係

```
TermMethods  ←  MonitorPeripheral  (+  setTextScale / getTextScale)
```

TermMethodsのメソッドについては [term_methods.md](./term_methods.md) を参照。

## モニター固有メソッド

| メソッド名 | 引数 | 戻り値 | 説明 |
|---|---|---|---|
| `setTextScale` | `scale: number` | `nil` | モニターのテキストスケールを設定する（0.5〜5.0、0.5刻み） |
| `getTextScale` | — | `number` | 現在のテキストスケールを返す |

## メソッド詳細

### setTextScale(scale)

モニターのテキストスケールを設定する。

スケールを大きくすると解像度が下がるが、テキストが大きく表示される。

**引数:**
- `scale: number` - モニターのスケール（0.5〜5.0、0.5刻み）

**戻り値:**
- なし

**例:**
```lua
local monitor = peripheral.find("monitor")
monitor.setTextScale(2.0)  -- テキストを2倍のサイズで表示
```

**エラー:**
- スケールが範囲外（0.5〜5.0以外）の場合

---

### getTextScale()

モニターの現在のテキストスケールを取得する。

**引数:**
- なし

**戻り値:**
- `number` - 現在のテキストスケール

**例:**
```lua
local monitor = peripheral.find("monitor")
local scale = monitor.getTextScale()
print("Current scale: " .. scale)
```

**エラー:**
- モニターが切断されている場合

---

## TermMethodsから継承されるメソッド

モニターは以下のTermMethodsメソッドを継承する（詳細は [term_methods.md](./term_methods.md) を参照）：

### テキスト操作
- `write(text)` - テキストを書き込む
- `blit(text, textColor, backgroundColor)` - カラー付きテキストを書き込む
- `clear()` - 画面をクリア
- `clearLine()` - 現在の行をクリア
- `scroll(lines)` - 画面をスクロール

### カーソル操作
- `getCursorPos()` - カーソル位置を取得
- `setCursorPos(x, y)` - カーソル位置を設定
- `getCursorBlink()` - カーソルの点滅状態を取得
- `setCursorBlink(blink)` - カーソルの点滅を設定

### カラー操作
- `isColor()` / `isColour()` - カラー対応かどうかを返す
- `setTextColor(color)` / `setTextColour(color)` - テキスト色を設定
- `getTextColor()` / `getTextColour()` - テキスト色を取得
- `setBackgroundColor(color)` / `setBackgroundColour(color)` - 背景色を設定
- `getBackgroundColor()` / `getBackgroundColour()` - 背景色を取得
- `getPaletteColor(color)` / `getPaletteColour(color)` - パレット色を取得
- `setPaletteColor(color, r, g, b)` / `setPaletteColour(color, r, g, b)` - パレット色を設定

### サイズ取得
- `getSize()` - モニターのサイズ（幅、高さ）を取得

---

## イベント

### monitor_resize

モニターのサイズが変更されたときに発生するイベント。

モニターブロックを追加/削除したり、`setTextScale` を呼び出したりすると発生する。

**パラメータ:**
1. `string` - イベント名（`"monitor_resize"`）
2. `string` - リサイズされたモニターの側面またはネットワークID

**例:**
```lua
while true do
  local event, side = os.pullEvent("monitor_resize")
  print("The monitor on side " .. side .. " was resized.")
  
  local monitor = peripheral.wrap(side)
  local w, h = monitor.getSize()
  print("New size: " .. w .. "x" .. h)
end
```

---

### monitor_touch

アドバンスドモニターが右クリックされたときに発生するイベント。

通常モニターではこのイベントは発生しない。

**パラメータ:**
1. `string` - イベント名（`"monitor_touch"`）
2. `string` - タッチされたモニターの側面またはネットワークID
3. `number` - タッチされたX座標（文字単位）
4. `number` - タッチされたY座標（文字単位）

**例:**
```lua
while true do
  local event, side, x, y = os.pullEvent("monitor_touch")
  print("The monitor on side " .. side .. " was touched at (" .. x .. ", " .. y .. ")")
end
```

---

## 使用例

### 例1: 基本的なテキスト表示

```lua
local monitor = peripheral.find("monitor")
monitor.clear()
monitor.setCursorPos(1, 1)
monitor.write("Hello, world!")
```

### 例2: ターミナルリダイレクト

```lua
local monitor = peripheral.find("monitor")
term.redirect(monitor)

-- 以降、通常のterm APIがモニターに出力される
print("This appears on the monitor")
term.clear()
term.setCursorPos(1, 1)
```

### 例3: カラー表示（アドバンスドモニター）

```lua
local monitor = peripheral.find("monitor")

if monitor.isColor() then
  monitor.setTextScale(2.0)
  monitor.clear()
  
  monitor.setTextColor(colors.red)
  monitor.setCursorPos(1, 1)
  monitor.write("Red Text")
  
  monitor.setTextColor(colors.blue)
  monitor.setCursorPos(1, 2)
  monitor.write("Blue Text")
else
  print("This monitor does not support color")
end
```

### 例4: タッチスクリーンボタン

```lua
local monitor = peripheral.find("monitor")
monitor.clear()
monitor.setTextScale(2.0)

-- ボタンを描画
local function drawButton(x, y, text, color)
  monitor.setCursorPos(x, y)
  monitor.setBackgroundColor(color)
  monitor.write(" " .. text .. " ")
  monitor.setBackgroundColor(colors.black)
end

drawButton(2, 2, "Button 1", colors.red)
drawButton(2, 4, "Button 2", colors.green)
drawButton(2, 6, "Button 3", colors.blue)

-- タッチイベントを処理
while true do
  local event, side, x, y = os.pullEvent("monitor_touch")
  
  if y == 2 then
    print("Button 1 pressed")
  elseif y == 4 then
    print("Button 2 pressed")
  elseif y == 6 then
    print("Button 3 pressed")
  end
end
```

### 例5: 動的なスケール調整

```lua
local monitor = peripheral.find("monitor")
local scale = 0.5

while true do
  monitor.setTextScale(scale)
  monitor.clear()
  monitor.setCursorPos(1, 1)
  monitor.write("Scale: " .. scale)
  
  local w, h = monitor.getSize()
  monitor.setCursorPos(1, 2)
  monitor.write("Size: " .. w .. "x" .. h)
  
  -- スケールを変更
  scale = scale + 0.5
  if scale > 5.0 then
    scale = 0.5
  end
  
  sleep(2)
end
```

### 例6: マルチモニター表示

```lua
local monitors = { peripheral.find("monitor") }

for i, monitor in ipairs(monitors) do
  monitor.clear()
  monitor.setCursorPos(1, 1)
  monitor.write("Monitor " .. i)
end
```

---

## 備考

- モニターは `peripheral.wrap("monitor_0")` 等で接続し、`term.redirect()` に渡して通常の描画APIを使う用途が主
- カラーモニター（Advanced Monitor）は `isColour()` が `true` を返す
- モニターブロックを複数接続すると、自動的に1つの大きなモニターとして機能する
- テキストスケールは0.5〜5.0の範囲で、0.5刻みで設定可能
- スケールを大きくすると解像度が下がるが、テキストが大きく表示される
- `monitor_resize` イベントはモニターのサイズ変更時に発生
- `monitor_touch` イベントはアドバンスドモニターでのみ発生

---

## 関連項目

- [term_methods.md](./term_methods.md) - TermMethodsの詳細
- [CC:Tweaked Documentation](https://tweaked.cc/) - 公式ドキュメント
- `term` API - ターミナル操作API
