mod accounts;
mod data;
mod processor;

pub use accounts::*;
pub use data::*;



use crate::instructions::impl_instructions::define_instruction;

define_instruction!(Withdraw, WithdrawAccounts<'a>, WithdrawData);
