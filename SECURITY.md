# Security Policy

## Supported Versions

Security updates are provided for the following versions:

| Version | Status | Support Until |
|---------|--------|---------------|
| 1.0.x | Active | Ongoing |
| 0.3.x | Maintenance | 2026-06-30 |
| 0.2.x | Unsupported | No longer supported |
| 0.1.x | Unsupported | No longer supported |

Only the latest minor version of each major release receives active security support.

Users are strongly encouraged to upgrade to the latest stable version.

---

## Reporting a Vulnerability

If you discover a security vulnerability in Obfusku, please report it responsibly.

### Do Not

- Do not open a public GitHub issue
- Do not disclose the vulnerability publicly
- Do not post on social media or forums

### Do

1. **Email the maintainers** at the project contact (see CONTRIBUTING.md for contact information)
2. **Include details**:
   - Version affected
   - Description of the vulnerability
   - Steps to reproduce (if applicable)
   - Potential impact
   - Suggested fix (if available)

3. **Allow time for response**: Maintainers will acknowledge receipt within 7 days

### Expected Timeline

- **Acknowledgment**: Within 7 days of initial report
- **Assessment**: Within 14 days
- **Patch preparation**: Varies by severity (1-30 days typical)
- **Public disclosure**: Coordinated with reporter, typically 30 days after patch release

Urgent critical vulnerabilities may be expedited.

---

## Scope

### In Scope

Security issues are those affecting:

1. **Runtime Safety**
   - Undefined behavior leading to crashes
   - Memory safety violations
   - Stack corruption
   - Buffer overflows

2. **Bytecode Security**
   - Malformed bytecode causing crashes
   - Bytecode deserialization flaws
   - Bytecode injection vulnerabilities

3. **Exception Handling**
   - Exception handler bypass
   - Stack unwinding failures
   - Frame corruption

4. **Variable Scope**
   - Scope violation allowing access to variables outside intended scope
   - Closure environment leakage
   - Variable shadowing bugs

5. **Compilation**
   - Compiler bugs producing unsafe bytecode
   - Type safety violations
   - Symbol handling vulnerabilities

### Out of Scope

The following are NOT considered security vulnerabilities:

1. **Performance Issues**
   - Slow execution
   - Excessive memory use
   - Algorithmic inefficiency

2. **Missing Features**
   - Unimplemented language features
   - Features in FUTURE.md not yet implemented

3. **Design Limitations**
   - Lack of static typing
   - No implicit type conversion
   - Module system not yet implemented (v1.1.0)

4. **Documentation Issues**
   - Inaccurate or misleading documentation
   - Missing examples

5. **Denial of Service**
   - Programs that consume excessive CPU/memory (user's responsibility)
   - Stack exhaustion through deep recursion
   - Large allocation requests (VM limits are documented)

6. **Educational/Esoteric Issues**
   - "Security through obscurity" as a language design choice
   - Symbolic syntax that may be confusing

---

## Disclosure Policy

### Coordinated Disclosure

Obfusku follows responsible disclosure practices:

1. **Initial Report**: Vulnerability reported to maintainers (not public)
2. **Assessment Period**: 14 days for triage and severity assessment
3. **Fix Period**: Up to 30 days to prepare patch
4. **Patch Release**: Security patch released
5. **Public Disclosure**: Details disclosed after patch is available

### Embargo

Reporters are asked to observe an embargo period:
- Do not disclose publicly until patch is released
- Exception: Vulnerability is already known public knowledge

### Credit

Reporters of valid security vulnerabilities will be credited in release notes unless they request anonymity.

---

## Security Best Practices for Users

While Obfusku is suitable for learning and experimentation, users should note:

1. **Input Validation**: Validate all external input in programs
2. **Bytecode Sources**: Only load bytecode from trusted sources
3. **Resource Limits**: Set appropriate stack/execution limits for untrusted code
4. **Updates**: Keep Obfusku updated to the latest stable version
5. **Code Review**: Have security-critical programs reviewed

---

## Security Considerations

### What Obfusku Does NOT Provide

- Sandboxing or process isolation
- Encryption or cryptographic functions
- Memory protection beyond Rust's guarantees
- Formal verification
- Certification or compliance guarantees

### What Obfusku Does Provide

- Stack safety through VM bounds checking
- Type safety at runtime
- Clear error reporting
- Defined behavior for common error cases

---

## Questions

For security-related questions that are not vulnerability reports, please:

1. Check CONTRIBUTING.md for contact information
2. Avoid public disclosure of partial information
3. Allow 7-14 days for response

---

**Last Updated**: January 30, 2026  
**Policy Version**: 1.0
