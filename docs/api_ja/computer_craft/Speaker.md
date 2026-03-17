# Speaker

**Mod:** CC:Tweaked  
**ペリフェラルタイプ:** `speaker`  
**ソース:** `SpeakerPeripheral.java`

## 概要

Speakerペリフェラルはワールドで音を再生できます。3種類のオーディオをサポートしています：ノートブロック音、Minecraftサウンドイベント、生PCMオーディオ。スピーカーはスパム防止のためレート制限があります。

## 3つの関数パターン

Speaker APIは全メソッドで3つの関数パターンを使用します：

1. **`book_next_*`** — 次のティックのリクエストをスケジュール
2. **`read_last_*`** — 前のティックの結果を読み取り
3. **`async_*`** — book、待機、読み取りを1つの呼び出しで行う便利メソッド

### パターン説明

```lua
-- 方法1: book_next / read_last パターン
speaker.book_next_play_note("harp", 1.0, 12)
wait_for_next_tick()
speaker.read_last_play_note()

-- 方法2: async パターン（推奨）
speaker.async_play_note("harp", 1.0, 12)
```

## メソッド

### `playNote(instrument, volume?, pitch?)` / `book_next_play_note(...)` / `read_last_play_note()` / `async_play_note(...)`

ノートブロック音を再生します。

**Lua署名:**
```lua
function playNote(instrument: string, volume?: number, pitch?: number) -> boolean
```

**Rust署名:**
```rust
pub fn book_next_play_note(&mut self, instrument: &str, volume: Option<f32>, pitch: Option<f32>)
pub fn read_last_play_note(&self) -> Result<bool, PeripheralError>
pub async fn async_play_note(&self, instrument: &str, volume: Option<f32>, pitch: Option<f32>) -> Result<bool, PeripheralError>
```

**パラメータ:**
- `instrument: string` — 楽器名（下記のリストを参照）
- `volume?: number` — 音量レベル 0.0–3.0（デフォルト: 1.0）
- `pitch?: number` — ピッチ（セミトーン単位）0–24（デフォルト: 12）

**戻り値:** `boolean` — 音が再生された場合は `true`、レート制限に達した場合は `false`

**有効な楽器:**
- `"harp"` — ハープ
- `"basedrum"` — バスドラム
- `"snare"` — スネアドラム
- `"hat"` — ハイハット
- `"bass"` — ベース
- `"flute"` — フルート
- `"bell"` — ベル
- `"guitar"` — ギター
- `"chime"` — チャイム
- `"xylophone"` — 木琴
- `"iron_xylophone"` — 鉄琴
- `"cow_bell"` — カウベル
- `"didgeridoo"` — ディジュリドゥ
- `"bit"` — ビット
- `"banjo"` — バンジョー
- `"pling"` — プリング

**ピッチリファレンス:**
- ピッチ 0, 12, 24 = F#
- ピッチ 6, 18 = C
- ピッチ 12 = 中央C

**レート制限:**
- スピーカーあたり1ティックあたり最大8音

**例:**
```lua
local speaker = peripheral.find("speaker")

-- ハープ音を再生
speaker.async_play_note("harp", 1.0, 12)

-- 高音量でベース音を再生
speaker.async_play_note("bass", 3.0, 0)
```

---

### `playSound(name, volume?, pitch?)` / `book_next_play_sound(...)` / `read_last_play_sound()` / `async_play_sound(...)`

Minecraftサウンドイベントを再生します。

**Lua署名:**
```lua
function playSound(name: string, volume?: number, pitch?: number) -> boolean
```

**Rust署名:**
```rust
pub fn book_next_play_sound(&mut self, name: &str, volume: Option<f32>, pitch: Option<f32>)
pub fn read_last_play_sound(&self) -> Result<bool, PeripheralError>
pub async fn async_play_sound(&self, name: &str, volume: Option<f32>, pitch: Option<f32>) -> Result<bool, PeripheralError>
```

**パラメータ:**
- `name: string` — サウンドリソース名（例: `"minecraft:entity.creeper.primed"`）
- `volume?: number` — 音量レベル 0.0–3.0（デフォルト: 1.0）
- `pitch?: number` — 再生速度 0.5–2.0（デフォルト: 1.0）

**戻り値:** `boolean` — サウンドが再生された場合は `true`、できなかった場合は `false`

**一般的なサウンド:**
- `"minecraft:entity.creeper.primed"` — クリーパーのヒス音
- `"minecraft:entity.enderman.teleport"` — エンダーマンのテレポート音
- `"minecraft:entity.generic.explode"` — 爆発音
- `"minecraft:block.note_block.harp"` — ノートブロックハープ
- `"minecraft:block.note_block.bell"` — ノートブロックベル
- `"minecraft:ui.button.click"` — ボタンクリック音

**レート制限:**
- 一度に1つのサウンドのみ再生可能
- `playAudio` がまだ再生中の場合は再生できません

**例:**
```lua
local speaker = peripheral.find("speaker")

-- クリーパーのヒス音を再生
speaker.async_play_sound("minecraft:entity.creeper.primed", 1.0, 1.0)

-- 高音量で爆発音を再生
speaker.async_play_sound("minecraft:entity.generic.explode", 3.0, 1.0)
```

---

### `stop()` / `book_next_stop()` / `read_last_stop()` / `async_stop()`

全オーディオ再生を停止します。

