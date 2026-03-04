# W-2: Java ↔ WASM ブリッジ詳細設計

## 概要

RustComputers の核となる Java と WASM の相互呼び出し機構を詳細に設計します。

**確定事項**:
- ✅ 固定 Shared Buffer 方式（案①）採用
- ✅ すべての干渉結果は Future として返却（`parallel!` に `.await` **なし**で渡す）
- ✅ Timeout + Fuel 上限で悪意あるプログラム対策
- ✅ Feature ベース mod 検出関数を用意

---

## 1. ホスト関数の種類と役割

### 1.1 情報取得系（Read API）

```
[Rust] → host_request_info() で Java にリクエスト登録
             ↓ (GT:N → GT:N+1 遅れ)
         [Java] 情報を取得してバッファに書き込み
             ↓
         host_poll_result() で Java に結果確認
         → Result.Ok(データ) or Result.Err(エラー)
```

**シグネチャ**:
```rust
/// 情報取得リクエストを Java に登録
/// 戻り値: request_id (>0) or エラーコード(<0)
#[no_mangle]
pub extern fn host_request_info(
    peripheral_id: u32,      // ペリフェラルID（1tick前に登録済み）
    method_id: u16,           // メソッドID（@LuaFunction の ID）
    arg_count: u16,           // 引数個数
    args_ptr: i32,            // SHARED_BUFFER 内のポインタ（引数データ）
) -> i32;

/// リクエストの完了状態と結果を確認
/// 戻り値: >0 = 結果バッファアドレス（Ready）
///         =0 = 未完了（pending）
///        <0 = エラーコード（Java側エラー）
#[no_mangle]
pub extern fn host_poll_result(
    request_id: i32,
) -> i32;
```

### 1.2 ワールド干渉系（Write API）

```
[Rust] → host_do_action() で干渉指示を Java に送る
             ↓ (GT:N → GT:N+1)
         [Java] ワールド干渉を実行して結果を保存
             ↓
         host_poll_result() で結果確認
         → Result.Ok(成功) or Result.Err(失敗原因)
```

**シグネチャ**:
```rust
/// ワールド干渉リクエストを Java に登録
/// 戻り値: request_id (>0) or エラーコード(<0)
/// ※ Write API も Read と同じ polling パターン
#[no_mangle]
pub extern fn host_do_action(
    peripheral_id: u32,
    method_id: u16,
    arg_count: u16,
    args_ptr: i32,
) -> i32;
```

### 1.3 情報取得不要の即時干渉（Fire・結果確認なし）

**廃止**: すべて Future に統一

```rust
// Before: Fire and forget
// host_fire_and_forget(peripheral_id, method_id, args_ptr)

// After: parallel! で一括制御（引数は Future、.await なし）
parallel!(
    action1.do_action(...),
    action2.do_action(...),
    action3.do_action(...),
)
```

---

## 2. Shared Buffer 設計

### 2.1 固定サイズバッファ（案①採用）

```
┌────────────────────────────────────────────────────────────┐
│ SHARED_BUFFER (Java: ByteBuffer, Rust: [u8; 65536])        │
├────────────────────────────────────────────────────────────┤
│ [0x00000] Request Argument Area (16KB)                      │  ← Write
│           - 複数ペリフェラルの引数（オーバーラップ可）      │
│           - Java が読む                                   │
├────────────────────────────────────────────────────────────┤
│ [0x04000] Result Area 1 (8KB)                              │  ← Read
│           - request_id: 1～N の結果格納                    │
│           - Rust が読む                                   │
├────────────────────────────────────────────────────────────┤
│ [0x06000] Result Area 2 (8KB)                              │
│           - ダブルバッファ（スワップ可能）                 │
├────────────────────────────────────────────────────────────┤
│ [0x08000] Metadata Area (8KB)                              │
│           - Request State Table                             │
│           - timestamp, status_flags                         │
│           - 予約領域                                      │
├────────────────────────────────────────────────────────────┤
│ [0x0A000] 予約・高速メモリ (22KB)                           │
│           - 変動領域、未使用                               │
└────────────────────────────────────────────────────────────┘
 Total: 65536 bytes (64 KB)
```

### 2.2 引数セリアライゼーション

