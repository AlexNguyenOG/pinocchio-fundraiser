use pinocchio::error::ProgramError;

use crate::error::FundraiserError;

pub struct InitializeData {
    pub goal: u64,
    pub deadline: i64,
}

impl<'a> TryFrom<&'a [u8]> for InitializeData {
    type Error = ProgramError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        require_len!(data, 16);

        let goal = u64::from_le_bytes(
            data[0..8]
                .try_into()
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        );
        let deadline = i64::from_le_bytes(
            data[8..16]
                .try_into()
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        );

        if goal == 0 {
            return Err(FundraiserError::InvalidGoal.into());
        }

        Ok(Self { goal, deadline })
    }
}
