use anchor_lang::prelude::*;

declare_id!("GfN9UbCqhCwPQzpZL8aVqDSwaFpDGZ9SdRii9YvJaoVZ");

#[program]
pub mod integer_overflow {
    use super::*;

    // ❌ VULNERABLE: Uses wrapping_add which silently overflows
    // u64::MAX + 1 = 0, can drain accounts or bypass limits
    pub fn insecure_add(_ctx: Context<Empty>, a: u64, b: u64) -> Result<u64> {
        let result = a.wrapping_add(b);
        msg!("Result: {}", result);
        Ok(result)
    }

    // ✅ SECURE: Uses checked_add which returns error on overflow
    // Prevents silent overflow, transaction fails with clear error
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
