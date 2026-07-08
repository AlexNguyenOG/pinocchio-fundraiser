//! Custom program errors.
//!
//! WHEN: business-rule failures that aren't covered by built-in ProgramError
//!       (e.g. deadline passed, not the authority, goal is zero).
//! HOW:  enum + `impl From<YourError> for ProgramError` via `Custom(n)`.
//! WHY:  clients can map numeric codes → friendly messages; on-chain stays tiny.
//!
//! Analogy: instead of one generic "no", you stamp specific refusal reasons.

use pinocchio::error::ProgramError;

// TODO: #[repr(u32)] enum FundraiserError { ... }
// TODO: impl From<FundraiserError> for ProgramError
#[repr(u32)]
pub enum FundraiserError {
    InvalidGoal = 1,
    InvalidAmount = 2,
    DeadlinePassed = 3,
    Unauthorized = 4,
    CannotClose = 5,
    InsufficientRaised = 6,
}


impl From<FundraiserError> for ProgramError {
    fn from(e: FundraiserError) -> Self {
        ProgramError::Custom(e as u32)
    }
}