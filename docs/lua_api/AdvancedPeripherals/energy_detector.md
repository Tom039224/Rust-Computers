# EnergyDetector

**mod**: AdvancedPeripherals  
**peripheral type**: `energy_detector`  
**source**: `EnergyDetectorPeripheral.java`

## 概要

エネルギー検出器。FE（Forge Energy）の転送量・転送レート制限をモニタリング・制御するペリフェラル。

## メソッド一覧

| メソッド名 | 引数 | 戻り値 | imm | 説明 |
|---|---|---|---|---|
| `getTransferRate` | — | `number` | ✗ | 現在の 1 tick あたりエネルギー転送量 (FE/t) を返す |
| `getTransferRateLimit` | — | `number` | ✗ | 設定されている転送レート制限 (FE/t) を返す |
| `setTransferRateLimit` | `rate: number` | — | ✗ | 転送レート上限 (FE/t) を設定する |
