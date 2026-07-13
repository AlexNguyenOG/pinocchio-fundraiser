use pinocchio::{account::AccountView, error::ProgramError};
use pinocchio_system::ID as SYSTEM_PROGRAM_ID;

use crate::ID;

pub struct DonateAccounts<'a> {
    pub donor: &'a AccountView,
    pub campaign: &'a mut AccountView,
    pub system_program: &'a AccountView,
}

impl<'a> TryFrom<&'a mut [AccountView]> for DonateAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a mut [AccountView]) -> Result<Self, Self::Error> {
        let [donor, campaign, system_program, ..] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        if !donor.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }
        if !donor.is_writable() {
            return Err(ProgramError::InvalidAccountData);
        }
        if !campaign.is_writable() {
            return Err(ProgramError::InvalidAccountData);
        }
        if !campaign.owned_by(&ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }
        if system_program.address() != &SYSTEM_PROGRAM_ID {
            return Err(ProgramError::IncorrectProgramId);
        }

        Ok(Self {
            donor,
            campaign,
            system_program,
        })
    }
}