```rust
// SHARED_BUFFER[:16KB] に以下形式で記録

// [Header] 4 bytes
// ┌─────────────────────────────────────┐
// │ request_id (u32)                    │
// └─────────────────────────────────────┘
//
// [Arg Count] 2 bytes
// ┌─────────────────────────────────────┐
// │ arg_count (u16)                     │
// └─────────────────────────────────────┘
//
// [Args] 可変長（1 arg = 4～16 bytes）
// ┌─────────────────────────────────────┐
// │ arg[0]: i32 / f32 / bool            │ 4 bytes
// │ arg[1]: string -> (offset, len)     │ 8 bytes
// │ arg[2]: table -> (offset, len)      │ 8 bytes
// │ ...                                │
// └─────────────────────────────────────┘
//
// [String/Table Data] 可変長
// ┌─────────────────────────────────────┐
// │ "hello world" (UTF-8)               │ 11 bytes
// │ <padding>                           │
// └─────────────────────────────────────┘
```

**Rust helper 関数**:

```rust
pub struct SharedBuffer {
    // SHARED_BUFFER への不変参照
}

impl SharedBuffer {
    /// 引数領域を初期化
    pub fn write_args(
        &mut self,
        request_id: u32,
        args: &[Arg],
    ) -> Result<usize> {
        // 引数を serialized format で記録
        // 戻り値: 使用バイト数
    }

    /// 結果領域から結果を読む
    pub fn read_result(&self, result_ptr: i32, max_len: usize) -> Result<Vec<u8>> {
        // result_ptr 位置から max_len bytes 読む
    }

    /// 結果領域に書く（Java側）
    pub fn write_result(
        &mut self,
        request_id: u32,
        data: &[u8],
    ) -> i32 {
        // 結果を記録、ポインタ返す
    }
}
```

---

## 3. Request ID 管理

### 3.1 ID 生成戦略（シンプル インクリメント）

```java
class RequestManager {
    private int nextRequestId = 1;              // 初期値 1
    private static final int REQUEST_ID_MAX = Integer.MAX_VALUE;
    
    private Map<Integer, PendingRequest> pending = new HashMap<>();
    
    public synchronized int allocateRequestId(Request req) throws Exception {
        if (nextRequestId >= REQUEST_ID_MAX) {
            // ラップアラウンド（稀）
            nextRequestId = 1;
        }
        
        int id = nextRequestId++;
        pending.put(id, new PendingRequest(req));
        
        return id;
    }
    
    public synchronized PendingRequest pollResult(int requestId) {
        return pending.get(requestId);
    }
    
    public synchronized void removeRequest(int requestId) {
        pending.remove(requestId);
    }
}
```

**特性**:
- スレッドセーフ（Minecraft は multi-threaded）
- メモリ効率的（8 bytes per pending request）
- Timeout チェック可能

---

## 4. エラーハンドリング

### 4.1 エラーコード体系

```java
public class HostApiError {
    // 戻り値エラーコード
    public static final int OK = 0;
    public static final int PENDING = 1;
    
    // エラー code (負数)
    public static final int ERR_INVALID_REQUEST_ID = -1;
    public static final int ERR_INVALID_PERIPHERAL = -2;
    public static final int ERR_METHOD_NOT_FOUND = -3;
    public static final int ERR_JAVA_EXCEPTION = -4;
    public static final int ERR_TIMEOUT = -5;
    public static final int ERR_FUEL_EXHAUSTED = -6;
    public static final int ERR_BUFFER_OVERFLOW = -7;
    public static final int ERR_MOD_NOT_AVAILABLE = -8;
    public static final int ERR_RESULT_LOST = -9;  // GC'd
}
```

### 4.2 Java 側での例外処理

```java
public int hostRequestInfo(int periphId, int methodId, int argCount, int argsPtr) {
    try {
        // 1. ペリフェラル存在確認
        IPeripheral periph = PeripheralRegistry.get(periphId);
        if (periph == null) {
            return HostApiError.ERR_INVALID_PERIPHERAL;
        }
        
        // 2. メソッド存在確認
        PeripheralMethod method = MethodRegistry.get(methodId);
        if (method == null) {
            return HostApiError.ERR_METHOD_NOT_FOUND;
        }
        
        // 3. バッファ読み込み
        byte[] args = readFromSharedBuffer(argsPtr, argCount * 4);
        if (args == null) {
            return HostApiError.ERR_BUFFER_OVERFLOW;
        }
        
        // 4. リクエスト登録
        int requestId = requestMgr.allocateRequestId(
            new JavaRequest(periph, method, args)
        );
        
        return requestId;
    } catch (Exception e) {
        log.error("hostRequestInfo failed", e);
        return HostApiError.ERR_JAVA_EXCEPTION;
    }
}
```

