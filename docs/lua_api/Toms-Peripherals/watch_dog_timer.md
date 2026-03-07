# WatchDogTimer

**mod**: Toms-Peripherals  
**peripheral type**: `tm_wdt`  
**source**: `WatchDogTimerPeripheral.java`

## 概要

ウォッチドッグタイマー。指定した ticks 以内に `reset()` が呼ばれない場合、コンピューターを自動再起動させる監視タイマー。プログラムのハング検出に使用する。

## メソッド一覧

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `isEnabled` | — | `boolean` | ✓ | ウォッチドッグが有効かどうかを返す |
| `getTimeout` | — | `number` | ✓ | 現在のタイムアウト値 (ticks) を返す |
| `setEnabled` | `enabled: boolean` | — | ✗ | ウォッチドッグの有効/無効を切り替える |
| `setTimeout` | `ticks: number` | — | ✗ | タイムアウト値 (ticks) を設定する |
| `reset` | — | — | ✗ | タイマーをリセットする（ハング検出をリセット） |

## 使用例

```lua
local wdt = peripheral.find("tm_wdt")
wdt.setTimeout(100)   -- 5 秒（20 ticks/秒）のタイムアウト
wdt.setEnabled(true)

while true do
  -- メイン処理
  doSomething()
  wdt.reset()  -- 定期的にリセットしないとコンピューターが再起動される
  sleep(1)
end
```

## 備考

- タイムアウトが発生すると CC コンピューターが再起動（`reboot`）される。
- `setTimeout` の引数は game tick 単位（20 ticks = 1 秒）。
