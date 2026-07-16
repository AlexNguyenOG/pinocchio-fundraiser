mod accounts;
mod data;
mod processor;

pub use accounts::*;
pub use data::*;

// ★ STEP 8 — wire define_instruction!(Withdraw, ...)
// Reminder: System Transfer cannot pull FROM a data-carrying PDA —
// move lamports with set_lamports instead.

// TODO: define_instruction!(Withdraw, WithdrawAccounts<'a>, WithdrawData);
