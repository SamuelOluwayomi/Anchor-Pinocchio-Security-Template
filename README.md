#  Solana Security Template: Educational Vulnerability Examples

An educational Anchor framework project demonstrating common Solana smart contract vulnerabilities with **vulnerable vs. secure** implementations.

---

## 📚 Lessons Overview

This template includes **9 security lessons**, each with:
- **Vulnerable implementation** (what NOT to do)
- **Secure implementation** (best practices)
- **README** explaining the vulnerability and exploit
- **Tests** demonstrating both approaches

| Lesson | Vulnerability | Description |
|--------|--------------|-------------|
| 1 | **Signer Check** | Missing or improper signer validation |
| 2 | **PDA Sharing** | Accepting PDAs without seed/bump verification |
| 3 | **Reinitialization** | Allowing accounts to be initialized multiple times |
| 4 | **Integer Overflow** | Arithmetic operations without overflow protection |
| 5 | **Pinocchio vs Anchor** | Framework comparison and low-level security |
| 6 | **Account Closing** | Improper account closure leaving data exposed |
| 7 | **Bump Seed Canonicalization** | Accepting non-canonical PDA bumps |
| 8 | **Owner Checks** | Missing program ownership validation |
| 9 | **Type Cosplay** | Account discriminator bypass/substitution |

> 📖 **Want an in-depth analysis?** Check out the [**SECURITY_PATTERNS_DEEP_DIVE.md**](./SECURITY_PATTERNS_DEEP_DIVE.md) guide for comprehensive explanations, attack vectors, and best practices for all 9 patterns.

---

## 🚀 Quick Start

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) and [Cargo](https://doc.rust-lang.org/cargo/)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) (v1.18+)
- [Anchor CLI](https://www.anchor-lang.com/docs/installation) (v0.30.0)
- [Node.js](https://nodejs.org/) (v18+) and npm

### Installation

```bash
# Clone the repository
git clone https://github.com/YourUsername/solana-security-template
cd solana-security-template

# Install dependencies
npm install

# Build programs
anchor build

# Run tests
anchor test
```

---

## 📖 Lesson Structure

Each lesson is located in `programs/<lesson_name>/`:

```
programs/
├── signer_check/
│   ├── src/lib.rs          # Vulnerable & secure implementations
│   ├── Cargo.toml
│   └── README.md           # Detailed explanation
└── tests/
    └── 01_signer_check.ts  # Test demonstrations
```

### Example: Signer Check Vulnerability

**Vulnerable Code:**
```rust
#[derive(Accounts)]
pub struct InsecureWithdraw<'info> {
    /// CHECK: ❌ Missing Signer check - accepts ANY public key
    #[account(mut)]
    pub owner: AccountInfo<'info>,
}
```

**Secure Code:**
```rust
#[derive(Accounts)]
pub struct SecureWithdraw<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,  // ✅ Enforces signer validation
}
```

---

## 🎓 Learning Path

1. **Start with Lesson 1 (Signer Check)** - Foundation of authentication
2. **Lesson 2-4** - Core Anchor security patterns
3. **Lesson 5** - Framework internals and low-level concepts
4. **Lesson 6-9** - Advanced attack vectors

Each lesson's README includes:
- Vulnerability explanation
- Exploit scenario
- Prevention strategies
- Real-world implications

---

## 🧪 Running Tests

```bash
# Run all tests
anchor test

# Run specific lesson test
anchor test -- --grep "signer_check"

# Start local validator for manual testing
solana-test-validator

# Deploy to localnet (in another terminal)
anchor deploy
```

---

## 🛡️ Security Best Practices

Based on these lessons, always:

✅ **Use Anchor's type system** (`Signer`, `Account<T>`, etc.)  
✅ **Validate PDA seeds and bumps** explicitly  
✅ **Use `init` constraint** for account initialization  
✅ **Use `checked_*` arithmetic** operations  
✅ **Leverage Anchor constraints** (`#[account(close)]`, `owner`, etc.)  
✅ **Verify account discriminators** when using `AccountInfo`  

---

## 📂 Project Structure

```
.
├── programs/              # 9 vulnerability lesson programs
├── tests/                 # TypeScript tests for each lesson
├── Anchor.toml           # Anchor workspace configuration
├── Cargo.toml            # Rust workspace manifest
└── README.md             # This file
```

---

## 🤝 Contributing

This is an educational resource. Contributions are welcome:
- Additional vulnerability examples
- Improved explanations
- Test coverage enhancements
- Documentation fixes

---

## 📜 License

MIT License - See [LICENSE](LICENSE) for details

---

## ⚠️ Disclaimer

**FOR EDUCATIONAL PURPOSES ONLY**

The vulnerable code in this repository is intentionally flawed to demonstrate security issues. **Never use vulnerable patterns in production code.**

---

## 🔗 Resources

- [Anchor Documentation](https://www.anchor-lang.com/)
- [Solana Security Best Practices](https://docs.solana.com/developing/programming-model/security)
- [Neodyme Security Audits](https://blog.neodyme.io/)
- [Sec3 Vulnerability Database](https://github.com/sec3-product/learning-center)

---

**Built with ❤️ for Solana developers**
