# 🜏 Obfusku v1.0.0 — Conceptual Design Document

**Version**: v1.0.0 Design Specification  
**Status**: DESIGN PHASE  
**Date**: January 30, 2026

---

## Preamble

Obfusku v1.0.0 is not merely "more features."

It is Obfusku becoming **complete** — a stable, intentional, mystical language where symbols are semantic vessels and execution is ritual.

This document defines what v1.0.0 **is** and what it **is not**.

---

# ═══════════════════════════════════════════════════════════════
# PART I: CORE FEATURE COMPLETION
# ═══════════════════════════════════════════════════════════════

These features have syntax and opcodes defined but lack runtime implementation. They MUST be completed for v1.0.0.

## 1. Closures and First-Class Functions

### Philosophy

Functions in Obfusku are not mere subroutines — they are **captured rituals** that carry their environment with them.

### Symbol Design

| Symbol | Meaning |
|:------:|---------|
| `λ` | Define ritual (function) |
| `Λ` | Seal ritual |
| `⤷` | Invoke ritual |
| `⤶` | Return from ritual |
| `⊃` | Capture symbol (new) |

### Semantic Model

```
λmake_adder[⟁x]
    λinner[⟁y]
        ⤶[x ✚ y]    // x captured from outer scope
    Λ
    ⤶[inner]        // return the closure
Λ

⟁add5=⤷make_adder[5]
⟁result=⤷add5[10]   // result = 15
```

### Implementation Requirements

1. **Capture Analysis** (compile-time):
   - Identify free variables in function body
   - Determine capture semantics (value vs reference)
   - For simplicity: **capture by value** (copy at closure creation)

2. **Closure Runtime Structure**:
   ```rust
   pub struct Closure {
       pub function_index: usize,
       pub captures: Vec<Value>,  // Captured values
       pub capture_names: Vec<String>,  // For debugging
   }
   ```

3. **Modified Call Semantics**:
   - When calling `ClosureVal`, restore captured environment
   - Push captures as locals in new frame
   - Execute function body

4. **Bytecode**:
   - `MakeClosure(func_idx, capture_count)` — already defined
   - `LoadCapture(idx)` — load from closure's captures
   - `StoreCapture(idx)` — store to closure's captures (for mutable captures, later)

### Deferred to Post-v1.0.0

- Mutable captures (upvalues)
- Recursive closures (self-reference)

---

## 2. Module System

### Philosophy

Modules are **ritual scrolls** that can be invoked into other spells. They provide isolation and reusability without polluting the global mystical namespace.

### Symbol Design

| Symbol | Meaning |
|:------:|---------|
| `⟲` | Import scroll (module) |
| `⟳` | Export symbol to public interface |
| `⊷` | Access module member |

### Semantic Model

**File: `math.obk`**
```obfusku
⟳square    // Export square function

λsquare[⟁n]
    ⤶[n ✱ n]
Λ

❧
```

**File: `main.obk`**
```obfusku
⟲"math"    // Import the math scroll

⟁result=⤷math⊷square[7]   // Use module function
⚡[result]

❧
```

### Implementation Requirements

1. **Module Loader Integration**:
   - Connect `ModuleLoader` (already exists) to runtime
   - Resolve module paths (current dir, search paths, `.obx` packages)
   - Circular dependency detection (already exists)

2. **Module Compilation**:
   - Compile module source to `Chunk`
   - Extract exported symbols
   - Store in `Module` struct

3. **Module Execution**:
   - Execute module once (initialize exports)
   - Cache evaluated exports
   - Make available via `⊷` access

4. **Bytecode**:
   - `Import(module_name_idx)` — load and execute module
   - `Export(symbol_name_idx)` — mark symbol as exported
   - `LoadModule(module_idx, symbol_idx)` — access module symbol

5. **Namespace Isolation**:
   - Each module has its own global scope
   - Only exported symbols visible externally
   - No implicit namespace pollution

### Module Types

