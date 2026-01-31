# Solana Security Patterns: Deep-Dive Guide

> **A Comprehensive Analysis of 9 Critical Security Vulnerabilities in Solana Smart Contracts**

## Table of Contents

1. [Introduction](#introduction)
2. [Signer Verification](#1-signer-verification-lesson-1)
3. [PDA Sharing & Seed Verification](#2-pda-sharing--seed-verification-lesson-2)
4. [Reinitialization Attacks](#3-reinitialization-attacks-lesson-3)
5. [Integer Overflow/Underflow](#4-integer-overflow--underflow-lesson-4)
6. [Framework Comparison: Pinocchio vs Anchor](#5-framework-comparison-lesson-5)
7. [Account Closing & Data Leakage](#6-account-closing--data-leakage-lesson-6)
8. [Bump Seed Canonicalization](#7-bump-seed-canonicalization-lesson-7)
9. [Owner Checks](#8-owner-checks-lesson-8)
10. [Type Cosplay / Account Discrimination](#9-type-cosplay--account-discrimination-lesson-9)
11. [Best Practices Summary](#best-practices-summary)

---

## Introduction

Solana smart contracts (programs) are powerful but carry unique security risks due to the Account Model architecture. This guide examines 9 critical security patterns through vulnerable and secure implementations, helping developers build safer decentralized applications.

### Why This Matters

> Each vulnerability discussed here has led to millions of dollars in losses across the Solana ecosystem. Understanding these patterns is not optional—it's essential for any serious Solana developer.

**Repository Structure:**
```
programs/
├── signer_check/           # Lesson 1: Signer Verification
├── pda_sharing/            # Lesson 2: PDA Sharing
├── reinitialization/       # Lesson 3: Reinitialization Attacks
├── integer_overflow/       # Lesson 4: Integer Overflow/Underflow
├── pinocchio_vs_anchor/    # Lesson 5: Framework Comparison
├── account_closing/        # Lesson 6: Account Closing
├── bump_seed_canonicalization/ # Lesson 7: Bump Seed Canonicalization
├── owner_checks/           # Lesson 8: Owner Checks
└── type_cosplay/           # Lesson 9: Type Cosplay
```

---

## 1. Signer Verification (Lesson 1)

### 🎯 Security Principle
**Never trust public keys without cryptographic signature verification.**

### ❌ The Vulnerability

```rust
pub fn insecure_withdraw(ctx: Context<InsecureWithdraw>, amount: u64) -> Result<()> {
    let pot = &mut ctx.accounts.pot;
    let owner = &mut ctx.accounts.owner;

    // ISSUE: Only checks if public keys match, NOT if owner signed
    if pot.owner != owner.key() {
         return Err(ErrorCode::InvalidOwner.into());
    }

    **pot.to_account_info().try_borrow_mut_lamports()? -= amount;
    **owner.to_account_info().try_borrow_mut_lamports()? += amount;
    Ok(())
}

#[derive(Accounts)]
pub struct InsecureWithdraw<'info> {
    #[account(mut)]
    pub pot: Account<'info, Pot>,
    // ❌ AccountInfo doesn't enforce signature verification
    #[account(mut)]
    pub owner: AccountInfo<'info>, 
}
```

**Attack Vector:**
1. Attacker finds victim's pot address and public key
2. Attacker calls `insecure_withdraw` passing victim's public key
3. Check passes (public keys match), but attacker didn't need victim's private key
4. Funds stolen from victim's pot

### ✅ The Fix

```rust
pub fn secure_withdraw(ctx: Context<SecureWithdraw>, amount: u64) -> Result<()> {
    let pot = &mut ctx.accounts.pot;
    let owner = &mut ctx.accounts.owner;

    require!(pot.owner == owner.key(), ErrorCode::InvalidOwner);

    **pot.to_account_info().try_borrow_mut_lamports()? -= amount;
    **owner.to_account_info().try_borrow_mut_lamports()? += amount;
    Ok(())
}

#[derive(Accounts)]
pub struct SecureWithdraw<'info> {
    #[account(mut)]
    pub pot: Account<'info, Pot>,
    // ✅ Signer type requires valid signature from owner's private key
    #[account(mut)]
    pub owner: Signer<'info>, 
}
```

### 🔑 Key Takeaway
Use `Signer<'info>` instead of `AccountInfo<'info>` for any account that must prove ownership through a signature. Anchor enforces signature verification automatically for `Signer` types.

---

## 2. PDA Sharing & Seed Verification (Lesson 2)

### 🎯 Security Principle
**Always validate that PDAs (Program Derived Addresses) are derived from the expected seeds.**

### ❌ The Vulnerability

```rust
pub fn insecure_withdraw(ctx: Context<InsecureWithdraw>, amount: u64) -> Result<()> {
    let cpi_accounts = Transfer {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.destination.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    token::transfer(CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts), amount)
}

#[derive(Accounts)]
pub struct InsecureWithdraw<'info> {
    // ❌ No seeds/bump constraint - accepts ANY TokenAccount
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub destination: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
```

**Attack Vector:**
1. Attacker creates their own TokenAccount (fake vault)
2. Attacker calls `insecure_withdraw` with their fake vault
3. Program doesn't validate PDA derivation
4. Attacker can manipulate funds or bypass intended logic

### ✅ The Fix

```rust
pub fn secure_withdraw(ctx: Context<SecureWithdraw>, amount: u64) -> Result<()> {
    let bump = ctx.bumps.vault_authority;
    let seeds = &[b"vault".as_ref(), &[bump]];
    let cpi_accounts = Transfer {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.destination.to_account_info(),
        authority: ctx.accounts.vault_authority.to_account_info(),
    };
    token::transfer(CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(), 
        cpi_accounts, 
        &[&seeds[..]]
    ), amount)
}

#[derive(Accounts)]
pub struct SecureWithdraw<'info> {
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    // ✅ Validates vault PDA is derived from correct seeds and canonical bump
    #[account(seeds = [b"vault"], bump)]
    pub vault_authority: AccountInfo<'info>,
    #[account(mut)]
    pub destination: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
```

### 🔑 Key Takeaway
Always use `seeds` and `bump` constraints to validate PDAs. This ensures only the canonical PDA can be used, preventing attackers from substituting fake accounts.

---

## 3. Reinitialization Attacks (Lesson 3)

### 🎯 Security Principle
**Prevent accounts from being initialized multiple times to avoid state hijacking.**

### ❌ The Vulnerability

```rust
pub fn insecure_init(ctx: Context<InsecureInit>) -> Result<()> {
    ctx.accounts.state.admin = ctx.accounts.user.key();
    Ok(())
}

#[derive(Accounts)]
pub struct InsecureInit<'info> {
    // ❌ Just 'mut' - no init check, allows re-initialization
    #[account(mut)] 
    pub state: Account<'info, State>,
    pub user: Signer<'info>,
}
```

**Attack Vector:**
1. Legitimate admin initializes state with their public key
2. Attacker calls `insecure_init` again with the same state account
3. State gets "reinitialized" with attacker's public key as admin
4. Attacker gains admin privileges

### ✅ The Fix

```rust
pub fn secure_init(ctx: Context<SecureInit>) -> Result<()> {
    ctx.accounts.state.admin = ctx.accounts.user.key();
    Ok(())
}

#[derive(Accounts)]
pub struct SecureInit<'info> {
    // ✅ 'init' constraint ensures account is created only once
    #[account(
        init, 
        payer = user, 
        space = 8 + 32,
        seeds = [b"state", user.key().as_ref()], 
        bump
    )]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

### 🔑 Key Takeaway
Use Anchor's `init` constraint to prevent reinitialization. For manual checks, verify the account discriminator or use an `is_initialized` flag that's validated before allowing state changes.

---

## 4. Integer Overflow / Underflow (Lesson 4)

### 🎯 Security Principle
**Always use checked arithmetic operations to prevent silent wrap-around errors.**

### ❌ The Vulnerability

```rust
pub fn insecure_add(_ctx: Context<Empty>, a: u64, b: u64) -> Result<u64> {
    // ❌ wrapping_add silently wraps around on overflow
    let result = a.wrapping_add(b);
    msg!("Result: {}", result);
    Ok(result)
}
```

**Attack Vector:**
1. User has balance of `u64::MAX - 100` tokens
2. Attacker calls function to add `200` tokens
3. `wrapping_add` causes overflow: `(u64::MAX - 100) + 200 = 99`
4. User's balance becomes `99` instead of failing
5. Attacker can drain accounts or bypass withdrawal limits

### ✅ The Fix

```rust
pub fn secure_add(_ctx: Context<Empty>, a: u64, b: u64) -> Result<u64> {
    // ✅ checked_add returns error on overflow
    let result = a.checked_add(b).ok_or(ErrorCode::Overflow)?;
    msg!("Result: {}", result);
    Ok(result)
}

#[error_code]
pub enum ErrorCode {
    #[msg("Integer overflow occurred")]
    Overflow,
}
```

### Safe Arithmetic Methods

| Method | Safe? | Behavior |
|--------|-------|----------|
| `a + b` | ❌ | Panics in debug, wraps in release |
| `a.wrapping_add(b)` | ❌ | Always wraps on overflow |
| `a.checked_add(b)` | ✅ | Returns `None` on overflow |
| `a.saturating_add(b)` | ⚠️ | Caps at max value (use with caution) |

### 🔑 Key Takeaway
**Always use `checked_*` methods** (`checked_add`, `checked_sub`, `checked_mul`, `checked_div`) for financial calculations. Wrap them with `.ok_or(error)?` to propagate errors.

---

## 5. Framework Comparison (Lesson 5)

### 🎯 Security Principle
**Understand the security abstractions your framework provides—and their limitations.**

### Anchor Framework (High-Level)

```rust
// ✅ ANCHOR: Security handled automatically
pub fn anchor_say_hello(_ctx: Context<Hello>) -> Result<()> {
    msg!("Hello from Anchor!");
    Ok(())
}

#[derive(Accounts)]
pub struct Hello {}
```

**Anchor provides:**
- ✅ Automatic discriminator checks
- ✅ Type-safe account validation
- ✅ Built-in constraints (`Signer`, `Owner`, `init`, `close`, etc.)
- ✅ CPI (Cross-Program Invocation) safety
- ✅ Automatic rent exemption checks

### Pinocchio/Raw Solana (Low-Level)

```rust
// ❌ RAW: Must manually verify everything
pub fn pinocchio_style_logic(
    _program_id: &PinocchioPubkey,
    accounts: &[PinocchioAccountInfo],
) -> Result<()> {
    let account = &accounts[0];

    // Manual Signer Check (Anchor does this with Signer<T>)
    if !account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature.into());
    }

    msg!("Hello from Pinocchio!");
    Ok(())
}
```

**Manual checks required:**
- ❌ Manual signer verification
- ❌ Manual owner checks
- ❌ Manual discriminator validation
- ❌ Manual rent calculations
- ❌ Manual PDA derivation verification

### 🔑 Key Takeaway
**Use Anchor unless you have specific performance or size constraints.** The security abstractions are well-tested and prevent common mistakes. If you must use raw Solana, treat every account as untrusted and manually verify signatures, owners, discriminators, and PDAs.

---

## 6. Account Closing & Data Leakage (Lesson 6)

### 🎯 Security Principle
**When closing accounts, always zero out data to prevent information leakage.**

### ❌ The Vulnerability

```rust
pub fn insecure_close(ctx: Context<InsecureClose>) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    
    **ctx.accounts.destination.lamports.borrow_mut() += vault.to_account_info().lamports();
    **vault.to_account_info().lamports.borrow_mut() = 0;
    // ❌ Account data (owner, balance) is NOT zeroed - data leakage!
    
    Ok(())
}
```

**Attack Vector:**
1. User closes their vault account
2. Lamports transferred, but data (owner pubkey, balance history, etc.) remains
3. Attacker reads historical blockchain data
4. Sensitive information exposed (addresses, transaction patterns, etc.)
5. Privacy compromised, metadata leaked

### ✅ The Fix

```rust
pub fn secure_close(ctx: Context<SecureClose>) -> Result<()> {
    // Anchor's close constraint automatically:
    // 1. Transfers all lamports to destination
    // 2. Zeros out all account data
    // 3. Sets discriminator to CLOSED_ACCOUNT_DISCRIMINATOR
    Ok(())
}

#[derive(Accounts)]
pub struct SecureClose<'info> {
    #[account(
        mut,
        close = destination  // ✅ Securely closes the account
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub destination: SystemAccount<'info>,
}
```

### Manual Close Checklist (if not using `close` constraint)

1. ✅ Transfer all lamports to destination
2. ✅ Zero out account data with `account.data.borrow_mut().fill(0)`
3. ✅ Set discriminator to `CLOSED_ACCOUNT_DISCRIMINATOR`
4. ✅ Validate destination account accepts lamports

### 🔑 Key Takeaway
Always use Anchor's `close` constraint when closing accounts. If manually closing, ensure you zero out ALL account data to prevent information leakage.

---

## 7. Bump Seed Canonicalization (Lesson 7)

### 🎯 Security Principle
**Always use the canonical bump seed to ensure PDA uniqueness.**

### ❌ The Vulnerability

```rust
pub fn insecure_init(ctx: Context<InsecureInit>, bump: u8) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    vault.authority = ctx.accounts.authority.key();
    vault.bump = bump;  // ❌ Uses user-provided bump
    Ok(())
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InsecureInit<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 1,
        seeds = [b"vault", authority.key().as_ref()],
        bump  // ❌ Accepts any valid bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

**Attack Vector:**
1. For given seeds, multiple bump values (0-255) produce valid PDAs
2. Canonical bump is the first one that produces a valid PDA (typically 255-254)
3. Attacker creates vault with non-canonical bump (e.g., bump=253)
4. Legitimate vault (bump=255) is also created
5. **Two vaults exist** for same authority, breaking uniqueness assumptions
6. Logic relying on "one vault per user" is compromised

### ✅ The Fix

```rust
pub fn secure_init(ctx: Context<SecureInit>) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    vault.authority = ctx.accounts.authority.key();
    vault.bump = ctx.bumps.vault;  // ✅ Uses canonical bump
    Ok(())
}

#[derive(Accounts)]
pub struct SecureInit<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 1,
        seeds = [b"vault", authority.key().as_ref()],
        bump  // ✅ Anchor automatically uses canonical bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

### Understanding Bump Seeds

```rust
// Finding the canonical bump
let (pda, canonical_bump) = Pubkey::find_program_address(
    &[b"vault", authority.as_ref()],
    program_id
);
// canonical_bump is usually 255 or 254
// It's the FIRST bump that produces a valid PDA
```

### 🔑 Key Takeaway
Never accept bump seeds as user input. Always use `ctx.bumps.<account_name>` which provides the canonical bump, ensuring PDA uniqueness.

---

## 8. Owner Checks (Lesson 8)

### 🎯 Security Principle
**Verify that accounts are owned by the expected program.**

### ❌ The Vulnerability

```rust
pub fn insecure_update(ctx: Context<InsecureUpdate>, new_data: u64) -> Result<()> {
    let config_info = &ctx.accounts.config;
    let mut config_data = Config::try_deserialize(&mut config_info.data.borrow().as_ref())?;
    
    config_data.data = new_data;
    config_data.try_serialize(&mut *config_info.try_borrow_mut_data()?)?;
    
    Ok(())
}

#[derive(Accounts)]
pub struct InsecureUpdate<'info> {
    // ❌ No owner check - could be from any program
    #[account(mut)]
    pub config: AccountInfo<'info>,
    pub authority: Signer<'info>,
}
```

**Attack Vector:**
1. Attacker creates malicious program with same `Config` data layout
2. Attacker creates fake config account owned by malicious program
3. Attacker calls `insecure_update` with fake config account
4. No owner validation occurs
5. Attacker manipulates state in unintended ways or bypasses access controls

### ✅ The Fix

```rust
pub fn secure_update(ctx: Context<SecureUpdate>, new_data: u64) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.data = new_data;
    Ok(())
}

#[derive(Accounts)]
pub struct SecureUpdate<'info> {
    // ✅ Account<T> + owner constraint validates program ownership
    #[account(
        mut,
        owner = ID  // Ensures account is owned by THIS program
    )]
    pub config: Account<'info, Config>,
    pub authority: Signer<'info>,
}
```

### Owner Verification Methods

| Method | Owner Check | Discriminator Check | Type Safety |
|--------|-------------|---------------------|-------------|
| `AccountInfo` | ❌ Manual | ❌ Manual | ❌ None |
| `Account<T>` | ✅ Automatic | ✅ Automatic | ✅ Full |
| `UncheckedAccount` | ❌ Manual | ❌ Manual | ❌ None |

### 🔑 Key Takeaway
Use `Account<T>` with the `owner` constraint to automatically validate that accounts are owned by your program. Never trust `AccountInfo` without explicit owner checks.

---

## 9. Type Cosplay / Account Discrimination (Lesson 9)

### 🎯 Security Principle
**Validate account discriminators to prevent type confusion attacks.**

### ❌ The Vulnerability

```rust
pub fn insecure_withdraw(ctx: Context<InsecureWithdraw>, amount: u64) -> Result<()> {
    let user_account_info = &ctx.accounts.user_account;
    let mut user_account_data = UserAccount::try_deserialize(
        &mut user_account_info.data.borrow().as_ref()
    )?;
    
    require!(user_account_data.balance >= amount, ErrorCode::InsufficientFunds);
    user_account_data.balance -= amount;
    
    user_account_data.try_serialize(&mut *user_account_info.try_borrow_mut_data()?)?;
    Ok(())
}

#[derive(Accounts)]
pub struct InsecureWithdraw<'info> {
    // ❌ No discriminator check - accepts any account with matching data
    #[account(mut)]
    pub user_account: AccountInfo<'info>,
    pub authority: Signer<'info>,
}
```

**Attack Vector:**
1. Both `UserAccount` and `AdminAccount` have same data layout:
   ```rust
   pub struct UserAccount {
       pub authority: Pubkey,  // 32 bytes
       pub balance: u64,        // 8 bytes
   }
   
   pub struct AdminAccount {
       pub authority: Pubkey,  // 32 bytes
       pub balance: u64,        // 8 bytes (might represent different thing)
   }
   ```
2. Attacker creates `AdminAccount` with high "balance" value
3. Attacker passes `AdminAccount` to `insecure_withdraw` (expecting `UserAccount`)
4. No discriminator check occurs
5. Attacker withdraws using admin's balance value, bypassing intended logic

### ✅ The Fix

```rust
pub fn secure_withdraw(ctx: Context<SecureWithdraw>, amount: u64) -> Result<()> {
    let user_account = &mut ctx.accounts.user_account;
    require!(user_account.balance >= amount, ErrorCode::InsufficientFunds);
    user_account.balance -= amount;
    Ok(())
}

#[derive(Accounts)]
pub struct SecureWithdraw<'info> {
    // ✅ Account<UserAccount> validates 8-byte discriminator
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
    pub authority: Signer<'info>,
}
```

### Understanding Discriminators

Anchor automatically adds an 8-byte discriminator to the beginning of each account:

```
Account Data Layout:
┌──────────────────────┬──────────────┬──────────────┐
│ Discriminator (8)    │ Pubkey (32)  │ u64 (8)     │
├──────────────────────┼──────────────┼──────────────┤
│ SHA256("account:")   │ authority    │ balance     │
│ [:8]                 │              │             │
└──────────────────────┴──────────────┴──────────────┘
```

The discriminator is derived from:
```rust
hash("account:<StructName>")[..8]
```

This ensures `UserAccount` and `AdminAccount` have **different** discriminators even with identical data layouts.

### 🔑 Key Takeaway
Always use `Account<T>` instead of `AccountInfo` to automatically validate discriminators and prevent type confusion attacks.

---

## Best Practices Summary

### ✅ DO

1. **Use `Signer<'info>`** for all accounts that must prove signature
2. **Use `Account<T>`** instead of `AccountInfo` for type safety
3. **Validate PDA seeds** with `seeds` and `bump` constraints
4. **Use `init` constraint** to prevent reinitialization
5. **Use `checked_*` arithmetic** for all financial calculations
6. **Use `close` constraint** when closing accounts
7. **Use canonical bump seeds** from `ctx.bumps`
8. **Validate account owners** with `owner` constraint
9. **Let Anchor handle discriminators** by using `Account<T>`
10. **Prefer Anchor over raw Solana** unless you have specific needs

