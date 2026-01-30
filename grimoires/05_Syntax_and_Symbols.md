# Syntax and Symbols

Complete reference for all Obfusku symbols.

---

## Type Symbols

| Symbol | Name | Purpose | Example |
|:------:|------|---------|---------|
| `⟁` | Integer | Whole numbers | `⟁x=42` |
| `⧆` | Real | Floating point | `⧆pi=3.14` |
| `⌘` | String | Text | `⌘name="Ada"` |
| `☍` | Boolean | True/false | `☍flag=◉` |
| `ᚱ` | Rune | Single character | `ᚱc='a'` |

---

## Collection Symbols

| Symbol | Name | Purpose | Example |
|:------:|------|---------|---------|
| `⌬` | Array | Ordered list | `⌬arr=[1, 2, 3]` |
| `⌖` | Map | Key-value pairs | `⌖m={"a"⇒1}` |

---

## Boolean Values

| Symbol | Meaning |
|:------:|---------|
| `◉` | True |
| `◎` | False |
| `∅` | Null |

---

## Arithmetic Operators

| Symbol | Operation | Example |
|:------:|-----------|---------|
| `✚` | Addition | `a ✚ b` |
| `✖` | Subtraction | `a ✖ b` |
| `✱` | Multiplication | `a ✱ b` |
| `÷` | Division | `a ÷ b` |
| `⌑` | Modulo | `a ⌑ b` |

---

## Comparison Operators

| Symbol | Operation | Example |
|:------:|-----------|---------|
| `⩵` | Equal | `a ⩵ b` |
| `≠` | Not equal | `a ≠ b` |
| `◁` | Less than | `a ◁ b` |
| `▷` | Greater than | `a ▷ b` |
| `⊴` | Less or equal | `a ⊴ b` |
| `⊵` | Greater or equal | `a ⊵ b` |

---

## Logical Operators

| Symbol | Operation | Example |
|:------:|-----------|---------|
| `∧` | Logical AND | `a ∧ b` |
| `∨` | Logical OR | `a ∨ b` |
| `¬` | Logical NOT | `¬a` |

---

## Control Flow

### Conditionals

| Symbol | Purpose |
|:------:|---------|
| `⟨` | If (condition start) |
| `]` | Condition end |
| `⟩` | Else |
| `⟫` | End if |

```obfusku
⟨condition]
    // then branch
⟩
    // else branch
⟫
```

### Loops

| Symbol | Purpose |
|:------:|---------|
| `⊂` | Loop start |
| `⊃` | Loop end |
| `↯` | Break |
| `↻` | Continue |

```obfusku
⊂[condition]
    // loop body
⊃
```

### Pattern Matching

| Symbol | Purpose |
|:------:|---------|
| `⟡` | Match start |
| `⟢` | Match arm |
| `⟣` | Match end |
| `◇` | Wildcard |

```obfusku
⟡value]
    ⟢pattern1] action1
    ⟢pattern2] action2
    ⟢◇] default
⟣
```

---

## Functions

| Symbol | Purpose |
|:------:|---------|
| `λ` | Function definition start |
| `Λ` | Function definition end |
| `⤷` | Function call |
| `⤶` | Return |

```obfusku
λname[params]
    // body
    ⤶[value]
Λ

⟁result=⤷name[args]
```

---

## Exception Handling

| Symbol | Purpose |
|:------:|---------|
| `☄` | Try block start |
| `☊` | Catch block |
| `☋` | Finally block |
| `⚠` | Throw exception |
| `⟣` | End try-catch |

```obfusku
☄
    // risky code
    ⚠["error"]
☊[e]
    // handle error
⟣
```

---

## Input/Output

| Symbol | Purpose | Example |
|:------:|---------|---------|
| `⚡` | Output value | `⚡[x]` |
| `✤` | Print literal | `✤"Hello"` |
| `⚓` | Read input | `⚓⌘name` |

---

## Accumulator

| Symbol | Purpose |
|:------:|---------|
| `✹` | Accumulator reference |
| `✹=n` | Set accumulator |
| `✹⊕` | Increment |
| `✹⊖` | Decrement |

```obfusku
✹=5
⊂[✹ ▷ 0]
    ⚡[✹]
    ✹⊖
⊃
```

---

## Assignment

| Symbol | Purpose | Example |
|:------:|---------|---------|
| `=` | Simple assign | `⟁x=5` |
| `→` | Arrow assign | `⚙︎[expr]→var` |

---

## Map Syntax

| Symbol | Purpose |
|:------:|---------|
| `⇒` | Key-value separator |
| `⋄` | Entry separator |

```obfusku
⌖map={
    "key1" ⇒ value1⋄
    "key2" ⇒ value2
}
```

---

## Special Symbols

| Symbol | Purpose |
|:------:|---------|
| `❧` | End program (required) |
| `//` | Line comment |
| `,` | Parameter separator |

---

## Brackets

| Symbol | Context |
|:------:|---------|
| `[` `]` | Parameters, conditions, expressions |
| `{` `}` | Map literals |
| `(` `)` | Grouping (optional) |

---

*Next: [Values and Types](06_Values_and_Types.md)*
