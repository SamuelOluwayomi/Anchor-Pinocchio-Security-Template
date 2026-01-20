use anchor_lang::prelude::*;

declare_id!("BMELQtEEmhxRvzsZ8tUfdP4QhshEzCEHVyECS1QepT3n");

#[program]
pub mod signer_check {
    use super::*;

    // ❌ VULNERABLE: Does not verify that 'owner' actually signed.
    // An attacker can pass any user's public key as 'owner' and since they
    // passed the check `pot.owner == owner.key()`, the program allows the withdrawal.
    // The missing piece is verifying `owner` is a signer interactively.
    pub fn insecure_withdraw(ctx: Context<InsecureWithdraw>, amount: u64) -> Result<()> {
        let pot = &mut ctx.accounts.pot;
        let owner = &mut ctx.accounts.owner;

        if pot.owner != owner.key() {
             return Err(ErrorCode::InvalidOwner.into());
        }

        **pot.to_account_info().try_borrow_mut_lamports()? -= amount;
        **owner.to_account_info().try_borrow_mut_lamports()? += amount;

        Ok(())
    }

    // ✅ SECURE: Uses the 'Signer' type.
    // Anchor automatically verifies the signature of any account typed as 'Signer'
    // before executing the instruction.
    pub fn secure_withdraw(ctx: Context<SecureWithdraw>, amount: u64) -> Result<()> {
        let pot = &mut ctx.accounts.pot;
        let owner = &mut ctx.accounts.owner;

        // Note: Anchor's `Signer` type guarantees the account signed the transaction.
        // We still check if this signer matches the pot's owner field.
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
    /// CHECK: ❌ VULNERABLE: This account is not typed as Signer.
    /// We can pass any account here, and if it matches pot.owner, we steal funds.
    #[account(mut)]
    pub owner: AccountInfo<'info>, 
}

#[derive(Accounts)]
pub struct SecureWithdraw<'info> {
    #[account(mut)]
    pub pot: Account<'info, Pot>,
    
    // ✅ SECURE: This ensures the account signed the transaction.
    #[account(mut)]
    pub owner: Signer<'info>, 
}

#[account]
pub struct Pot {
    pub owner: Pubkey,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid owner.")]
    InvalidOwner,
}