### ❌ DON'T

1. **DON'T trust public keys without signature verification**
2. **DON'T accept user-provided accounts without PDA validation**
3. **DON'T use `#[account(mut)]` alone for initialization**
4. **DON'T use wrapping arithmetic** for financial operations
5. **DON'T manually transfer lamports without zeroing data**
6. **DON'T accept user-provided bump seeds**
7. **DON'T skip owner checks** when using `AccountInfo`
8. **DON'T skip discriminator validation**
9. **DON'T assume data layout alone prevents type confusion**

### Security Checklist for Code Reviews

- [ ] All privileged operations require `Signer` verification
- [ ] All PDAs validated with `seeds` and `bump` constraints
- [ ] All initialization uses `init` or manual `is_initialized` checks
- [ ] All arithmetic uses `checked_*` methods
- [ ] All account closes use `close` constraint or manual data zeroing
- [ ] All bump seeds use canonical value from `ctx.bumps`
- [ ] All accounts validated with `Account<T>` or manual owner checks
- [ ] No type confusion possible (discriminators validated)
- [ ] All CPI calls use proper signer seeds
- [ ] All error handling is explicit and returns custom errors

---

## Testing Your Security

Each program in this repository includes test cases demonstrating both vulnerable and secure patterns:

```bash
# Run all tests
anchor test

# Run specific program test
anchor test -- --features signer_check
```

