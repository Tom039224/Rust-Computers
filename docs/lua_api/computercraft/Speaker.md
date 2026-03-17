# Speaker

**mod**: CC:Tweaked  
**peripheral type**: `speaker`  
**source**: `SpeakerPeripheral.java`

## 概要

スピーカーブロックへの接続ペリフェラル。ノート音・サウンドイベント・PCMオーディオの再生が可能。

スピーカーは3種類のサウンドを再生できる（複雑さの順）：
1. **playNote** - ノートブロックの音を再生
2. **playSound** - Minecraftのサウンドイベントを再生
3. **playAudio** - 任意のPCMオーディオを再生

## メソッド一覧

| メソッド名 | 引数 | 戻り値 | 説明 |
|---|---|---|---|
| `playNote` | `instrument: string, volume?: number, pitch?: number` | `boolean` | 指定楽器でノート音を再生する |
| `playSound` | `name: string, volume?: number, pitch?: number` | `boolean` | サウンドイベントを再生する |
| `playAudio` | `audio: table, volume?: number` | `boolean` | PCMオーディオデータを再生する |
| `stop` | — | `nil` | 再生中のオーディオを停止する |

## メソッド詳細

### playNote(instrument, volume?, pitch?)

ノートブロックの音をスピーカーで再生する。

楽器名、音量、ピッチを指定できる。ピッチはセミトーン単位で、ノートブロックのクリック数に直接対応する。

**制限:** 1ティックあたり最大8音まで再生可能。この制限に達すると `false` を返す。

**引数:**
- `instrument: string` - 楽器名（下記の有効な楽器を参照）
- `volume?: number` - 音量（0.0〜3.0、デフォルト: 1.0）
- `pitch?: number` - ピッチ（セミトーン単位、0〜24、デフォルト: 12）
  - 0, 12, 24 = F#
  - 6, 18 = C

**戻り値:**
- `boolean` - 音を再生できた場合は `true`、制限に達した場合は `false`

**有効な楽器:**
- `"harp"` - ハープ
- `"basedrum"` - バスドラム
- `"snare"` - スネアドラム
- `"hat"` - ハイハット
- `"bass"` - ベース
- `"flute"` - フルート
- `"bell"` - ベル
- `"guitar"` - ギター
- `"chime"` - チャイム
- `"xylophone"` - 木琴
- `"iron_xylophone"` - 鉄琴
- `"cow_bell"` - カウベル
- `"didgeridoo"` - ディジュリドゥ
- `"bit"` - ビット
- `"banjo"` - バンジョー
- `"pling"` - プリング

**例:**
```lua
local speaker = peripheral.find("speaker")

-- ハープでC音を再生
speaker.playNote("harp", 1.0, 6)

-- ベースで低音を再生
speaker.playNote("bass", 3.0, 0)
```

**エラー:**
- 楽器名が無効な場合

---

### playSound(name, volume?, pitch?)

Minecraftのサウンドイベントをスピーカーで再生する。

バニラMinecraftまたはmodのサウンド（例: `"minecraft:block.note_block.harp"`）を再生できる。

**制限:** 
- 一度に1つのサウンドのみ再生可能
- 他のサウンドが同じティックで開始された場合、または `playAudio` がまだ再生中の場合は `false` を返す
- 音楽ディスクは再生できない

**引数:**
- `name: string` - サウンド名（ResourceLocation形式、最大512文字）
- `volume?: number` - 音量（0.0〜3.0、デフォルト: 1.0）
- `pitch?: number` - 再生速度（0.5〜2.0、デフォルト: 1.0）

**戻り値:**
- `boolean` - サウンドを再生できた場合は `true`、できなかった場合は `false`

**例:**
```lua
local speaker = peripheral.find("speaker")

-- クリーパーのヒス音を再生
speaker.playSound("minecraft:entity.creeper.primed")

-- エンダーマンのテレポート音を再生（音量2.0、ピッチ1.5）
speaker.playSound("minecraft:entity.enderman.teleport", 2.0, 1.5)
```

**エラー:**
- サウンド名が無効な場合
- サウンド名が512文字を超える場合

---

### playAudio(audio, volume?)

PCMオーディオデータをスピーカーにストリーミングする。

-128〜127の振幅を持つオーディオサンプルのリストを受け取り、48kHzで再生する。内部バッファがいっぱいの場合は `false` を返す。プログラムは `speaker_audio_empty` イベントを待ってから再度オーディオを再生する必要がある。

**重要:** 
- スピーカーは一度に1回の `playAudio` 呼び出しのみをバッファリングする
- 少量のサンプルを再生すると、音が途切れる可能性がある
- 1回の呼び出しでできるだけ多くのサンプル（最大128×1024）を再生することを推奨
- 8ビットPCMオーディオを受け取るが、再生前に再エンコードされるため、供給されたサンプルが正確に再生されるとは限らない

**引数:**
- `audio: table` - 振幅のリスト（-128〜127の整数）
- `volume?: number` - 音量（省略時は前回の `playAudio` で指定した音量を使用）

**戻り値:**
- `boolean` - オーディオデータを受け入れる余地があった場合は `true`、バッファがいっぱいの場合は `false`

