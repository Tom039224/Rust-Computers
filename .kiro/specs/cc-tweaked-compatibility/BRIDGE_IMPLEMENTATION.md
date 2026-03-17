# Bridge Implementation Guide: Java ↔ Rust Communication

## Overview

The Java-Rust bridge enables RustComputers to access Minecraft peripherals through CC:Tweaked's IPeripheral interface. This document details the implementation strategy for both sides.

## Current Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Minecraft World                          │
│  ┌──────────────────────────────────────────────────────┐   │
│  │         CC:Tweaked Peripherals (IPeripheral)        │   │
│  │  - Inventory, Modem, Monitor, Speaker, etc.         │   │
│  │  - AdvancedPeripherals, Create, etc.                │   │
│  └──────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                            ↑
                            │ Java Reflection
                            │
┌─────────────────────────────────────────────────────────────┐
│                    Java Bridge Layer                        │
│  ┌──────────────────────────────────────────────────────┐   │
│  │         PeripheralProvider                          │   │
│  │  - Peripheral discovery & registration             │   │
│  │  - Method invocation                                │   │
│  │  - Event monitoring                                 │   │
│  └──────────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────────┐   │
│  │         Request/Result Buffers                      │   │
│  │  - book_next_* → request buffer                     │   │
│  │  - read_last_* ← result buffer                      │   │
│  │  - Event queue                                      │   │
│  └──────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                            ↑
                            │ FFI (MessagePack)
                            │
┌─────────────────────────────────────────────────────────────┐
│                    Rust API Layer                           │
│  ┌──────────────────────────────────────────────────────┐   │
│  │         Peripheral Wrappers                         │   │
│  │  - book_next_*() → book_request()                   │   │
│  │  - read_last_*() ← read_result()                    │   │
│  │  - async_*() → book + wait + read                   │   │
│  └──────────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────────┐   │
│  │         User Code                                   │   │
│  │  - async fn main() { ... }                          │   │
│  └──────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

## Java Side Implementation

### 1. Peripheral Discovery & Registration

**File**: `forge/src/main/java/com/rustcomputers/peripheral/PeripheralProvider.java`

```java
public class PeripheralProvider {
    // Peripheral registry: Block → PeripheralType supplier
    private static final Map<Block, Supplier<PeripheralType>> REGISTRY = new HashMap<>();
    
    // Attached peripherals: PeriphAddr → AttachedPeripheral
    private static final Map<Integer, AttachedPeripheral> ATTACHED = new HashMap<>();
    
    public static void register(Block block, Supplier<PeripheralType> supplier) {
        REGISTRY.put(block, supplier);
    }
    
    public static Map<Integer, AttachedPeripheral> scanAdjacent(ServerLevel level, BlockPos computerPos) {
        Map<Integer, AttachedPeripheral> result = new HashMap<>();
        
        // Scan adjacent blocks
        for (Direction dir : Direction.values()) {
            BlockPos pos = computerPos.relative(dir);
            BlockState state = level.getBlockState(pos);
            Block block = state.getBlock();
            
            if (REGISTRY.containsKey(block)) {
                PeripheralType type = REGISTRY.get(block).get();
                IPeripheral peripheral = type.create(level, pos, state);
                
                if (peripheral != null) {
                    AttachedPeripheral attached = new AttachedPeripheral(
                        dir.ordinal(),
                        type.getName(),
                        peripheral
                    );
                    result.put(dir.ordinal(), attached);
                }
            }
        }
        
        return result;
    }
}
```

### 2. Request/Result Buffer Management

**File**: `forge/src/main/java/com/rustcomputers/peripheral/PeripheralRequestBuffer.java`

