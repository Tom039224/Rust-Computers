# Speaker

**モジュール:** CC:Tweaked  
**ペリフェラルタイプ:** `speaker`  
**ソース:** `SpeakerPeripheral.java`

## 概要

Speakerペリフェラルは、Minecraftワールドで音を再生できます。3種類のオーディオをサポートします：ノートブロックノート、Minecraftサウンドイベント、生PCMオーディオ。スピーカーはスパム防止のためのレート制限があります。

## 3つの関数パターン

Speaker APIは、すべてのメソッドに対して3つの関数パターンを使用します：

1. **`book_next_*`** - 次のティックのリクエストをスケジュール
2. **`read_last_*`** - 前のティックの結果を読み取り
3. **`async_*`** - 便利なメソッド（book、待機、読み取りを1つの呼び出しで実行）

### パターンの説明

```rust
// Rust example to be added
```
## 実装状況

### ✅ 実装済み

- book_next_play_note / read_last_play_note
- book_next_play_sound / read_last_play_sound
- book_next_stop / read_last_stop

- speaker_audio_empty event


## メソッド

### `playNote(instrument, volume?, pitch?)` / `book_next_play_note(...)` / `read_last_play_note()` / `async_play_note(...)`

ノートブロックノートを再生します。

**Lua署名:**
```rust
// Rust example to be added
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
- `pitch?: number` — ピッチ（半音） 0–24（デフォルト: 12）

**戻り値:** `boolean` — ノートが再生された場合は`true`、レート制限に達した場合は`false`

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
- `"xylophone"` — シロフォン
- `"iron_xylophone"` — 鉄琴
- `"cow_bell"` — カウベル
- `"didgeridoo"` — ディジェリドゥ
- `"bit"` — ビット
- `"banjo"` — バンジョー
- `"pling"` — プリング

**ピッチリファレンス:**
- ピッチ 0, 12, 24 = F#
- ピッチ 6, 18 = C
- ピッチ 12 = ミドルC

**レート制限:**
- スピーカーあたり1ティックあたり最大8ノート

**例:**
```rust
// Rust example to be added
```
---

### `playSound(name, volume?, pitch?)` / `book_next_play_sound(...)` / `read_last_play_sound()` / `async_play_sound(...)`

Minecraftサウンドイベントを再生します。

**Lua署名:**
```rust
// Rust example to be added
```
**Rust署名:**
```rust
pub fn book_next_play_sound(&mut self, name: &str, volume: Option<f32>, pitch: Option<f32>)
pub fn read_last_play_sound(&self) -> Result<bool, PeripheralError>
pub async fn async_play_sound(&self, name: &str, volume: Option<f32>, pitch: Option<f32>) -> Result<bool, PeripheralError>
```

**パラメータ:**
- `name: string` — サウンドリソース名（例：`"minecraft:entity.creeper.primed"`）
- `volume?: number` — 音量レベル 0.0–3.0（デフォルト: 1.0）
- `pitch?: number` — 再生速度 0.5–2.0（デフォルト: 1.0）

**戻り値:** `boolean` — サウンドが再生された場合は`true`、別のサウンドが再生中の場合は`false`

**一般的なサウンド:**
- `"minecraft:entity.creeper.primed"` — クリーパーのシュー音
- `"minecraft:entity.enderman.teleport"` — エンダーマンのテレポート
- `"minecraft:entity.generic.explode"` — 爆発
- `"minecraft:block.note_block.harp"` — ノートブロックハープ
- `"minecraft:block.note_block.bell"` — ノートブロックベル
- `"minecraft:ui.button.click"` — ボタンクリック

**レート制限:**
- 一度に1つのサウンドのみ再生可能
- `playAudio` がアクティブな間は再生できません

**例:**
```rust
// Rust example to be added
```
---

### `stop()` / `book_next_stop()` / `read_last_stop()` / `async_stop()`

すべてのオーディオ再生を停止します。

**Lua署名:**
```rust
// Rust example to be added
```
**Rust署名:**
```rust
pub fn book_next_stop(&mut self)
pub fn read_last_stop(&self) -> Result<(), PeripheralError>
pub async fn async_stop(&self) -> Result<(), PeripheralError>
```

**戻り値:** `nil`

**例:**
```rust
// Rust example to be added
```
---

## イベント

### `speaker_audio_empty` 🚧

スピーカーのオーディオバッファが空になり、より多くのオーディオデータの準備ができたときに発生します。

**イベントパラメータ:**
1. `string` — イベント名（`"speaker_audio_empty"`）
2. `string` — スピーカー名

**例:**
```rust
// Rust example to be added
```
---

## 使用例

### 例1: シンプルなメロディ

```rust
// Rust example to be added
```
### 例2: サウンドエフェクト

```rust
// Rust example to be added
```
### 例3: コード

```rust
// Rust example to be added
```
### 例4: アラーム

```rust
// Rust example to be added
```
### 例5: ピッチスイープ

```rust
// Rust example to be added
```
### 例6: 音量フェード

```rust
// Rust example to be added
```
---

## エラーハンドリング

すべてのメソッドは以下の場合にエラーをスローする可能性があります：

- **スピーカーが見つからない**: ペリフェラルが切断されている
- **無効な楽器**: 楽器名が認識されない
- **無効なサウンド**: サウンドリソース名が無効または長すぎる（>512文字）
- **無効なパラメータ**: 音量またはピッチ値が範囲外

**エラーハンドリングの例:**
```rust
// Rust example to be added
```
---

## 型定義

### Instrument
```rust
// Rust example to be added
```
### Volume
```rust
// Rust example to be added
```
### Pitch
```rust
// Rust example to be added
```
---

## 注記

- スピーカーあたり1ティックあたり最大8ノート
- 一度に1つのサウンドのみ再生可能
- `playNote` と `playSound` は同時に再生できません
- 3つの関数パターンは効率的なバッチ操作を可能にします
- サウンド名はMinecraftの ResourceLocation 形式を使用します
- ピッチ値はノートブロッククリック数に直接対応します

---

## 関連

- [Monitor](./Monitor.md) — ディスプレイペリフェラル
- [CC:Tweaked ドキュメント](https://tweaked.cc/) — 公式ドキュメント
- `cc.audio.dfpwm` — DFPWMオーディオコーデック（高度なオーディオ用）
