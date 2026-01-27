use anchor_lang::prelude::*;

declare_id!("6PpTEM9LKNPCSUHgwqxdXK6nmSuMwTkh8k81D56SFVhf");

#[program]
pub mod signer_check {
    use super::*;

    // ❌ VULNERABLE: Accepts AccountInfo for owner, allowing anyone to pass any public key
    // An attacker can call this with someone else's public key and withdraw their funds
    pub fn insecure_withdraw(ctx: Context<InsecureWithdraw>, amount: u64) -> Result<()> {
        let pot = &mut ctx.accounts.pot;
        let owner = &mut ctx.accounts.owner;

        // This check only verifies the public key matches, NOT that they actually signed
        if pot.owner != owner.key() {
             return Err(ErrorCode::InvalidOwner.into());
        }

        **pot.to_account_info().try_borrow_mut_lamports()? -= amount;
        **owner.to_account_info().try_borrow_mut_lamports()? += amount;
        Ok(())
    }

    // ✅ SECURE: Uses Signer type to enforce cryptographic signature validation
    // Only the actual owner can call this function with their private key
    pub fn secure_withdraw(ctx: Context<SecureWithdraw>, amount: u64) -> Result<()> {
        let pot = &mut ctx.accounts.pot;
        let owner = &mut ctx.accounts.owner;

        require!(pot.owner == owner.key(), ErrorCode::InvalidOwner);

        **pot.to_account_info().try_borrow_mut_lamports()? -= amount;
        **owner.to_account_info().try_borrow_mut_lamports()? += amount;
        Ok(())
    }

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let pot = &mut ctx.accounts.pot;
        pot.owner = ctx.accounts.owner.key();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, 
        payer = owner, 
        space = 8 + 32,
        seeds = [b"pot", owner.key().as_ref()],
        bump
    )]
    pub pot: Account<'info, Pot>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InsecureWithdraw<'info> {
    #[account(mut)]
    pub pot: Account<'info, Pot>,
    // ❌ ISSUE: AccountInfo doesn't enforce signature verification
    /// CHECK: VULNERABLE - Missing Signer check
    #[account(mut)]
    pub owner: AccountInfo<'info>, 
}

#[derive(Accounts)]
pub struct SecureWithdraw<'info> {
    #[account(mut)]
    pub pot: Account<'info, Pot>,
    // ✅ FIX: Signer type requires valid signature from owner's private key
    #[account(mut)]
    pub owner: Signer<'info>, 
}

#[account]
pub struct Pot {
    pub owner: Pubkey,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid owner")]
    InvalidOwner,
}