| Extension | Type | Description |
|-----------|------|-------------|
| `.obk` | Source | Obfusku source file |
| `.obc` | Compiled | Pre-compiled bytecode |
| `.obx` | Package | Standard library package |

---

## 3. Exception Handling

### Philosophy

Exceptions in Obfusku are not errors — they are **magical disruptions** that require ritual containment and recovery.

### Symbol Design

| Symbol | Meaning |
|:------:|---------|
| `☄` | Begin protection ritual (try) |
| `☊` | Containment circle (catch) |
| `☋` | Closing seal (finally) |
| `⚠` | Invoke disruption (throw) |
| `⟣` | End protection block |

### Semantic Model

```obfusku
☄
    ✤"Attempting dangerous incantation..."
    ⚠["Spell backfired!"]
    ✤"This won't print"
☊[error]
    ✤"Contained disruption:"
    ⚡[error]
☋
    ✤"Cleanup ritual (always runs)"
⟣
```

### Implementation Requirements

1. **Exception Handler Stack**:
   ```rust
   struct ExceptionHandler {
       handler_ip: usize,       // Where to jump on exception
       finally_ip: Option<usize>, // Finally block location
       stack_depth: usize,      // Stack state to restore
       frame_depth: usize,      // Call frame to return to
   }
   ```

2. **Stack Unwinding**:
   - On `Throw`: search handler stack for active handler
   - Restore stack to handler's `stack_depth`
   - Pop frames until `frame_depth`
   - Jump to `handler_ip`
   - Execute catch block with exception value

3. **Finally Semantics**:
   - Always executes (normal exit, throw, or return through try)
   - Requires tracking "pending action" (normal, exception, return)

4. **Bytecode Semantics**:
   - `TryBegin(handler_offset)` — push handler onto handler stack
   - `TryEnd` — pop handler (normal exit)
   - `Throw` — trigger exception
   - `Catch(var_idx)` — bind exception to variable
   - `Finally` — mark finally block

### Exception Values

Exceptions are regular `Value`s — typically strings or maps:

```obfusku
⚠["Simple message"]
⚠[{"type"⇒"SpellFailure"⋄ "code"⇒42}]
```

---

# ═══════════════════════════════════════════════════════════════
# PART II: LANGUAGE EVOLUTION FEATURES
# ═══════════════════════════════════════════════════════════════

## 4. Type Inference

### Philosophy

Types in Obfusku should **emerge** from usage, not be declared verbosely. The essence reveals itself.

### Design Principles

1. **No explicit type annotations by default**
2. **Types inferred from literals and operations**
3. **Type symbols (`⟁`, `⌘`, etc.) optional for declaration**
4. **Errors when inference fails or contradicts**

### Semantic Model

```obfusku
// Type inferred from literal
x = 42         // x is Integer
name = "Merlin" // name is String

// Type inferred from operation
result = x ✚ 10  // result is Integer

// Explicit when needed (disambiguation)
⟁counter = 0    // Explicit Integer
```

### Implementation Approach

**Bidirectional Type Inference** (simplified):

1. **Forward propagation**: literal → variable → expression
2. **Backward propagation**: operation requirements → operands
3. **Unification**: resolve type variables

For v1.0.0, implement **basic inference** only:
- Infer from literals
- Infer from known variable types
- Report error on ambiguity (require explicit type)

### Deferred

- Full Hindley-Milner inference
- Polymorphic functions
- Generic types

---

## 5. Algebraic Data Types

### Philosophy

ADTs allow defining **symbolic essences** — custom types that embody specific meanings in the ritual.

### Symbol Design

| Symbol | Meaning |
|:------:|---------|
| `⌻` | Define struct (product type) |
| `⎊` | Define enum (sum type) |
| `⌿` | Access field |

### Semantic Model

**Struct (Product Type)**:
```obfusku
⌻Wizard {
    name: ⌘,
    power: ⟁,
    spells: ⌬
}

wizard = Wizard{"Merlin", 9001, ["Fire", "Ice"]}
⚡[wizard⌿name]  // "Merlin"
```

