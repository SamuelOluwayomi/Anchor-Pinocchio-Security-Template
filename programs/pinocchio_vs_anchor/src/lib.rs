use anchor_lang::prelude::*;
use pinocchio::{
    account_info::AccountInfo as PinocchioAccountInfo,
    pubkey::Pubkey as PinocchioPubkey,
};

declare_id!("rZP3GCrKbBRrVgA4t14HEve5s1VCXfye1NahKuBUps2");

#[program]
pub mod pinocchio_vs_anchor {
    use super::*;
    
    // ✅ ANCHOR: High-level framework handles security automatically
    // - Automatic discriminator checks
    // - Type-safe account validation
    // - Built-in constraints (Signer, Owner, etc.)
    pub fn anchor_say_hello(_ctx: Context<Hello>) -> Result<()> {
        msg!("Hello from Anchor!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Hello {}

// ❌ PINOCCHIO/RAW: Low-level approach requires manual security checks
// - Must manually verify signers, owners, discriminators
// - More control but higher risk of mistakes
// - Useful for understanding what Anchor does under the hood
pub fn pinocchio_style_logic(
    _program_id: &PinocchioPubkey,
    accounts: &[PinocchioAccountInfo],
) -> Result<()> {
    let account = &accounts[0];

    // Manual Signer Check (Anchor does this automatically with Signer<T>)
    if !account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature.into());
    }

    msg!("Hello from Pinocchio!");
    Ok(())
}
