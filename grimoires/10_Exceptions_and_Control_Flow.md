# Exceptions and Control Flow

Handling disruptions and controlling program flow.

---

## Exception Handling

Obfusku provides exception handling with `☄ ☊ ⚠ ⟣`.

### Basic Try-Catch

```obfusku
☄
    // risky code
    ⚠["Something went wrong!"]
☊[error]
    // handle error
    ⚡[error]
⟣
```

- `☄` — begin protection ritual (try)
- `⚠[value]` — invoke disruption (throw)
- `☊[var]` — containment circle (catch) with binding
- `⟣` — end protection block

---

## Throwing Exceptions

Use `⚠` to throw any value as an exception:

```obfusku
⚠["Error message"]           // throw string
⚠[42]                        // throw integer
⚠[{"type"⇒"Error"⋄"msg"⇒"Failed"}]  // throw map
```

When thrown:
1. Execution stops at `⚠`
2. Stack unwinds to nearest handler
3. Handler receives the thrown value

---

## Catching Exceptions

The catch block receives the exception value:

```obfusku
☄
    ⚠["Spell backfired!"]
☊[e]
    ✤"Caught:"
    ⚡[e]      // outputs: Spell backfired!
⟣
```

**Important**: Use unique variable names in each catch block.

---

## Nested Exception Handling

Handlers can be nested:

```obfusku
☄
    ✤"Outer try"
    ☄
        ✤"Inner try"
        ⚠["Inner error"]
    ☊[inner_e]
        ✤"Inner caught:"
        ⚡[inner_e]
    ⟣
    ✤"Outer continues"
☊[outer_e]
    ✤"Outer caught"
⟣
```

Output:
```
Outer try
Inner try
Inner caught:
Inner error
Outer continues
```

---

## Stack Unwinding

When an exception is thrown:

1. **Find handler**: Search for active `☄` block
2. **Restore stack**: Pop values to handler's depth
3. **Unwind frames**: Pop call frames if needed
4. **Jump to catch**: Execute `☊` block

### Across Function Calls

```obfusku
λrisky[]
    ⚠["Error in risky"]
    ⤶[∅]
Λ

☄
    ⟁_=⤷risky[]
☊[e]
    ⚡[e]     // catches error from risky
⟣
```

---

## Unhandled Exceptions

Exceptions without a handler cause program termination:

```obfusku
⚠["Unhandled!"]   // no ☄ block

// Output:
// ❌ Type mismatch: expected "exception handler", got "unhandled exception: Unhandled!"
```

---

## Conditional Throwing

Combine with conditionals:

```obfusku
λdivide[⟁a, ⟁b]
    ⟨b ⩵ 0]
        ⚠["Division by zero!"]
    ⟫
    ⤶[a ÷ b]
Λ

☄
    ⟁result=⤷divide[10, 0]
☊[e]
    ✤"Error:"
    ⚡[e]
⟣
```

---

## Control Flow Symbols

### Loops

| Symbol | Purpose |
|:------:|---------|
| `⊂` | Loop start |
| `⊃` | Loop end |
| `↯` | Break (exit loop) |
| `↻` | Continue (next iteration) |

```obfusku
✹=5
⊂[✹ ▷ 0]
    ⟨✹ ⩵ 3]
        ↯         // break when ✹ is 3
    ⟫
    ⚡[✹]
    ✹⊖
⊃
```

Output:
```
5
4
```

### Conditionals

| Symbol | Purpose |
|:------:|---------|
| `⟨` | If start |
| `]` | Condition end |
| `⟩` | Else |
| `⟫` | End if |

```obfusku
⟨condition]
    // then
⟩
    // else
⟫
```

---

## The Accumulator (`✹`)

A special counter for loops:

```obfusku
✹=10         // set to 10
✹⊕          // increment
✹⊖          // decrement
⚡[✹]        // read value
```

### Accumulator Loop Pattern

```obfusku
✹=5
⊂[✹ ▷ 0]
    ⚡[✹]
    ✹⊖
⊃
```

Output: `5 4 3 2 1`

---

## Finally Block (`☋`)

**Note**: Finally blocks (`☋`) are parsed but have limited support in v1.0.0.

```obfusku
☄
    // try code
☊[e]
    // catch code
☋
    // finally code (always runs)
⟣
```

---

## Best Practices

1. **Always handle errors**: Don't let exceptions propagate unexpectedly
2. **Use descriptive errors**: Throw meaningful messages or structured data
3. **Clean up resources**: Use finally or explicit cleanup
4. **Don't overuse**: Exceptions are for exceptional cases

---

*Next: [Modules and Namespaces](11_Modules_and_Namespaces.md)*