**Enum (Sum Type)**:
```obfusku
⎊Element {
    Fire,
    Water,
    Earth[⟁strength],
    Air
}

elem = Element⊷Fire
elem2 = Element⊷Earth[100]

// Pattern match on enum
⟡elem]
    ⟢Element⊷Fire] ✤"Hot!"
    ⟢Element⊷Water] ✤"Wet!"
    ⟢Element⊷Earth[s]] ⚡[s]
    ⟢◇] ✤"Unknown"
⟣
```

### Implementation Requirements

1. **Type Definition Storage**:
   - Struct/enum definitions in type registry
   - Field/variant information

2. **Runtime Values**:
   ```rust
   Value::Struct { type_id: usize, fields: Vec<Value> }
   Value::Enum { type_id: usize, variant: usize, data: Option<Box<Value>> }
   ```

3. **Pattern Matching Integration**:
   - Destructuring in match arms
   - Field binding

---

## 6. Traits / Interfaces

### Philosophy

Traits define **symbolic contracts** — capabilities that types can embody without inheritance hierarchies.

### Symbol Design

| Symbol | Meaning |
|:------:|---------|
| `⍟` | Define trait |
| `⍜` | Implement trait for type |

### Semantic Model

```obfusku
⍟Printable {
    λto_string[] → ⌘
}

⍜Printable for Wizard {
    λto_string[]
        ⤶[self⌿name]
    Λ
}
```

### Design Principles

- Minimal syntax
- No method overloading (single implementation per trait)
- No inheritance
- Composition over hierarchy

### Deferred to Post-v1.0.0

Full trait implementation is complex. For v1.0.0:
- Define trait syntax
- Basic single-trait implementation
- No trait bounds or generics

---

## 7. Async / Concurrent Execution

### Philosophy

Async operations are **parallel incantations** — rituals that proceed without blocking the main flow.

### Symbol Design

| Symbol | Meaning |
|:------:|---------|
| `⊛` | Spawn async task |
| `⊙` | Await result |
| `⊘` | Cancel task |

### Semantic Model

```obfusku
// Spawn async task
task = ⊛[slow_ritual[]]

// Do other work
✤"Working..."

// Await result
result = ⊙[task]
⚡[result]
```

### Implementation Approach (v1.0.0)

**Cooperative Coroutines** (not OS threads):

1. **Task Structure**:
   ```rust
   struct Task {
       id: usize,
       chunk_index: usize,
       frame: CallFrame,
       stack_snapshot: Vec<Value>,
       state: TaskState,
   }
   ```

2. **Scheduler**:
   - Round-robin task switching
   - Yield points at function calls
   - No preemption (cooperative)

3. **Await Semantics**:
   - If task complete: return result
   - If task pending: suspend current task, switch to awaited

### Deferred

- OS thread integration
- Channels for communication
- Parallel execution

---

## 8. Metaprogramming

### Philosophy

Metaprogramming in Obfusku is **ritual reflection** — code that knows itself and can transform.

### Symbol Design

| Symbol | Meaning |
|:------:|---------|
| `⎔` | Quote (prevent evaluation) |
| `⎕` | Unquote (splice into quote) |
| `⍊` | Evaluate quoted code |

### Semantic Model

```obfusku
// Quote creates AST representation
code = ⎔[x ✚ y]

// Unquote splices values
x_val = 5
y_val = 10
code2 = ⎔[⎕x_val ✚ ⎕y_val]  // ⎔[5 ✚ 10]

// Evaluate
result = ⍊[code2]  // 15
```

### v1.0.0 Scope

**Minimal metaprogramming**:
- Quote/unquote syntax
- Basic AST representation
- Simple evaluation

**Deferred**:
- Hygienic macros
- Compile-time evaluation
- Full AST manipulation

---

# ═══════════════════════════════════════════════════════════════
# PART III: VM / RUNTIME EVOLUTION
# ═══════════════════════════════════════════════════════════════

