# Modules and Namespaces

Module system design and current status.

---

## Current Status

> **Note**: The module system has **syntax support** in v1.0.0, but **runtime loading is deferred to v1.1.0**.

The syntax is defined and parseable, but modules are not actually loaded or executed.

---

## Module Syntax

### Import

```obfusku
⟲"module_name"
```

- `⟲` — import symbol
- Module name in quotes

### Export

```obfusku
⟳symbol_name
```

- `⟳` — export symbol
- Marks a symbol as publicly available

### Module Access

```obfusku
module⊷symbol
```

- `⊷` — member access operator

---

## Planned Design

### Module File (`math.obk`)

```obfusku
⟳square    // export square function

λsquare[⟁n]
    ⤶[n ✱ n]
Λ

❧
```

### Usage (`main.obk`)

```obfusku
⟲"math"

⟁result=⤷math⊷square[7]
⚡[result]

❧
```

---

## File Extensions

| Extension | Purpose |
|-----------|---------|
| `.obk` | Obfusku source file |
| `.obc` | Compiled bytecode |
| `.obx` | Standard library package |

---

## Search Paths

When implemented, module resolution will search:

1. Current directory
2. Configured search paths
3. Standard library location

---

## Namespace Isolation

Each module will have:
- Its own global scope
- Only exported symbols visible externally
- No implicit namespace pollution

---

## Demonstration Example

This example shows the syntax (but doesn't actually load modules):

```obfusku
// modules.obk
✤"Module system syntax is ready!"
✤"Use ⟲ followed by module name in quotes"
✤"Module loading will search:"
✤"  1. Current directory"
✤"  2. Configured search paths"
✤"  3. Standard library location"
❧
```

---

## What Works Now

✅ Syntax is recognized  
✅ Opcodes exist (`Import`, `Export`, `LoadModule`)  
✅ Module loader infrastructure exists  
❌ Actual module loading at runtime  
❌ Namespace isolation enforcement  
❌ Circular dependency handling  

---

## Future (v1.1.0)

The module system will provide:

- Actual file loading and compilation
- Namespace isolation
- Symbol export/import binding
- Search path configuration
- Standard library packages

---

## Workaround for v1.0.0

Until modules are implemented, you can:

1. **Copy code**: Include needed functions directly
2. **Single file**: Keep all code in one file
3. **Preprocessing**: Use external tools to combine files

---

*Next: [Runtime and Memory](12_Runtime_and_Memory.md)*
