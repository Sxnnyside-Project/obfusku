# Values and Types

Understanding Obfusku's type system.

---

## Core Types

### Integer (`⟁`)

Whole numbers, 64-bit signed.

```obfusku
⟁age=25
⟁negative=-100
⟁zero=0
```

Range: -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807

### Real (`⧆`)

Floating-point numbers, 64-bit.

```obfusku
⧆pi=3.14159
⧆rate=0.05
⧆scientific=1.5e10
```

### String (`⌘`)

UTF-8 text.

```obfusku
⌘greeting="Hello, World!"
⌘unicode="символы 🎉"
⌘empty=""
```

Strings support:
- Unicode characters
- Escape sequences: `\n`, `\t`, `\\`, `\"`

### Boolean (`☍`)

Logical true/false.

```obfusku
☍active=◉     // true
☍disabled=◎   // false
```

Values:
- `◉` — true
- `◎` — false

### Rune (`ᚱ`)

Single Unicode character.

```obfusku
ᚱletter='A'
ᚱsymbol='★'
```

---

## Collection Types

### Array (`⌬`)

Ordered, indexable collection.

```obfusku
⌬numbers=[1, 2, 3, 4, 5]
⌬mixed=[1, "two", ◉]
⌬empty=[]
```

Operations:
- Access: `arr⌷index` (index-based)
- Length: Built-in function

### Map (`⌖`)

Key-value dictionary.

```obfusku
⌖person={
    "name" ⇒ "Alice"⋄
    "age" ⇒ 30
}
```

Keys must be hashable: integers, strings, booleans, runes.

---

## Special Values

### Null (`∅`)

Absence of value.

```obfusku
⟁∅maybe       // optional integer, initially null
⚡[∅]          // outputs: ∅
```

Used for:
- Optional values
- Uninitialized state
- Function with no return

---

## Type Checking

Obfusku is **statically typed** at declaration but **dynamically checked** at runtime.

### Declaration Type

```obfusku
⟁x=5          // x is integer
⌘s="hello"    // s is string
```

### Runtime Checks

Operations verify types:

```obfusku
⟁a=5
⟁b=3
⟁sum=a ✚ b    // OK: integer + integer

⌘s="hi"
⟁n=s ✚ 5      // ERROR: type mismatch
```

---

## Type Conversion

Explicit conversion is required between types.

```obfusku
⟁n=42
⌘s=⤷to_string[n]  // Convert to string (if function exists)
```

**Note**: Built-in conversion functions are limited in v1.0.0.

---

## Truthiness

Values have implicit boolean meaning in conditions:

| Type | Truthy | Falsy |
|------|--------|-------|
| Integer | Non-zero | 0 |
| Real | Non-zero | 0.0 |
| String | Non-empty | "" |
| Boolean | `◉` | `◎` |
| Array | Non-empty | [] |
| Map | Non-empty | {} |
| Null | — | Always falsy |

```obfusku
⟁x=5
⟨x]              // truthy if x != 0
    ✤"x is truthy"
⟫
```

---

## Value Display

How values appear when output:

| Type | Display |
|------|---------|
| Integer | `42` |
| Real | `3.14` |
| String | `hello` |
| Boolean | `◉` or `◎` |
| Array | `⌬[5]` (showing length) |
| Map | `⌖{3}` (showing size) |
| Null | `∅` |
| Function | `λ#0` |
| Closure | `λ⊃#0` |

---

## Type Inference

Variable type is inferred from the declaration symbol:

```obfusku
⟁x=5        // integer
⧆y=3.14     // real
⌘z="hi"     // string
```

The symbol determines the expected type.

---

*Next: [Functions and Calls](07_Functions_and_Calls.md)*
