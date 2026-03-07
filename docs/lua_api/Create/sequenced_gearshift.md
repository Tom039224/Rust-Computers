# SequencedGearshift (Create_SequencedGearshift)

シーケンスドギアシフト。指定角度または距離だけ回転を伝達する。

## Functions

### rotate(amount: number, speedModifier?: number)
- `mainThread = true`
- 指定した角度（度）だけ回転させる。speedModifier は速度係数（デフォルト 1）。

### move(amount: number, speedModifier?: number)
- `mainThread = true`
- 指定した距離（ブロック単位）だけ移動させる。

### isRunning() -> boolean
- `mainThread = false`（アノテーションなし）
- シーケンス実行中かどうかを返す。
- imm対応

## Events
なし
