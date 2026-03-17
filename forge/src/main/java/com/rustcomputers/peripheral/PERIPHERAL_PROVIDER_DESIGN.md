# PeripheralProvider Extension Design

## Overview

This document describes the extension of `PeripheralProvider` to support:
1. Event monitoring framework
2. Request/result buffer management
3. Multiple request/multiple response handling
4. book_next/read_last/async pattern support

## Current Architecture

```
PeripheralProvider (existing)
├── Block → PeripheralType registry
├── Adjacent peripheral scanning (periph_id 0-5)
├── Wired modem network scanning (periph_id 6+)
└── Type-based peripheral search
```

## Extended Architecture

```
PeripheralProvider (extended)
├── Block → PeripheralType registry (existing)
├── Adjacent peripheral scanning (existing)
├── Wired modem network scanning (existing)
├── Type-based peripheral search (existing)
└── NEW: PeripheralRequestManager
    ├── RequestBuffer (query & action requests)
    ├── ResultBuffer (cached results)
    ├── EventMonitor (event listeners & queue)
    └── Tick execution (executeAll)
```

## New Components

### 1. PeripheralRequestManager

**File**: `forge/src/main/java/com/rustcomputers/peripheral/PeripheralRequestManager.java`

**Responsibilities**:
- Manage request/result buffers per computer
- Execute pending requests on tick
- Monitor events
- Provide FFI interface for Rust

**Lifecycle**:
```
Computer Start → Create PeripheralRequestManager
Computer Tick  → executeAll() + pollEvents()
Computer Stop  → Cleanup buffers
```

### 2. RequestBuffer

**File**: `forge/src/main/java/com/rustcomputers/peripheral/RequestBuffer.java`

**Structure**:
```java
public class RequestBuffer {
    // Query requests: periph_id → method_name → latest request
    private final Map<Integer, Map<String, PendingRequest>> queryRequests;
    
    // Action requests: periph_id → method_name → list of requests
    private final Map<Integer, Map<String, List<PendingRequest>>> actionRequests;
    
    public void bookQuery(int periphId, String methodName, Object[] args);
    public void bookAction(int periphId, String methodName, Object[] args);
    public void clear();
}
```

**Behavior**:
- **Query requests**: Latest request overwrites previous (information retrieval)
- **Action requests**: All requests are accumulated (world interaction)

### 3. ResultBuffer

**File**: `forge/src/main/java/com/rustcomputers/peripheral/ResultBuffer.java`

**Structure**:
```java
public class ResultBuffer {
    // Query results: periph_id → method_name → single result
    private final Map<Integer, Map<String, Object>> queryResults;
    
    // Action results: periph_id → method_name → list of results
    private final Map<Integer, Map<String, List<Object>>> actionResults;
    
    // Error storage
    private final Map<Integer, Map<String, PeripheralError>> errors;
    
    public void storeQueryResult(int periphId, String methodName, Object result);
    public void storeActionResult(int periphId, String methodName, Object result);
    public void storeError(int periphId, String methodName, String error, boolean isAction);
    
    public Object getQueryResult(int periphId, String methodName);
    public List<Object> getActionResults(int periphId, String methodName);
    public void clear();
}
```

### 4. EventMonitor

**File**: `forge/src/main/java/com/rustcomputers/peripheral/EventMonitor.java`

**Structure**:
```java
public class EventMonitor {
    // Event listeners: periph_id → event_name → listener
    private final Map<Integer, Map<String, EventListener>> listeners;
    
    // Event queue: periph_id → event_name → queue of events
    private final Map<Integer, Map<String, Queue<Object>>> eventQueues;
    
    public void registerListener(int periphId, String eventName);
    public void pollEvents(Map<Integer, AttachedPeripheral> peripherals);
    public List<Object> getEvents(int periphId, String eventName);
    public void clearEvents(int periphId, String eventName);
}
```

**Event Types**:
```java
public enum EventType {
    MODEM_MESSAGE("modem_message"),
    MONITOR_TOUCH("monitor_touch"),
    CHAT("chat"),
    PLAYER_JOIN("playerJoin"),
    PLAYER_LEAVE("playerLeave"),
    KEY("key"),
    KEY_UP("key_up"),
    TRAIN_PASS("train_pass");
    
    private final String eventName;
    
    EventType(String eventName) {
        this.eventName = eventName;
    }
    
    public String getEventName() {
        return eventName;
    }
}
```

### 5. PendingRequest

**File**: `forge/src/main/java/com/rustcomputers/peripheral/PendingRequest.java`

**Structure**:
```java
public record PendingRequest(
    String methodName,
    Object[] args,
    boolean isAction
) {}
```

### 6. PeripheralError

**File**: `forge/src/main/java/com/rustcomputers/peripheral/PeripheralError.java`

