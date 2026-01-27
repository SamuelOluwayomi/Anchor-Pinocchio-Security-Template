# Lesson 7: Bump Seed Canonicalization

## Vulnerability

PDAs can be derived with multiple bump seeds (0-255), but only one is "canonical" (the first valid bump found by `find_program_address`). Accepting any valid bump allows attackers to create multiple accounts with the same seeds but different bumps.

## Insecure Implementation

```rust
#[instruction(bump: u8)]
pub struct InsecureInit<'info> {
    #[account(
        init,
        seeds = [b"vault", authority.key().as_ref()],
        bump  // ❌ Accepts ANY valid bump from the user
    )]
    pub vault: Account<'info, Vault>,
}
```

## Secure Implementation

```rust
pub struct SecureInit<'info> {
    #[account(
        init,
        seeds = [b"vault", authority.key().as_ref()],
        bump  // ✅ Anchor automatically uses canonical bump
    )]
    pub vault: Account<'info, Vault>,
}
```

## Exploit

An attacker can:
1. Find multiple valid bumps for the same seed
2. Create different accounts with identical seeds but different bumps
3. Bypass account uniqueness assumptions
4. Cause state confusion or double-spending

## Prevention

- Always use `ctx.bumps.<account_name>` to get the canonical bump
- Never accept bump seeds as instruction parameters for `init`
- When validating PDAs, verify the bump matches the canonical one
