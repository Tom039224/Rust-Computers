# Jet

**mod**: Control-Craft  
**peripheral type**: `attacker`  
**source**: `JetPeripheral.java`

## 概要

ジェット（推力機器）。VS2 ship にジェット推力を与え、水平・垂直チルト角度も制御できる。

## メソッド一覧

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `setOutputThrust` | `thrust: number` | — | ✗ | 出力推力スケールを設定する |
| `setHorizontalTilt` | `angle: number` | — | ✗ | 水平方向チルト角度 (deg) を設定する |
| `setVerticalTilt` | `angle: number` | — | ✗ | 垂直方向チルト角度 (deg) を設定する |