**Structure**:
```java
public record PeripheralError(
    ErrorType type,
    String message
) {
    public enum ErrorType {
        NOT_FOUND,
        METHOD_NOT_FOUND,
        INVALID_ARGUMENT,
        EXECUTION_FAILED,
        DISCONNECTED,
        TIMEOUT,
        SERIALIZATION_ERROR
    }
}
```

## FFI Interface

### Method ID Mapping

**File**: `forge/src/main/java/com/rustcomputers/peripheral/MethodRegistry.java`

```java
public class MethodRegistry {
    // method_name → method_id
    private static final Map<String, Integer> METHOD_IDS = new ConcurrentHashMap<>();
    
    // method_id → method_name
    private static final Map<Integer, String> METHOD_NAMES = new ConcurrentHashMap<>();
    
    private static int nextId = 0;
    
    public static int register(String methodName) {
        return METHOD_IDS.computeIfAbsent(methodName, name -> {
            int id = nextId++;
            METHOD_NAMES.put(id, name);
            return id;
        });
    }
    
    public static String getName(int methodId) {
        return METHOD_NAMES.get(methodId);
    }
    
    public static int getId(String methodName) {
        return METHOD_IDS.getOrDefault(methodName, -1);
    }
}
```

### FFI Methods

**File**: `forge/src/main/java/com/rustcomputers/wasm/PeripheralFFI.java`

```java
public class PeripheralFFI {
    private final PeripheralRequestManager requestManager;
    
    // Called from Rust: book_request(periph_id, method_id, args_msgpack)
    public void bookRequest(int periphId, int methodId, byte[] args) {
        String methodName = MethodRegistry.getName(methodId);
        Object[] decodedArgs = MessagePackDecoder.decode(args);
        requestManager.bookQuery(periphId, methodName, decodedArgs);
    }
    
    // Called from Rust: book_action(periph_id, method_id, args_msgpack)
    public void bookAction(int periphId, int methodId, byte[] args) {
        String methodName = MethodRegistry.getName(methodId);
        Object[] decodedArgs = MessagePackDecoder.decode(args);
        requestManager.bookAction(periphId, methodName, decodedArgs);
    }
    
    // Called from Rust: read_result_size(periph_id, method_id) → size or error code
    public int readResultSize(int periphId, int methodId) {
        String methodName = MethodRegistry.getName(methodId);
        Object result = requestManager.getQueryResult(periphId, methodName);
        
        if (result instanceof PeripheralError error) {
            return -error.type().ordinal() - 1;
        }
        
        byte[] encoded = MessagePackEncoder.encode(result);
        return encoded.length;
    }
    
    // Called from Rust: read_result(periph_id, method_id, buffer, size)
    public void readResult(int periphId, int methodId, byte[] buffer, int size) {
        String methodName = MethodRegistry.getName(methodId);
        Object result = requestManager.getQueryResult(periphId, methodName);
        byte[] encoded = MessagePackEncoder.encode(result);
        System.arraycopy(encoded, 0, buffer, 0, Math.min(encoded.length, size));
    }
    
    // Called from Rust: read_action_results_count(periph_id, method_id) → count
    public int readActionResultsCount(int periphId, int methodId) {
        String methodName = MethodRegistry.getName(methodId);
        List<Object> results = requestManager.getActionResults(periphId, methodName);
        return results.size();
    }
    
    // Called from Rust: read_action_result_size(periph_id, method_id, index) → size or error
    public int readActionResultSize(int periphId, int methodId, int index) {
        String methodName = MethodRegistry.getName(methodId);
        List<Object> results = requestManager.getActionResults(periphId, methodName);
        
        if (index < 0 || index >= results.size()) {
            return -1;
        }
        
        Object result = results.get(index);
        if (result instanceof PeripheralError error) {
            return -error.type().ordinal() - 1;
        }
        
        byte[] encoded = MessagePackEncoder.encode(result);
        return encoded.length;
    }
    
    // Called from Rust: read_action_result(periph_id, method_id, index, buffer, size)
    public void readActionResult(int periphId, int methodId, int index, byte[] buffer, int size) {
        String methodName = MethodRegistry.getName(methodId);
        List<Object> results = requestManager.getActionResults(periphId, methodName);
        
        if (index < 0 || index >= results.size()) {
            return;
        }
        
        Object result = results.get(index);
        byte[] encoded = MessagePackEncoder.encode(result);
        System.arraycopy(encoded, 0, buffer, 0, Math.min(encoded.length, size));
    }
    
    // Called from Rust: book_event(periph_id, event_id)
    public void bookEvent(int periphId, int eventId) {
        String eventName = EventType.values()[eventId].getEventName();
        requestManager.registerEventListener(periphId, eventName);
    }
    
    // Called from Rust: read_events_count(periph_id, event_id) → count
    public int readEventsCount(int periphId, int eventId) {
        String eventName = EventType.values()[eventId].getEventName();
        List<Object> events = requestManager.getEvents(periphId, eventName);
        return events.size();
    }
    
    // Called from Rust: read_event_size(periph_id, event_id, index) → size or -1
    public int readEventSize(int periphId, int eventId, int index) {
        String eventName = EventType.values()[eventId].getEventName();
        List<Object> events = requestManager.getEvents(periphId, eventName);
        
        if (index < 0 || index >= events.size()) {
            return -1;
        }
        
        Object event = events.get(index);
        if (event == null) {
            return -1;
        }
        
        byte[] encoded = MessagePackEncoder.encode(event);
        return encoded.length;
    }
    
    // Called from Rust: read_event(periph_id, event_id, index, buffer, size)
    public void readEvent(int periphId, int eventId, int index, byte[] buffer, int size) {
        String eventName = EventType.values()[eventId].getEventName();
        List<Object> events = requestManager.getEvents(periphId, eventName);
        
        if (index < 0 || index >= events.size()) {
            return;
        }
        
        Object event = events.get(index);
        if (event == null) {
            return;
        }
        
        byte[] encoded = MessagePackEncoder.encode(event);
        System.arraycopy(encoded, 0, buffer, 0, Math.min(encoded.length, size));
    }
}
```

