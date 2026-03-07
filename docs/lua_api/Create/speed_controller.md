# RotationSpeedController (Create_RotationSpeedController)

回転速度コントローラー。歯車ネットワークの目標速度を制御する。

## Functions

### setTargetSpeed(speed: number)
- `mainThread = true`
- 目標回転速度を設定する（RPM）。負の値で逆回転。

### getTargetSpeed() -> number
- `mainThread = false`（アノテーションなし）
- 現在の目標速度を返す。
- imm対応

## Events
なし
