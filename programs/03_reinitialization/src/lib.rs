use anchor_lang::prelude::*;

declare_id!("C1e5v5k1a2d3f4g5h6j7k8l9m0n1o2p3q4r5s6t7u8v9");

#[program]
pub mod reinitialization {
    use super::*;

    // ❌ VULNERABLE: Setup function that doesn't check if already initialized.
    // If 'admin' was already set, an attacker can call this again to overwrite it with their own key.
    pub fn insecure_init(ctx: Context<InsecureInit>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.admin = ctx.accounts.user.key();
        Ok(())
    }

    // ✅ SECURE: Uses 'init' constraint or check.
    // Anchor's `init` constraint automatically checks the discriminator to ensure the account hasn't been initialized.
    pub fn secure_init(ctx: Context<SecureInit>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.admin = ctx.accounts.user.key();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InsecureInit<'info> {
    // ❌ No `init` constraint, just `mut`.
    // This assumes the account is pre-created (e.g. by client) or reuses creation.
    // But it fails to check if the data inside (e.g. discriminator or flag) is already set.
    #[account(mut)] 
    pub state: Account<'info, State>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct SecureInit<'info> {
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

#[account]
pub struct State {
    pub admin: Pubkey,
}
