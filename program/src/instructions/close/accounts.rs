use pinocchio::{account::AccountView, error::ProgramError};

use crate::ID;

pub struct CloseAccounts<'a> {
    pub authority: &'a AccountView,
    pub campaign: &'a mut AccountView,
    pub recipient: &'a mut AccountView,
}

impl<'a> TryFrom<&'a mut [AccountView]> for CloseAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a mut [AccountView]) -> Result<Self, Self::Error> {
        let [authority, campaign, recipient, ..] = accounts else {
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

        Ok(Self {
            authority,
            campaign,
            recipient,
        })
    }
}
