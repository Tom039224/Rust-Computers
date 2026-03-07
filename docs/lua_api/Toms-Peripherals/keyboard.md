# Keyboard

**mod**: Toms-Peripherals  
**peripheral type**: `tm_keyboard`  
**source**: `KeyboardPeripheral.java`

## 概要

キーボードペリフェラル。プレイヤーのキー入力をイベントとして受信する。ネイティブイベント転送の有効/無効を切り替えられる。

## メソッド一覧

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `setFireNativeEvents` | `enabled: boolean` | — | ✗ | ネイティブキー入力イベントの発火を有効/無効にする |

## イベント

| イベント名 | パラメーター | 説明 |
|---|---|---|
| `tm_keyboard_key` | `keyCode, scanCode, isRepeat` | キーを押したとき |
| `tm_keyboard_key_up` | `keyCode, scanCode` | キーを離したとき |
| `tm_keyboard_char` | `char` | 文字キーを入力したとき |

## 備考

- `keyCode` は GLFW キーコード（CC-Tweaked の `keys` テーブルと互換）。
- `setFireNativeEvents(true)` にすると通常の CC キーイベント（`key`, `char` 等）も発火する。
