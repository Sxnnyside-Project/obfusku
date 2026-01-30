# Introduction to Obfusku

Welcome to Obfusku — The Magical Programming Language.

---

## Getting Started

### Installation

```bash
# Clone the repository
git clone https://github.com/Sxnnyside-Project/obfusku.git
cd obfusku/obfusku

# Build
cargo build --release

# Run
./target/release/obfusku --help
```

### Your First Spell

Create a file named `hello.obk`:

```obfusku
✤"Hello, World!"
❧
```

Run it:

```bash
obfusku run hello.obk
```

Output:
```
Hello, World!
✨ Spell complete!
```

---

## Program Structure

Every Obfusku program:

1. **Contains statements** — declarations, expressions, control flow
2. **Must end with `❧`** — the seal that closes the ritual

```obfusku
// Comments start with //
⟁x=10        // Declare integer x
⚡[x]         // Output x
❧            // End program (required)
```

---

## Running Programs

### Execute a File

```bash
obfusku run program.obk
```

### Interactive REPL

```bash
obfusku repl
```

In the REPL:
- Enter statements line by line
- End with `❧` to execute
- Type `:help` for commands
- Type `:quit` to exit

### Debug Mode

```bash
obfusku run program.obk --debug
```

Shows bytecode and execution trace.

---

## File Extensions

| Extension | Purpose |
|-----------|---------|
| `.obk` | Obfusku source code |
| `.obc` | Compiled bytecode |

### Compiling to Bytecode

```bash
obfusku compile program.obk --output program.obc
obfusku load program.obc
```

---

## Key Concepts

### 1. Everything is Symbolic

There are no keywords like `if`, `while`, `function`.  
Instead: `⟨`, `⊂`, `λ`.

### 2. Variables Have Types

Declare with type symbol:

```obfusku
⟁count=0      // Integer
⌘name="Ada"   // String
☍active=◉    // Boolean (true)
```

### 3. Programs Are Sealed

Every program must end with `❧`.  
Without it, the spell is incomplete and will not execute.

---

## Next Steps

- [Philosophy & Symbol Primacy](02_Philosophy_and_Symbol_Primacy.md)
- [Hello World Examples](03_Hello_World.md)
- [Complete Syntax Reference](05_Syntax_and_Symbols.md)
