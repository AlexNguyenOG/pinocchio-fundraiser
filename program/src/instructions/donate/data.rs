use pinocchio::error::ProgramError;

use crate::error::FundraiserError;

pub struct DonateData {
    pub amount: u64,
}

impl<'a> TryFrom<&'a [u8]> for DonateData {
    type Error = ProgramError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        require_len!(data, 8);

        let amount = u64::from_le_bytes(
            data[0..8]
                .try_into()
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        );

        if amount == 0 {
            return Err(FundraiserError::InvalidAmount.into());
        }

        Ok(Self { amount })
    }
}
