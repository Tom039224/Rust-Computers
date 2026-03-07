# Speedometer (Create_Speedometer)

回転速度計。接続された回転機構の速度をモニターする。

## Functions

### getSpeed() -> number
- `mainThread = false`（アノテーションなし）
- 現在の回転速度（RPM）を返す。過負荷時は 0。
- imm対応

## Events

### speed_change
- 引数: `speed: number`（過負荷時は 0）
- 回転速度が変化したときに発火。
