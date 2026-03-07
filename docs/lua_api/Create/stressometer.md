# Stressometer (Create_Stressometer)

ストレスメーター。ネットワーク全体のストレスと容量をモニターする。

## Functions

### getStress() -> number
- `mainThread = false`（アノテーションなし）
- 現在のネットワークストレスを返す（SU）。
- imm対応

### getStressCapacity() -> number
- `mainThread = false`（アノテーションなし）
- ネットワークの最大ストレス容量を返す（SU）。
- imm対応

## Events

### overstressed
- 引数: なし
- ネットワークが過負荷になったときに発火。

### stress_change
- 引数: `stress: number`, `capacity: number`
- ストレス量が変化したときに発火（過負荷でない場合）。
