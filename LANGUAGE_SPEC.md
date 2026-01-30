# Obfusku Language Specification

**Version**: 1.0.0  
**Status**: Normative  
**Last Updated**: January 30, 2026

---

## 1. Introduction and Scope

### 1.1 Purpose

This document specifies the Obfusku programming language as implemented in version 1.0.0.

It defines:
- Lexical structure
- Semantic behavior
- Runtime requirements
- Error conditions

It does NOT define:
- Library APIs (beyond core language)
- Optimization strategies
- Implementation details not affecting semantics
- Features deferred to v1.1.0+

### 1.2 Conformance

A compliant Obfusku implementation MUST:
1. Recognize all symbols defined in this specification
2. Implement all control flow constructs as specified
3. Provide stack-based execution with defined semantics
4. Produce defined error messages for specified error conditions
5. Maintain variable scope as defined

### 1.3 Normative References

- Unicode Standard 13.0 or later
- IEEE 754 for floating-point arithmetic

---

## 2. Lexical Structure

### 2.1 Source Code Encoding

Obfusku source code MUST be UTF-8 encoded.

All Unicode characters are permitted in:
- String literals
- Comments
- Identifiers (non-symbolic)

### 2.2 Whitespace and Line Breaks

Whitespace (space, tab, newline) MUST be recognized between tokens.

Newlines:
- Separate statements
- Have no other syntactic significance
- May appear anywhere whitespace is valid

### 2.3 Comments

Line comments begin with `//` and extend to end of line.

Comments MAY appear anywhere whitespace is valid.

### 2.4 Symbols

Obfusku uses Unicode symbols as primary tokens. All symbols defined in Section 5 MUST be recognized.

### 2.5 Identifiers

Identifiers:
- Begin with letter or underscore
- Contain letters, digits, underscores
- Are case-sensitive
- MUST NOT be empty

### 2.6 Literals

#### 2.6.1 Integer Literals

Integer literals are sequences of decimal digits, optionally preceded by `-`.

Range: -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807 (64-bit signed)

#### 2.6.2 Real Literals

Real literals consist of:
- Optional sign
- Digits, decimal point, digits
- Optional exponent (e or E followed by optional sign and digits)

Example: `3.14`, `-2.5e10`, `1e-5`

#### 2.6.3 String Literals

String literals are enclosed in double quotes.

Escape sequences:
- `\"` = double quote
- `\\` = backslash
- `\n` = newline
- `\t` = tab

All other sequences are literal.

#### 2.6.4 Rune Literals

Rune literals are single characters enclosed in single quotes.

Range: All Unicode scalar values

#### 2.6.5 Array Literals

Array literals use syntax: `[` elements `]`

Elements are separated by spaces or commas.

Empty arrays are valid: `[]`

#### 2.6.6 Map Literals

Map literals use syntax: `{ key ⇒ value ... }`

Entries are separated by `⋄`.

---

## 3. Execution Model

### 3.1 Stack-Based Virtual Machine

Obfusku execution uses a stack-based virtual machine.

#### 3.1.1 Stack

The execution stack:
- Stores operands for operations
- Has maximum capacity of 4096 values
- Exceeding capacity causes stack overflow error

#### 3.1.2 Call Stack

Function invocations create call frames.

- Maximum depth: 1024 frames
- Exceeding causes call stack overflow error
- Each frame has its own local scope

#### 3.1.3 Instruction Pointer

Current instruction is tracked by instruction pointer (ip).

### 3.2 Program Execution

A program:
1. MUST end with the `❧` symbol
2. MUST initialize as main entry point
3. Executes sequentially until end or return

Without `❧`, the program is malformed.

### 3.3 Compilation

Source code is compiled to bytecode before execution.

Compilation MUST detect and report:
- Syntax errors
- Undefined symbols
- Duplicate definitions
- Type mismatches (where statically detectable)

---

## 4. Values and Runtime Types

### 4.1 Value Types

Obfusku has the following runtime value types:

| Type | Representation | Notes |
|------|----------------|-------|
| Integer | 64-bit signed | -2^63 to 2^63-1 |
| Real | IEEE 754 64-bit | Float |
| String | UTF-8 sequence | Mutable length |
| Boolean | true/false | `◉` = true, `◎` = false |
| Rune | Unicode scalar | Single character |
| Array | Ordered sequence | Homogeneous or mixed |
| Map | Key-value pairs | Keys must be hashable |
| Null | Absence | Singleton value `∅` |
| Function | Index reference | Refers to function definition |
| Closure | Function + captures | Captured environment |

### 4.2 Type Behavior

#### 4.2.1 Integer

Integer operations:
- `✚` (add): overflow behavior is undefined
- `✖` (subtract): overflow behavior is undefined
- `✱` (multiply): overflow behavior is undefined
- `÷` (divide): division by zero raises error
- `⌑` (modulo): modulo by zero raises error

#### 4.2.2 Real

