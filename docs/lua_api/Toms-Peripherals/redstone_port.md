# RedstonePort

**mod**: Toms-Peripherals  
**peripheral type**: `tm_rsPort`  
**source**: `RedstonePortPeripheral.java`

## 概要

レッドストーンポート。バンドル入出力を含む高機能レッドストーン制御ペリフェラル。CC-Tweaked 標準の `redstone` API より多くの機能を提供する。

## メソッド一覧

### 入力読み取り

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getSides` | — | `string[]` | ✓ | 利用可能な方向一覧を返す |
| `getInput` | `side: string` | `boolean` | ✗ | 指定方向のデジタル入力を返す |
| `getAnalogInput` | `side: string` | `number` | ✗ | 指定方向のアナログ入力値 (0〜15) を返す |
| `getAnalogueInput` | `side: string` | `number` | ✗ | `getAnalogInput` と同義（英国スペル） |
| `getBundledInput` | `side: string` | `number` | ✗ | 指定方向のバンドル入力マスクを返す |

### 出力読み取り

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getOutput` | `side: string` | `boolean` | ✓ | 指定方向のデジタル出力を返す |
| `getAnalogOutput` | `side: string` | `number` | ✓ | 指定方向のアナログ出力値 (0〜15) を返す |
| `getAnalogueOutput` | `side: string` | `number` | ✓ | `getAnalogOutput` と同義（英国スペル） |
| `getBundledOutput` | `side: string` | `number` | ✓ | 指定方向のバンドル出力マスクを返す |

### 出力設定

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `setOutput` | `side: string`, `value: boolean` | — | ✗ | 指定方向のデジタル出力を設定する |
| `setAnalogOutput` | `side: string`, `value: number` | — | ✗ | 指定方向のアナログ出力値 (0〜15) を設定する |
| `setAnalogueOutput` | `side: string`, `value: number` | — | ✗ | `setAnalogOutput` と同義（英国スペル） |
| `setBundledOutput` | `side: string`, `mask: number` | — | ✗ | 指定方向のバンドル出力マスクを設定する |

### ユーティリティ

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `testBundledInput` | `side: string`, `mask: number` | `boolean` | ✗ | バンドル入力が指定マスクと一致するかを返す |

## イベント

| イベント名 | パラメーター | 説明 |
|---|---|---|
| `tm_redstone` | `side` | レッドストーン信号が変化したとき |

## 備考

- `side` には `"north"`, `"south"`, `"east"`, `"west"`, `"up"`, `"down"` を使用。
- バンドル出力マスクは 16 ビット整数（各ビットが色に対応）。