```java
public class PeripheralRequestBuffer {
    // Pending requests: PeriphAddr → List<PendingRequest>
    private final Map<Integer, List<PendingRequest>> requests = new HashMap<>();
    
    // Cached results: PeriphAddr → Map<MethodName, Result>
    private final Map<Integer, Map<String, Object>> results = new HashMap<>();
    
    // Action results: PeriphAddr → Map<MethodName, List<Result>>
    private final Map<Integer, Map<String, List<Object>>> actionResults = new HashMap<>();
    
    public void bookRequest(int periphId, String methodName, Object[] args) {
        requests.computeIfAbsent(periphId, k -> new ArrayList<>())
            .add(new PendingRequest(methodName, args));
    }
    
    public void bookAction(int periphId, String methodName, Object[] args) {
        // Actions accumulate (not overwritten)
        requests.computeIfAbsent(periphId, k -> new ArrayList<>())
            .add(new PendingRequest(methodName, args, true));  // isAction=true
    }
    
    public void executeAll(Map<Integer, AttachedPeripheral> attached) {
        for (Map.Entry<Integer, List<PendingRequest>> entry : requests.entrySet()) {
            int periphId = entry.getKey();
            List<PendingRequest> reqs = entry.getValue();
            AttachedPeripheral periph = attached.get(periphId);
            
            if (periph == null) continue;
            
            for (PendingRequest req : reqs) {
                try {
                    Object result = periph.invoke(req.methodName, req.args);
                    
                    if (req.isAction) {
                        // Accumulate action results
                        actionResults.computeIfAbsent(periphId, k -> new HashMap<>())
                            .computeIfAbsent(req.methodName, k -> new ArrayList<>())
                            .add(result);
                    } else {
                        // Overwrite query results
                        results.computeIfAbsent(periphId, k -> new HashMap<>())
                            .put(req.methodName, result);
                    }
                } catch (Exception e) {
                    // Store error
                    storeError(periphId, req.methodName, e, req.isAction);
                }
            }
        }
        
        // Clear requests for next tick
        requests.clear();
    }
    
    public Object getResult(int periphId, String methodName) {
        return results.getOrDefault(periphId, new HashMap<>()).get(methodName);
    }
    
    public List<Object> getActionResults(int periphId, String methodName) {
        return actionResults.getOrDefault(periphId, new HashMap<>())
            .getOrDefault(methodName, new ArrayList<>());
    }
}
```

### 3. Event Monitoring Framework

**File**: `forge/src/main/java/com/rustcomputers/peripheral/EventMonitor.java`

```java
public class EventMonitor {
    // Event listeners: PeriphAddr → List<EventListener>
    private final Map<Integer, List<EventListener>> listeners = new HashMap<>();
    
    // Event queue: PeriphAddr → Map<EventName, Queue<Event>>
    private final Map<Integer, Map<String, Queue<Object>>> eventQueues = new HashMap<>();
    
    public void registerListener(int periphId, String eventName) {
        listeners.computeIfAbsent(periphId, k -> new ArrayList<>())
            .add(new EventListener(eventName));
    }
    
    public void pollEvents(Map<Integer, AttachedPeripheral> attached) {
        for (Map.Entry<Integer, List<EventListener>> entry : listeners.entrySet()) {
            int periphId = entry.getKey();
            List<EventListener> eventListeners = entry.getValue();
            AttachedPeripheral periph = attached.get(periphId);
            
            if (periph == null) continue;
            
            for (EventListener listener : eventListeners) {
                try {
                    // Poll for events (implementation depends on peripheral type)
                    Object event = periph.pollEvent(listener.eventName);
                    
                    if (event != null) {
                        eventQueues.computeIfAbsent(periphId, k -> new HashMap<>())
                            .computeIfAbsent(listener.eventName, k -> new LinkedList<>())
                            .offer(event);
                    }
                } catch (Exception e) {
                    // Log error
                }
            }
        }
    }
    
    public List<Object> getEvents(int periphId, String eventName) {
        Queue<Object> queue = eventQueues.getOrDefault(periphId, new HashMap<>())
            .getOrDefault(eventName, new LinkedList<>());
        
        List<Object> result = new ArrayList<>(queue);
        queue.clear();
        return result;
    }
}
```

### 4. FFI Interface

**File**: `forge/src/main/java/com/rustcomputers/wasm/PeripheralFFI.java`

```java
public class PeripheralFFI {
    private final PeripheralRequestBuffer requestBuffer;
    private final EventMonitor eventMonitor;
    private final Map<Integer, AttachedPeripheral> attached;
    
    // Called from Rust: book_request(periph_id, method_id, args)
    public void bookRequest(int periphId, int methodId, byte[] args) {
        String methodName = methodIdToName(methodId);
        Object[] decodedArgs = decodeArgs(args);
        requestBuffer.bookRequest(periphId, methodName, decodedArgs);
    }
    
    // Called from Rust: book_action(periph_id, method_id, args)
    public void bookAction(int periphId, int methodId, byte[] args) {
        String methodName = methodIdToName(methodId);
        Object[] decodedArgs = decodeArgs(args);
        requestBuffer.bookAction(periphId, methodName, decodedArgs);
    }
    
    // Called from Rust: read_result(periph_id, method_id)
    public byte[] readResult(int periphId, int methodId) {
        String methodName = methodIdToName(methodId);
        Object result = requestBuffer.getResult(periphId, methodName);
        return encodeResult(result);
    }
    
    // Called from Rust: read_action_results(periph_id, method_id)
    public byte[] readActionResults(int periphId, int methodId) {
        String methodName = methodIdToName(methodId);
        List<Object> results = requestBuffer.getActionResults(periphId, methodName);
        return encodeResults(results);
    }
    
    // Called from Rust: book_event(periph_id, event_id)
    public void bookEvent(int periphId, int eventId) {
        String eventName = eventIdToName(eventId);
        eventMonitor.registerListener(periphId, eventName);
    }
    
    // Called from Rust: read_events(periph_id, event_id)
    public byte[] readEvents(int periphId, int eventId) {
        String eventName = eventIdToName(eventId);
        List<Object> events = eventMonitor.getEvents(periphId, eventName);
        return encodeEvents(events);
    }
    
    // Called from Minecraft tick
    public void tick() {
        requestBuffer.executeAll(attached);
        eventMonitor.pollEvents(attached);
    }
}
```