## Execution Flow

### Tick Execution

```
┌─────────────────────────────────────────────────────────────┐
│ Minecraft Tick N                                            │
├─────────────────────────────────────────────────────────────┤
│ 1. Rust: book_next_*() → RequestBuffer                      │
│ 2. Rust: wait_for_next_tick().await → FFI flush            │
│ 3. Java: PeripheralRequestManager.tick()                   │
│    a. executeAll() - Execute all booked requests           │
│    b. pollEvents() - Poll for events                       │
│    c. Store results in ResultBuffer                        │
│ 4. Rust: read_last_*() ← ResultBuffer                      │
└─────────────────────────────────────────────────────────────┘
```

### Request Execution

```java
public void executeAll(Map<Integer, AttachedPeripheral> peripherals) {
    // Execute query requests
    for (Map.Entry<Integer, Map<String, PendingRequest>> entry : 
            requestBuffer.getQueryRequests().entrySet()) {
        int periphId = entry.getKey();
        AttachedPeripheral periph = peripherals.get(periphId);
        
        if (periph == null) continue;
        
        for (Map.Entry<String, PendingRequest> reqEntry : entry.getValue().entrySet()) {
            String methodName = reqEntry.getKey();
            PendingRequest req = reqEntry.getValue();
            
            try {
                Object result = periph.type().invoke(
                    periph.peripheralPos(), 
                    methodName, 
                    req.args()
                );
                resultBuffer.storeQueryResult(periphId, methodName, result);
            } catch (Exception e) {
                resultBuffer.storeError(periphId, methodName, e.getMessage(), false);
            }
        }
    }
    
    // Execute action requests
    for (Map.Entry<Integer, Map<String, List<PendingRequest>>> entry : 
            requestBuffer.getActionRequests().entrySet()) {
        int periphId = entry.getKey();
        AttachedPeripheral periph = peripherals.get(periphId);
        
        if (periph == null) continue;
        
        for (Map.Entry<String, List<PendingRequest>> reqEntry : entry.getValue().entrySet()) {
            String methodName = reqEntry.getKey();
            List<PendingRequest> requests = reqEntry.getValue();
            
            for (PendingRequest req : requests) {
                try {
                    Object result = periph.type().invoke(
                        periph.peripheralPos(), 
                        methodName, 
                        req.args()
                    );
                    resultBuffer.storeActionResult(periphId, methodName, result);
                } catch (Exception e) {
                    resultBuffer.storeError(periphId, methodName, e.getMessage(), true);
                }
            }
        }
    }
    
    // Clear request buffer for next tick
    requestBuffer.clear();
}
```

### Event Polling

```java
public void pollEvents(Map<Integer, AttachedPeripheral> peripherals) {
    for (Map.Entry<Integer, Map<String, EventListener>> entry : 
            eventMonitor.getListeners().entrySet()) {
        int periphId = entry.getKey();
        AttachedPeripheral periph = peripherals.get(periphId);
        
        if (periph == null) continue;
        
        for (Map.Entry<String, EventListener> listenerEntry : entry.getValue().entrySet()) {
            String eventName = listenerEntry.getKey();
            
            try {
                // Poll for event (implementation depends on peripheral type)
                Object event = periph.type().pollEvent(
                    periph.peripheralPos(), 
                    eventName
                );
                
                if (event != null) {
                    eventMonitor.queueEvent(periphId, eventName, event);
                }
            } catch (Exception e) {
                LOGGER.warn("Event polling failed for {}: {}", eventName, e.getMessage());
            }
        }
    }
}
```

