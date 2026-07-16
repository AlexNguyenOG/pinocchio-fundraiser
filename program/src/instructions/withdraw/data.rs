//! Withdraw args: amount:u64. Reject 0.
//! (u64::MAX can mean "withdraw all raised" in process.)
//!
//! ★ STEP 8

use pinocchio::error::ProgramError;

// TODO: pub struct WithdrawData { pub amount: u64 }
// TODO: impl TryFrom<&[u8]> for WithdrawData

#[allow(dead_code)]
type _Keep = ProgramError;
