//! Withdraw accounts: [authority, campaign, recipient, system_program]
//!
//! ★ STEP 8
//! authority signer; campaign+recipient writable; campaign owned by ID.
//! recipient needs &mut for direct lamport moves.

use pinocchio::{account::AccountView, error::ProgramError};

// TODO: pub struct WithdrawAccounts<'a> { ... }
// TODO: impl TryFrom for WithdrawAccounts

#[allow(dead_code)]
type _Keep = (AccountView, ProgramError);
