# Hello World Examples

A collection of simple programs to demonstrate Obfusku basics.

---

## The Simplest Spell

```obfusku
✤"Hello, World!"
❧
```

**Output:**
```
Hello, World!
```

**Explanation:**
- `✤` prints the literal string that follows
- `❧` seals the program

---

## Hello with a Variable

```obfusku
⌘greeting="Hello, Obfusku!"
⚡[greeting]
❧
```

**Output:**
```
Hello, Obfusku!
```

**Explanation:**
- `⌘` declares a string variable
- `⚡` outputs the value of an expression

---

## Greeting with Input

```obfusku
✤"What is your name?"
⚓⌘name
✤"Hello,"
⚡[name]
❧
```

**Output:**
```
What is your name?
⚓ Alice
Hello,
Alice
```

**Explanation:**
- `⚓` reads input into a variable
- `⌘name` declares `name` as a string

---

## Counting Hello

```obfusku
✹=3
⊂[✹ ▷ 0]
    ✤"Hello!"
    ✹⊖
⊃
❧
```

**Output:**
```
Hello!
Hello!
Hello!
```

**Explanation:**
- `✹=3` sets the accumulator to 3
- `⊂[condition]...⊃` is a loop
- `✹⊖` decrements the accumulator

---

## Conditional Hello

```obfusku
⟁hour=14

⟨hour ◁ 12]
    ✤"Good morning!"
⟩
    ✤"Good afternoon!"
⟫
❧
```

**Output:**
```
Good afternoon!
```

**Explanation:**
- `⟨condition]` starts a conditional
- `⟩` marks the else branch
- `⟫` closes the conditional

---

## Hello Function

```obfusku
λsay_hello[⌘name]
    ✤"Hello,"
    ⚡[name]
    ⤶[∅]
Λ

⟁_=⤷say_hello["World"]
❧
```

**Output:**
```
Hello,
World
```

**Explanation:**
- `λname[params]...Λ` defines a function
- `⤷function[args]` invokes it
- `⤶[value]` returns (here returning null `∅`)

---

## Pattern Matched Hello

```obfusku
⟁lang=2

⟡lang]
    ⟢1] ✤"Hello!"
    ⟢2] ✤"Hola!"
    ⟢3] ✤"Bonjour!"
    ⟢◇] ✤"Hi!"
⟣
❧
```

**Output:**
```
Hola!
```

**Explanation:**
- `⟡value]...⟣` is pattern matching
- `⟢pattern]` is a match arm
- `◇` is the wildcard (default case)

---

## Summary

| Goal | Key Symbols |
|------|-------------|
| Print literal | `✤` |
| Print value | `⚡` |
| Read input | `⚓` |
| Loop | `⊂ ⊃` |
| Condition | `⟨ ⟩ ⟫` |
| Function | `λ Λ ⤷` |
| Match | `⟡ ⟢ ⟣` |
| End program | `❧` |

---

*Next: [Execution Model and VM](04_Execution_Model_and_VM.md)*
