# Design Limits and Non-Goals

What Obfusku deliberately does NOT do.

---

## Philosophy of Constraints

Obfusku is intentionally constrained. These limits are **features**, not bugs.

---

## Language Non-Goals

### Not a General-Purpose Language

Obfusku is esoteric by design:
- Not optimized for production systems
- Not intended for large-scale applications
- Not a replacement for conventional languages

### Not Dynamically Typed

Variables must be declared with type symbols:
- No implicit typing
- No runtime type inference
- No duck typing

### Not Object-Oriented

Obfusku has no:
- Classes
- Inheritance
- Method dispatch
- `this` or `self`

### Not Functional-Pure

Obfusku allows:
- Mutation
- Side effects (I/O)
- Imperative style

It is **procedural with functional features**.

---

## Deliberate Limitations

### No Implicit Operations

Everything must be explicit:

```obfusku
// No implicit string conversion
⟁n=42
⌘s=⤷to_string[n]   // must convert explicitly

// No implicit truthiness in some contexts
⟨x ≠ 0]   // explicit comparison
```

### No Operator Overloading

Operators have fixed meanings:
- `✚` always adds
- `⩵` always compares equality
- Cannot be redefined

### No Macros (v1.0.0)

Code cannot generate code at compile time.
Metaprogramming is deferred to future versions.

### No Generics

Functions are monomorphic:
- No type parameters
- No generic collections
- Each function has fixed types

### No Concurrency (v1.0.0)

Single-threaded execution only:
- No threads
- No async/await
- No parallelism

(Deferred to v1.1.0)

---

## Syntactic Constraints

### No Keywords

Obfusku uses symbols, not words:
- No `if`, use `⟨`
- No `while`, use `⊂`
- No `function`, use `λ`

### No Semicolons

Statements are newline-terminated.  
No explicit statement separators.

### No Braces for Blocks

Blocks use symbolic delimiters:
- `⊂ ⊃` for loops
- `⟨ ⟫` for conditionals
- `λ Λ` for functions

### Mandatory Seal

Every program must end with `❧`.  
There is no exception to this rule.

---

## What Won't Be Added

These features are explicitly **out of scope**:

| Feature | Reason |
|---------|--------|
| Classes/OOP | Against symbol-first philosophy |
| Implicit conversions | Violates explicitness principle |
| Variadic functions | Complicates type model |
| Default parameters | Adds implicit behavior |
| Operator overloading | Symbols have fixed meaning |
| Nullable types | Use explicit `∅` instead |
| Package management | External tooling concern |

---

## Performance Limits

Obfusku prioritizes clarity over speed:

- **Interpreted**: No JIT (yet)
- **Linear map access**: O(n) lookup
- **No tail-call optimization**: Deep recursion may overflow
- **Copy semantics**: No references (except closures)

---

## Scale Limits

Appropriate for:
- Learning
- Experimentation
- Small scripts
- Artistic expression

Not appropriate for:
- Production services
- Performance-critical code
- Large codebases
- Team development

---

## Future Considerations

Some limitations may be relaxed in future versions:

| Feature | Version |
|---------|---------|
| Modules | v1.1.0 |
| Async | v1.1.0 |
| Type inference | v1.1.0 |
| ADTs | v1.1.0 |
| JIT | v2.0.0+ |

But the core philosophy remains:
- **Symbols are semantics**
- **Explicit over implicit**
- **Discipline is power**

---

## Accepting Constraints

Constraints are not failures.  
They are **design choices** that give Obfusku its identity.

Working within constraints develops creativity and precision.

Embrace the ritual.

---

*End of Grimoire Collection*

---

## Index

1. [Overview](00_Obfusku_Overview.md)
2. [Introduction](01_Introduction.md)
3. [Philosophy](02_Philosophy_and_Symbol_Primacy.md)
4. [Hello World](03_Hello_World.md)
5. [Execution Model](04_Execution_Model_and_VM.md)
6. [Syntax & Symbols](05_Syntax_and_Symbols.md)
7. [Values & Types](06_Values_and_Types.md)
8. [Functions](07_Functions_and_Calls.md)
9. [Arrays & Maps](08_Arrays_and_Maps.md)
10. [Pattern Matching](09_Pattern_Matching.md)
11. [Exceptions](10_Exceptions_and_Control_Flow.md)
12. [Modules](11_Modules_and_Namespaces.md)
13. [Runtime](12_Runtime_and_Memory.md)
14. [Debugging](13_Debugging_and_Error_Messages.md)
15. [Design Limits](14_Design_Limits_and_Non_Goals.md)
