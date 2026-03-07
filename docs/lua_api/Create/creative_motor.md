# CreativeMotor (Create_CreativeMotor)

クリエイティブモーター。指定した回転速度で動力を供給する。

## Functions

### setGeneratedSpeed(speed: number)
- `mainThread = true`
- 生成する回転速度を設定する（RPM）。負の値で逆回転。

### getGeneratedSpeed() -> number
- `mainThread = false`（アノテーションなし）
- 現在設定されている生成速度を返す。
- imm対応

## Events
なし
