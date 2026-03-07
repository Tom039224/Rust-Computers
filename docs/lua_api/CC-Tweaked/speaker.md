# Speaker

**mod**: CC:Tweaked  
**peripheral type**: `speaker`  
**source**: `SpeakerPeripheral.java`

## 概要

スピーカーブロックへの接続ペリフェラル。ノート音・サウンドイベント・PCMオーディオの再生が可能。

## メソッド一覧

| メソッド名 | 引数 | 戻り値 | flags | imm | 説明 |
|---|---|---|---|---|---|
| `playNote` | `instrument: string, volume?: number, pitch?: number` | `boolean` | — | ✗ | 指定楽器でノート音を再生する。音量 0.0〜3.0、ピッチ 0.5〜2.0。再生できれば `true` |
| `playSound` | `name: string, volume?: number, pitch?: number` | `boolean` | — | ✗ | サウンドイベントを再生する（例: `"minecraft:entity.creeper.primed"`）。再生できれば `true` |
| `playAudio` | `audio: table, volume?: number` | `boolean` | — | ✗ | PCMオーディオデータ（-128〜127 の整数テーブル）を再生する。`unsafe=true` で `LuaTable` を直接受け取る。再生できれば `true` |
| `stop` | — | `nil` | — | ✗ | 再生中のオーディオを停止する |

## 備考

- `instrument` に指定できる値: `"harp"`, `"basedrum"`, `"snare"`, `"hat"`, `"bass"`, `"flute"`, `"bell"`, `"guitar"`, `"chime"`, `"xylophone"`, `"iron_xylophone"`, `"cow_bell"`, `"didgeridoo"`, `"bit"`, `"banjo"`, `"pling"` など（ノートブロックのサウンド名）。
- `playAudio` は dfpwm エンコードされた PCM データに対応。1 回の呼び出しで最大 128 KB を送信できる。
- 1 ゲームティックあたりの再生回数に上限がある（スパム防止）。