**例:**
```lua
local dfpwm = require("cc.audio.dfpwm")
local speaker = peripheral.find("speaker")

local decoder = dfpwm.make_decoder()
for chunk in io.lines("data/example.dfpwm", 16 * 1024) do
  local buffer = decoder(chunk)
  
  while not speaker.playAudio(buffer) do
    os.pullEvent("speaker_audio_empty")
  end
end
```

**エラー:**
- オーディオデータが空の場合
- オーディオデータが128×1024サンプルを超える場合

---

### stop()

スピーカーで再生中のすべてのオーディオを停止する。

`playAudio` でキューに入れられたオーディオをクリアし、`playSound` で再生された最新のサウンドを停止する。

**引数:**
- なし

**戻り値:**
- なし

**例:**
```lua
local speaker = peripheral.find("speaker")

-- サウンドを再生
speaker.playSound("minecraft:entity.creeper.primed")

-- 1秒後に停止
sleep(1)
speaker.stop()
```

---

## イベント

### speaker_audio_empty

スピーカーのオーディオバッファが空になり、さらにオーディオを再生できるようになったときに発生するイベント。

`playAudio` が `false` を返した場合、このイベントを待ってから再度オーディオを再生する必要がある。

**パラメータ:**
1. `string` - イベント名（`"speaker_audio_empty"`）
2. `string` - オーディオを再生できるスピーカーの名前

**例:**
```lua
local speaker = peripheral.find("speaker")

-- オーディオを再生
local success = speaker.playAudio(audioData)

if not success then
  -- バッファがいっぱいの場合、イベントを待つ
  os.pullEvent("speaker_audio_empty")
  speaker.playAudio(audioData)
end
```

---

## 使用例

### 例1: 簡単なメロディ

```lua
local speaker = peripheral.find("speaker")

local notes = {0, 2, 4, 5, 7, 9, 11, 12}  -- Cメジャースケール

for _, pitch in ipairs(notes) do
  speaker.playNote("harp", 1.0, pitch)
  sleep(0.5)
end
```

### 例2: 効果音の再生

```lua
local speaker = peripheral.find("speaker")

-- 爆発音
speaker.playSound("minecraft:entity.generic.explode", 3.0, 1.0)
sleep(1)

-- ベル音
speaker.playSound("minecraft:block.note_block.bell", 2.0, 1.5)
```

### 例3: DFPWMオーディオの再生

```lua
local dfpwm = require("cc.audio.dfpwm")
local speaker = peripheral.find("speaker")

local decoder = dfpwm.make_decoder()

-- ファイルから16KiBチャンクでオーディオを読み込み
for chunk in io.lines("data/music.dfpwm", 16 * 1024) do
  local buffer = decoder(chunk)
  
  -- バッファがいっぱいの場合は待機
  while not speaker.playAudio(buffer) do
    os.pullEvent("speaker_audio_empty")
  end
end

print("Playback complete")
```

### 例4: 複数のノートを同時に再生

```lua
local speaker = peripheral.find("speaker")

-- 和音を再生（C, E, G）
speaker.playNote("harp", 1.0, 0)   -- C
speaker.playNote("harp", 1.0, 4)   -- E
speaker.playNote("harp", 1.0, 7)   -- G

sleep(2)

-- 別の和音（F, A, C）
speaker.playNote("harp", 1.0, 5)   -- F
speaker.playNote("harp", 1.0, 9)   -- A
speaker.playNote("harp", 1.0, 12)  -- C
```

### 例5: アラーム音

```lua
local speaker = peripheral.find("speaker")

for i = 1, 10 do
  speaker.playNote("bell", 3.0, 12)
  sleep(0.2)
  speaker.playNote("bell", 3.0, 0)
  sleep(0.2)
end
```

### 例6: カスタムPCMオーディオ生成

```lua
local speaker = peripheral.find("speaker")

-- 440Hz（A音）のサイン波を生成
local sampleRate = 48000
local frequency = 440
local duration = 1  -- 1秒

local samples = {}
for i = 1, sampleRate * duration do
  local t = (i - 1) / sampleRate
  local value = math.sin(2 * math.pi * frequency * t) * 127
  samples[i] = math.floor(value)
end

-- オーディオを再生
local chunkSize = 16 * 1024
for i = 1, #samples, chunkSize do
  local chunk = {}
  for j = 1, chunkSize do
    chunk[j] = samples[i + j - 1] or 0
  end
  
  while not speaker.playAudio(chunk) do
    os.pullEvent("speaker_audio_empty")
  end
end
```

---

## 備考

- **playNote**: 1ティックあたり最大8音まで再生可能
- **playSound**: 一度に1つのサウンドのみ再生可能、音楽ディスクは再生不可
- **playAudio**: 
  - サンプルレート: 48kHz
  - 最大バッファサイズ: 128×1024サンプル
  - 8ビットPCM（-128〜127）
  - DFPWMエンコードされたPCMデータに対応
- スピーカーは1ゲームティックあたりの再生回数に上限がある（スパム防止）
- `cc.audio.dfpwm` モジュールを使用してDFPWMオーディオファイルをデコードできる

---

## 関連項目

- [CC:Tweaked Documentation](https://tweaked.cc/) - 公式ドキュメント
- `cc.audio.dfpwm` - DFPWMオーディオのデコード用ユーティリティ
- [speaker_audio](https://tweaked.cc/guide/speaker_audio.html) - `playAudio` 関数の完全なガイド
