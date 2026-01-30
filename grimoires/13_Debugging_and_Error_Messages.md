# Debugging and Error Messages

Understanding and resolving errors in Obfusku.

---

## Error Categories

### Compile-Time Errors

Detected during compilation.

| Symbol | Category |
|:------:|----------|
| 🔮 | Syntax / Unexpected token |
| 📜 | Missing program seal |
| 🔄 | Undefined function |
| ⚔️ | Duplicate definition |

### Runtime Errors

Detected during execution.

| Symbol | Category |
|:------:|----------|
| 🧮 | Arithmetic error |
| 🌀 | Context / scope error |
| 📊 | Index out of bounds |
| 🚫 | Invalid operation |

---

## Common Errors and Solutions

### Missing Program Seal

```
❌ 📜 Missing end program seal (❧)
```

**Cause**: Program doesn't end with `❧`.

**Fix**: Add `❧` at the end.

```obfusku
✤"Hello"
❧           // required
```

---

### Unexpected Token

```
❌ 🔮 Unexpected token '⤷' at line 5, column 1
   Expected: statement
```

**Cause**: Invalid syntax at that position.

**Common Cases**:
- Bare function call (needs assignment)
- Missing operator
- Unclosed bracket

**Fix**:
```obfusku
// Wrong:
⤷func[]

// Right:
⟁_=⤷func[]
```

---

### Variable Not Declared

```
❌ 🌀 Context corruption: Variable 'x' is not declared in this spell
```

**Cause**: Using a variable before declaring it.

**Fix**:
```obfusku
// Wrong:
⚡[x]

// Right:
⟁x=5
⚡[x]
```

---

### Variable Already Declared

```
❌ 🌀 Context corruption: Variable 'e' is already declared in this scope
```

**Cause**: Redeclaring a variable in the same scope.

**Fix**: Use a different name.
```obfusku
// Wrong:
☄
    ⚠["error1"]
☊[e]
    ⚡[e]
⟣
☄
    ⚠["error2"]
☊[e]      // conflict!
    ⚡[e]
⟣

// Right:
☊[e1]     // unique names
...
☊[e2]
```

---

### Division by Zero

```
❌ 🧮 Cannot divide by zero — arithmetic ritual disrupted
```

**Cause**: Dividing by zero.

**Fix**: Check before dividing.
```obfusku
⟨b ≠ 0]
    ⚙︎[a ÷ b]→result
⟫
```

---

### Index Out of Bounds

```
❌ 📊 Array index 10 out of bounds (length 3)
```

**Cause**: Accessing array with invalid index.

**Fix**: Ensure index is within range.

---

### Stack Overflow

```
❌ 🌀 Stack overflow: call depth exceeded
```

**Cause**: Too many nested function calls (recursion).

**Fix**: Add base case or limit recursion depth.

---

### Function Not Defined

```
❌ ❓ Function 'foo' is not defined
```

**Cause**: Calling a function that doesn't exist.

**Fix**: Define the function before calling.

---

### Return Outside Function

```
❌ 🚫 Return statement outside of function — nowhere to return to
```

**Cause**: Using `⤶` outside any function.

**Fix**: Only use return inside functions.

---

## Debug Mode

Run with `--debug` for detailed output:

```bash
obfusku run program.obk --debug
```

Shows:
- Bytecode disassembly
- Execution trace
- Stack state

---

## REPL Commands

Interactive debugging:

| Command | Purpose |
|---------|---------|
| `:help` | Show commands |
| `:debug` | Toggle debug mode |
| `:stack` | Show stack (if implemented) |
| `:symbols` | Show symbol reference |
| `:clear` | Clear screen |
| `:reset` | Reset runtime |
| `:quit` | Exit REPL |

---

## Error Message Format

Obfusku errors follow this format:

```
❌ [Symbol] [Message]
   [Context if available]
      │ [Source line]
```

Example:
```
❌ 🔮 Unexpected token ']' at line 3, column 5
   Expected: expression
      │ ⟨x ▷ ]
```

---

## Best Practices

1. **Read the full message**: Symbol indicates category
2. **Check line/column**: Error location is precise
3. **Look at context**: Source line shows the problem
4. **Check syntax**: Most errors are syntax-related
5. **Use debug mode**: When behavior is unexpected

---

*Next: [Design Limits and Non-Goals](14_Design_Limits_and_Non_Goals.md)*
