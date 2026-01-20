use anchor_lang::prelude::*;

declare_id!("GfN9UbCqhCwPQzpZL8aVqDSwaFpDGZ9SdRii9YvJaoVZ");

#[program]
pub mod integer_overflow {
    use super::*;

    pub fn insecure_add(_ctx: Context<Empty>, a: u64, b: u64) -> Result<u64> {
        let result = a.wrapping_add(b);
        msg!("Result: {}", result);
        Ok(result)
    }

    pub fn secure_add(_ctx: Context<Empty>, a: u64, b: u64) -> Result<u64> {
        let result = a.checked_add(b).ok_or(ErrorCode::Overflow.into())?;
        msg!("Result: {}", result);
        Ok(result)
    }
}

#[derive(Accounts)]
pub struct Empty {}

#[error_code]
pub enum ErrorCode {
    #[msg("Integer overflow occurred")]
    Overflow,
}
