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
