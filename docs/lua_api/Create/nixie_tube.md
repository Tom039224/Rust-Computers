# NixieTube (Create_NixieTube)

ニクシー管。カスタムテキストや色を表示できる。コンピューターが接続されると既存のテキストがクリアされ、切断するとレッドストーン表示に戻る。

## Functions

### setText(text: string, colour?: string)
- `mainThread = true`
- テキストを設定する。colour は DyeColor 名（"white", "red", "blue", "grey" など）。

### setTextColour(colour: string)
- `mainThread = true`
- テキスト色を変更する（UK/US どちらのスペリングでも可）。

### setTextColor(color: string)
- `mainThread = true`
- `setTextColour` のエイリアス。

### setSignal(front: table, back?: table)
- `mainThread = true`
- 各管の発光シグナルを直接設定する。テーブルのフィールド：
  - `r: number (0-255)`
  - `g: number (0-255)`
  - `b: number (0-255)`
  - `glowWidth: number (1-4)`
  - `glowHeight: number (1-4)`
  - `blinkPeriod: number (0-255)` — 0 で点滅なし
  - `blinkOffTime: number (0-255)`

## Events
なし
