use anchor_lang::prelude::*;

declare_id!("Ty2F8Q6C9m695p29DS6gMziMhGBhr4KPh7RkCp4p21a");

#[program]
pub mod type_cosplay {
    use super::*;

    // ❌ VULNERABLE: Uses AccountInfo without checking account discriminator
    // Attacker can pass AdminAccount instead of UserAccount (if data layout matches)
    // Discriminator bypass allows privilege escalation
    pub fn insecure_withdraw(ctx: Context<InsecureWithdraw>, amount: u64) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        require!(user_account.balance >= amount, ErrorCode::InsufficientFunds);
        user_account.balance -= amount;
        Ok(())
    }

    // ✅ SECURE: Account<UserAccount> validates discriminator matches UserAccount type
    // Cannot substitute AdminAccount or other account types
    pub fn secure_withdraw(ctx: Context<SecureWithdraw>, amount: u64) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        require!(user_account.balance >= amount, ErrorCode::InsufficientFunds);
        user_account.balance -= amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InsecureWithdraw<'info> {
    // ❌ ISSUE: No discriminator check - accepts any account with matching data shape
    /// CHECK: No discriminator check
    #[account(mut)]
    pub user_account: AccountInfo<'info>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct SecureWithdraw<'info> {
    // ✅ FIX: Account<UserAccount> validates 8-byte discriminator
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
    pub authority: Signer<'info>,
}

#[account]
pub struct UserAccount {
    pub authority: Pubkey,
    pub balance: u64,
}

#[account]
pub struct AdminAccount {
    pub authority: Pubkey,
    pub balance: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient funds")]
    InsufficientFunds,
}
