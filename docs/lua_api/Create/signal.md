# Signal (Create_Signal)

鉄道信号機。列車の通行を制御し、閉塞区間の状態を取得する。

## Functions

### getState() -> string
- `mainThread = false`（アノテーションなし）
- 信号の状態を返す（"GREEN", "YELLOW_LONG", "YELLOW", "RED", "INVALID" など）。
- imm対応

### isForcedRed() -> boolean
- `mainThread = false`（アノテーションなし）
- 強制赤信号状態かどうかを返す（レッドストーン給電状態）。
- imm対応

### setForcedRed(powered: boolean)
- `mainThread = true`
- 強制赤信号を設定/解除する。

### listBlockingTrainNames() -> table
- `mainThread = false`（アノテーションなし）
- この信号を閉塞している列車名のリストを返す。
- imm対応

### getSignalType() -> string
- `mainThread = false`（アノテーションなし）
- 信号タイプを返す（"ENTRY_SIGNAL" または "CROSS_SIGNAL"）。
- imm対応

### cycleSignalType()
- `mainThread = true`
- 信号タイプをサイクルする（ENTRY ↔ CROSS）。

## Events

### train_signal_state_change
- 引数: `state: string`
- 信号の状態が変化したときに発火。
