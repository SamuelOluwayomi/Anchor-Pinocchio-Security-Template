# Lesson 6: Account Closing

## Vulnerability

When closing an account manually, developers might forget to zero out the account's data before transferring lamports. This leaves sensitive data in a "closed" account that can still be read on-chain.

## Insecure Implementation

```rust
pub fn insecure_close(ctx: Context<InsecureClose>) -> Result<()> {
    **ctx.accounts.destination.lamports.borrow_mut() += vault.to_account_info().lamports();
    **vault.to_account_info().lamports.borrow_mut() = 0;
    // ❌ Account data is NOT zeroed out
    Ok(())
}
```

## Secure Implementation

```rust
#[account(
    mut,
    close = destination  // ✅ Anchor's close constraint zeros data automatically
)]
pub vault: Account<'info, Vault>,
```

## Exploit

An attacker can read the "closed" account's data field to extract sensitive information like private keys, user data, or business logic state.

## Prevention

Always use Anchor's `close` constraint, which:
1. Transfers all lamports to the destination
2. **Zeros out the account data** (preventing data leakage)
3. Sets discriminator to `CLOSED_ACCOUNT_DISCRIMINATOR`
