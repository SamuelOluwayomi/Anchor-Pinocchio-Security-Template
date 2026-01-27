# Lesson 9: Type Cosplay (Account Discrimination)

## Vulnerability

Using `AccountInfo` without checking the account discriminator allows attackers to pass accounts of the wrong type. An attacker can substitute a different account type that happens to have compatible data layout.

## Insecure Implementation

```rust
/// CHECK: No discriminator check
#[account(mut)]
pub user_account: AccountInfo<'info>,  // ❌ Accepts ANY account data
```

## Secure Implementation

```rust
#[account(mut)]
pub user_account: Account<'info, UserAccount>,  // ✅ Validates discriminator
```

## Exploit

An attacker can:
1. Create an `AdminAccount` with high privileges
2. Pass it to a function expecting `UserAccount`
3. If both have the same data layout, the program treats admin as user
4. Bypass access controls or privilege escalation

## Prevention

- Always use `Account<'info, T>` which automatically checks the discriminator
- If using `AccountInfo`, manually verify the discrimin ator matches expected type
- Anchor's discriminator is an 8-byte hash derived from account type name
- Never assume account type based solely on data shape
