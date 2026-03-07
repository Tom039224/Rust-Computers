# ElectricMotor

**mod**: createaddition  
**peripheral type**: `electric_motor`  
**source**: `ElectricMotorPeripheral.java`

## 概要

電気モーター。FE（Forge Energy）を消費して Create mod のキネティックネットワークに回転力を供給する。

## メソッド一覧

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getType` | — | `string` | ✓ | ペリフェラルタイプ (`"electric_motor"`) を返す |
| `setSpeed` | `speed: number` | — | ✗ | モーターの速度 (RPM) を設定する（符号で方向） |
| `stop` | — | — | ✗ | モーターを停止する |
| `getSpeed` | — | `number` | ✗ | 現在の速度 (RPM) を返す |
| `getStressCapacity` | — | `number` | ✗ | 現在の応力容量 (SU) を返す |
| `getEnergyConsumption` | — | `number` | ✗ | 現在の FE 消費量 (FE/t) を返す |
| `rotate` | `degrees: number`, `rpm?: number` | `number` | ✗ | 指定角度 (deg) を指定回転数で回転させ、所要秒数を返す |
| `translate` | `distance: number`, `rpm?: number` | `number` | ✗ | 指定距離 (blocks) を指定回転数で移動させ、所要秒数を返す |
| `getMaxInsert` | — | `number` | ✗ | 1 tick あたりの最大 FE 入力量を返す |
| `getMaxExtract` | — | `number` | ✗ | 1 tick あたりの最大 FE 出力量を返す |

## 備考

- `rotate` / `translate` は内部的にキネティックネットワークを通じて他の機械（プーリー・ピストン等）を動作させる。
- `setSpeed` の引数が正なら前進、負なら逆転方向。
