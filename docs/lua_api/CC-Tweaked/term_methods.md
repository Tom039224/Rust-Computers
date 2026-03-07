# TermMethods (共通ターミナル API)

**mod**: CC:Tweaked  
**peripheral type**: (*MonitorPeripheral が継承*)  
**source**: `TermMethods.java`

## 概要

`term.Redirect` 互換のターミナル操作メソッド群。Monitor ペリフェラルが継承して使用する。  
コンピューター本体の `term` API と同一インターフェースを持つ。

## メソッド一覧

| メソッド名 | 引数 | 戻り値 | flags | imm | 説明 |
|---|---|---|---|---|---|
| `write` | `text: string` | `nil` | — | ✗ | 現在のカーソル位置にテキストを書き込む（任意の値を文字列に変換） |
| `scroll` | `y: number` | `nil` | — | ✗ | 画面を y 行分スクロールする（負値で逆方向） |
| `getCursorPos` | — | `number, number` | — | ✓ | カーソル位置 `(x, y)` を返す |
| `setCursorPos` | `x: number, y: number` | `nil` | — | ✗ | カーソル位置を設定する |
| `getCursorBlink` | — | `boolean` | — | ✓ | カーソルが点滅中かを返す |
| `setCursorBlink` | `blink: boolean` | `nil` | — | ✗ | カーソルの点滅を有効/無効にする |
| `getSize` | — | `number, number` | — | ✓ | 端末サイズ `(width, height)` を文字単位で返す |
| `clear` | — | `nil` | — | ✗ | 端末全体を現在の背景色でクリアする |
| `clearLine` | — | `nil` | — | ✗ | カーソル行を現在の背景色でクリアする |
| `getTextColour` / `getTextColor` | — | `number` | — | ✓ | 現在のテキスト色を返す (colors 定数) |
| `setTextColour` / `setTextColor` | `colour: number` | `nil` | — | ✗ | テキスト色を設定する |
| `getBackgroundColour` / `getBackgroundColor` | — | `number` | — | ✓ | 現在の背景色を返す |
| `setBackgroundColour` / `setBackgroundColor` | `colour: number` | `nil` | — | ✗ | 背景色を設定する |
| `isColour` / `isColor` | — | `boolean` | — | ✓ | カラー表示をサポートしているか（Advanced のみ `true`） |
| `blit` | `text: string, textColour: string, bgColour: string` | `nil` | — | ✗ | テキストと前景/背景色を一括書き込み（各文字列は同じ長さ、16進1文字ずつ） |
| `setPaletteColour` / `setPaletteColor` | `index: number, colour: number` または `index: number, r: number, g: number, b: number` | `nil` | — | ✗ | パレット色を変更する（24-bit 整数または RGB 0〜1） |
| `getPaletteColour` / `getPaletteColor` | `colour: number` | `number, number, number` | — | ✓ | パレット色の現在の RGB 値を返す (各 0〜1) |

## 備考

- `blit` の `textColour`/`bgColour` は16進1文字ずつ（`colors` 定数のビット位置に対応）。  
  例: `"0"` = white, `"f"` = black（`colors.white = 2^0` なので index 0）。
- `setPaletteColour` は 2 引数だと 24-bit RGB 整数、4 引数だと (index, r, g, b)。
