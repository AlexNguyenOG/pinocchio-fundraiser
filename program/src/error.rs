use pinocchio::error::ProgramError;

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
