use anchor_lang::prelude::*;

declare_id!("ACmY1XMJy94VkriJN6o77UjDr1qRWBYe67JrMagY2Gto");

#[program]
pub mod account_closing {
    use super::*;

    // ❌ VULNERABLE: Manually transfers lamports but doesn't zero out account data
    // The account data remains readable on-chain even after "closing"
    // Sensitive data like keys, balances, or state can be extracted by attackers
    pub fn insecure_close(ctx: Context<InsecureClose>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        **ctx.accounts.destination.lamports.borrow_mut() += vault.to_account_info().lamports();
        **vault.to_account_info().lamports.borrow_mut() = 0;
        // ❌ ISSUE: Account data (owner, balance) is NOT zeroed - data leakage!
        
        Ok(())
    }

    // ✅ SECURE: Anchor's close constraint automatically:
    // 1. Transfers all lamports to destination
    // 2. Zeros out all account data (prevents data leakage)
    // 3. Sets discriminator to CLOSED_ACCOUNT_DISCRIMINATOR
    pub fn secure_close(ctx: Context<SecureClose>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InsecureClose<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub destination: SystemAccount<'info>,
}

#[derive(Accounts)]
pub struct SecureClose<'info> {
    // ✅ FIX: The 'close' constraint securely closes the account
    #[account(
        mut,
        close = destination  // Automatically zeros data and transfers lamports
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub destination: SystemAccount<'info>,
}

#[account]
pub struct Vault {
    pub owner: Pubkey,
    pub balance: u64,
}
