use pinocchio::{
    address::Address,
    error::ProgramError,
    sysvars::{clock::Clock, Sysvar},
    ProgramResult,
};

use pinocchio_system::instructions::Transfer;

use crate::error::FundraiserError;
use crate::state::Campaign;
use crate::traits::{AccountDeserialize, PdaAccount};

use super::Donate;

impl<'a> Donate<'a> {
    pub fn process(&mut self, program_id: &Address) -> ProgramResult {
        // T - type-check campaign + read deadline (drop borrow before CPI)
        let deadline = {
            let data = self.accounts.campaign.try_borrow()?;
            let campaign = Campaign::from_bytes(&data)?;
            campaign.validate_self(self.accounts.campaign, program_id)?;
            campaign.deadline
        };

        // I - inspect deadline (0 = no deadline)
        if deadline != 0 {
            let clock = Clock::get()?;
            if clock.unix_timestamp >= deadline {
                return Err(FundraiserError::DeadlinePassed.into());
            }
        }
        // P - pay (donor already signed -> plain invoke)

        Transfer {
            from: self.accounts.donor,
            to: self.accounts.campaign,
            lamports: self.data.amount,
        }
        .invoke()?;

        //Update raised
        let mut data = self.accounts.campaign.try_borrow_mut()?;
        let campaign = Campaign::from_bytes_mut(&mut data)?;
        campaign.raised = campaign
            .raised
            .checked_add(self.data.amount)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        Ok(())
    }
}
