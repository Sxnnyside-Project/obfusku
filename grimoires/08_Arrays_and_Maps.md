# Arrays and Maps

Working with collections in Obfusku.

---

## Arrays (`⌬`)

Ordered, indexable collections of values.

### Declaration

```obfusku
⌬numbers=[1, 2, 3, 4, 5]
⌬names=["Alice", "Bob", "Carol"]
⌬empty=[]
```

Elements are separated by spaces or commas.

### Mixed Types

Arrays can contain different types:

```obfusku
⌬mixed=[1, "two", ◉, 3.14]
```

### Display

When output, arrays show their length:

```obfusku
⌬arr=[1, 2, 3]
⚡[arr]     // outputs: ⌬[3]
```

---

## Array Operations

### Access by Index

```obfusku
⌬arr=[10, 20, 30]
// Access: arr⌷index
```

**Note**: Direct index access syntax may vary. Check implementation.

### Negative Indexing

Negative indices count from the end:

- `-1` = last element
- `-2` = second to last

### Bounds Checking

Out-of-bounds access raises an error:

```
📊 Array index 10 out of bounds (length 3)
```

---

## Maps (`⌖`)

Key-value dictionaries.

### Declaration

```obfusku
⌖person={
    "name" ⇒ "Alice"⋄
    "age" ⇒ 30⋄
    "active" ⇒ ◉
}
```

Syntax:
- `⇒` separates key from value
- `⋄` separates entries
- Last entry doesn't need `⋄`

### Valid Keys

Keys must be hashable:
- Integers
- Strings
- Booleans
- Runes
- Null

### Display

When output, maps show their size:

```obfusku
⌖m={"a"⇒1⋄ "b"⇒2}
⚡[m]     // outputs: ⌖{2}
```

---

## Map Operations

### Get Value

```obfusku
⌖m={"key"⇒"value"}
// Access: m⌷"key"
```

### Check Existence

Maps have a `has` operation:

```obfusku
// Check if key exists
```

### Keys and Values

Extract all keys or values:

```obfusku
// Get all keys as array
// Get all values as array
```

---

## Common Patterns

### Iteration with Accumulator

Process array elements using the accumulator:

```obfusku
⌬arr=[1, 2, 3, 4, 5]
✹=5
⊂[✹ ▷ 0]
    // Process element at index (5 - ✹)
    ✹⊖
⊃
```

### Building Arrays

```obfusku
⌬result=[]
// Add elements through operations
```

### Nested Collections

```obfusku
⌬matrix=[
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9]
]

⌖nested={
    "users" ⇒ [
        {"name"⇒"Alice"}⋄
        {"name"⇒"Bob"}
    ]
}
```

---

## Examples

### Sum Array Elements

```obfusku
⌬numbers=[1, 2, 3, 4, 5]
⟁sum=0
// Iterate and accumulate
// sum = 15
```

### Map Lookup

```obfusku
⌖colors={
    "red" ⇒ "#FF0000"⋄
    "green" ⇒ "#00FF00"⋄
    "blue" ⇒ "#0000FF"
}

// Look up "red" → "#FF0000"
```

### Spellbook Pattern

```obfusku
⌖spellbook={
    "fireball" ⇒ 50⋄
    "heal" ⇒ 30⋄
    "shield" ⇒ 20
}
```

---

## Limitations

- No built-in map/filter/reduce in v1.0.0
- Map iteration requires manual key extraction
- Array concatenation through re-declaration

---

*Next: [Pattern Matching](09_Pattern_Matching.md)*
