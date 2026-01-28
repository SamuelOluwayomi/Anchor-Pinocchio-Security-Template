# 🔓 Lesson 2: PDA Sharing / Missing Verification

## 📖 Overview
PDAs (Program Derived Addresses) are the core way Solana programs govern data. A common mistake is accepting *any* PDA as an account without verifying that it was derived from the correct seeds that the program expects. This allows attackers to create "fake" PDAs with their own data and pass them to your program.

## 💀 The Vulnerability
If your program expects an account argument but relies on the **user** to provide the correct one without validation, an attacker can create a separate account (that looks like the real one) and pass it in.

This is often called **Input Validation Failure** regarding Account ownership or derivation.

### Vulnerable Code Example
```rust
// ❌ VULNERABLE
#[derive(Accounts)]
pub struct InsecureWithdraw<'info> {
    // We trust the user passed the correct Vault PDA...
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    
    // ...and that this authority owns it.
    pub authority: Signer<'info>,
}
```

## 💥 The Attack Scenario
**Fake Account Substitution** allows an attacker to operate on data they control instead of the protocol's authoritative data.

1. **The Setup**: 
   - A `RealVault` exists, holding 1000 Tokens, owned by the program.
   - `Mallory` creates a `FakeVault` (just a regular account she initialized), putting her own data in it (e.g., claiming she has 1,000,000 tokens or `admin: Mallory`).
2. **The Attack**:
   - `Mallory` calls `withdraw` (or `admin_instruction`).
   - Instead of passing `RealVault`, she passes `FakeVault`.
3. **The Execution**:
   - The program reads `FakeVault.state`.
   - It sees `admin: Mallory` inside the fake vault.
   - It proceeds to execute privileged commands because it believes the account data is valid.
   - **Result**: Mallory bypasses checks by supplying an account that *looks* right but isn't the canonical one.

## 🛡️ The Secure Solution
Use Anchor's `seeds` constraint to force the program to calculate what the PDA address *should* be and check if the passed account matches it.

### Secure Code Example
```rust
// ✅ SECURE
#[derive(Accounts)]
pub struct SecureWithdraw<'info> {
    // Anchor recalculates the address: hash("vault", authority.key)
    // If the user passes anything else, the instruction FAILS immediately.
    #[account(
        mut,
        seeds = [b"vault", authority.key().as_ref()],
        bump = vault.bump
    )]
    pub vault: Account<'info, Vault>,
    
    pub authority: Signer<'info>,
}
```

## 🧠 Key Takeaway
Never trust that an account passed by the user is the one you "expect" it to be. Always use `seeds[...]` to bind PDAs to their canonical derivation, or use `has_one` to link accounts together.
