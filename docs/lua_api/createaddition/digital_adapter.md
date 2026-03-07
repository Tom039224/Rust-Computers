# DigitalAdapter

**mod**: createaddition  
**peripheral type**: `digital_adapter`  
**source**: `DigitalAdapterPeripheral.java`

## 概要

デジタルアダプター。Create mod の各種機械（ゲージ・プーリー・ピストン・ベアリング・エレベーター等）の状態読み取りと制御、およびディスプレイへのテキスト出力ができる汎用アダプター。

## メソッド一覧

### ディスプレイ操作

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `clearLine` | `line: number` | — | ✗ | 指定行をクリアする |
| `clear` | — | — | ✗ | 全行をクリアする |
| `print` | `text: string` | — | ✗ | 次の空き行にテキストを出力する |
| `getLine` | `line: number` | `string` | ✗ | 指定行のテキストを取得する |
| `setLine` | `line: number`, `text: string` | — | ✗ | 指定行にテキストを設定する |
| `getMaxLines` | — | `number` | ✗ | ディスプレイの最大行数を返す |

### キネティック制御

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `setTargetSpeed` | `dir: string`, `speed: number` | — | ✗ | 指定方向の機械の目標速度を設定する |
| `getTargetSpeed` | `dir: string` | `number` | ✗ | 指定方向の目標速度を取得する |
| `getKineticStress` | `dir: string` | `number` | ✗ | 指定方向の応力 (SU) を取得する |
| `getKineticCapacity` | `dir: string` | `number` | ✗ | 指定方向の応力容量 (SU) を取得する |
| `getKineticSpeed` | `dir: string` | `number` | ✗ | 指定方向の実際の速度 (RPM) を取得する |
| `getKineticTopSpeed` | — | `number` | ✗ | キネティックネットワークの最高速度 (RPM) を返す |

### 機械状態読み取り

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getPulleyDistance` | `dir: string` | `number` | ✗ | 指定方向のプーリーの伸長距離 (blocks) を返す |
| `getPistonDistance` | `dir: string` | `number` | ✗ | 指定方向のピストンの伸長距離 (blocks) を返す |
| `getBearingAngle` | `dir: string` | `number` | ✗ | 指定方向のベアリングの角度 (deg) を返す |
| `getElevatorFloor` | `dir: string` | `number` | ✗ | 指定方向のエレベーターの現在フロア番号を返す |
| `hasElevatorArrived` | `dir: string` | `boolean` | ✗ | 指定方向のエレベーターが目的フロアに到着したかを返す |
| `getElevatorFloors` | `dir: string` | `number` | ✗ | 指定方向のエレベーターの総フロア数を返す |
| `getElevatorFloorName` | `dir: string`, `index: number` | `string` | ✗ | 指定フロアの名前を返す |
| `gotoElevatorFloor` | `dir: string`, `index: number` | `number` | ✗ | 指定フロアへ移動し、Y 座標差分を返す |

### ユーティリティ

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getDurationAngle` | `degrees: number`, `rpm: number` | `number` | ✗ | 指定角度を指定 RPM で回転する所要秒数を返す |
| `getDurationDistance` | `blocks: number`, `rpm: number` | `number` | ✗ | 指定距離を指定 RPM で移動する所要秒数を返す |

## 備考

- `dir` 引数には `"north"`, `"south"`, `"east"`, `"west"`, `"up"`, `"down"` を使用する。
- `gotoElevatorFloor` の戻り値は移動する Y 座標の差分（移動量）。
