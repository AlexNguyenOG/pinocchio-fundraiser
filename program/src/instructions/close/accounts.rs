//! Close accounts: [authority, campaign, recipient]
//!
//! ★ STEP 9

use pinocchio::{account::AccountView, error::ProgramError};

// TODO: pub struct CloseAccounts<'a> { ... }
// TODO: impl TryFrom for CloseAccounts

#[allow(dead_code)]
type _Keep = (AccountView, ProgramError);
