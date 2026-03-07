# CompactFlap

**mod**: Control-Craft  
**peripheral type**: `compact_flap`  
**source**: `CompactFlapPeripheral.java`

## 概要

コンパクトフラップ。コンパクトサイズのフラップ（制御翼面）。角度とチルトを個別に制御できる。

## メソッド一覧

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getAngle` | — | `number` | ✓ | 現在のフラップ角度 (deg) を返す |
| `setAngle` | `angle: number` | — | ✗ | フラップ角度 (deg) を設定する |
| `getTilt` | — | `number` | ✓ | 現在のチルト角度 (deg) を返す |
| `setTilt` | `tilt: number` | — | ✗ | チルト角度 (deg) を設定する |