## Rust Side Implementation

### 1. Request Booking

**File**: `crates/rust-computers-api/src/peripheral.rs`

```rust
pub fn book_request(addr: impl Into<PeriphAddr>, method_name: &str, args: &[u8]) {
    let addr = addr.into();
    let method_id = method_id(method_name);
    
    // Call Java FFI
    unsafe {
        ffi::book_request(addr.raw(), method_id, args.as_ptr(), args.len() as u32);
    }
}

pub fn book_action(addr: impl Into<PeriphAddr>, method_name: &str, args: &[u8]) {
    let addr = addr.into();
    let method_id = method_id(method_name);
    
    // Call Java FFI
    unsafe {
        ffi::book_action(addr.raw(), method_id, args.as_ptr(), args.len() as u32);
    }
}
```

### 2. Result Reading

**File**: `crates/rust-computers-api/src/peripheral.rs`

```rust
pub fn read_result(
    addr: impl Into<PeriphAddr>,
    method_name: &str,
) -> Result<Vec<u8>, PeripheralError> {
    let addr = addr.into();
    let method_id = method_id(method_name);
    
    // Call Java FFI to get result size
    let size = unsafe {
        ffi::read_result_size(addr.raw(), method_id)
    };
    
    if size < 0 {
        return Err(PeripheralError::ExecutionFailed("Result not ready".to_string()));
    }
    
    // Allocate buffer and read result
    let mut buffer = vec![0u8; size as usize];
    unsafe {
        ffi::read_result(addr.raw(), method_id, buffer.as_mut_ptr(), size as u32);
    }
    
    Ok(buffer)
}

pub fn read_action_results(
    addr: impl Into<PeriphAddr>,
    method_name: &str>,
) -> Vec<Result<Vec<u8>, PeripheralError>> {
    let addr = addr.into();
    let method_id = method_id(method_name);
    
    // Call Java FFI to get results
    let size = unsafe {
        ffi::read_action_results_size(addr.raw(), method_id)
    };
    
    let mut results = Vec::new();
    for i in 0..size {
        let result_size = unsafe {
            ffi::read_action_result_size(addr.raw(), method_id, i)
        };
        
        if result_size < 0 {
            results.push(Err(PeripheralError::ExecutionFailed("Action failed".to_string())));
        } else {
            let mut buffer = vec![0u8; result_size as usize];
            unsafe {
                ffi::read_action_result(addr.raw(), method_id, i, buffer.as_mut_ptr(), result_size as u32);
            }
            results.push(Ok(buffer));
        }
    }
    
    results
}
```

### 3. Event Booking and Reading

**File**: `crates/rust-computers-api/src/peripheral.rs`

```rust
pub fn book_event(addr: impl Into<PeriphAddr>, event_name: &str) {
    let addr = addr.into();
    let event_id = event_id(event_name);
    
    unsafe {
        ffi::book_event(addr.raw(), event_id);
    }
}

pub fn read_events(
    addr: impl Into<PeriphAddr>,
    event_name: &str>,
) -> Vec<Option<Vec<u8>>> {
    let addr = addr.into();
    let event_id = event_id(event_name);
    
    // Get number of events
    let count = unsafe {
        ffi::read_events_count(addr.raw(), event_id)
    };
    
    let mut events = Vec::new();
    for i in 0..count {
        let size = unsafe {
            ffi::read_event_size(addr.raw(), event_id, i)
        };
        
        if size < 0 {
            events.push(None);
        } else {
            let mut buffer = vec![0u8; size as usize];
            unsafe {
                ffi::read_event(addr.raw(), event_id, i, buffer.as_mut_ptr(), size as u32);
            }
            events.push(Some(buffer));
        }
    }
    
    events
}
```

### 4. Peripheral Wrapper Implementation

**File**: `crates/rust-computers-api/src/{mod_name}/{peripheral}.rs`

