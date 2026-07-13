use pinocchio::{account::AccountView, error::ProgramError};
use pinocchio_system::ID as SYSTEM_PROGRAM_ID;

use crate::ID;

pub struct WithdrawAccounts<'a> {
    pub authority: &'a AccountView,
    pub campaign: &'a mut AccountView,
    pub recipient: &'a mut AccountView,
    pub system_program: &'a AccountView,
}

impl<'a> TryFrom<&'a mut [AccountView]> for WithdrawAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a mut [AccountView]) -> Result<Self, Self::Error> {
        let [authority, campaign, recipient, system_program, ..] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        if !authority.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }
        if !campaign.is_writable() {
            return Err(ProgramError::InvalidAccountData);
        }
        if !campaign.owned_by(&ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }
        if !recipient.is_writable() {
            return Err(ProgramError::InvalidAccountData);
        }
        if system_program.address() != &SYSTEM_PROGRAM_ID {
            return Err(ProgramError::IncorrectProgramId);
        }

        Ok(Self {
            authority,
            campaign,
            recipient,
            system_program,
        })
    }
}
