# Lesson 4: Integer Overflow / Underflow

## 💀 The Vulnerability
In older versions of Solana/Rust programs (or if `overflow-checks = false` is explicitly set in `Cargo.toml`), arithmetic operations like `+` and `-` would "wrap around" instead of panicking on overflow.

For example, `u64::MAX + 1` becomes `0`.
Or `0 - 1` becomes `u64::MAX` (huge number!).

## 💥 The Exploit
Imagine a withdrawal check:
```rust
let new_balance = user_balance - withdraw_amount; // If this wraps, attacker gets HUGE balance
```
If `user_balance` is 10 and `withdraw_amount` is 20:
- **Checked (Secure)**: Panics or returns Error.
- **Unchecked (Vulnerable)**: `new_balance` becomes `18446744073709551606`. The user is now rich.

## 🛡️ The Fix
1.  Ensure `overflow-checks = true` is set in `Cargo.toml` (Anchor does this by default now).
2.  Use `checked_add`, `checked_sub`, `checked_mul` for critical math to handle errors gracefully instead of crashing the transaction (panic).