use anchor_lang::prelude::*;
use pinocchio::{
    account_info::AccountInfo as PinocchioAccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey as PinocchioPubkey,
};

declare_id!("E3g7x7m3c4f5h6i7j8k9l0m1n2o3p4q5r6s7t8u9v0");

#[program]
pub mod pinocchio_vs_anchor {
    use super::*;

    // Plain Anchor instruction for comparison
    pub fn anchor_say_hello(ctx: Context<Hello>) -> Result<()> {
        msg!("Hello, Anchor!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Hello {}

// --- PINOCCHIO IMPLEMENTATION ---

// Pinocchio doesn't use the #[program] macro in the same way for routing usually, 
// usually it takes over the entrypoint. 
// BUT we can't have two entrypoints in one crate easily if using Anchor.
// Anchor defines the `entrypoint`.
// So we cannot really "mix" them in the *same* crate cleanly if we want both to own the entrypoint.
// However, the bounty asks for "Comparative security" & "Bonus: implementation".

// STRATEGY: 
// 1. We'll just define a raw function that *looks* like a Pinocchio handler 
//    and document that this is how it would look.
// 2. OR, we can try to conditionally compile, but that's messy.
// 3. BEST: Just show the logic as a standalone function that COULD be called if we dispatched to it.

// For the purpose of the template, illustrating the *code difference* is key.
// The user asked for: "Pinocchio allows you to see exactly how a signature is checked at the low level"

pub fn pinocchio_style_entrypoint(
    program_id: &PinocchioPubkey,
    accounts: &[PinocchioAccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // 1. Iterate accounts manually (No automated deserialization)
    let account_iter = &mut accounts.iter();
    let account = next_account_info(account_iter)?;

    // 2. Manual Signer Check
    // In Anchor: `pub signer: Signer<'info>`
    // In Pinocchio/Raw:
    if !account.is_signer() {
        return Err(pinocchio::program_error::ProgramError::MissingRequiredSignature);
    }
    
    // 3. Logic...
    msg!("Hello, Pinocchio!");
    
    Ok(())
}

fn next_account_info<'a, 'b: 'a>(
    iter: &mut std::slice::Iter<'a, PinocchioAccountInfo<'b>>,
) -> Result<&'a PinocchioAccountInfo<'b>, pinocchio::program_error::ProgramError> {
    iter.next().ok_or(pinocchio::program_error::ProgramError::NotEnoughAccountKeys)
}
