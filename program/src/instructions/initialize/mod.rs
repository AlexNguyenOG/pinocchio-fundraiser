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