use anchor_lang::prelude::*;
use pinocchio::{
    account_info::AccountInfo as PinocchioAccountInfo,
    pubkey::Pubkey as PinocchioPubkey,
};

declare_id!("rZP3GCrKbBRrVgA4t14HEve5s1VCXfye1NahKuBUps2");

#[program]
pub mod pinocchio_vs_anchor {
    pub fn anchor_say_hello(_ctx: Context<Hello>) -> Result<()> {
        msg!("Hello from Anchor!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Hello {}

// Raw entrypoint for comparison
pub fn pinocchio_style_logic(
    _program_id: &PinocchioPubkey,
    accounts: &[PinocchioAccountInfo],
) -> Result<()> {
    let account = &accounts[0];

    // Manual Signer Check
    if !account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature.into());
    }

    msg!("Hello from Pinocchio!");
    Ok(())
}
