use anchor_lang::prelude::*;

declare_id!("FhtVsPAxuvMJhgsMMnegK9Qv4dTwxn6JdKwPypWSMYag");

#[program]
pub mod reinitialization {
    use super::*;

    pub fn insecure_init(ctx: Context<InsecureInit>) -> Result<()> {
        ctx.accounts.state.admin = ctx.accounts.user.key();
        Ok(())
    }

    pub fn secure_init(ctx: Context<SecureInit>) -> Result<()> {
        ctx.accounts.state.admin = ctx.accounts.user.key();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InsecureInit<'info> {
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
