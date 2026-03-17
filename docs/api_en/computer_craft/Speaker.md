# Speaker

**Mod:** CC:Tweaked  
**Peripheral Type:** `speaker`  
**Source:** `SpeakerPeripheral.java`

## Overview

The Speaker peripheral allows you to play sounds in the Minecraft world. It supports three types of audio: note block notes, Minecraft sound events, and raw PCM audio. Speakers have rate limits to prevent spam.

## Three-Function Pattern

The Speaker API uses the three-function pattern for all methods:

1. **`book_next_*`** - Schedule a request for the next tick
2. **`read_last_*`** - Read the result from the previous tick
3. **`async_*`** - Convenience method that books, waits, and reads in one call

### Pattern Explanation

```rust
// Rust example to be added
```
## Implementation Status

### ✅ Implemented

- book_next_play_note / read_last_play_note
- book_next_play_sound / read_last_play_sound
- book_next_stop / read_last_stop

### 🚧 Not Yet Implemented

- async_* variants for all methods
- speaker_audio_empty event


## Methods

### `playNote(instrument, volume?, pitch?)` / `book_next_play_note(...)` / `read_last_play_note()` / `async_play_note(...)`

Play a note block note.

**Rust Signatures:**
```rust
pub fn book_next_play_note(&mut self, instrument: &str, volume: Option<f32>, pitch: Option<f32>)
pub fn read_last_play_note(&self) -> Result<bool, PeripheralError>
pub async fn async_play_note(&self, instrument: &str, volume: Option<f32>, pitch: Option<f32>) -> Result<bool, PeripheralError>
```

**Parameters:**
- `instrument: string` — Instrument name (see list below)
- `volume?: number` — Volume level 0.0–3.0 (default: 1.0)
- `pitch?: number` — Pitch in semitones 0–24 (default: 12)

**Returns:** `boolean` — `true` if note was played, `false` if rate limit reached

**Valid Instruments:**
- `"harp"` — Harp
- `"basedrum"` — Bass drum
- `"snare"` — Snare drum
- `"hat"` — Hi-hat
- `"bass"` — Bass
- `"flute"` — Flute
- `"bell"` — Bell
- `"guitar"` — Guitar
- `"chime"` — Chime
- `"xylophone"` — Xylophone
- `"iron_xylophone"` — Iron xylophone
- `"cow_bell"` — Cow bell
- `"didgeridoo"` — Didgeridoo
- `"bit"` — Bit
- `"banjo"` — Banjo
- `"pling"` — Pling

**Pitch Reference:**
- Pitch 0, 12, 24 = F#
- Pitch 6, 18 = C
- Pitch 12 = Middle C

**Rate Limit:**
- Maximum 8 notes per tick per speaker

**Example:**
```rust
// Rust example to be added
```
---

### `playSound(name, volume?, pitch?)` / `book_next_play_sound(...)` / `read_last_play_sound()` / `async_play_sound(...)`

Play a Minecraft sound event.

**Rust Signatures:**
```rust
pub fn book_next_play_sound(&mut self, name: &str, volume: Option<f32>, pitch: Option<f32>)
pub fn read_last_play_sound(&self) -> Result<bool, PeripheralError>
pub async fn async_play_sound(&self, name: &str, volume: Option<f32>, pitch: Option<f32>) -> Result<bool, PeripheralError>
```

**Parameters:**
- `name: string` — Sound resource name (e.g., `"minecraft:entity.creeper.primed"`)
- `volume?: number` — Volume level 0.0–3.0 (default: 1.0)
- `pitch?: number` — Playback speed 0.5–2.0 (default: 1.0)

**Returns:** `boolean` — `true` if sound was played, `false` if another sound is playing

**Common Sounds:**
- `"minecraft:entity.creeper.primed"` — Creeper hiss
- `"minecraft:entity.enderman.teleport"` — Enderman teleport
- `"minecraft:entity.generic.explode"` — Explosion
- `"minecraft:block.note_block.harp"` — Note block harp
- `"minecraft:block.note_block.bell"` — Note block bell
- `"minecraft:ui.button.click"` — Button click

**Rate Limit:**
- Only one sound can play at a time
- Cannot play while `playAudio` is active

**Example:**
```rust
// Rust example to be added
```
---

### `stop()` / `book_next_stop()` / `read_last_stop()` / `async_stop()`

Stop all audio playback.

**Rust Signatures:**
```rust
pub fn book_next_stop(&mut self)
pub fn read_last_stop(&self) -> Result<(), PeripheralError>
pub async fn async_stop(&self) -> Result<(), PeripheralError>
```

**Returns:** `nil`

**Example:**
```rust
// Rust example to be added
```
---

## Events

### `speaker_audio_empty` 🚧

Fired when the speaker's audio buffer becomes empty and is ready for more audio data.

**Event Parameters:**
1. `string` — Event name (`"speaker_audio_empty"`)
2. `string` — Speaker name

**Example:**
```rust
// Rust example to be added
```
---

## Usage Examples

### Example 1: Simple Melody

```rust
// Rust example to be added
```
### Example 2: Sound Effects

```rust
// Rust example to be added
```
### Example 3: Chord

```rust
// Rust example to be added
```
### Example 4: Alarm

```rust
// Rust example to be added
```
### Example 5: Pitch Sweep

```rust
// Rust example to be added
```
### Example 6: Volume Fade

```rust
// Rust example to be added
```
---

## Error Handling

All methods may throw errors in the following cases:

- **Speaker not found**: Peripheral is disconnected
- **Invalid instrument**: Instrument name is not recognized
- **Invalid sound**: Sound resource name is invalid or too long (>512 chars)
- **Invalid parameters**: Volume or pitch values are out of range

**Example Error Handling:**
```rust
// Rust example to be added
```
---

## Type Definitions

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

## Notes

- Maximum 8 notes per tick per speaker
- Only one sound can play at a time
- `playNote` and `playSound` cannot play simultaneously
- The three-function pattern allows for efficient batch operations
- Sound names use Minecraft's ResourceLocation format
- Pitch values correspond directly to note block click counts

---

## Related

- [Monitor](./Monitor.md) — Display peripheral
- [CC:Tweaked Documentation](https://tweaked.cc/) — Official documentation
- `cc.audio.dfpwm` — DFPWM audio codec (for advanced audio)