### 4.3 Rust 側での error handling

```rust
use std::result::Result;

#[derive(Debug)]
pub enum BridgeError {
    InvalidRequestId,
    Timeout,
    FuelExhausted,
    JavaException(i32),  // error code
    BufferOverflow,
    Unknown,
}

impl From<i32> for BridgeError {
    fn from(code: i32) -> Self {
        match code {
            -1 => BridgeError::InvalidRequestId,
            -5 => BridgeError::Timeout,
            -6 => BridgeError::FuelExhausted,
            -4 => BridgeError::JavaException(code),
            -7 => BridgeError::BufferOverflow,
            _ => BridgeError::Unknown,
        }
    }
}

pub async fn get_info<T>(
    peripheral: &PeripheralHandle<T>,
    method: u16,
    args: &[u8],
) -> Result<Vec<u8>, BridgeError> {
    let request_id = unsafe {
        host_request_info(
            peripheral.id,
            method,
            args.len() as u16,
            args.as_ptr() as i32,
        )
    };
    
    if request_id < 0 {
        return Err(BridgeError::from(request_id));
    }
    
    // Poll loop with timeout
    let start = get_tick();
    loop {
        let result_addr = unsafe {
            host_poll_result(request_id)
        };
        
        match result_addr {
            addr if addr > 0 => {
                // Ready
                return Ok(read_buffer(addr));
            }
            0 => {
                // Pending
                if get_tick() - start > MAX_TICK_WAIT {
                    return Err(BridgeError::Timeout);
                }
                yield_to_java().await;
            }
            error_code => {
                return Err(BridgeError::from(error_code));
            }
        }
    }
}
```

---

## 5. Thread Safety & Synchronization

### 5.1 Minecraft のスレッド構成

```
┌──────────────────────────────────────────────────────────┐
│ Main Server Thread (ServerTickEvent)                     │
│  - ワールド更新                                          │
│  - Java リクエスト処理                                   │
│  - WASM poll() 呼び出し注意（ブロッキング）              │
└──────────────────────────────────────────────────────────┘
        ↓
┌──────────────────────────────────────────────────────────┐
│ WASM Worker Thread Pool (Optional)                       │
│  - 複数コンピューター parallel poll                      │
│  - 各 thread は独立した WASM Instance                   │
└──────────────────────────────────────────────────────────┘
```

### 5.2 Request State Table（Thread-safe state machine）

```java
class RequestState {
    enum Status {
        PENDING,      // リクエスト登録済み、未処理
        PROCESSING,   // Java 側で処理中
        READY,        // 結果準備完了
        CONSUMED,     // Rust が読み込み済み
        EXPIRED,      // timeout or GC
    }
    
    Status status;
    long createdAt;
    byte[] result;
    Throwable error;
    
    public synchronized boolean trySetReady(byte[] resultData) {
        if (status == Status.PROCESSING) {
            this.result = resultData;
            this.status = Status.READY;
            return true;
        }
        return false;
    }
    
    public synchronized byte[] tryConsume() {
        if (status == Status.READY) {
            this.status = Status.CONSUMED;
            return result;
        }
        return null;
    }
}
```

### 5.3 Shared Buffer Access Synchronization

```java
class SharedBuffer {
    private final ByteBuffer buffer = ByteBuffer.allocateDirect(65536);
    private final Object argLock = new Object();
    private final Object resultLock = new Object();
    
    public synchronized int writeArgs(int argCount, byte[] data) throws IOException {
        // 引数領域を独占的にアクセス
        synchronized(argLock) {
            buffer.position(ARG_AREA_OFFSET);
            buffer.put(data);
            return ARG_AREA_OFFSET;
        }
    }
    
    public synchronized int writeResult(int requestId, byte[] data) {
        // 結果領域を独占的にアクセス
        synchronized(resultLock) {
            int offset = RESULT_AREA_OFFSET + (requestId % RESULT_SLOTS) * RESULT_SLOT_SIZE;
            buffer.position(offset);
            buffer.put(data);
            return offset;
        }
    }
}
```

---

## 6. Timeout & Fuel Limits

### 6.1 Timeout 戦略

