use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("ECg8PSWDnt1bxBxoQrmp7T2eUTSfo7aYecAiamSRdACg");

#[program]
pub mod pda_sharing {
    use super::*;

    // ❌ VULNERABLE: Accepts any TokenAccount for vault without validating PDA seeds
    // Attacker can create their own vault account and drain funds
    pub fn insecure_withdraw(ctx: Context<InsecureWithdraw>, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.destination.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        token::transfer(CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts), amount)
    }

    // ✅ SECURE: Validates vault is derived from correct PDA seeds
    // Only the canonical vault PDA can be used, preventing fake vaults
    pub fn secure_withdraw(ctx: Context<SecureWithdraw>, amount: u64) -> Result<()> {
        let bump = ctx.bumps.vault_authority;
        let seeds = &[b"vault".as_ref(), &[bump]];
        let cpi_accounts = Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.destination.to_account_info(),
            authority: ctx.accounts.vault_authority.to_account_info(),
        };
        token::transfer(CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), cpi_accounts, &[&seeds[..]]), amount)
    }
}

#[derive(Accounts)]
pub struct InsecureWithdraw<'info> {
    // ❌ ISSUE: No seeds/bump constraint - accepts ANY TokenAccount
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub destination: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct SecureWithdraw<'info> {
    // ✅ FIX: Validates vault PDA is derived from correct seeds and canonical bump
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    #[account(seeds = [b"vault"], bump)]
    /// CHECK: PDA authority
    pub vault_authority: AccountInfo<'info>,
    #[account(mut)]
    pub destination: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
