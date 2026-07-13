use pinocchio::{
    address::Address,
    error::ProgramError,
    sysvars::{clock::Clock, Sysvar},
    ProgramResult,
};

use crate::error::FundraiserError;
use crate::state::Campaign;
use crate::traits::{AccountDeserialize, PdaAccount};

use super::Close;

impl<'a> Close<'a> {
    pub fn process(&mut self, program_id: &Address) -> ProgramResult {
        // E — Enforce
        let (goal, raised, deadline) = {
            let data = self.accounts.campaign.try_borrow()?;
            let campaign = Campaign::from_bytes(&data)?;
            campaign.validate_self(self.accounts.campaign, program_id)?;

            if self.accounts.authority.address().as_array() != &campaign.authority {
                return Err(FundraiserError::Unauthorized.into());
            }

            (campaign.goal, campaign.raised, campaign.deadline)
        };

        let goal_met = raised >= goal;
        let expired = if deadline == 0 {
            false
        } else {
            Clock::get()?.unix_timestamp >= deadline
        };

        if !(goal_met || expired) {
            return Err(FundraiserError::CannotClose.into());
        }

        // N — Nudge all lamports to recipient
        let lamports = self.accounts.campaign.lamports();
        let recipient_lamports = self.accounts.recipient.lamports();
        self.accounts.recipient.set_lamports(
            recipient_lamports
                .checked_add(lamports)
                .ok_or(ProgramError::ArithmeticOverflow)?,
        );
        self.accounts.campaign.set_lamports(0);

        // D — Destroy
        self.accounts.campaign.close()?;

        Ok(())
    }
}
