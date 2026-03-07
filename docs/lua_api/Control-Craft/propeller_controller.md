# PropellerController

**mod**: Control-Craft  
**peripheral type**: `PropellerController`  
**source**: `PropellerControllerPeripheral.java`

## 概要

プロペラコントローラー。プロペラの目標回転速度を設定・取得する。

## メソッド一覧

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `setTargetSpeed` | `speed: number` | — | ✗ | プロペラの目標速度 (RPM) を設定する |
| `getTargetSpeed` | — | `number` | ✓ | 現在の目標速度 (RPM) を返す |
