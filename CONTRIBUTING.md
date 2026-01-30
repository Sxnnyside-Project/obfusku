# Contributing to Obfusku

Thank you for your interest in contributing to Obfusku — The Magical Programming Language.

## Ways to Contribute

### 1. Proposing New Symbols

Obfusku is a symbol-driven language. New symbols must:

- **Have visual meaning**: The glyph should visually suggest its purpose
- **Not conflict**: Must not overlap with existing symbols
- **Be Unicode-compatible**: Should render properly across platforms
- **Follow ritual philosophy**: Fit the mystical nature of the language

**Process**:
1. Open an issue titled `[Symbol Proposal] <symbol> — <meaning>`
2. Include:
   - The proposed symbol(s)
   - Semantic meaning
   - Example usage
   - Justification for visual choice
3. Wait for discussion and approval before implementing

### 2. Adding Examples

Example programs help others learn Obfusku. Good examples:

- Demonstrate a single concept clearly
- Include comments explaining each symbol
- End with `❧` (seal the ritual)
- Are placed in `examples/` folder

**Process**:
1. Create a new `.obk` file in `examples/`
2. Add a header comment explaining the purpose
3. Test with `obfusku run examples/your_file.obk`
4. Submit a pull request

### 3. Fixing Bugs

Bug fixes are always welcome. Please:

1. Check existing issues first
2. Create an issue describing the bug
3. Include reproduction steps
4. Submit a PR referencing the issue

### 4. Improving Documentation

Documentation lives in `grimoires/`. Improvements should:

- Be accurate to the current implementation
- Not describe speculative features
- Maintain the professional yet mystical tone
- Include correct symbol usage

---

## Rules for Language Semantics

### v1.0.x is FROZEN

The core language semantics are now locked. This means:

- ❌ No changes to existing symbol meanings
- ❌ No breaking syntax changes
- ❌ No VM behavior modifications
- ✅ Bug fixes that restore intended behavior
- ✅ Performance improvements
- ✅ Error message improvements

### Proposing New Features

New features (v1.1.0+) must:

1. Be proposed as an issue with `[Feature Proposal]` prefix
2. Include design rationale
3. Show how it fits Obfusku philosophy
4. Not break backward compatibility
5. Be approved before implementation

---

## Code Style

### Rust Code

- Use `rustfmt` for formatting
- Follow Rust idioms
- Document public APIs
- Write tests for new functionality

### Obfusku Code

- Use symbolic style consistently
- Include comments for non-obvious constructs
- Always seal programs with `❧`

---

## Pull Request Process

1. Fork the repository
2. Create a feature branch
3. Make changes with clear commits
4. Ensure tests pass: `cargo test`
5. Update documentation if needed
6. Submit PR with description of changes

---

## Questions?

Open an issue with `[Question]` prefix for any clarifications.

## Security Issues

**Do not** open public issues for security vulnerabilities.

See **[SECURITY.md](SECURITY.md)** for responsible disclosure procedures.

---

*Symbols carry meaning. Contributions shape the future.*

```
❧
```
