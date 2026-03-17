# Modem

**Mod:** CC:Tweaked  
**Peripheral Type:** `modem`  
**Source:** `ModemPeripheral.java`

## Overview

The Modem peripheral provides wireless and wired network communication. It allows you to open channels, transmit messages, and receive data from other modems. Modems can operate in wireless mode (with range limits) or wired mode (unlimited range on connected networks).

## Three-Function Pattern

The Modem API uses the three-function pattern for all methods:

1. **`book_next_*`** - Schedule a request for the next tick
2. **`read_last_*`** - Read the result from the previous tick
3. **`async_*`** - Convenience method that books, waits, and reads in one call

### Pattern Explanation

```lua
-- Method 1: book_next / read_last pattern
modem.book_next_open(15)
wait_for_next_tick()
modem.read_last_open()

-- Method 2: async pattern (recommended)
modem.async_open(15)
```

## Modem Types

CC:Tweaked provides three types of modems:

1. **Wireless Modem** - Communicates with other wireless modems within range (default 64 blocks, up to 384 blocks at y≥96)
2. **Ender Modem** - Wireless modem with unlimited range and cross-dimension support
3. **Wired Modem** - Connects to network cable for unlimited range on connected networks

## Methods

### `open(channel)` / `book_next_open(channel)` / `read_last_open()` / `async_open(channel)`

Open a channel to listen for incoming messages.

**Lua Signature:**
```lua
function open(channel: number) -> nil
```

**Rust Signatures:**
```rust
pub fn book_next_open(&mut self, channel: u32)
pub fn read_last_open(&self) -> Result<(), PeripheralError>
pub async fn async_open(&self, channel: u32) -> Result<(), PeripheralError>
```

**Parameters:**
- `channel: number` — Channel number to open (0–65535)

**Returns:** `nil`

**Notes:**
- Maximum 128 channels can be open simultaneously
- You don't need to open a channel to transmit, only to receive

**Example:**
```lua
local modem = peripheral.find("modem")
modem.async_open(15)
print("Channel 15 opened")
```

**Error Handling:**
- Throws error if channel number is out of range
- Throws error if 128 channels are already open

---

### `isOpen(channel)` / `book_next_is_open(channel)` / `read_last_is_open()` / `async_is_open(channel)`

Check if a channel is currently open.

**Lua Signature:**
```lua
function isOpen(channel: number) -> boolean
```

**Rust Signatures:**
```rust
pub fn book_next_is_open(&mut self, channel: u32)
pub fn read_last_is_open(&self) -> Result<bool, PeripheralError>
pub async fn async_is_open(&self, channel: u32) -> Result<bool, PeripheralError>
```

**Parameters:**
- `channel: number` — Channel number to check (0–65535)

**Returns:** `boolean` — `true` if open, `false` if closed

**Example:**
```lua
local modem = peripheral.find("modem")
if modem.async_is_open(15) then
  print("Channel 15 is open")
else
  print("Channel 15 is closed")
end
```

---

### `close(channel)` / `book_next_close(channel)` / `read_last_close()` / `async_close(channel)`

Close an open channel.

**Lua Signature:**
```lua
function close(channel: number) -> nil
```

**Rust Signatures:**
```rust
pub fn book_next_close(&mut self, channel: u32)
pub fn read_last_close(&self) -> Result<(), PeripheralError>
pub async fn async_close(&self, channel: u32) -> Result<(), PeripheralError>
```

**Parameters:**
- `channel: number` — Channel number to close (0–65535)

**Returns:** `nil`

**Example:**
```lua
local modem = peripheral.find("modem")
modem.async_close(15)
print("Channel 15 closed")
```

---

### `closeAll()` / `book_next_close_all()` / `read_last_close_all()` / `async_close_all()`

Close all open channels.

**Lua Signature:**
```lua
function closeAll() -> nil
```

**Rust Signatures:**
```rust
pub fn book_next_close_all(&mut self)
pub fn read_last_close_all(&self) -> Result<(), PeripheralError>
pub async fn async_close_all(&self) -> Result<(), PeripheralError>
```

**Returns:** `nil`

**Example:**
```lua
local modem = peripheral.find("modem")
modem.async_close_all()
print("All channels closed")
```

---

### `transmit(channel, replyChannel, payload)` / `book_next_transmit(...)` / `read_last_transmit()` / `async_transmit(...)`

Transmit a message on a channel.

**Lua Signature:**
```lua
function transmit(channel: number, replyChannel: number, payload: any) -> nil
```

**Rust Signatures:**
```rust
pub fn book_next_transmit<T: Serialize>(&mut self, channel: u32, reply_channel: u32, payload: &T)
pub fn read_last_transmit(&self) -> Result<(), PeripheralError>
pub async fn async_transmit<T: Serialize>(&self, channel: u32, reply_channel: u32, payload: &T) -> Result<(), PeripheralError>
```

**Parameters:**
- `channel: number` — Target channel (0–65535)
- `replyChannel: number` — Reply channel for responses (0–65535)
- `payload: any` — Message data (primitives, tables, or serializable types)

**Returns:** `nil`

**Notes:**
- You don't need to open the target channel to transmit
- The reply channel should be open on your modem to receive responses
- Payload can be any Lua value (boolean, number, string, table)
- Functions and metatables are not transmitted

