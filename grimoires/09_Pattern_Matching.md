# Pattern Matching

Matching values against patterns with `⟡ ⟢ ⟣`.

---

## Basic Syntax

```obfusku
⟡value]
    ⟢pattern1] action1
    ⟢pattern2] action2
    ⟢◇] default_action
⟣
```

- `⟡value]` — start match on value
- `⟢pattern]` — match arm
- `◇` — wildcard (matches anything)
- `⟣` — end match

---

## Simple Matching

### Integer Matching

```obfusku
⟁x=2

⟡x]
    ⟢1] ✤"one"
    ⟢2] ✤"two"
    ⟢3] ✤"three"
    ⟢◇] ✤"other"
⟣
```

Output: `two`

### String Matching

```obfusku
⌘cmd="start"

⟡cmd]
    ⟢"start"] ✤"Starting..."
    ⟢"stop"] ✤"Stopping..."
    ⟢"status"] ✤"Running"
    ⟢◇] ✤"Unknown command"
⟣
```

---

## The Wildcard (`◇`)

The wildcard matches any value not matched by previous arms.

```obfusku
⟁n=42

⟡n]
    ⟢0] ✤"zero"
    ⟢1] ✤"one"
    ⟢◇] ✤"something else"   // matches 42
⟣
```

**Best Practice**: Always include a wildcard for exhaustive matching.

---

## Match in Functions

Pattern matching works well inside functions:

```obfusku
λday_name[⟁day]
    ⟡day]
        ⟢1] ⤶["Monday"]
        ⟢2] ⤶["Tuesday"]
        ⟢3] ⤶["Wednesday"]
        ⟢4] ⤶["Thursday"]
        ⟢5] ⤶["Friday"]
        ⟢6] ⤶["Saturday"]
        ⟢7] ⤶["Sunday"]
        ⟢◇] ⤶["Invalid"]
    ⟣
Λ

⌘name=⤷day_name[3]
⚡[name]   // Wednesday
```

---

## Matching Boolean

```obfusku
☍active=◉

⟡active]
    ⟢◉] ✤"Active"
    ⟢◎] ✤"Inactive"
⟣
```

---

## Element Selection

Use matching for element-based logic:

```obfusku
⟁element=2

⟡element]
    ⟢1] ✤"🔥 Fire"
    ⟢2] ✤"💧 Water"
    ⟢3] ✤"🌍 Earth"
    ⟢4] ✤"💨 Air"
    ⟢◇] ✤"❓ Unknown"
⟣
```

---

## Match vs Conditional

### Use Conditional (`⟨⟫`) When:
- Binary choice (true/false)
- Range comparisons
- Complex conditions

```obfusku
⟨x ▷ 0 ∧ x ◁ 100]
    ✤"in range"
⟫
```

### Use Match (`⟡⟣`) When:
- Multiple discrete values
- Exhaustive case handling
- Value-based dispatch

```obfusku
⟡status]
    ⟢0] ✤"pending"
    ⟢1] ✤"active"
    ⟢2] ✤"complete"
    ⟢◇] ✤"unknown"
⟣
```

---

## Execution Semantics

1. Value is evaluated once
2. Arms are checked in order
3. First matching arm executes
4. Match ends after first match
5. If no match and no wildcard: undefined behavior

---

## Nested Matching

```obfusku
⟁outer=1
⟁inner=2

⟡outer]
    ⟢1]
        ⟡inner]
            ⟢1] ✤"1-1"
            ⟢2] ✤"1-2"
            ⟢◇] ✤"1-?"
        ⟣
    ⟢2] ✤"2-*"
    ⟢◇] ✤"?-*"
⟣
```

Output: `1-2`

---

## Limitations

In v1.0.0:
- No destructuring patterns
- No guards on arms
- No range patterns
- Patterns must be literal values

---

*Next: [Exceptions and Control Flow](10_Exceptions_and_Control_Flow.md)*