```java
// ServerTickEvent 内で定期的に timeout チェック
public class TickWatcher {
    private static final long MAX_REQUEST_AGE_MILLIS = 5000;  // 5 秒
    private static final int MAX_REQUEST_AGE_TICKS = 100;     // ≈ 5 秒
    
    public void checkTimeouts(int currentTick) {
        for (Map.Entry<Integer, RequestState> entry : requestMgr.pending.entrySet()) {
            int requestId = entry.getKey();
            RequestState state = entry.getValue();
            
            if (currentTick - state.createdAtTick > MAX_REQUEST_AGE_TICKS) {
                // Timeout!
                state.status = Status.EXPIRED;
                state.error = new TimeoutException("Request " + requestId);
                requestMgr.removeRequest(requestId);
            }
        }
    }
}
```

### 6.2 Fuel Limit（WASM execution cost）

**Fuel**: WASM の命令実行 cost。過度な計算を防ぐ。

```rust
// Rust 側 (ユーザープログラム)
#[entry]
pub async fn main() {
    loop {
        // Fuel は Tick ごとにリセット
        // 例: 1 Tick = 10,000,000 Fuel
        
        let data = get_sensor_data().await;  // ← Fuel 消費
        
        // Fuel が尽きると WasmTrap 発生
        // → Java 側で ERR_FUEL_EXHAUSTED を返す
    }
}
```

```java
// Java 側
Config config = new Config();
config.fuelConsumption(true);

Engine engine = new Engine(config);
Store<Void> store = new Store<>(engine);

// インスタンス生成時に Fuel 上限を設定
Instance instance = linker.instantiate(module);

// Tick ごとに fuel を補充
long FUEL_PER_TICK = 10_000_000L;
long refueled = store.refuel(FUEL_PER_TICK);
if (refueled < FUEL_PER_TICK) {
    log.warn("Fuel refuel incomplete: {} < {}", refueled, FUEL_PER_TICK);
}

// Poll 実行
try {
    pollFunc.call();
} catch (WasmTrap trap) {
    if (trap.message().contains("fuel")) {
        // Fuel exhausted
        requestMgr.markError(currentComputerId, HostApiError.ERR_FUEL_EXHAUSTED);
    }
}
```

---

## 7. Feature-based Mod Detection

### 7.1 初期化時の mod 確認関数

```java
public class ModAvailability {
    public static boolean isComputerCraftAvailable() {
        try {
            Class.forName("dan200.computercraft.api.peripheral.IPeripheral");
            return true;
        } catch (ClassNotFoundException e) {
            return false;
        }
    }
    
    public static boolean isSomePeripheralsAvailable() {
        try {
            Class.forName("net.spaceeye.someperipherals.api.SomePeripheralsAPI");
            return true;
        } catch (ClassNotFoundException e) {
            return false;
        }
    }
    
    public static boolean isCCVSAvailable() {
        try {
            Class.forName("io.github.techtastic.cc_vs.apis.ShipAPI");
            return true;
        } catch (ClassNotFoundException e) {
            return false;
        }
    }
    
    public static boolean isControlCraftAvailable() {
        try {
            Class.forName("com.verr1.controlcraft.content.blocks.jet.JetBlockEntity");
            return true;
        } catch (ClassNotFoundException e) {
            return false;
        }
    }
}
```

### 7.2 WASM 側での確認 API

```rust
// Rust WASM 側が初期化時に確認

#[no_mangle]
pub extern fn host_is_mod_available(mod_id: u16) -> u32 {
    // 0 = not available, 1 = available
    match mod_id {
        MOD_ID_COMPUTER_CRAFT => 1,      // always
        MOD_ID_SOME_PERIPHERALS => 1 or 0,
        MOD_ID_CC_VS => 1 or 0,
        MOD_ID_CONTROL_CRAFT => 1 or 0,
        _ => 0,
    }
}

// rust_computers crate
pub async fn check_mod(mod_id: u16) -> bool {
    unsafe {
        host_is_mod_available(mod_id) != 0
    }
}

#[entry]
pub async fn main() {
    // Startup: どの mod が available か確認
    if check_mod(MOD_ID_SOME_PERIPHERALS).await {
        // Feature: some_peripherals が有効
        initialize_radar().await;
    }
    
    if check_mod(MOD_ID_CC_VS).await {
        // Feature: cc_vs が有効
        initialize_ship_api().await;
    }
}
```

### 7.3 Cargo feature と連携

