# Answers — Step 2 — traits

> Practice stubs live under `program/src/`. Type first; peek here only when stuck.

## Step 2 — `program/src/traits/mod.rs`

```rust
//! Shared traits for zero-copy accounts + PDAs.

use pinocchio::{
    account::AccountView,
    address::Address,
    error::ProgramError,
    ProgramResult,
};

/// 1-byte type tag stored at account data[0].
pub trait Discriminator {
    const DISCRIMINATOR: u8;
}

/// Schema version stored at account data[1].
pub trait Versioned {
    const VERSION: u8;
}

/// Fixed sizes for zero-copy account buffers.
pub trait AccountSize {
    const DATA_LEN: usize;
    const LEN: usize = 1 + 1 + Self::DATA_LEN;
}

/// Read account bytes in place as `&Self` / `&mut Self`.
pub trait AccountDeserialize: Sized + Discriminator + AccountSize + Versioned {
    fn from_bytes(data: &[u8]) -> Result<&Self, ProgramError> {
        validate_discriminator!(data, Self::DISCRIMINATOR);
        require_account_len!(data, Self::LEN);
        Ok(unsafe { &*(data[2..].as_ptr() as *const Self) })
    }

    fn from_bytes_mut(data: &mut [u8]) -> Result<&mut Self, ProgramError> {
        validate_discriminator!(data, Self::DISCRIMINATOR);
        require_account_len!(data, Self::LEN);
        Ok(unsafe { &mut *(data[2..].as_mut_ptr() as *mut Self) })
    }

    fn init_bytes(data: &mut [u8]) -> Result<&mut Self, ProgramError> {
        require_account_len!(data, Self::LEN);
        data[0] = Self::DISCRIMINATOR;
        data[1] = <Self as Versioned>::VERSION;
        Ok(unsafe { &mut *(data[2..].as_mut_ptr() as *mut Self) })
    }
}

/// PDA seed helpers.
pub trait PdaSeeds {
    const PREFIX: &'static [u8];

    fn seeds(&self) -> [&[u8]; 2];

    fn derive_address(&self, program_id: &Address) -> (Address, u8) {
        let [a, b] = self.seeds();
        Address::derive_program_address(&[a, b], program_id).expect("valid PDA bump seed")
    }

    fn derive_address_with_bump(&self, program_id: &Address, bump: u8) -> Address {
        let [a, b] = self.seeds();
        Address::derive_address(&[a, b], Some(bump), program_id)
    }

    fn validate_pda(&self, account: &AccountView, program_id: &Address, bump: u8) -> ProgramResult {
        let expected = self.derive_address_with_bump(program_id, bump);
        if account.address() != &expected {
            return Err(ProgramError::InvalidSeeds);
        }
        Ok(())
    }
}

/// PDA accounts that store their own bump.
pub trait PdaAccount: PdaSeeds {
    fn bump(&self) -> u8;

    fn validate_self(&self, account: &AccountView, program_id: &Address) -> ProgramResult {
        self.validate_pda(account, program_id, self.bump())
    }
}
```

