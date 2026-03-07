# Monitor

**mod**: CC:Tweaked  
**peripheral type**: `monitor`  
**source**: `MonitorPeripheral.java`, `TermMethods.java`

## 概要

モニターブロックへの接続ペリフェラル。`TermMethods` の全メソッドを継承し、テキスト描画・カラー操作などターミナル API を提供する。加えてモニター固有の `setTextScale` / `getTextScale` を持つ。

## 継承関係

```
TermMethods  ←  MonitorPeripheral  (+  setTextScale / getTextScale)
```

TermMethods のメソッドについては [term_methods.md](./term_methods.md) を参照。

## 固有メソッド

| メソッド名 | 引数 | 戻り値 | flags | imm | 説明 |
|---|---|---|---|---|---|
| `setTextScale` | `scale: number` | `nil` | — | ✗ | モニターのテキストスケールを設定する (0.5 〜 5.0、0.5 刻み) |
| `getTextScale` | — | `number` | — | ✓ | 現在のテキストスケールを返す |

## 備考

- モニターに `peripheral.wrap("monitor_0")` 等で接続し、`term.redirect()` に渡して通常の描画 API を使う用途が主。
- カラーモニター（Advanced Monitor）は isColour() が true を返す。
