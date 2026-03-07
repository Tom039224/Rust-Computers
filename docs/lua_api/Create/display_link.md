# DisplayLink (Create_DisplayLink)

ディスプレイリンク。接続されたディスプレイブロックへテキストを出力する。モニター互換の一部 API を実装している。

## Functions

### setCursorPos(x: number, y: number)
- `mainThread = false`（アノテーションなし）
- カーソル位置を設定する（1-indexed）。

### getCursorPos() -> number, number
- `mainThread = false`（アノテーションなし）
- 現在のカーソル位置 (x, y) を返す（1-indexed）。
- imm対応

### getSize() -> number, number
- `mainThread = true`
- ディスプレイの最大行数・列数を返す。
- mainThread=true のためimm非対応

### isColor() -> boolean
### isColour() -> boolean
- `mainThread = false`（アノテーションなし）
- 常に `false` を返す（カラー非対応）。
- imm対応

### write(text: string)
- `mainThread = false`（アノテーションなし）
- カーソル位置にテキストを書き込む。

### writeBytes(data: string | table)
- `mainThread = false`（アノテーションなし）
- バイト配列または文字列をテキストとして書き込む。

### clearLine()
- `mainThread = false`（アノテーションなし）
- カーソル行をクリアする。

### clear()
- `mainThread = false`（アノテーションなし）
- 全内容をクリアする。

### update()
- `mainThread = true`
- ディスプレイに変更を反映させる（手動更新）。

## Events
なし
