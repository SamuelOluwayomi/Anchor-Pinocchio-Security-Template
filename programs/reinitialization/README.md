# 🔓 Lesson 3: Double Initialization / Reinitialization

## 📖 Overview
Account initialization is a sensitive operation because it sets the initial state (like `admin = msg.sender`). If a program allows an attacker to "initialize" an account that has already been initialized, they can reset the state, overwriting the admin key or other critical data with their own.

## 💀 The Vulnerability
If an instruction initializes an account (e.g. `init_user` or `init_config`) but doesn't check if the account is already in use, it can be called multiple times.

In raw Solana programming, you manually check `if account.discriminator != 0`. In Anchor, if you don't use `init` correctly or try to do manual initialization without checks, you are vulnerable.

### Vulnerable Code Example
```rust
// ❌ VULNERABLE
#[derive(Accounts)]
pub struct InsecureInit<'info> {
    // Missing the `init` constraint means Anchor won't check if it's new.
    // It just deserializes whatever data is there (or empty data).
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
    pub authority: Signer<'info>,
}

pub fn insecure_init(ctx: Context<InsecureInit>) -> Result<()> {
    // This allows OVERWRITING existing data!
    ctx.accounts.user_account.authority = ctx.accounts.authority.key();
    Ok(())
}
```

## 💥 The Attack Scenario
**Admin Takeover** via reinitialization.

1. **The Setup**: 
   - The protocol has a `GlobalConfig` account where `admin = Alice`.
   - This account holds the keys to the treasury.
2. **The Attack**:
   - `Mallory` calls the `initialize` instruction again, passing the *existing* `GlobalConfig` account.
   - She passes herself as the signer.
3. **The Execution**:
   - The program doesn't check if `GlobalConfig` is already initialized.
   - It executes: `config.admin = ctx.accounts.signer.key()`.
4. **The Outcome**: 
   - The `admin` field is overwritten. `Mallory` is now the admin.
   - She can drain the treasury.

## 🛡️ The Secure Solution
Use Anchor's `init` constraint. This constraint ensures 3 things:
1. The account is owned by the program.
2. The account is strictly new (discriminator is not set).
3. System program creation logic matches (if it's a PDA).

If you *must* have an instruction that updates settings (but isn't `init`), use a separate instruction context that doesn't set initial state blindy, or check a flag `if account.is_initialized { return Err(...) }`.

### Secure Code Example
```rust
// ✅ SECURE
#[derive(Accounts)]
pub struct SecureInit<'info> {
    // `init` ensures this account has NOT been initialized before.
    // It creates the account and writes the discriminator.
    #[account(
        init, 
        payer = authority, 
        space = 8 + 32,
        seeds = [b"user", authority.key().as_ref()], 
        bump
    )]
    pub user_account: Account<'info, UserAccount>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

## 🧠 Key Takeaway
Always distinguish between **creating** an account (use `init`) and **updating** an account (use `mut`). Never mix them in a way that allows `init` logic to run on an existing account.
