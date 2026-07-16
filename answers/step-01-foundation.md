# Answers — Step 1 — constants, macros, error

> Practice stubs live under `program/src/`. Type first; peek here only when stuck.

## Step 1a — `program/src/constants.rs`

_Keep the teaching comment if you want; the required line is `CAMPAIGN_SEED`._

```rust
//! PDA seed prefix.
//!
//! WHEN: any time you derive/sign a campaign PDA.
//! HOW:  `pub const CAMPAIGN_SEED: &[u8] = b"campaign";`
//! WHY:  seeds make PDA addresses reproducible and unique per authority.
//!
//! Analogy: the printed word on every locker label before the owner's name.

// TODO: define CAMPAIGN_SEED
pub const CAMPAIGN_SEED: &[u8] = b"campaign";
```


## Step 1b — `program/src/utils/macros.rs`

```rust

macro_rules! require_len {
    ($data:expr, $len:expr) => {
        if $data.len() < $len {
            return Err(pinocchio::error::ProgramError::InvalidInstructionData);
        }
    };
}

macro_rules! require_account_len {
    ($data:expr, $len:expr) => {
        if $data.len() < $len {
            return Err(pinocchio::error::ProgramError::InvalidAccountData);
        }
    };
}

macro_rules! validate_discriminator {
    ($data:expr, $disc:expr) => {
        if $data.is_empty() || $data[0] != $disc {
            return Err(pinocchio::error::ProgramError::InvalidAccountData);
        }
    };
}
     

macro_rules! assert_no_padding {
    ($t:ty, $expected:expr) => {
        const _: () = assert!(
            core::mem::size_of::<$t>() == $expected,
            "struct size mismatch — check field order / padding"
        );
    };
}
```


## Step 1c — `program/src/error.rs`

_You can drop the leftover TODO comments._

```rust
//! Custom program errors.
//!
//! WHEN: business-rule failures that aren't covered by built-in ProgramError
//!       (e.g. deadline passed, not the authority, goal is zero).
//! HOW:  enum + `impl From<YourError> for ProgramError` via `Custom(n)`.
//! WHY:  clients can map numeric codes → friendly messages; on-chain stays tiny.
//!
//! Analogy: instead of one generic "no", you stamp specific refusal reasons.

use pinocchio::error::ProgramError;

// TODO: #[repr(u32)] enum FundraiserError { ... }
// TODO: impl From<FundraiserError> for ProgramError
#[repr(u32)]
pub enum FundraiserError {
    InvalidGoal = 1,
    InvalidAmount = 2,
    DeadlinePassed = 3,
    Unauthorized = 4,
    CannotClose = 5,
    InsufficientRaised = 6,
}


impl From<FundraiserError> for ProgramError {
    fn from(e: FundraiserError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
```