```rust
pub struct SomePeripheral {
    addr: PeriphAddr,
}

impl SomePeripheral {
    pub fn book_next_get_data(&mut self) {
        peripheral::book_request(self.addr, "getData", &[]);
    }
    
    pub fn read_last_get_data(&self) -> Result<Data, PeripheralError> {
        let bytes = peripheral::read_result(self.addr, "getData")?;
        decode::<Data>(&bytes)
    }
    
    pub async fn async_get_data(&self) -> Result<Data, PeripheralError> {
        self.book_next_get_data();
        wait_for_next_tick().await;
        self.read_last_get_data()
    }
    
    pub fn book_next_receive_event(&mut self) {
        peripheral::book_event(self.addr, "receiveEvent");
    }
    
    pub fn read_last_receive_event(&self) -> Vec<Option<Event>> {
        let events = peripheral::read_events(self.addr, "receiveEvent");
        events.into_iter()
            .map(|opt| opt.and_then(|bytes| decode::<Event>(&bytes).ok()))
            .collect()
    }
    
    pub async fn async_receive_event(&self) -> Result<Event, PeripheralError> {
        loop {
            self.book_next_receive_event();
            wait_for_next_tick().await;
            
            if let Some(event) = self.read_last_receive_event()
                .into_iter()
                .next()
                .flatten()
            {
                return Ok(event);
            }
        }
    }
}
```

## Tick Execution Flow

```
┌─────────────────────────────────────────────────────────────┐
│ Minecraft Tick N                                            │
├─────────────────────────────────────────────────────────────┤
│ 1. Rust: book_next_*() → RequestBuffer                      │
│ 2. Rust: wait_for_next_tick().await → FFI flush            │
│ 3. Java: PeripheralFFI.tick()                              │
│    - Execute all booked requests                           │
│    - Poll events                                           │
│    - Store results in buffers                              │
│ 4. Rust: read_last_*() ← ResultBuffer                      │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│ Minecraft Tick N+1                                          │
├─────────────────────────────────────────────────────────────┤
│ 1. Rust: read_last_*() ← ResultBuffer (from Tick N)        │
│ 2. Rust: book_next_*() → RequestBuffer                      │
│ 3. Rust: wait_for_next_tick().await → FFI flush            │
│ 4. Java: PeripheralFFI.tick()                              │
│    - Execute all booked requests                           │
│    - Poll events                                           │
│    - Store results in buffers                              │
│ 5. Rust: read_last_*() ← ResultBuffer                      │
└─────────────────────────────────────────────────────────────┘
```

## Error Handling

### Java Side

```java
try {
    Object result = periph.invoke(methodName, args);
    // Store result
} catch (LuaException e) {
    // Store error message
    storeError(periphId, methodName, e.getMessage());
} catch (Exception e) {
    // Store generic error
    storeError(periphId, methodName, "Execution failed: " + e.getMessage());
}
```

### Rust Side

```rust
pub fn read_result(
    addr: impl Into<PeriphAddr>,
    method_name: &str>,
) -> Result<Vec<u8>, PeripheralError> {
    let addr = addr.into();
    let method_id = method_id(method_name);
    
    let size = unsafe {
        ffi::read_result_size(addr.raw(), method_id)
    };
    
    match size {
        -1 => Err(PeripheralError::NotFound),
        -2 => Err(PeripheralError::MethodNotFound),
        -3 => Err(PeripheralError::ExecutionFailed("Unknown error".to_string())),
        s if s < 0 => Err(PeripheralError::ExecutionFailed(format!("Error code: {}", s))),
        s => {
            // Read result
            let mut buffer = vec![0u8; s as usize];
            unsafe {
                ffi::read_result(addr.raw(), method_id, buffer.as_mut_ptr(), s as u32);
            }
            Ok(buffer)
        }
    }
}
```

## Performance Optimization

### 1. Batch Processing

- All `book_next_*()` calls are accumulated
- Single FFI call at `wait_for_next_tick().await`
- Reduces FFI overhead

### 2. Result Caching

- Results are cached in Java side
- Rust side reads from cache without FFI call
- Reduces round-trip latency

### 3. Event Queuing

- Events are queued in Java side
- Rust side reads from queue
- Prevents event loss

## Testing Strategy

### Java Side Tests

```java
@Test
public void testBookRequest() {
    PeripheralRequestBuffer buffer = new PeripheralRequestBuffer();
    buffer.bookRequest(0, "getData", new Object[]{});
    
    assertEquals(1, buffer.getPendingRequests(0).size());
}

@Test
public void testExecuteRequest() {
    // Mock peripheral
    AttachedPeripheral periph = mock(AttachedPeripheral.class);
    when(periph.invoke("getData", new Object[]{})).thenReturn("result");
    
    // Execute
    buffer.executeAll(Map.of(0, periph));
    
    // Verify result
    assertEquals("result", buffer.getResult(0, "getData"));
}
```

### Rust Side Tests

```rust
#[test]
fn test_book_request() {
    let addr = PeriphAddr::from_raw(0);
    peripheral::book_request(addr, "getData", &[]);
    // Verify request was booked
}

#[tokio::test]
async fn test_async_get_data() {
    let periph = SomePeripheral::new(PeriphAddr::from_raw(0));
    let result = periph.async_get_data().await;
    // Verify result
}
```

---

**Last Updated**: 2025-03-15
**Version**: 1.0
