use pinocchio::{
    address::Address,
    cpi::{Seed, Signer},
    ProgramResult,
};
use pinocchio_system::instructions::CreateAccount;

use crate::constants::CAMPAIGN_SEED;
use crate::state::Campaign;
use crate::traits::{AccountDeserialize, AccountSize};
use crate::ID;

use super::Initialize;

impl<'a> Initialize<'a> {
    pub fn process(&mut self, _program_id: &Address) -> ProgramResult {
        // O — obtain bump
        let authority_bytes = *self.accounts.authority.address().as_array();
        let seeds = Campaign::seeds_for(&authority_bytes);
        let (_expected, bump) = Address::derive_program_address(&seeds, &ID)
            .ok_or(pinocchio::error::ProgramError::InvalidSeeds)?;

        // P — provision account (PDA signs)
        let bump_seed = [bump];
        let signer_seeds = [
            Seed::from(CAMPAIGN_SEED),
            Seed::from(authority_bytes.as_ref()),
            Seed::from(&bump_seed),
        ];
        let signers = [Signer::from(&signer_seeds)];

        CreateAccount::with_minimum_balance(
            self.accounts.authority,
            self.accounts.campaign,
            Campaign::LEN as u64,
            &ID,
            None,
        )?
        .invoke_signed(&signers)?;

        // E + N — engrave state and store bump
        let mut data = self.accounts.campaign.try_borrow_mut()?;
        let campaign = Campaign::init_bytes(&mut data)?;
        campaign.authority = authority_bytes;
        campaign.goal = self.data.goal;
        campaign.raised = 0;
        campaign.deadline = self.data.deadline;
        campaign.bump = bump;
        campaign._reserved = [0u8; 7];

        Ok(())
    }
}