**Example:**
```lua
local modem = peripheral.find("modem")
modem.async_open(43)  -- Open reply channel

-- Send a message
modem.async_transmit(15, 43, "Hello, world!")
```

---

### `isWireless()` / `book_next_is_wireless()` / `read_last_is_wireless()` / `async_is_wireless()`

Check if this modem is wireless or wired.

**Lua Signature:**
```lua
function isWireless() -> boolean
```

**Rust Signatures:**
```rust
pub fn book_next_is_wireless(&mut self)
pub fn read_last_is_wireless(&self) -> Result<bool, PeripheralError>
pub async fn async_is_wireless(&self) -> Result<bool, PeripheralError>
```

**Returns:** `boolean` — `true` if wireless, `false` if wired

**Example:**
```lua
local modem = peripheral.find("modem")
if modem.async_is_wireless() then
  print("This is a wireless modem")
else
  print("This is a wired modem")
end
```

---

## Events

### `modem_message`

Fired when a message is received on an open channel.

**Event Parameters:**
1. `string` — Event name (`"modem_message"`)
2. `string` — Side the modem is on
3. `number` — Channel the message was sent on
4. `number` — Reply channel specified by sender
5. `any` — Message payload
6. `number | nil` — Distance to sender in blocks (nil for cross-dimension)

**Example:**
```lua
local modem = peripheral.find("modem")
modem.async_open(0)

while true do
  local event, side, channel, replyChannel, message, distance = os.pullEvent("modem_message")
  print(("Received on channel %d: %s"):format(channel, tostring(message)))
  
  if distance then
    print(("Distance: %d blocks"):format(distance))
  else
    print("Cross-dimension message")
  end
end
```

---

## Usage Examples

### Example 1: Basic Message Exchange

```lua
-- Sender
local modem = peripheral.find("modem")
modem.async_open(43)  -- Open reply channel
modem.async_transmit(15, 43, "Hello!")

-- Receiver
local modem = peripheral.find("modem")
modem.async_open(15)

local event, side, channel, replyChannel, message, distance = os.pullEvent("modem_message")
print("Received: " .. tostring(message))

-- Send reply
modem.async_transmit(replyChannel, 15, "Hello back!")
```

### Example 2: Simple Server

```lua
local modem = peripheral.find("modem")
modem.async_open(100)

print("Server listening on channel 100...")

while true do
  local event, side, channel, replyChannel, message, distance = os.pullEvent("modem_message")
  
  if channel == 100 then
    print("Request: " .. tostring(message))
    
    -- Send response
    modem.async_transmit(replyChannel, 100, "Response: " .. tostring(message))
  end
end
```

### Example 3: Broadcast Listener

```lua
local modem = peripheral.find("modem")

-- Listen on multiple channels
for i = 1, 5 do
  modem.async_open(i)
end

while true do
  local event, side, channel, replyChannel, message, distance = os.pullEvent("modem_message")
  print(("Channel %d: %s (distance: %s)"):format(
    channel,
    tostring(message),
    distance and tostring(distance) or "unknown"
  ))
end
```

### Example 4: Request-Reply Pattern

```lua
local modem = peripheral.find("modem")
modem.async_open(43)

-- Send request
modem.async_transmit(15, 43, {action = "query", data = "test"})

-- Wait for reply with timeout
local timeout = os.startTimer(5)
while true do
  local event, arg1, arg2, arg3, arg4, arg5 = os.pullEvent()
  
  if event == "modem_message" then
    local channel, replyChannel, message = arg3, arg4, arg5
    if channel == 43 then
      print("Reply: " .. tostring(message))
      break
    end
  elseif event == "timer" and arg1 == timeout then
    print("Request timed out")
    break
  end
end
```

### Example 5: Wired Network Discovery

```lua
local modem = peripheral.find("modem")

if modem.async_is_wireless() then
  print("Wireless modem detected")
  print("Range: 64 blocks (up to 384 at y≥96)")
else
  print("Wired modem detected")
  print("Range: Unlimited on connected network")
end
```

---

## Error Handling

All methods may throw errors in the following cases:

- **Modem not found**: Peripheral is disconnected
- **Invalid channel**: Channel number is out of range (0–65535)
- **Too many channels**: 128 channels already open
- **Network error**: Wired network connection is broken

**Example Error Handling:**
```lua
local modem = peripheral.find("modem")
if not modem then
  error("No modem found")
end

local success, result = pcall(function()
  modem.async_open(15)
end)

if not success then
  print("Error: " .. result)
else
  print("Channel opened successfully")
end
```

---

## Type Definitions

### ReceiveData
```lua
{
  channel: number,      -- Channel message was sent on
  replyChannel: number, -- Reply channel specified by sender
  payload: any,         -- Message data
  distance: number | nil, -- Distance in blocks (nil for cross-dimension)
}
```

---

## Notes

- Channel numbers range from 0 to 65535
- Maximum 128 channels can be open simultaneously
- Wireless modems have range limits; wired modems do not
- Messages are transmitted instantly but received as events
- The three-function pattern allows for efficient batch operations
- Ender modems can communicate across dimensions

---

## Related

- [Inventory](./Inventory.md) — Requires wired modem for network access
- [CC:Tweaked Documentation](https://tweaked.cc/) — Official documentation
- `rednet` API — Higher-level networking API built on modems
