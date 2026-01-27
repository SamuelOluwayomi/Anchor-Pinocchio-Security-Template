use anchor_lang::prelude::*;

declare_id!("FhtVsPAxuvMJhgsMMnegK9Qv4dTwxn6JdKwPypWSMYag");

#[program]
pub mod reinitialization {
    use super::*;

    // ❌ VULNERABLE: No check if account is already initialized
    // Attacker can call this multiple times to reset admin and take control
    pub fn insecure_init(ctx: Context<InsecureInit>) -> Result<()> {
        ctx.accounts.state.admin = ctx.accounts.user.key();
        Ok(())
    }

    // ✅ SECURE: Anchor's 'init' constraint prevents reinitialization
    // Account can only be initialized once, admin cannot be changed
    pub fn secure_init(ctx: Context<SecureInit>) -> Result<()> {
        ctx.accounts.state.admin = ctx.accounts.user.key();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InsecureInit<'info> {
    // ❌ ISSUE: Just 'mut' - no init check, allows re-initialization attacks
    #[account(mut)] 
    pub state: Account<'info, State>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct SecureInit<'info> {
    // ✅ FIX: 'init' constraint ensures account is created only once
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
