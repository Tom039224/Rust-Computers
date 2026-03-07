# FlapBearing

**mod**: Control-Craft  
**peripheral type**: `WingController`  
**source**: `FlapBearingPeripheral.java`

## 概要

フラップベアリング（翼制御）。ウィングコントローラーとして翼角度の設定とコントラプションの組み立て/分解を制御する。

## メソッド一覧

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getAngle` | — | `number` | ✓ | 現在の翼角度 (deg) を返す |
| `assembleNextTick` | — | — | ✗ | 次のティックにコントラプションを組み立てる |
| `disassembleNextTick` | — | — | ✗ | 次のティックにコントラプションを分解する |
| `setAngle` | `angle: number` | — | ✗ | 翼角度 (deg) を設定する |
