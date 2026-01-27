use anchor_lang::prelude::*;

declare_id!("owjTGQ5tcsF9jiy1rDeiBxKj7fAcz3h8tbs9q9f5Awm");

#[program]
pub mod owner_checks {
    use super::*;

    // ❌ VULNERABLE: Uses AccountInfo without verifying program ownership
    // Attacker can pass accounts from other programs with same data layout
    pub fn insecure_update(ctx: Context<InsecureUpdate>, new_data: u64) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.data = new_data;
        Ok(())
    }

    // ✅ SECURE: Account<T> automatically validates owner matches this program
    // Also validates discriminator to ensure correct account type
    pub fn secure_update(ctx: Context<SecureUpdate>, new_data: u64) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.data = new_data;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InsecureUpdate<'info> {
    // ❌ ISSUE: No owner check - could be from any program
    /// CHECK: Missing owner validation
    #[account(mut)]
    pub config: AccountInfo<'info>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct SecureUpdate<'info> {
    // ✅ FIX: Account<T> + owner constraint validates program ownership
    #[account(
        mut,
        owner = ID  // Ensures account is owned by this program
    )]
    pub config: Account<'info, Config>,
    pub authority: Signer<'info>,
}

#[account]
pub struct Config {
    pub authority: Pubkey,
    pub data: u64,
}