## Runtime Changes for v1.0.0

### 1. Value System Update

```rust
pub enum Value {
    // Existing
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
    
    // v1.0.0 additions
    Struct { type_id: usize, fields: Vec<Value> },
    Enum { type_id: usize, variant: usize, data: Option<Box<Value>> },
    Task(usize),
    Quote(Box<QuotedExpr>),
}
```

### 2. Context Additions

```rust
pub struct Context {
    // Existing fields...
    
    // v1.0.0 additions
    exception_handlers: Vec<ExceptionHandler>,
    type_registry: TypeRegistry,
    task_scheduler: TaskScheduler,
    modules: ModuleRegistry,
}
```

### 3. New Opcode Ranges

| Range | Category |
|-------|----------|
| 0xC0-0xCF | Type operations (ADT) |
| 0xD0-0xDF | Trait operations |
| 0xE0-0xEF | Async operations |
| 0xF0-0xFD | Metaprogramming |

---

# ═══════════════════════════════════════════════════════════════
# PART IV: WHAT v1.0.0 IS NOT
# ═══════════════════════════════════════════════════════════════

## Explicitly Deferred to Post-v1.0.0

| Feature | Reason |
|---------|--------|
| JIT compilation | Requires Cranelift integration |
| Garbage collection | Current RC/ownership sufficient |
| FFI | Complex, low priority |
| Debugger | Tooling phase |
| LSP | Tooling phase |
| Package manager | Tooling phase |
| Full metaprogramming | Too complex for v1.0.0 |
| Mutable closures | Requires upvalue cells |
| Channel-based concurrency | After basic async |

---

# ═══════════════════════════════════════════════════════════════
# PART V: IMPLEMENTATION ROADMAP
# ═══════════════════════════════════════════════════════════════

## Phase 1: Core Completion (Priority: CRITICAL)

1. **Closures** (2-3 weeks)
   - Capture analysis in compiler
   - Closure runtime execution
   - Tests for nested closures

2. **Modules** (2-3 weeks)
   - Module loader integration
   - Module execution and caching
   - Export/import binding

3. **Exceptions** (1-2 weeks)
   - Handler stack
   - Stack unwinding
   - Finally semantics

## Phase 2: Type System (Priority: HIGH)

4. **Basic Type Inference** (2 weeks)
   - Literal inference
   - Simple propagation
   - Error on ambiguity

5. **ADTs** (3 weeks)
   - Struct definition and instantiation
   - Enum definition with variants
   - Pattern matching integration

## Phase 3: Advanced Features (Priority: MEDIUM)

6. **Basic Traits** (2 weeks)
   - Trait definition
   - Single implementation
   - Method dispatch

7. **Async/Await** (3-4 weeks)
   - Task structure
   - Cooperative scheduler
   - Await mechanics

8. **Basic Metaprogramming** (2 weeks)
   - Quote/unquote
   - AST representation
   - Evaluation

## Phase 4: Polish (Priority: LOW)

9. Integrate source maps into errors
10. Auto-apply optimizer
11. Documentation and examples
12. Test suite completion

---

# ═══════════════════════════════════════════════════════════════
# DESIGN PRINCIPLES CHECKLIST
# ═══════════════════════════════════════════════════════════════

| Principle | v1.0.0 Compliance |
|-----------|-------------------|
| Symbol Primacy | ✅ All features use symbolic syntax |
| Visual Meaning | ✅ New symbols chosen for resonance |
| Ritual Feel | ✅ Exception = "disruption containment" |
| Simplicity | ✅ No over-engineering |
| Extensibility | ✅ Clean opcode ranges, type registry |
| No conventional syntax | ✅ Avoided keywords where possible |

---

# CONCLUSION

Obfusku v1.0.0 will be:

- **Complete**: All core features functional
- **Stable**: No stub implementations
- **Intentional**: Every symbol has meaning
- **Mystical**: Execution feels like ritual
- **Extensible**: Ready for future evolution

The magic will be ready.

```
❧
```
