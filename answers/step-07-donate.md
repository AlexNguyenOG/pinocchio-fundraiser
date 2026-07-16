# Answers — Step 7 — donate

> Practice stubs live under `program/src/`. Type first; peek here only when stuck.

## Step 7 — `donate/accounts.rs`

```rust
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
```


## Step 7 — `donate/data.rs`

```rust
use pinocchio::error::ProgramError;

use crate::error::FundraiserError;

pub struct DonateData {
    pub amount: u64,
}

impl<'a> TryFrom<&'a [u8]> for DonateData {
    type Error = ProgramError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        require_len!(data, 8);

        let amount = u64::from_le_bytes(
            data[0..8]
                .try_into()
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        );

        if amount == 0 {
            return Err(FundraiserError::InvalidAmount.into());
        }

        Ok(Self { amount })
    }
}
```


## Step 7 — `donate/processor.rs`

```rust
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
        // T — type-check campaign + read deadline (drop borrow before CPI)
        let deadline = {
            let data = self.accounts.campaign.try_borrow()?;
            let campaign = Campaign::from_bytes(&data)?;
            campaign.validate_self(self.accounts.campaign, program_id)?;
            campaign.deadline
        };

        // I — inspect deadline (0 = no deadline)
        if deadline != 0 {
            let clock = Clock::get()?;
            if clock.unix_timestamp >= deadline {
                return Err(FundraiserError::DeadlinePassed.into());
            }
        }

        // P — pay (donor already signed → plain invoke)
        Transfer {
            from: self.accounts.donor,
            to: self.accounts.campaign,
            lamports: self.data.amount,
        }
        .invoke()?;

        // Update raised
        let mut data = self.accounts.campaign.try_borrow_mut()?;
        let campaign = Campaign::from_bytes_mut(&mut data)?;
        campaign.raised = campaign
            .raised
            .checked_add(self.data.amount)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        Ok(())
    }
}
```


## Step 7 — `donate/mod.rs`

```rust
mod accounts;
mod data;
mod processor;

pub use accounts::*;
pub use data::*;

use crate::instructions::impl_instructions::define_instruction;

define_instruction!(Donate, DonateAccounts<'a>, DonateData);
```

