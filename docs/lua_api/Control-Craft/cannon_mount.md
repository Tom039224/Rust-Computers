# CannonMount

**mod**: Control-Craft  
**peripheral type**: `controlcraft$cannon_mount`  
**source**: `CannonMountPeripheral.java`

## 概要

キャノンマウント。砲台（Valkyrien Skies / Create 系）のピッチ・ヨー角度を Lua から制御し、コントラプションの組み立て/分解を行うペリフェラル。

## メソッド一覧

| メソッド名 | 引数 | 戻り値 | flags | imm | 説明 |
|---|---|---|---|---|---|
| `setPitch` | `pitch: number` | — | — | ✗ | 砲台のピッチ角度 (deg) を設定する |
| `setYaw` | `yaw: number` | — | — | ✗ | 砲台のヨー角度 (deg) を設定する |
| `getPitch` | — | `number` | — | ✓ | 現在のピッチ角度 (deg) を返す |
| `getYaw` | — | `number` | — | ✓ | 現在のヨー角度 (deg) を返す |
| `assemble` | — | — | mainThread | ✗ | コントラプションを組み立てる |
| `disassemble` | — | — | mainThread | ✗ | コントラプションを分解する |

## 備考

- `assemble` / `disassemble` はサーバーメインスレッドで実行する必要がある。
