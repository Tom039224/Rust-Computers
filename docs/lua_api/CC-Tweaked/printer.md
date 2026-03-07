# Printer

**mod**: CC:Tweaked  
**peripheral type**: `printer`  
**source**: `PrinterPeripheral.java`

## 概要

プリンターブロックへの接続ペリフェラル。インクと用紙を消費して印刷物（書類）を生成する。

## メソッド一覧

| メソッド名 | 引数 | 戻り値 | flags | imm | 説明 |
|---|---|---|---|---|---|
| `write` | `text: string` | `nil` | — | ✗ | 現在のカーソル位置にテキストを書き込む |
| `getCursorPos` | — | `number, number` | — | ✓ | ページ上のカーソル位置 `(x, y)` を返す |
| `setCursorPos` | `x: number, y: number` | `nil` | — | ✗ | ページ上のカーソル位置を設定する |
| `getPageSize` | — | `number, number` | — | ✓ | 現在のページサイズ `(width, height)` を返す |
| `newPage` | — | `boolean` | mainThread | ✗ | 新しいページの印刷を開始する。インクや用紙が不足している場合は `false` |
| `endPage` | — | `boolean` | mainThread | ✗ | 現在のページを完了してトレイに出力する。成功すれば `true` |
| `setPageTitle` | `title?: string` | `nil` | — | ✗ | 現在のページのタイトルを設定する（`nil` でクリア） |
| `getInkLevel` | — | `number` | — | ✓ | 残りインク量を返す |
| `getPaperLevel` | — | `number` | — | ✓ | 残り用紙枚数を返す |

## 備考

- 印刷の流れ: `newPage()` → `setCursorPos()` / `write()` → `setPageTitle()` → `endPage()`。
- `endPage()` が成功すると、印刷物がトレイスロットに生成される。
- `mainThread = true` のメソッドはメインスレッドで実行される（ブロックエンティティへの同期アクセスが必要）。
