# Execution Model and VM

Understanding how Obfusku programs execute.

---

## Overview

Obfusku uses a **stack-based virtual machine** (VM).

```
Source Code (.obk)
       ↓
    Lexer (tokenization)
       ↓
    Compiler (bytecode generation)
       ↓
    Bytecode (opcodes)
       ↓
    VM (execution)
       ↓
    Output
```

---

## Compilation Pipeline

### 1. Lexical Analysis

The lexer converts source text into tokens:

```obfusku
⟁x=42
```

Becomes:
```
[Symbol(TypeInteger), Identifier("x"), Equals, Integer(42)]
```

### 2. Compilation

The compiler transforms tokens into bytecode:

```
DeclareVar(0)    // declare x
Const(42)        // push 42
StoreVar(0)      // store to x
```

### 3. Execution

The VM executes bytecode instructions sequentially.

---

## Stack-Based Execution

The VM maintains an **execution stack** for operations.

### Example: Addition

```obfusku
⟁result=3 ✚ 5
```

Execution:
```
1. Push 3          Stack: [3]
2. Push 5          Stack: [3, 5]
3. Add             Stack: [8]
4. Store result    Stack: []
```

### Stack Operations

| Operation | Effect |
|-----------|--------|
| Push | Add value to top |
| Pop | Remove and return top |
| Peek | Read top without removing |
| Dup | Duplicate top value |

---

## Call Frames

Each function call creates a **call frame**:

```
┌─────────────────────┐
│ Frame: main         │
│   ip: 0             │
│   locals: {x: 5}    │
├─────────────────────┤
│ Frame: square       │
│   ip: 12            │
│   locals: {n: 5}    │
└─────────────────────┘
```

- **ip**: Instruction pointer
- **locals**: Local variable scope

### Maximum Call Depth

The default maximum is 1024 frames.  
Exceeding this causes a stack overflow error.

---

## Memory Model

### Values

All values are represented in the `Value` enum:

| Type | Representation |
|------|----------------|
| Integer | 64-bit signed |
| Real | 64-bit float |
| String | UTF-8 string |
| Boolean | true/false |
| Array | Vec of values |
| Map | Vec of key-value pairs |
| Function | Index reference |
| Closure | Function + captures |
| Null | Absence of value |

### Variables

Variables are stored in scopes:
- **Global scope**: Top-level declarations
- **Local scope**: Within functions

---

## Control Flow

### Jumps

Control flow uses jump instructions:

```
JumpIfFalse(offset)  // conditional jump
Jump(offset)         // unconditional jump
Loop(offset)         // backward jump
```

### Example: Conditional

```obfusku
⟨x ▷ 0]
    ✤"positive"
⟫
```

Bytecode:
```
LoadVar(x)
Const(0)
Greater
JumpIfFalse(+5)    // skip to after block
PrintLit("positive")
Jump(+0)           // to end
```

---

## Exception Handling

Exceptions use a **handler stack**:

```
┌─────────────────────┐
│ Handler             │
│   catch_ip: 45      │
│   stack_depth: 3    │
│   frame_depth: 1    │
└─────────────────────┘
```

When `⚠` (throw) is executed:
1. Pop handler from handler stack
2. Restore stack to saved depth
3. Unwind call frames if needed
4. Jump to catch_ip

---

## Closures

Closures capture values from outer scopes:

```obfusku
λmake_adder[⟁x]
    λinner[⟁y]
        ⤶[x ✚ y]    // x is captured
    Λ
    ⤶[inner]
Λ
```

At closure creation:
1. Identify captured variables
2. Copy their values
3. Store in closure structure

At closure call:
1. Create new frame
2. Inject captured values
3. Execute function body

---

## Bytecode Format

Compiled bytecode can be saved to `.obc` files:

```
Header:
  Magic: "OBFK"
  Version: 1.0.0

Sections:
  - Constants
  - Strings
  - Functions
  - Code
```

---

## Performance Characteristics

| Operation | Complexity |
|-----------|------------|
| Variable lookup | O(1) hash |
| Stack push/pop | O(1) |
| Function call | O(params) |
| Array access | O(1) |
| Map access | O(n) linear |

---

*Next: [Syntax and Symbols](05_Syntax_and_Symbols.md)*
