# Lesson 5: Framework Comparison (Anchor vs Pinocchio)

## ⚖️ The Comparison

- **Anchor**: 
  - "Batteries included".
  - Automates account deserialization, discriminator checks, and signer verification via `derive(Accounts)` macros.
  - **Secure by default** (mostly), but hides the magic.

- **Pinocchio / Raw Solana**:
  - "Zero cost abstractions".
  - You parse the byte array of accounts yourself.
  - **Manual Security**: You MUST write `if !account.is_signer { return Err(...) }`.
  - Great for understanding *what* the machine is actually doing.

## 📝 Code Walkthrough
See `src/lib.rs` to compare the 3 lines of Anchor code vs the 15 lines of Pinocchio/Raw code required to do the simple "Check signer and say Hello" task.
