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

```lua
-- Method 1: book_next / read_last pattern
speaker.book_next_play_note("harp", 1.0, 12)
wait_for_next_tick()
speaker.read_last_play_note()

-- Method 2: async pattern (recommended)
speaker.async_play_note("harp", 1.0, 12)
```

## Methods

### `playNote(instrument, volume?, pitch?)` / `book_next_play_note(...)` / `read_last_play_note()` / `async_play_note(...)`

Play a note block note.

**Lua Signature:**
```lua
function playNote(instrument: string, volume?: number, pitch?: number) -> boolean
```

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
```lua
local speaker = peripheral.find("speaker")

-- Play a harp note
speaker.async_play_note("harp", 1.0, 12)

-- Play a bass note at high volume
speaker.async_play_note("bass", 3.0, 0)
```

---

### `playSound(name, volume?, pitch?)` / `book_next_play_sound(...)` / `read_last_play_sound()` / `async_play_sound(...)`

Play a Minecraft sound event.

**Lua Signature:**
```lua
function playSound(name: string, volume?: number, pitch?: number) -> boolean
```

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
```lua
local speaker = peripheral.find("speaker")

-- Play a creeper hiss
speaker.async_play_sound("minecraft:entity.creeper.primed", 1.0, 1.0)

-- Play an explosion at high volume
speaker.async_play_sound("minecraft:entity.generic.explode", 3.0, 1.0)
```

---

### `stop()` / `book_next_stop()` / `read_last_stop()` / `async_stop()`

Stop all audio playback.

**Lua Signature:**
```lua
function stop() -> nil
```

**Rust Signatures:**
```rust
pub fn book_next_stop(&mut self)
pub fn read_last_stop(&self) -> Result<(), PeripheralError>
pub async fn async_stop(&self) -> Result<(), PeripheralError>
```

**Returns:** `nil`

**Example:**
```lua
local speaker = peripheral.find("speaker")

-- Play a sound
speaker.async_play_sound("minecraft:entity.creeper.primed")

-- Stop after 1 second
sleep(1)
speaker.async_stop()
```

---

## Events

### `speaker_audio_empty`

Fired when the speaker's audio buffer becomes empty and is ready for more audio data.

**Event Parameters:**
1. `string` — Event name (`"speaker_audio_empty"`)
2. `string` — Speaker name

**Example:**
```lua
local speaker = peripheral.find("speaker")

-- Play audio data
local success = speaker.playAudio(audioData)

if not success then
  -- Buffer is full, wait for it to empty
  os.pullEvent("speaker_audio_empty")
  speaker.playAudio(audioData)
end
```

---

## Usage Examples

### Example 1: Simple Melody

```lua
local speaker = peripheral.find("speaker")

local notes = {0, 2, 4, 5, 7, 9, 11, 12}  -- C major scale

for _, pitch in ipairs(notes) do
  speaker.async_play_note("harp", 1.0, pitch)
  sleep(0.5)
end
```

### Example 2: Sound Effects

```lua
local speaker = peripheral.find("speaker")

-- Explosion
speaker.async_play_sound("minecraft:entity.generic.explode", 3.0, 1.0)
sleep(1)

-- Bell
speaker.async_play_sound("minecraft:block.note_block.bell", 2.0, 1.5)
sleep(1)

-- Click
speaker.async_play_sound("minecraft:ui.button.click", 1.0, 1.0)
```

### Example 3: Chord

```lua
local speaker = peripheral.find("speaker")

-- Play C major chord (C, E, G)
speaker.async_play_note("harp", 1.0, 0)   -- C
speaker.async_play_note("harp", 1.0, 4)   -- E
speaker.async_play_note("harp", 1.0, 7)   -- G

sleep(2)

-- Play F major chord (F, A, C)
speaker.async_play_note("harp", 1.0, 5)   -- F
speaker.async_play_note("harp", 1.0, 9)   -- A
speaker.async_play_note("harp", 1.0, 12)  -- C
```

### Example 4: Alarm

```lua
local speaker = peripheral.find("speaker")

for i = 1, 10 do
  speaker.async_play_note("bell", 3.0, 12)
  sleep(0.2)
  speaker.async_play_note("bell", 3.0, 0)
  sleep(0.2)
end
```

### Example 5: Pitch Sweep

```lua
local speaker = peripheral.find("speaker")

-- Sweep from low to high
for pitch = 0, 24 do
  speaker.async_play_note("flute", 1.0, pitch)
  sleep(0.1)
end
```

### Example 6: Volume Fade

```lua
local speaker = peripheral.find("speaker")

-- Fade in
for volume = 0, 10 do
  speaker.async_play_note("harp", volume / 10, 12)
  sleep(0.1)
end

-- Fade out
for volume = 10, 0, -1 do
  speaker.async_play_note("harp", volume / 10, 12)
  sleep(0.1)
end
```

---

## Error Handling

All methods may throw errors in the following cases:

- **Speaker not found**: Peripheral is disconnected
- **Invalid instrument**: Instrument name is not recognized
- **Invalid sound**: Sound resource name is invalid or too long (>512 chars)
- **Invalid parameters**: Volume or pitch values are out of range

**Example Error Handling:**
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

## Type Definitions

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
-- Range: 0.0 to 3.0
type Volume = number
```

### Pitch
```lua
-- Range: 0 to 24 (semitones)
type Pitch = number
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