Real operations follow IEEE 754:
- `÷` by zero produces `Infinity` or `-Infinity`
- Invalid operations produce `NaN`

#### 4.2.3 String

Strings are sequences of UTF-8 code units.

Strings are immutable.

Operations:
- Concatenation via `✚`: produces new string
- Comparison: lexicographic ordering

#### 4.2.4 Array

Arrays are ordered, indexable sequences.

Indexing:
- Positive: 0-based from start
- Negative: -1 is last element
- Out of bounds raises error

#### 4.2.5 Map

Maps are key-value mappings.

Valid keys:
- Integer
- String
- Boolean
- Rune
- Null

Keys not in this set raise error.

Access to missing key is undefined (implementation may return null or error).

### 4.3 Truthiness

Values in boolean context (conditionals):

| Type | Truthy | Falsy |
|------|--------|-------|
| Integer | non-zero | 0 |
| Real | non-zero | 0.0 |
| String | non-empty | "" |
| Boolean | true | false |
| Array | non-empty | [] |
| Map | non-empty | {} |
| Null | — | always falsy |

### 4.4 Type Conversion

Explicit type conversion is required.

No implicit conversion between types occurs (except in comparison operators which may unify numeric types).

---

## 5. Symbol Semantics

### 5.1 Type Declaration Symbols

| Symbol | Type | Usage |
|:------:|------|-------|
| `⟁` | Integer | `⟁x=5` |
| `⧆` | Real | `⧆pi=3.14` |
| `⌘` | String | `⌘s="hello"` |
| `☍` | Boolean | `☍b=◉` |
| `ᚱ` | Rune | `ᚱc='a'` |

When a variable is declared with a type symbol, that variable's type is fixed.

### 5.2 Collection Symbols

| Symbol | Type | Syntax |
|:------:|------|--------|
| `⌬` | Array | `⌬arr=[1, 2, 3]` |
| `⌖` | Map | `⌖m={"k"⇒"v"}` |

### 5.3 Boolean Values

| Symbol | Meaning |
|:------:|---------|
| `◉` | True (boolean true) |
| `◎` | False (boolean false) |
| `∅` | Null |

### 5.4 Arithmetic Operators

| Symbol | Operation | Operands | Result |
|:------:|-----------|----------|--------|
| `✚` | Addition | Integer ⊕ Integer → Integer; Real ⊕ Real → Real; String ⊕ String → String | Type depends on operands |
| `✖` | Subtraction | Integer, Integer → Integer; Real, Real → Real | Type depends on operands |
| `✱` | Multiplication | Integer, Integer → Integer; Real, Real → Real | Type depends on operands |
| `÷` | Division | Integer, Integer → Integer; Real, Real → Real | Type depends on operands |
| `⌑` | Modulo | Integer, Integer → Integer | Integer |

Mixing Integer and Real: operation produces Real.

### 5.5 Comparison Operators

| Symbol | Semantics |
|:------:|-----------|
| `⩵` | Equality: true if values are equal |
| `≠` | Inequality: true if values differ |
| `◁` | Less than: numeric only |
| `▷` | Greater than: numeric only |
| `⊴` | Less or equal: numeric only |
| `⊵` | Greater or equal: numeric only |

Comparison of incompatible types raises error.

### 5.6 Logical Operators

| Symbol | Semantics |
|:------:|-----------|
| `∧` | AND: both operands truthy → true |
| `∨` | OR: either operand truthy → true |
| `¬` | NOT: operand falsy → true |

Operands are evaluated according to Section 4.3 (truthiness).

---

## 6. Functions and Calls

### 6.1 Function Definition

Function definition syntax:

```
λ name [ params ]
    statements
Λ
```

- `λ` begins function
- `Λ` ends function
- Parameters are declared with type symbols
- Function body executes sequentially

### 6.2 Parameter Semantics

Parameters:
- Are local variables in function scope
- Have fixed types as declared
- Receive values from arguments

Number of arguments MUST match number of parameters.

### 6.3 Function Invocation

Invocation syntax: `⤷ name [ args ]`

Evaluation:
1. Arguments are evaluated left to right
2. Function is looked up
3. New call frame is created
4. Arguments are bound to parameters
5. Function body executes
6. Return value is pushed to stack

### 6.4 Return Statement

Return statement syntax: `⤶ [ value ]`

- MUST appear only within function body
- Terminates function execution
- Optional value becomes return value (default: `∅`)

### 6.5 First-Class Functions

Functions may be:
- Stored in variables
- Passed as arguments
- Returned from functions

When a function identifier appears as a value (not immediately followed by `[`), it loads the function reference.

### 6.6 Closures

A closure captures variables from enclosing scopes.

Capture semantics:
- Values are captured BY VALUE at closure creation time
- Captured values are stored with closure
- Each closure instance has independent captures

Variable lookup in closure:
1. Check local variables
2. Check captures
3. Check global scope

---

## 7. Control Flow and Pattern Matching

### 7.1 Conditional Expression

