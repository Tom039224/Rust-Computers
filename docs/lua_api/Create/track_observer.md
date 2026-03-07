# TrackObserver (Create_TrainObserver)

線路オブザーバー。列車の通過を検知する。

## Functions

### isTrainPassing() -> boolean
- `mainThread = false`（アノテーションなし）
- 現在列車が通過中かどうかを返す。
- imm対応

### getPassingTrainName() -> string | nil
- `mainThread = false`（アノテーションなし）
- 通過中の列車名を返す。列車がいなければ nil。
- imm対応

## Events

### train_passing
- 引数: `name: string`（列車名）
- 列車が通過を開始したときに発火。

### train_passed
- 引数: `name: string`（列車名）
- 列車が通過し終えたときに発火。
