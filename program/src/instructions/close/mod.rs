mod accounts;
mod processor;

pub use accounts::*;

use pinocchio::error::ProgramError;

// ★ STEP 9 — CloseData TryFrom always Ok(Self), then define_instruction!
// TODO: pub struct CloseData;
// TODO: impl TryFrom<&[u8]> for CloseData
// TODO: define_instruction!(Close, CloseAccounts<'a>, CloseData);

#[allow(dead_code)]
type _Keep = ProgramError;
