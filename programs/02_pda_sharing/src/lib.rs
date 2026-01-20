use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod pda_sharing {
    use super::*;

    // ❌ VULNERABLE: Allows any account to be passed as the 'vault'.
    // If the attacker passes a token account they control (instead of the PDA), they can withdraw.
    // Even if it looks like a PDA, if we don't validate the seeds/bump, we might be using the wrong one
    // (e.g. one belonging to another user, or a fake one).
    pub fn insecure_withdraw(ctx: Context<InsecureWithdraw>, amount: u64) -> Result<()> {
        let vault = &ctx.accounts.vault;
        let authority = &ctx.accounts.authority;
        let destination = &ctx.accounts.destination;
        let token_program = &ctx.accounts.token_program;

        // In this vulnerable example, we just transfer FROM the passed vault to the destination.
        // If 'vault' is just a normal ATA owned by the user, this works fine.
        // But if 'vault' is supposed to be a Program Controlled account, we MUST verify it.
        // Here, we trust the client passed the correct account.
        
        let cpi_accounts = Transfer {
            from: vault.to_account_info(),
            to: destination.to_account_info(),
            authority: authority.to_account_info(),
        };
        let cpi_program = token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        token::transfer(cpi_ctx, amount)?;

        Ok(())
    }

    // ✅ SECURE: Uses Anchor's PDA validation.
    // The `seeds` and `bump` constraints ensure that `vault` is the CORRECT PDA derived from the correct seeds.
    pub fn secure_withdraw(ctx: Context<SecureWithdraw>, amount: u64) -> Result<()> {
        let vault = &ctx.accounts.vault;
        let destination = &ctx.accounts.destination;
        let token_program = &ctx.accounts.token_program;
        
        // We'll sign the transfer with the Vault's seeds since the Vault is the authority of its own token account.
        // Wait, for this example, let's assume the 'vault' is a Token Account owned by a PDA (or the PDA itself).
        // Let's assume the PDA is the 'authority' of the token account.
        
        // Revised Scenario: 
        // We are withdrawing funds FROM a vault. The vault Authority is a PDA.
        // We need to sign with the PDA seeds.
        
        let bump = ctx.bumps.vault_authority;
        let seeds = &[b"vault".as_ref(), &[bump]];
        let signer = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: vault.to_account_info(),
            to: destination.to_account_info(),
            authority: ctx.accounts.vault_authority.to_account_info(),
        };
        let cpi_program = token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        
        token::transfer(cpi_ctx, amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InsecureWithdraw<'info> {
    // ❌ Missing seeds constraint. We trust the caller provided the right vault.
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    pub authority: Signer<'info>, // Can be anyone
    #[account(mut)]
    pub destination: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct SecureWithdraw<'info> {
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    
    // ✅ Validating the PDA Authority
    #[account(
        seeds = [b"vault"],
        bump,
    )]
    /// CHECK: This is the PDA authority of the vault
    pub vault_authority: AccountInfo<'info>,
    
    #[account(mut)]
    pub destination: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