### Creating Your Own Tests

```typescript
it("Demonstrates signer attack", async () => {
  // 1. Setup legitimate user and pot
  const victim = anchor.web3.Keypair.generate();
  const pot = await initializePot(victim);
  
  // 2. Attacker tries to withdraw without signing
  const attacker = anchor.web3.Keypair.generate();
  
  try {
    await program.methods
      .insecureWithdraw(new BN(1000000))
      .accounts({
        pot: pot,
        owner: victim.publicKey,  // ❌ Attacker passes victim's key
      })
      .signers([attacker])  // ❌ But signs with attacker's key
      .rpc();
    
    // ❌ This should fail but doesn't with insecure version!
  } catch (err) {
    // ✅ Secure version properly rejects this
    expect(err).to.exist;
  }
});
```

---

## Conclusion

Solana security is about **explicit validation**. Unlike account-based blockchains, Solana's Account Model requires developers to manually verify:

- ✅ Signatures (Signer checks)
- ✅ Account ownership (Owner checks)
- ✅ Account types (Discriminator checks)
- ✅ PDA derivation (Seeds/bump validation)
- ✅ Initialization state (init constraints)
- ✅ Arithmetic safety (checked operations)

**Anchor abstracts most of this**, but you must understand what it's doing under the hood. When you use `Account<T>`, you're getting signature, owner, and discriminator checks for free. When you use raw `AccountInfo`, you must implement all checks manually.

### Next Steps

1. **Clone this repository** and experiment with the vulnerable implementations
2. **Write exploit tests** to understand attack vectors
3. **Review your existing programs** with this checklist
4. **Join the Solana security community** to stay updated on new attack patterns
5. **Consider security audits** before mainnet deployment

### Resources

- [Solana Security Best Practices](https://docs.solana.com/developing/programming-model/security)
- [Anchor Security Documentation](https://www.anchor-lang.com/docs/security)
- [Sealevel Attacks](https://github.com/coral-xyz/sealevel-attacks)
- [Neodyme Security Blog](https://blog.neodyme.io/)

---

**Built with ❤️ for the Solana security community**

*Remember: The difference between a secure and insecure program is often just one missing constraint.*