**Lua署名:**
```lua
function stop() -> nil
```

**Rust署名:**
```rust
pub fn book_next_stop(&mut self)
pub fn read_last_stop(&self) -> Result<(), PeripheralError>
pub async fn async_stop(&self) -> Result<(), PeripheralError>
```

**戻り値:** `nil`

**例:**
```lua
local speaker = peripheral.find("speaker")

-- サウンドを再生
speaker.async_play_sound("minecraft:entity.creeper.primed")

-- 1秒後に停止
sleep(1)
speaker.async_stop()
```

---

## イベント

### `speaker_audio_empty`

スピーカーのオーディオバッファが空になり、さらにオーディオデータを受け入れる準備ができたときに発火します。

**イベントパラメータ:**
1. `string` — イベント名（`"speaker_audio_empty"`）
2. `string` — スピーカー名

**例:**
```lua
local speaker = peripheral.find("speaker")

-- オーディオデータを再生
local success = speaker.playAudio(audioData)

if not success then
  -- バッファがいっぱいの場合、空になるまで待機
  os.pullEvent("speaker_audio_empty")
  speaker.playAudio(audioData)
end
```

---

## 使用例

### 例1: シンプルなメロディ

```lua
local speaker = peripheral.find("speaker")

local notes = {0, 2, 4, 5, 7, 9, 11, 12}  -- Cメジャースケール

for _, pitch in ipairs(notes) do
  speaker.async_play_note("harp", 1.0, pitch)
  sleep(0.5)
end
```

### 例2: 効果音

```lua
local speaker = peripheral.find("speaker")

-- 爆発音
speaker.async_play_sound("minecraft:entity.generic.explode", 3.0, 1.0)
sleep(1)

-- ベル音
speaker.async_play_sound("minecraft:block.note_block.bell", 2.0, 1.5)
sleep(1)

-- クリック音
speaker.async_play_sound("minecraft:ui.button.click", 1.0, 1.0)
```

### 例3: 和音

```lua
local speaker = peripheral.find("speaker")

-- Cメジャー和音を再生（C, E, G）
speaker.async_play_note("harp", 1.0, 0)   -- C
speaker.async_play_note("harp", 1.0, 4)   -- E
speaker.async_play_note("harp", 1.0, 7)   -- G

sleep(2)

-- Fメジャー和音を再生（F, A, C）
speaker.async_play_note("harp", 1.0, 5)   -- F
speaker.async_play_note("harp", 1.0, 9)   -- A
speaker.async_play_note("harp", 1.0, 12)  -- C
```

### 例4: アラーム

```lua
local speaker = peripheral.find("speaker")

for i = 1, 10 do
  speaker.async_play_note("bell", 3.0, 12)
  sleep(0.2)
  speaker.async_play_note("bell", 3.0, 0)
  sleep(0.2)
end
```

### 例5: ピッチスイープ

```lua
local speaker = peripheral.find("speaker")

-- 低から高へスイープ
for pitch = 0, 24 do
  speaker.async_play_note("flute", 1.0, pitch)
  sleep(0.1)
end
```

### 例6: ボリュームフェード

```lua
local speaker = peripheral.find("speaker")

-- フェードイン
for volume = 0, 10 do
  speaker.async_play_note("harp", volume / 10, 12)
  sleep(0.1)
end

-- フェードアウト
for volume = 10, 0, -1 do
  speaker.async_play_note("harp", volume / 10, 12)
  sleep(0.1)
end
```

---

## エラーハンドリング

全メソッドは以下の場合にエラーをスロー可能です：

- **スピーカーが見つからない**: ペリフェラルが切断されている
- **無効な楽器**: 楽器名が認識されない
- **無効なサウンド**: サウンドリソース名が無効または長すぎる（>512文字）
- **無効なパラメータ**: 音量またはピッチ値が範囲外

**エラーハンドリング例:**
```lua
local speaker = peripheral.find("speaker")
if not speaker then
  error("No speaker found")
end

local success, result = pcall(function()
  return speaker.async_play_note("harp", 1.0, 12)
end)

if not success then
  print("Error: " .. result)
else
  print("Note played successfully")
end
```

---

## 型定義

### Instrument
```lua
type Instrument = 
  | "harp"
  | "basedrum"
  | "snare"
  | "hat"
  | "bass"
  | "flute"
  | "bell"
  | "guitar"
  | "chime"
  | "xylophone"
  | "iron_xylophone"
  | "cow_bell"
  | "didgeridoo"
  | "bit"
  | "banjo"
  | "pling"
```

### Volume
```lua
-- 範囲: 0.0 から 3.0
type Volume = number
```

### Pitch
```lua
-- 範囲: 0 から 24（セミトーン単位）
type Pitch = number
```

---

## 注記

- スピーカーあたり1ティックあたり最大8音
- 一度に1つのサウンドのみ再生可能
- `playNote` と `playSound` は同時に再生できません
- 3つの関数パターンは効率的なバッチ操作を可能にします
- サウンド名はMinecraftのResourceLocation形式を使用します
- ピッチ値はノートブロッククリック数に直接対応します

---

## 関連

- [Monitor](./Monitor.md) — ディスプレイペリフェラル
- [CC:Tweaked Documentation](https://tweaked.cc/) — 公式ドキュメント
- `cc.audio.dfpwm` — DFPWMオーディオコーデック（高度なオーディオ用）