Syntax: `⟨ condition ] ... ⟩ ... ⟫`

- `⟨` begins conditional
- `]` ends condition
- `⟩` begins else branch (optional)
- `⟫` ends conditional

Execution:
1. Condition is evaluated to boolean
2. If true: then branch executes
3. If false and else exists: else branch executes
4. Continues after `⟫`

### 7.2 Loop

Syntax: `⊂ [ condition ] ... ⊃`

- `⊂` begins loop
- Condition is evaluated before each iteration
- Body executes if condition is truthy
- After body, jumps back to condition
- When condition is falsy, continues after `⊃`

### 7.3 Break and Continue

| Symbol | Semantics |
|:------:|-----------|
| `↯` | Exit loop immediately |
| `↻` | Jump to next iteration (condition re-evaluation) |

These MUST appear only within loop body.

### 7.4 Pattern Matching

Syntax: `⟡ value ] ⟢ pattern ] ... ⟣`

- `⟡` begins match
- `⟢` begins match arm
- `◇` is wildcard (matches any value)
- `⟣` ends match

Execution:
1. Value is evaluated once
2. Arms are checked in order
3. First matching arm executes
4. Remaining arms are skipped
5. If no arm matches and no wildcard: undefined behavior

---

## 8. Exception Semantics

### 8.1 Exception Handling

Syntax: `☄ ... ☊ [ var ] ... ⟣`

- `☄` begins try block
- `☊` begins catch block with exception binding
- `⟣` ends try-catch

Execution:
1. Try block executes
2. If exception thrown: catch block executes
3. Exception value is bound to variable
4. Continues after `⟣`

### 8.2 Throw Statement

Syntax: `⚠ [ value ]`

- Throws value as exception
- Unwinds stack to nearest handler
- If no handler exists: program terminates with error

### 8.3 Stack Unwinding

On throw:
1. Handler stack is searched for active handler
2. Execution stack is popped to handler's depth
3. Call frames are popped to handler's frame depth
4. Jump to handler's catch block

### 8.4 Finally Block

Syntax: `☄ ... ☊ [ var ] ... ☋ ... ⟣`

The finally block (`☋`):
- Is parsed but execution is compiler-controlled
- SHOULD execute in normal exit and exception cases
- Implementation note: Currently has limited support

---

## 9. Modules and Namespaces

### 9.1 Status

Module system in v1.0.0:
- SYNTAX is defined and parsed
- RUNTIME loading is NOT implemented
- Deferred to v1.1.0

### 9.2 Import Syntax

`⟲ "module_name"`

Parsed but not executed.

### 9.3 Export Syntax

`⟳ symbol_name`

Parsed but not executed.

### 9.4 Module Access

`module ⊷ symbol`

Parsed but not executed.

---

## 10. Runtime Errors and Undefined Behavior

### 10.1 Defined Error Conditions

These errors MUST be detected and reported:

| Error | Condition |
|-------|-----------|
| Stack overflow | > 4096 values on stack |
| Call stack overflow | > 1024 frames |
| Division by zero | `÷` or `⌑` by zero |
| Index out of bounds | Array/map index invalid |
| Undefined variable | Variable not declared |
| Undefined function | Function not defined |
| Type mismatch | Incompatible types in operation |
| Return outside function | `⤶` not in function |
| Unhandled exception | `⚠` with no handler |
| Missing program seal | No `❧` at end |

### 10.2 Undefined Behavior

These conditions have undefined behavior:

| Condition |
|-----------|
| Integer overflow in arithmetic |
| Real precision loss |
| Map access with missing key |
| Pattern match with no matching arm and no wildcard |
| Function call with wrong arity (not checked at compile time in all cases) |
| Comparison of incompatible types |

Implementations MAY:
- Raise an error
- Return null
- Perform automatic conversion
- Crash

---

## 11. Versioning and Compatibility Guarantees

### 11.1 Version Scheme

Obfusku uses semantic versioning: MAJOR.MINOR.PATCH

### 11.2 v1.0.0 Guarantees

v1.0.0 semantics are FROZEN:

- Symbol meanings MUST NOT change
- No breaking syntax changes
- No VM behavior changes
- Future patches are backward compatible

### 11.3 Future Versions

v1.1.0 and later MAY add:
- New symbols
- New types
- New constructs

But MUST NOT break v1.0.0 programs.

### 11.4 Deprecated Features

None in v1.0.0.

---

## 12. Normative Summary

A compliant Obfusku v1.0.0 implementation MUST:

1. ✅ Accept UTF-8 source with all defined symbols
2. ✅ Implement stack-based execution model
3. ✅ Provide all value types and operations
4. ✅ Enforce function definition and call semantics
5. ✅ Implement control flow: conditionals, loops, pattern matching
6. ✅ Implement exception handling: try-catch-finally
7. ✅ Enforce variable scope rules
8. ✅ Implement closure capture (by value)
9. ✅ Report defined error conditions
10. ✅ Require `❧` to seal programs

---

**End of Specification**
