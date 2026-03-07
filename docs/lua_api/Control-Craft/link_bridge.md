# LinkBridge

**mod**: Control-Craft  
**peripheral type**: `cc_link_bridge`  
**source**: `LinkBridgePeripheral.java`

## 概要

リンクブリッジ。インデックス付きの入出力値を介して、別のシステムとの数値データ連携を行うブリッジペリフェラル。

## メソッド一覧

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `setInput` | `index: number`, `value: number` | — | ✗ | 指定インデックスの入力値を設定する |
| `getOutput` | `index: number` | `number` | ✓ | 指定インデックスの出力値を取得する |
