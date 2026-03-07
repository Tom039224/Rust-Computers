# DiskDrive

**mod**: CC:Tweaked  
**peripheral type**: `drive`  
**source**: `DiskDrivePeripheral.java`

## 概要

ディスクドライブブロックへの接続ペリフェラル。データディスクやオーディオディスクの操作を提供する。

## メソッド一覧

| メソッド名 | 引数 | 戻り値 | flags | imm | 説明 |
|---|---|---|---|---|---|
| `isDiskPresent` | — | `boolean` | — | ✓ | ディスクが挿入されているか返す |
| `getDiskLabel` | — | `string \| nil` | — | ✓ | ディスクのラベルを返す。なければ `nil` |
| `setDiskLabel` | `label?: string` | `nil` | mainThread | ✗ | ディスクのラベルを設定/削除する（`nil` で削除） |
| `hasData` | — | `boolean` | — | ✓ | データディスクが挿入されているか返す |
| `getMountPath` | — | `string \| nil` | — | ✓ | データディスクのマウントパスを返す |
| `hasAudio` | — | `boolean` | — | ✓ | オーディオディスクが挿入されているか返す |
| `getAudioTitle` | — | `string \| nil \| false` | — | ✓ | オーディオディスクのタイトルを返す。ディスク未挿入は `false`、オーディオなしは `nil` |
| `playAudio` | — | `nil` | — | ✗ | ディスクのオーディオを再生する |
| `stopAudio` | — | `nil` | — | ✗ | 再生中のオーディオを停止する |
| `ejectDisk` | — | `nil` | — | ✗ | ディスクを排出する |
| `getDiskID` | — | `number \| nil` | — | ✓ | ディスクの ID を返す。ID なしなら `nil` |