```toml
# rust_computers/Cargo.toml

[features]
default = ["computer_craft"]
computer_craft = []        # 常に有効（CC:Tweaked は必須）
some_peripherals = []
cc_vs = []
control_craft = []
all = ["computer_craft", "some_peripherals", "cc_vs", "control_craft"]

[dependencies]
# Conditional dependencies で実装
```

```rust
// lib.rs: feature gate で予約関数定義

#[cfg(feature = "some_peripherals")]
pub mod some_peripherals {
    pub use ...
}

#[cfg(feature = "cc_vs")]
pub mod cc_vs {
    pub use ...
}
```

---

## 8. Poll Loop の実装イメージ

### 8.1 Rust 側

```rust
#[entry]
pub async fn main() {
    let monitor = PeripheralHandle::new(MONITOR_PERIPH_ID);
    
    loop {
        //複数の async 操作を並列待機
        let (light, temp, humidity) = parallel!(
            get_light(&monitor),
            get_temperature(&sensor),
            get_humidity(&sensor),
        );
        
        // データ処理
        if light < 5 {
            do_action(&lamp, SET_ON, &[]).await;  // lamp on
        }
        
        // 次の Tick へ（最小 1 Tick）
        sleep(1).await;  // ← internal: yield_to_java()
    }
}

async fn get_light(periph: &PeripheralHandle<Monitor>) -> Result<i32> {
    // 1. Request to Java (GT:N)
    let request_id = unsafe {
        host_request_info(periph.id, METHOD_GET_LIGHT, 0, 0)
    };
    
    // 2. Poll loop (GT:N → GT:N+1 遅れ)
    loop {
        let result_addr = unsafe {
            host_poll_result(request_id)
        };
        
        match result_addr {
            addr if addr > 0 => {
                // Ready: 結果バッファから reads
                let result: i32 = read_i32(addr);
                return Ok(result);
            }
            0 => {
                // Pending: yield
                yield_to_java().await;
            }
            error_code => {
                return Err(BridgeError::from(error_code));
            }
        }
    }
}
```

### 8.2 Java 側

```java
public class WasmComputerTick {
    // Tick ごと実行
    public void update(int tickNumber) {
        // 1. Timeout チェック
        tickWatcher.checkTimeouts(tickNumber);
        
        // 2. 前 tick のリクエスト結果を処理
        processResults();
        
        // 3. Rust poll() を実行
        try {
            pollFunc.call();
        } catch (WasmTrap trap) {
            handleTrap(trap);
        }
        
        // 4. Fuel refuel
        store.refuel(FUEL_PER_TICK);
    }
    
    private void processResults() {
        for (RequestState state : requestMgr.getProcessingStates()) {
            try {
                // Java 側でメソッド実行
                Object result = state.method.invoke(
                    state.peripheral,
                    deserializeArgs(state.args)
                );
                
                // 結果を SHARED_BUFFER に書き込み
                byte[] serialized = serializeResult(result);
                int resultAddr = sharedBuffer.writeResult(state.requestId, serialized);
                
                // State を Ready に
                state.trySetReady(serialized);
                
            } catch (Exception e) {
                state.setError(e);
            }
        }
    }
}
```

---

## 9. 実装チェックリスト（W-2 詳細設計）

- [ ] W-2-1: Request ID 管理（シンプルインクリメント）確定
- [ ] W-2-2: Shared Buffer 65536 bytes 固定 確定
- [ ] W-2-3: メモリ割り当て案①（固定）確定、案②への移行パス記載
- [ ] W-2-4: エラーハンドリング（-9 エラーコード体系）確定
- [ ] W-2-5: Thread Safety（synchronized state machine）確定
- [ ] W-2-6: Timeout（MAX_AGE_TICKS=100）& Fuel 上限（10M/Tick）確定
- [ ] W-2-7: Feature-based mod detection (`host_is_mod_available()`) 確定
- [ ] Java 実装スケルトン作成
- [ ] Rust 実装スケルトン作成

---

## 10. 次フェーズへの依存項目

### W-2 → W-3 (@LuaFunction 自動生成)

- Method ID 割り当てスキーム
  - CC:Tweaked の @LuaFunction (name + static Method object) から ID を生成
  - ID のハッシュ or インデックス

### W-2 → Implementation Phase

- `rust_computers` クレート骨組み
- `WasmComputerImpl` クラス実装
- Module.fromBytes() + Instance pool 管理

