# Answers — Step 8 — withdraw

> Practice stubs live under `program/src/`. Type first; peek here only when stuck.

## Step 8 — `withdraw/accounts.rs`

_`recipient` is `&mut` so `set_lamports` works._

```rust
use pinocchio::{account::AccountView, error::ProgramError};
use pinocchio_system::ID as SYSTEM_PROGRAM_ID;

use crate::ID;

pub struct WithdrawAccounts<'a> {
    pub authority: &'a AccountView,
    pub campaign: &'a mut AccountView,
    pub recipient: &'a mut AccountView,
    pub system_program: &'a AccountView,
}

impl<'a> TryFrom<&'a mut [AccountView]> for WithdrawAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a mut [AccountView]) -> Result<Self, Self::Error> {
        let [authority, campaign, recipient, system_program, ..] = accounts else {
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
        if system_program.address() != &SYSTEM_PROGRAM_ID {
            return Err(ProgramError::IncorrectProgramId);
        }

        Ok(Self {
            authority,
            campaign,
            recipient,
            system_program,
        })
    }
}
```


## Step 8 — `withdraw/data.rs`

```rust
use pinocchio::error::ProgramError;

use crate::error::FundraiserError;

pub struct WithdrawData {
    pub amount: u64,
}

impl<'a> TryFrom<&'a [u8]> for WithdrawData {
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


## Step 8 — `withdraw/processor.rs`

_Direct lamport moves — System Transfer cannot pull from a data-carrying PDA._

```rust
use pinocchio::{
    address::Address,
    error::ProgramError,
    ProgramResult,
};

use crate::error::FundraiserError;
use crate::state::Campaign;
use crate::traits::{AccountDeserialize, PdaAccount};

use super::Withdraw;

impl<'a> Withdraw<'a> {
    pub fn process(&mut self, program_id: &Address) -> ProgramResult {
        // S — Secure
        let raised = {
            let data = self.accounts.campaign.try_borrow()?;
            let campaign = Campaign::from_bytes(&data)?;
            campaign.validate_self(self.accounts.campaign, program_id)?;

            if self.accounts.authority.address().as_array() != &campaign.authority {
                return Err(FundraiserError::Unauthorized.into());
            }

            campaign.raised
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

        // G — Give (program owns the campaign → move lamports directly;
        //     System Transfer rejects `from` accounts that carry data)
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
```


## Step 8 — `withdraw/mod.rs`

```rust
mod accounts;
mod data;
mod processor;

pub use accounts::*;
pub use data::*;

use crate::instructions::impl_instructions::define_instruction;

define_instruction!(Withdraw, WithdrawAccounts<'a>, WithdrawData);
```

