# ModularAccumulator

**mod**: createaddition  
**peripheral type**: `modular_accumulator`  
**source**: `ModularAccumulatorPeripheral.java`

## 概要

モジュラーアキュムレーター（FE バッテリー）。複数ブロックを積み重ねてエネルギー容量を拡張できるマルチブロック型エネルギー蓄積装置。

## メソッド一覧

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getEnergy` | — | `number` | ✗ | 現在の蓄積エネルギー (FE) を返す |
| `getCapacity` | — | `number` | ✗ | 最大蓄積容量 (FE) を返す |
| `getPercent` | — | `number` | ✗ | 充電率 (0.0〜100.0 %) を返す |
| `getMaxInsert` | — | `number` | ✗ | 1 tick あたりの最大入力 (FE/t) を返す |
| `getMaxExtract` | — | `number` | ✗ | 1 tick あたりの最大出力 (FE/t) を返す |
| `getHeight` | — | `number` | ✗ | マルチブロックの高さ（ブロック数）を返す |
| `getWidth` | — | `number` | ✗ | マルチブロックの幅（ブロック数）を返す |
