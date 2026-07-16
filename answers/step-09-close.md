# Answers — Step 9 — close

> Practice stubs live under `program/src/`. Type first; peek here only when stuck.

## Step 9 — `close/accounts.rs`

```rust
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
```


## Step 9 — `close/mod.rs`

```rust
mod accounts;
mod processor;

pub use accounts::*;

use pinocchio::error::ProgramError;

/// Close has no args beyond the discriminator byte.
pub struct CloseData;

impl<'a> TryFrom<&'a [u8]> for CloseData {
    type Error = ProgramError;

    fn try_from(_data: &'a [u8]) -> Result<Self, Self::Error> {
        Ok(Self)
    }
}

use crate::instructions::impl_instructions::define_instruction;

define_instruction!(Close, CloseAccounts<'a>, CloseData);
```


## Step 9 — `close/processor.rs`

```rust
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
```

