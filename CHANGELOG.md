# Changelog

All notable changes to Obfusku are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

---

## [Unreleased]

No unreleased changes at this time.

---

## [1.0.0] - 2026-01-30

### Added

#### Language Features
- **Closures with environment capture** — Functions capture variables from outer scopes by value
- **First-class functions** — Functions can be stored in variables, passed as arguments, and returned from functions
- **Exception handling** — Try-catch-finally blocks with stack unwinding semantics (`☄` `☊` `☋` `⚠`)
- **Nested function support** — Functions can be defined inside other functions with proper scoping

#### Implementation
- Closure capture analysis at compile time
- Full exception handler stack with frame restoration
- Source map integration for error reporting with line/column information
- Bytecode serialization support (save/load compiled programs)
- Basic optimization passes (constant folding, nop removal)
- Improved REPL with command history and multi-line support

#### Documentation
- 15 comprehensive grimoires (pedagogical documentation)
- Formal language specification (LANGUAGE_SPEC.md)
- Security policy (SECURITY.md)
- Contribution guidelines (CONTRIBUTING.md)
- Code of conduct (CODE_OF_CONDUCT.md)
- Example validation report

### Changed

#### Language Semantics
- Function compilation now uses `function_stack` instead of single `current_function` to support nesting
- Function identifier resolution checks closure captures before global scope
- Exception variable bindings require unique names per catch block (name shadowing not allowed)

#### Documentation Structure
- README.md refactored as gateway document (minimal, focused)
- Created grimoires/ folder for comprehensive, organized documentation
- Formal specification added as source of truth for implementers

### Fixed

#### Compiler Issues
- Fixed `FunctionScope` fields being unused (added proper documentation)
- Integrated source maps into error reporting (all errors now include context)
- Added missing function call context in error messages

#### Examples
- `functions.obk` — Fixed bare function call (must be assigned to variable)
- `exceptions_v100.obk` — Fixed duplicate variable names in catch blocks

#### Runtime
- Improved error messages with source line context
- Better exception propagation across nested handlers

### Known Limitations

#### Deferred to v1.1.0
- **Module system** — Syntax is parsed, but module loading and namespace isolation are not implemented
- **Type inference** — No automatic type inference; declarations require explicit type symbols
- **Algebraic data types** — No struct or enum support
- **Traits/interfaces** — No capability-based contracts
- **Async/concurrency** — Single-threaded execution only; no async/await
- **Metaprogramming** — No quote/unquote or reflection
- **Mutable closures** — Captures are immutable (by design, upvalues deferred)

#### By Design
- No implicit type conversion
- No operator overloading
- No variadic functions
- No default parameters
- No method dispatch
- Stack limit: 4096 values
- Call stack limit: 1024 frames

---

## [0.3.0] - 2026-01-20

### Added

#### Language Features
- **Maps/Dictionaries** (`⌖`) — Key-value collections with string, integer, boolean, rune, and null keys
- **Pattern matching** (`⟡⟢⟣`) — Match expressions with wildcard support
- **Array operations** — Complete array support with indexing, push, len operations

#### Implementation
- Map access, insertion, deletion, key/value enumeration
- Pattern arm compilation with proper jump patching
- Bytecode serialization (save compiled programs to .obc files)
- Constant folding optimization
- Nop instruction removal
- Improved REPL with `:history`, `:symbols`, `:debug` commands

### Changed

#### Runtime
- Stack-based execution with improved opcode dispatch
- Better type checking and error reporting

### Fixed

#### Bugs
- Pattern matching stack discipline corrected
- Array bounds checking improved
- Map access error handling

### Known Limitations

#### Not Implemented
- Module system (syntax only)
- Closures (infrastructure only, no capture)
- Exception handling (syntax only, not executable)
- Type inference
- ADTs
- Async

---

## [0.2.0] - 2026-01-10

### Added

#### Language Features
- **Conditionals** (`⟨⟩⟫`) — If-then-else expressions with condition evaluation
- **Functions** (`λΛ⤷⤶`) — Function definition, invocation, and return
- **Arrays** (`⌬`) — Array literals and basic operations
- **Better string interpolation** — Basic support for string operations

#### Implementation
- Source maps for error context (line/column tracking)
- Basic optimization framework
- Improved REPL with:
  - Command history (`:history`, `:!N`)
  - Help system (`:help`)
  - Symbol reference (`:symbols`)

#### Runtime
- Proper function call frames
- Parameter binding
- Return value handling
- Implicit null return for functions without explicit return

### Changed

#### Architecture
- Parser enhanced for statement types
- Compiler supports nested scopes
- Runtime adds function invocation handling

### Fixed

#### Initial Implementation
- Basic type system working
- Variable declaration and assignment
- Arithmetic operations correct
- Comparison operators functional

---

## [0.1.0] - 2026-01-01

### Added

#### Initial Release
- **Core language** — Symbol-based syntax with Unicode support
- **Basic types** — Integer, Real, String, Boolean, Rune, Null
- **Variables** — Declaration with type symbols, assignment
- **Operators** — Arithmetic (`✚` `✖` `✱` `÷` `⌑`), comparison (`⩵` `≠` `◁` `▷` `⊴` `⊵`), logical (`∧` `∨` `¬`)
- **I/O** — Print (`⚡` `✤`), input (`⚓`)
- **Accumulator** — Special counter (`✹`) with increment/decrement
- **Loops** — Basic loop construct (`⊂⊃`)
- **Stack-based VM** — Execution engine with 4096-value stack limit

#### Tooling
- CLI interface
- Interactive REPL
- Bytecode compilation support
- Symbol reference system

#### Documentation
- Basic README
- Symbol reference
- Example programs

### Known Limitations

#### Not Implemented
- Functions
- Conditionals
- Exception handling
- Pattern matching
- Maps
- Modules
- Type inference
- Closures

---

## Architecture Notes

### v1.0.0 Stability Guarantee

The v1.0.0 release locks core semantics:
- Symbol meanings are immutable
- Type system is stable
- VM behavior is defined for all documented operations
- No breaking changes for v1.x.x releases

### Version Compatibility

| Version | Status | Breaking Changes |
|---------|--------|-----------------|
| 1.0.x | Active | None |
| 0.3.x | Maintenance | N/A |
| 0.2.x | Unsupported | N/A |
| 0.1.x | Unsupported | N/A |

---

## Development History

### Major Milestones

1. **v0.1.0** — Foundation (symbol-based VM, basic types, I/O, loops)
2. **v0.2.0** — Control flow (conditionals, functions, arrays, source maps)
3. **v0.3.0** — Collections (maps, pattern matching, optimization)
4. **v1.0.0** — Maturity (closures, exceptions, formal specification, professional documentation)

### Testing & Validation

- 24 unit tests (all passing)
- 17 example programs (all working)
- Comprehensive validation report
- Example validation report with corrections applied

---

## Links

- **Repository**: https://github.com/Sxnnyside-Project/obfusku
- **Specification**: [LANGUAGE_SPEC.md](LANGUAGE_SPEC.md)
- **Roadmap**: [FUTURE.md](FUTURE.md)
- **Security**: [SECURITY.md](SECURITY.md)
- **Contributing**: [CONTRIBUTING.md](CONTRIBUTING.md)

---

**Current Release**: v1.0.0  
**Last Updated**: January 30, 2026