## Integration with WasmEngine

**File**: `forge/src/main/java/com/rustcomputers/wasm/WasmEngine.java`

```java
public class WasmEngine {
    private final PeripheralRequestManager requestManager;
    
    public void tick() {
        // Scan peripherals
        Map<Integer, AttachedPeripheral> peripherals = 
            PeripheralProvider.scanAdjacent(level, computerPos);
        
        // Execute requests and poll events
        requestManager.tick(peripherals);
        
        // Continue with WASM execution...
    }
}
```

## Error Handling

### Error Codes

```
 0 or positive: Success (result size in bytes)
-1: NOT_FOUND
-2: METHOD_NOT_FOUND
-3: INVALID_ARGUMENT
-4: EXECUTION_FAILED
-5: DISCONNECTED
-6: TIMEOUT
-7: SERIALIZATION_ERROR
```

### Error Recovery

```rust
// Rust side
pub fn read_result(addr: PeriphAddr, method_name: &str) -> Result<Vec<u8>, PeripheralError> {
    let method_id = method_id(method_name);
    let size = unsafe { ffi::read_result_size(addr.raw(), method_id) };
    
    match size {
        s if s >= 0 => {
            let mut buffer = vec![0u8; s as usize];
            unsafe {
                ffi::read_result(addr.raw(), method_id, buffer.as_mut_ptr(), s as u32);
            }
            Ok(buffer)
        }
        -1 => Err(PeripheralError::NotFound),
        -2 => Err(PeripheralError::MethodNotFound),
        -3 => Err(PeripheralError::InvalidArgument),
        -4 => Err(PeripheralError::ExecutionFailed("Unknown error".to_string())),
        -5 => Err(PeripheralError::Disconnected),
        -6 => Err(PeripheralError::Timeout),
        -7 => Err(PeripheralError::SerializationError),
        _ => Err(PeripheralError::ExecutionFailed(format!("Unknown error code: {}", size))),
    }
}
```

## Testing Strategy

### Unit Tests

```java
@Test
public void testBookQuery() {
    RequestBuffer buffer = new RequestBuffer();
    buffer.bookQuery(0, "getData", new Object[]{});
    
    assertEquals(1, buffer.getQueryRequests().get(0).size());
}

@Test
public void testBookAction() {
    RequestBuffer buffer = new RequestBuffer();
    buffer.bookAction(0, "setValue", new Object[]{100});
    buffer.bookAction(0, "setValue", new Object[]{200});
    
    assertEquals(2, buffer.getActionRequests().get(0).get("setValue").size());
}

@Test
public void testExecuteQuery() {
    PeripheralRequestManager manager = new PeripheralRequestManager();
    manager.bookQuery(0, "getData", new Object[]{});
    
    // Mock peripheral
    AttachedPeripheral periph = mock(AttachedPeripheral.class);
    when(periph.type().invoke(any(), eq("getData"), any())).thenReturn("result");
    
    manager.executeAll(Map.of(0, periph));
    
    assertEquals("result", manager.getQueryResult(0, "getData"));
}
```

### Integration Tests

```java
@Test
public void testMultipleActions() {
    PeripheralRequestManager manager = new PeripheralRequestManager();
    manager.bookAction(0, "pushItems", new Object[]{"chest", 1, 64});
    manager.bookAction(0, "pushItems", new Object[]{"chest", 2, 32});
    
    // Execute
    manager.executeAll(peripherals);
    
    // Verify all results
    List<Object> results = manager.getActionResults(0, "pushItems");
    assertEquals(2, results.size());
}
```

## Performance Considerations

### Optimization Strategies

1. **Batch Processing**: All requests executed in single tick
2. **Result Caching**: Results cached until next tick
3. **Event Queuing**: Events queued to prevent loss
4. **Memory Management**: Buffers cleared after each tick

### Benchmarking

- FFI call overhead: <100μs per call
- Request execution: <1ms per request
- Event polling: <500μs per peripheral
- Memory usage: <1MB per computer

## Migration Path

### Phase 1: Add New Components (Non-Breaking)
- Add PeripheralRequestManager
- Add RequestBuffer, ResultBuffer, EventMonitor
- Add FFI methods

### Phase 2: Update Existing Peripherals
- Update PeripheralType interface
- Add invoke() and pollEvent() methods
- Implement for existing peripherals

### Phase 3: Deprecate Old Methods
- Mark old direct-call methods as deprecated
- Provide migration guide
- Remove after grace period

---

**Last Updated**: 2025-01-XX  
**Version**: 1.0  
**Maintainer**: RustComputers Team
