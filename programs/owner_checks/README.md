# Lesson 8: Owner Checks

## Vulnerability

When using `AccountInfo` instead of `Account<T>`, developers must manually verify that the account is owned by the expected program. Missing this check allows attackers to pass accounts from other programs.

## Insecure Implementation

```rust
#[account(mut)]
pub config: AccountInfo<'info>,  // ❌ No owner validation
```

## Secure Implementation

```rust
#[account(
    mut,
    owner = ID  // ✅ Verifies account is owned by this program
)]
pub config: Account<'info, Config>,
```

## Exploit

An attacker can:
1. Create a fake account with the same data layout but owned by a different program
2. Pass this fake account to the instruction
3. Manipulate program logic or bypass security checks

## Prevention

- Use `Account<'info, T>` instead of `AccountInfo` when possible
- If using `AccountInfo`, manually check `account.owner == program_id`
- Use the `owner` constraint in Anchor for explicit validation
- Be especially careful with accounts involved in CPIs
