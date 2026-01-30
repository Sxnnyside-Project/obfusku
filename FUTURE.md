# Obfusku Future Evolution Guide

This document outlines potential future developments for the Obfusku language and its Rust implementation.

## 🎯 Short-Term Goals (v0.2.0)

### Language Features
- [x] **Conditional expressions** (`⟨condition⟩ then ⟩ else ⟫`)
- [x] **Functions** (`λname[params] ... Λ`)
- [x] **Arrays** (`⌬arr=[1, 2, 3]`)
- [x] **Better string interpolation**

### Implementation
- [x] Source maps for better error messages
- [x] Optimization passes
- [x] Better REPL with history and completion

## 🚀 Medium-Term Goals (v0.3.0)

### Language Features
- [x] **Maps/Dictionaries** (`⌖map`)
- [x] **Pattern matching** (`⟡ ... ⟢ ... ⟣`)
- [ ] ~~Closures and first-class functions~~ (moved to v1.0.0)
- [ ] ~~Module system~~ (moved to v1.0.0)
- [ ] ~~Exception handling~~ (moved to v1.0.0)

### Implementation
- [x] Bytecode serialization (save compiled spells)
- [x] Basic optimizations (constant folding, dead code elimination)
- [ ] Memory pooling for values (deferred)
- [ ] Parallel compilation (deferred)

**Note**: Closures, Modules, and Exception handling have syntax parsing and opcodes defined, but runtime execution is stubbed. These are moved to v1.0.0 for proper implementation.

## 🌟 v1.0.0 — STABLE RELEASE

### Core Language Features (COMPLETE)
- [x] **First-class functions** - Functions as values, indirect calls ✅
- [x] **Closures with environment capture** - Variable capture from outer scopes ✅
- [x] **Exception handling** (`☄ ... ☊ ... ☋`) - Stack unwinding, try/catch/finally ✅
- [x] **Maps/Dictionaries** (`⌖`) - Full operations ✅
- [x] **Pattern Matching** (`⟡ ⟢ ⟣`) - With wildcards ✅
- [x] **Arrays** (`⌬`) - Full operations ✅
- [x] **Conditionals** (`⟨ ⟩ ⟫`) ✅
- [x] **Loops with accumulator** (`⊂ ⊃ ✹`) ✅
- [x] **Bytecode serialization** - Save/load compiled spells ✅

### Deferred to v1.1.0 (OPTIONAL for v1.0.0)
- [ ] **Module system** (`⟲"module"`) - Syntax exists, runtime deferred
- [ ] **Type inference** - Complex type system required
- [ ] **Algebraic data types** (`⌻` struct, `⎊` enum)
- [ ] **Traits/Interfaces** (`⍟` define, `⍜` implement)
- [ ] **Async/concurrent execution** (`⊛` spawn, `⊙` await)
- [ ] **Metaprogramming** (`⎔` quote, `⎕` unquote)

### Tooling (OPTIONAL)
- [ ] **Debugger** - Step-through execution
- [ ] **Profiler** - Execution timing
- [ ] **JIT compilation** (Cranelift)
- [ ] **FFI** (Rust/C interop)

## 🔮 v1.1.0 — Language Evolution

### Runtime
- [ ] **JIT compilation** (optional, via Cranelift)
- [ ] **Garbage collection** improvements
- [ ] **FFI** for calling Rust/C functions
- [ ] **Debugger** with step-through execution
- [ ] **Profiler** integration

### Tooling
- [ ] **Language Server Protocol (LSP)** support
- [ ] **VS Code extension** with syntax highlighting
- [ ] **Package manager** for Obfusku libraries
- [ ] **Documentation generator**
- [ ] **Formatter** (`obfusku fmt`)
- [ ] **Linter** (`obfusku lint`)

## 🔮 Experimental Ideas

### Symbol Extensions
```
⌭  - Generator/Iterator
⎔  - Channel (for concurrency)
⏣  - Promise/Future
⌬  - Vector/Array
⌖  - Map/Dictionary  
⎈  - Set
⌻  - Struct definition
⎊  - Enum definition
```

### New Control Flow
```
⟡  - Match expression start
⟢  - Match arm
⟣  - Match expression end

⊛  - Spawn async task
⊙  - Await result
⊘  - Cancel task
```

### Memory/Reference Operations
```
⌘  - Reference (borrow)
⌦  - Dereference
⌫  - Move ownership
⌧  - Drop/deallocate
```

### Advanced Operators
```
⋈  - Pipe operator (|>)
⋉  - Compose functions
⋊  - Bind/partial application
⧉  - Map over collection
⧊  - Filter collection
⧋  - Reduce/fold
```

## 📦 Potential Standard Library Packages

### `math.obx`
- Trigonometric functions
- Statistical operations
- Complex numbers
- Matrix operations

### `string.obx`
- Regular expressions
- Unicode operations
- String formatting
- Parsing utilities

### `io.obx`
- File operations
- Network I/O
- Serialization (JSON, YAML, etc.)

### `time.obx`
- Date/time handling
- Timers and delays
- Scheduling

### `collections.obx`
- Advanced data structures
- Sorting algorithms
- Search utilities

## 🎨 Design Principles to Maintain

1. **Symbol Primacy**: Symbols should always be the primary semantic units
2. **Visual Meaning**: Choose symbols that visually represent their function
3. **Ritual Feel**: Maintain the mystical, ceremonial nature of the language
4. **Simplicity**: Avoid unnecessary complexity
5. **Extensibility**: Always allow for growth without breaking changes

## 🤝 Contributing Guidelines

### Adding New Symbols
1. Propose the symbol in an issue with justification
2. Ensure it doesn't conflict with existing symbols
3. The symbol should have visual/semantic meaning
4. Add to `symbols/meaning.rs` with documentation
5. Update the README and grimoire

### Code Style
- Follow Rust idioms
- Document public APIs
- Write tests for new functionality
- Keep error messages "magical"

---

*The future of Obfusku is written in the stars... and in Rust 🦀✨*
