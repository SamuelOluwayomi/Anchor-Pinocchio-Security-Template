use anchor_lang::prelude::*;

declare_id!("D2f6w6l2b3e4g5h6i7j8k9l0m1n2o3p4q5r6s7t8u9v0");

#[program]
pub mod integer_overflow {
    use super::*;

    // ❌ VULNERABLE: Uses `wrapping_add` to simulate what happens if overflow checks are disabled.
    // In many legacy programs or when `overflow-checks = false` is set in Cargo.toml (default for release in older Rust),
    // simple `+` would wrap. Here we force wrapping to demonstrate the logic bug.
    // If a hacker finds a way to overflow, they can print money or bypass checks.
    pub fn insecure_add(ctx: Context<Empty>, a: u64, b: u64) -> Result<u64> {
        // Simulates silent overflow
        let result = a.wrapping_add(b);
        msg!("Result: {}", result);
        Ok(result)
    }

    // ✅ SECURE: explicitly handles overflow via `checked_add`.
    // Even if overflow checks are on, it's safer to handle the Option than to rely on panic.
    pub fn secure_add(ctx: Context<Empty>, a: u64, b: u64) -> Result<u64> {
        let result = a.checked_add(b).ok_or(ErrorCode::Overflow)?;
        msg!("Result: {}", result);
        Ok(result)
    }
}

#[derive(Accounts)]
pub struct Empty {}

#[error_code]
pub enum ErrorCode {
    #[msg("Example error.")]
    ExampleError,
    #[msg("Integer overflow occurred.")]
    Overflow,
}
