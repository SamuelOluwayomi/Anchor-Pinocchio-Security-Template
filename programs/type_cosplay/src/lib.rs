use anchor_lang::prelude::*;

declare_id!("Ty2F8Q6C9m695p29DS6gMziMhGBhr4KPh7RkCp4p21a");

#[program]
pub mod type_cosplay {
    use super::*;

    // ❌ VULNERABLE: Uses AccountInfo without checking account discriminator
    // Attacker can pass AdminAccount instead of UserAccount (if data layout matches)
    // Discriminator bypass allows privilege escalation
    pub fn insecure_withdraw(ctx: Context<InsecureWithdraw>, amount: u64) -> Result<()> {
        // Deserialize the raw account data manually as UserAccount
        let user_account_info = &ctx.accounts.user_account;
        let mut user_account_data = UserAccount::try_deserialize(&mut user_account_info.data.borrow().as_ref())?;
        
        require!(user_account_data.balance >= amount, ErrorCode::InsufficientFunds);
        user_account_data.balance -= amount;
        
        // Serialize it back into the raw account buffer
        user_account_data.try_serialize(&mut *user_account_info.try_borrow_mut_data()?)?;
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
