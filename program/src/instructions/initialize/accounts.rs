use pinocchio::{account::AccountView, address::Address, error::ProgramError};
use pinocchio_system::ID as SYSTEM_PROGRAM_ID;

use crate::constants::CAMPAIGN_SEED;
use crate::ID;

pub struct InitializeAccounts<'a> {
    pub authority: &'a AccountView,
    pub campaign: &'a mut AccountView,
    pub system_program: &'a AccountView,
}

impl<'a> TryFrom<&'a mut [AccountView]> for InitializeAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a mut [AccountView]) -> Result<Self, Self::Error> {
        let [authority, campaign, system_program, ..] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        if !authority.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }
        if !authority.is_writable() {
            return Err(ProgramError::InvalidAccountData);
        }
        if !campaign.is_writable() {
            return Err(ProgramError::InvalidAccountData);
        }
        if system_program.address() != &SYSTEM_PROGRAM_ID {
            return Err(ProgramError::IncorrectProgramId);
        }

        let (expected, _bump) =
            Address::derive_program_address(&[CAMPAIGN_SEED, authority.address().as_ref()], &ID)
                .ok_or(ProgramError::InvalidSeeds)?;

        if campaign.address() != &expected {
            return Err(ProgramError::InvalidSeeds);
        }

        if !campaign.owned_by(&SYSTEM_PROGRAM_ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }
        if !campaign.is_data_empty() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        Ok(Self {
            authority,
            campaign,
            system_program,
        })
    }
}
