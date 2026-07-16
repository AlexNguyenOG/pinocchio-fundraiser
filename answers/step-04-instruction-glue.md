# Answers — Step 4 — define_instruction! + disc

> Practice stubs live under `program/src/`. Type first; peek here only when stuck.

## Step 4 — `program/src/instructions/impl_instructions.rs`

```rust
//! Glue: accounts + data → one Instruction type.

macro_rules! define_instruction {
    ($name:ident, $accounts:ty, $data:ty) => {
        pub struct $name<'a> {
            pub accounts: $accounts,
            pub data: $data,
        }

        impl<'a> From<($accounts, $data)> for $name<'a> {
            fn from((accounts, data): ($accounts, $data)) -> Self {
                Self { accounts, data }
            }
        }

        impl<'a> TryFrom<(&'a [u8], &'a mut [pinocchio::AccountView])> for $name<'a> {
            type Error = pinocchio::error::ProgramError;

            fn try_from(
                (data, accounts): (&'a [u8], &'a mut [pinocchio::AccountView]),
            ) -> Result<Self, Self::Error> {
                Ok(Self {
                    accounts: <$accounts>::try_from(accounts)?,
                    data: <$data>::try_from(data)?,
                })
            }
        }
    };
}

pub(crate) use define_instruction;
```


## Step 4 — `program/src/instructions/mod.rs` (disc + re-exports)

```rust
pub mod close;
pub mod donate;
pub mod initialize;
pub mod withdraw;

pub mod impl_instructions;

/// 1-byte instruction discriminators — remote-control button IDs.
pub mod disc {
    pub const INITIALIZE: u8 = 0;
    pub const DONATE: u8 = 1;
    pub const WITHDRAW: u8 = 2;
    pub const CLOSE: u8 = 3;
}

pub use close::*;
pub use donate::*;
pub use initialize::*;
pub use withdraw::*;
```

