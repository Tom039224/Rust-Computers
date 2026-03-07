# WiredModem

**mod**: CC:Tweaked  
**peripheral type**: `modem` + `peripheral_hub`  
**source**: `WiredModemPeripheral.java`

## 概要

有線モデム（Wired Modem）ペリフェラル。[Modem](./modem.md) の全メソッドに加え、有線ネットワーク上のリモートペリフェラルを操作するメソッドを提供する。

## 固有メソッド（Modem に追加）

| メソッド名 | 引数 | 戻り値 | flags | imm | 説明 |
|---|---|---|---|---|---|
| `getNamesRemote` | — | `{ string... }` | — | ✓ | ネットワーク上のリモートペリフェラル名一覧を返す（自分自身は除く） |
| `isPresentRemote` | `name: string` | `boolean` | — | ✓ | 指定名のペリフェラルがネットワーク上に存在するか返す |
| `getTypeRemote` | `name: string` | `string \| nil` | — | ✓ | リモートペリフェラルのタイプを返す（存在しなければ `nil`） |
| `hasTypeRemote` | `name: string, type: string` | `boolean \| nil` | — | ✓ | リモートペリフェラルが特定タイプを持つか返す（存在しなければ `nil`） |
| `getMethodsRemote` | `name: string` | `{ string... } \| nil` | — | ✓ | リモートペリフェラルのメソッド名一覧を返す（存在しなければ `nil`） |
| `callRemote` | `remoteName: string, method: string, ...: any` | `any` | — | ✗ | ネットワーク上のリモートペリフェラルのメソッドを呼び出す |
| `getNameLocal` | — | `string \| nil` | — | ✓ | このコンピューターの有線ネットワーク名を返す（モデムがオフなら `nil`） |
