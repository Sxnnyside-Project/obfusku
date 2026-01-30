# Functions and Calls

Defining and invoking rituals in Obfusku.

---

## Function Definition

Functions are defined with `λ` and sealed with `Λ`:

```obfusku
λfunction_name[parameters]
    // body
Λ
```

### Basic Example

```obfusku
λsquare[⟁n]
    ⤶[n ✱ n]
Λ
```

### Parameters

Parameters are declared with type symbols:

```obfusku
λgreet[⌘name, ⟁times]
    // name is string, times is integer
Λ
```

Multiple parameters are separated by commas.

---

## Function Invocation

Call functions with `⤷`:

```obfusku
⟁result=⤷square[5]      // result = 25
⟁_=⤷greet["Alice", 3]   // call with multiple args
```

**Important**: Function calls must be part of an assignment.  
Bare calls like `⤷func[]` are not allowed.

---

## Return Values

Use `⤶` to return:

```obfusku
λadd[⟁a, ⟁b]
    ⤶[a ✚ b]
Λ
```

### Implicit Return

Functions without explicit return implicitly return `∅` (null).

```obfusku
λprint_hello[]
    ✤"Hello!"
    // implicit ⤶[∅]
Λ
```

### Early Return

Return can appear anywhere:

```obfusku
λabs[⟁n]
    ⟨n ◁ 0]
        ⤶[0 ✖ n]    // early return for negative
    ⟫
    ⤶[n]
Λ
```

---

## First-Class Functions

Functions are values that can be:

### Stored in Variables

```obfusku
λdouble[⟁x]
    ⤶[x ✱ 2]
Λ

⟁f=double           // f holds the function
⟁result=⤷f[21]      // result = 42
```

### Passed as Arguments

```obfusku
λapply[⟁fn, ⟁x]
    ⤶[⤷fn[x]]
Λ

⟁result=⤷apply[double, 5]   // result = 10
```

### Returned from Functions

```obfusku
λget_doubler[]
    λdoubler[⟁x]
        ⤶[x ✱ 2]
    Λ
    ⤶[doubler]
Λ

⟁d=⤷get_doubler[]
⟁result=⤷d[10]      // result = 20
```

---

## Closures

Nested functions can capture variables from outer scopes:

```obfusku
λmake_adder[⟁x]
    λadder[⟁y]
        ⤶[x ✚ y]    // x is captured from make_adder
    Λ
    ⤶[adder]
Λ

⟁add5=⤷make_adder[5]
⟁result=⤷add5[10]   // result = 15
```

### How Capture Works

1. Inner function references outer variable
2. Value is captured at closure creation time
3. Captured values are stored with the closure
4. Each closure instance has its own captures

### Capture Semantics

Captures are **by value** (copied at creation):

```obfusku
λmake_counter[]
    ⟁count=0
    λget_count[]
        ⤶[count]    // captures initial value (0)
    Λ
    ⤶[get_count]
Λ
```

**Note**: Mutable captures are not supported in v1.0.0.

---

## Recursion

Functions can call themselves:

```obfusku
λfactorial[⟁n]
    ⟨n ⊴ 1]
        ⤶[1]
    ⟫
    ⤶[n ✱ ⤷factorial[n ✖ 1]]
Λ

⟁result=⤷factorial[5]   // result = 120
```

### Stack Limit

Maximum recursion depth is 1024 frames.

---

## Function Examples

### Fibonacci

```obfusku
λfib[⟁n]
    ⟨n ◁ 2]
        ⤶[n]
    ⟫
    ⤶[⤷fib[n ✖ 1] ✚ ⤷fib[n ✖ 2]]
Λ
```

### Higher-Order

```obfusku
λmap_array[⟁fn, ⌬arr]
    // Apply fn to each element
    // (simplified - arrays need iteration)
    ⤶[arr]
Λ
```

### With Pattern Matching

```obfusku
λdescribe[⟁n]
    ⟡n]
        ⟢0] ⤶["zero"]
        ⟢1] ⤶["one"]
        ⟢◇] ⤶["many"]
    ⟣
Λ
```

---

## Limitations

- No default parameters
- No variadic arguments
- No overloading
- Mutable closures deferred to v1.1.0

---

*Next: [Arrays and Maps](08_Arrays_and_Maps.md)*
