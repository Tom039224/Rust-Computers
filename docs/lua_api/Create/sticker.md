# Sticker (Create_Sticker)

スティッカー。コントラプションを接着・分離させるブロック。

## Functions

### isExtended() -> boolean
- `mainThread = false`（アノテーションなし）
- スティッカーが拡張（接着）状態かどうかを返す。
- imm対応

### isAttachedToBlock() -> boolean
- `mainThread = false`（アノテーションなし）
- 拡張状態で実際にブロックと接着しているかどうかを返す。
- imm対応

### extend() -> boolean
- `mainThread = true`
- スティッカーを拡張状態にする。既に拡張済みの場合は false を返す。

### retract() -> boolean
- `mainThread = true`
- スティッカーを収縮状態にする。既に収縮済みの場合は false を返す。

### toggle() -> boolean
- `mainThread = true`
- 拡張/収縮を切り替える。

## Events
なし
