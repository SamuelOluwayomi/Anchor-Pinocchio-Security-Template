# 🔓 Lesson 1: Missing Signer Check

## 📖 Overview
One of the most fundamental vulnerabilities in Solana development is failing to verify that a transaction was actually authorized by the user it claims to be from. In Solana, simply passing an account's public key (`AccountInfo`) does **not** prove ownership or authorization.

## 💀 The Vulnerability
When you define an account in Anchor as `AccountInfo` (or `Account<'info, T>`), the program only checks that the account *exists* and matches the type (if specified). It does **not** automatically check if the transaction was signed by the private key corresponding to that address.

### Vulnerable Code Example
```rust
// ❌ VULNERABLE
#[derive(Accounts)]
pub struct InsecureWithdraw<'info> {
    // Requires authorization, but only checks address!
    // Anyone can pass Alice's address here.
    #[account(mut)]
    pub owner: AccountInfo<'info>, 
    
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    ...
}
```

## 💥 The Attack Scenario
**Identity Spoofing** allows an attacker to impersonate a privileged user.

1. **The Setup**: A user `Alice` has a vault with 100 SOL managed by this program. The vault stores `owner = Alice_Pubkey`.
2. **The Attack**:
   - `Mallory` (attacker) calls the `insecure_withdraw` instruction.
   - `Mallory` passes `Alice`'s public key as the `owner` account argument.
   - `Mallory` passes her own wallet as the `destination`.
3. **The Execution**:
   - The program checks: `if ctx.accounts.owner.key() == ctx.accounts.vault.owner`.
   - **Result**: `true`! The keys match.
4. **The Outcome**: The program transfers 100 SOL to Mallory. Alice loses her funds without ever touching her wallet.

## 🛡️ The Secure Solution
Use the `Signer<'info>` type provided by Anchor. This adds a constraint that the runtime must verify a cryptographic signature for this account.

### Secure Code Example
```rust
// ✅ SECURE
#[derive(Accounts)]
pub struct SecureWithdraw<'info> {
    // Enforces that this account SIGNED the transaction
    #[account(mut)]
    pub owner: Signer<'info>,
    
    #[account(mut, has_one = owner)]
    pub vault: Account<'info, Vault>,
    ...
}
```

## 🧠 Key Takeaway
Always use `Signer<'info>` for any account that is authorizing an action (like spending funds, updating config, or closing accounts). `AccountInfo` is only for *reading* data or when the account is just a destination/source without authority.
