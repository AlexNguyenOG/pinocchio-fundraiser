use pinocchio::{
    address::Address,
    cpi::{Seed, Signer},
    error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::Transfer;

use crate::constants::CAMPAIGN_SEED;
use crate::error::FundraiserError;
use crate::state::Campaign;
use crate::traits::{AccountDeserialize, PdaAccount};

use super::Withdraw;

impl<'a> Withdraw<'a> {
    pub fn process(&mut self, program_id: &Address) -> ProgramResult {
        // S — Secure
        let (authority, bump, raised) = {
            let data = self.accounts.campaign.try_borrow()?;
            let campaign = Campaign::from_bytes(&data)?;
            campaign.validate_self(self.accounts.campaign, program_id)?;

            if self.accounts.authority.address().as_array() != &campaign.authority {
                return Err(FundraiserError::Unauthorized.into());
            }

            (campaign.authority, campaign.bump, campaign.raised)
        };

        // I — Inventory
        let amount = if self.data.amount == u64::MAX {
            raised
        } else {
            self.data.amount
        };

        if amount == 0 || amount > raised {
            return Err(FundraiserError::InsufficientRaised.into());
        }

        // G — Give (PDA signs)
        let bump_seed = [bump];
        let signer_seeds = [
            Seed::from(CAMPAIGN_SEED),
            Seed::from(authority.as_ref()),
            Seed::from(&bump_seed),
        ];
        let signers = [Signer::from(&signer_seeds)];

        Transfer {
            from: self.accounts.campaign,
            to: self.accounts.recipient,
            lamports: amount,
        }
        .invoke_signed(&signers)?;

        // N — Note
        let mut data = self.accounts.campaign.try_borrow_mut()?;
        let campaign = Campaign::from_bytes_mut(&mut data)?;
        campaign.raised = campaign
            .raised
            .checked_sub(amount)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        Ok(())
    }
}
