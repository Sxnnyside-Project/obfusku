# Runtime and Memory

Understanding Obfusku's runtime behavior.

---

## Memory Model

### Value Storage

All values are allocated as Rust `Value` enum variants:

```rust
pub enum Value {
    Integer(i64),
    Real(f64),
    String(String),
    Boolean(bool),
    Rune(char),
    Array(Vec<Value>),
    Map(Vec<(Value, Value)>),
    Null,
    Function(usize),
    ClosureVal(Box<Closure>),
    Module(usize),
}
```

### Stack

The execution stack holds temporary values:

- **Operations**: Push operands, pop for operations
- **Capacity**: Default 4096 values
- **Overflow**: Raises stack overflow error

### Heap

Collections (arrays, maps, strings) are heap-allocated:

- Arrays: `Vec<Value>`
- Maps: `Vec<(Value, Value)>` (linear search)
- Strings: Rust `String` (UTF-8)

---

## Scope System

### Global Scope

Top-level declarations:

```obfusku
⟁globalVar=42    // accessible everywhere
```

### Local Scope

Function-local variables:

```obfusku
λfunc[]
    ⟁localVar=10    // only in this function
Λ
```

### Scope Hierarchy

1. **Local scope**: Current function
2. **Closure captures**: Captured from outer functions
3. **Global scope**: Top-level declarations

Variable lookup searches in this order.

---

## Call Stack

### Call Frames

Each function call creates a frame:

```
CallFrame {
    chunk_index: usize,    // bytecode chunk
    ip: usize,             // instruction pointer
    base_pointer: usize,   // stack base
    scope: Scope,          // local variables
    closure: Option<Closure>,  // captured environment
}
```

### Maximum Depth

Default: 1024 frames

Exceeding causes:
```
❌ 🌀 Stack overflow: call depth exceeded
```

---

## Closure Captures

### Capture Mechanism

When a closure is created:

1. Compiler identifies free variables
2. Values are copied (capture by value)
3. Stored in `Closure.captures`

```rust
pub struct Closure {
    pub function_index: usize,
    pub captures: Vec<Value>,
}
```

### Capture Lifetime

Captured values live as long as the closure.

---

## Exception Handlers

### Handler Stack

Active exception handlers are tracked:

```rust
ExceptionHandler {
    handler_ip: usize,      // catch block location
    stack_depth: usize,     // stack to restore
    frame_depth: usize,     // frames to unwind
    chunk_index: usize,
}
```

### Unwinding

On throw:
1. Find active handler
2. Pop stack to `stack_depth`
3. Pop frames to `frame_depth`
4. Jump to `handler_ip`

---

## Garbage Collection

### Current Model

Obfusku uses Rust's ownership model:

- **Stack values**: Dropped when popped
- **Local variables**: Dropped on scope exit
- **Global variables**: Live for program duration
- **Closures**: Reference-counted via `Box`

### No Explicit GC

There is no traditional garbage collector.  
Memory is managed through Rust's RAII.

---

## Performance Characteristics

### Time Complexity

| Operation | Complexity |
|-----------|------------|
| Variable lookup | O(1) amortized |
| Stack push/pop | O(1) |
| Array access | O(1) |
| Map access | O(n) |
| Function call | O(params) |
| Closure creation | O(captures) |

### Space Complexity

| Structure | Space |
|-----------|-------|
| Integer | 8 bytes |
| Real | 8 bytes |
| String | Variable |
| Array | 24 + n × element |
| Map | 24 + n × (key + value) |

---

## Bytecode Compilation

### Compilation Pipeline

```
Source → Tokens → Bytecode → Execution
```

### Saving Bytecode

```bash
obfusku compile input.obk --output output.obc
```

### Loading Bytecode

```bash
obfusku load output.obc
```

### Bytecode Format

```
Header:
  Magic: "OBFK"
  Version: 1.0.0

Sections:
  Constants
  Strings
  Functions
  Code
```

---

## Resource Limits

| Resource | Default Limit |
|----------|---------------|
| Stack size | 4096 values |
| Call depth | 1024 frames |
| String length | System memory |
| Array size | System memory |

---

*Next: [Debugging and Error Messages](13_Debugging_and_Error_Messages.md)*
