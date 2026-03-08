# cbc_cannon_mount

**mod**: cbc_cc_control  
**peripheral type**: `cbc_cannon_mount`  
**source**: `CompactCannonMountMethods.java`  
**author**: HaruYoshiharu (Andesilsk)

## 概要

Create Big Cannons (CBC) のキャノンマウントを CC:Tweaked から制御するためのペリフェ
ラル。砲台のピッチ・ヨー角度の読み取り・設定、コントラプションの組み立て・分解、発射
を行う。

## メソッド一覧

| メソッド名 | 引数 | 戻り値 | flags | imm | 説明 |
|---|---|---|---|---|---|
| `assemble` | — | `boolean` | mainThread | ✗ | コントラプションを組み立てる。成功すると `true` を返す |
| `disassemble` | — | — | mainThread | ✗ | コントラプションを分解する |
| `isRunning` | — | `boolean` | — | ✓ | コントラプションが稼働中かどうかを返す |
| `getYaw` | — | `number` | mainThread | ✓ | 現在のヨー角度 (deg) を返す |
| `getPitch` | — | `number` | mainThread | ✓ | 現在のピッチ角度 (deg) を返す |
| `setYaw` | `yaw: number` | — | mainThread | ✗ | ヨー角度 (deg) を設定する |
| `setPitch` | `pitch: number` | — | mainThread | ✗ | ピッチ角度 (deg) を設定する |
| `fire` | — | — | — | ✗ | 発射する（`onRedstoneUpdate` を呼び出す） |

## 内部実装メモ

- `assemble`: `tryUpdatingSpeed()` → `assemble()` → `sendData()` → `isRunning()` を返す
- `disassemble`: `disassemble()` → `sendData()`
- `getYaw`: `getYawOffset()` を返す
- `getPitch`: `getDisplayPitch()` または `getPitchOffset()` を返す
- `fire`: `onRedstoneUpdate()` を呼び出す
- `mainThread=true` のメソッドはサーバーメインスレッドで実行される

## 備考

- `assemble` / `disassemble` / `setYaw` / `setPitch` はメインスレッド実行が必要。
- `fire` は Redstone 信号の模倣（`onRedstoneUpdate`）で発射を行う。
- ペリフェラルは `CannonMountBlockEntity` に対して Forge Capability 経由で登録される。
