# Answers — Step 5 — initialize

> Practice stubs live under `program/src/`. Type first; peek here only when stuck.

## Step 5 — `initialize/accounts.rs`

```rust
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

        let (expected, _bump) = Address::derive_program_address(
            &[CAMPAIGN_SEED, authority.address().as_ref()],
            &ID,
        )
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
```


## Step 5 — `initialize/data.rs`

```rust
use pinocchio::error::ProgramError;

use crate::error::FundraiserError;

pub struct InitializeData {
    pub goal: u64,
    pub deadline: i64,
}

impl<'a> TryFrom<&'a [u8]> for InitializeData {
    type Error = ProgramError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        require_len!(data, 16);

        let goal = u64::from_le_bytes(
            data[0..8]
                .try_into()
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        );
        let deadline = i64::from_le_bytes(
            data[8..16]
                .try_into()
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        );

        if goal == 0 {
            return Err(FundraiserError::InvalidGoal.into());
        }

        Ok(Self { goal, deadline })
    }
}
```


## Step 5 — `initialize/processor.rs`

```rust
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
```


## Step 5 — `initialize/mod.rs`

_Wire `define_instruction!` once accounts/data/processor exist._

```rust
mod accounts;
mod data;
mod processor;

pub use accounts::*;
pub use data::*;

// TODO: after accounts + data compile:
// use crate::instructions::impl_instructions::define_instruction;
// define_instruction!(Initialize, InitializeAccounts<'a>, InitializeData);
use crate::instructions::impl_instructions::define_instruction;
define_instruction!(Initialize, InitializeAccounts<'a>, InitializeData);
```

