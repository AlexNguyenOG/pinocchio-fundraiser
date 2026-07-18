use pinocchio::{address::Address, error::ProgramError, ProgramResult};

use crate::error::FundraiserError;
use crate::state::Campaign;
use crate::traits::{AccountDeserialize, PdaAccount};

use super::Withdraw;

impl<'a> Withdraw<'a> {
    pub fn process(&mut self, program_id: &Address) -> ProgramResult {
        // S - Secure
        let raised = {
            let data = self.accounts.campaign.try_borrow()?;
            let campaign = Campaign::from_bytes(&data)?;
            campaign.validate_self(self.accounts.campaign, program_id)?;

            if self.accounts.authority.address().as_array() != &campaign.authority {
                return Err(FundraiserError::Unauthorized.into());
            }
            campaign.raised
        };

        // I - Inventory
        let amount = if self.data.amount == u64::MAX {
            raised
        } else {
            self.data.amount
        };

        if amount == 0 || amount > raised {
            return Err(FundraiserError::InsufficientRaised.into());
        }

        let campaign_lamports = self.accounts.campaign.lamports();
        let recipient_lamports = self.accounts.recipient.lamports();
        self.accounts.campaign.set_lamports(
            campaign_lamports
                .checked_sub(amount)
                .ok_or(ProgramError::ArithmeticOverflow)?,
        );
        self.accounts.recipient.set_lamports(
            recipient_lamports
                .checked_add(amount)
                .ok_or(ProgramError::ArithmeticOverflow)?,
        );

        // N- Note
        let mut data = self.accounts.campaign.try_borrow_mut()?;
        let campaign = Campaign::from_bytes_mut(&mut data)?;
        campaign.raised = campaign
            .raised
            .checked_sub(amount)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        Ok(())
    }
}
