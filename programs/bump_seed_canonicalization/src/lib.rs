use anchor_lang::prelude::*;

declare_id!("Bumj5wt9dm2cK2o9ayeguHUtpnwnUFLQHaqUaewFegV2");

#[program]
pub mod bump_seed_canonicalization {
    use super::*;

    // ❌ VULNERABLE: Accepts any valid bump seed from user input
    // Multiple PDAs can exist with same seeds but different bumps
    // Attacker can create duplicate accounts to bypass uniqueness checks
    pub fn insecure_init(ctx: Context<InsecureInit>, bump: u8) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.authority = ctx.accounts.authority.key();
        vault.bump = bump;
        Ok(())
    }

    // ✅ SECURE: Uses canonical bump from ctx.bumps (first valid bump)
    // Only one PDA possible for given seeds, enforces uniqueness
    pub fn secure_init(ctx: Context<SecureInit>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.authority = ctx.accounts.authority.key();
        vault.bump = ctx.bumps.vault;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InsecureInit<'info> {
    // ❌ ISSUE: Accepts user-provided bump, not canonical one
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 1,
        seeds = [b"vault", authority.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SecureInit<'info> {
    // ✅ FIX: Anchor automatically uses canonical bump (ctx.bumps.vault)
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 1,
        seeds = [b"vault", authority.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault {
    pub authority: Pubkey,
    pub bump: u8,
}
