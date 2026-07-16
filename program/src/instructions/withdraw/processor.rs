//! Withdraw processor — authorize, debit lamports, update raised.
//!
//! ★ STEP 8
//! S — secure (PDA + authority match)  
//! I — inventory amount  
//! G — give via set_lamports (not System Transfer from data account)  
//! N — note raised -= amount  

use pinocchio::{address::Address, ProgramResult};

// TODO: impl Withdraw { pub fn process(...) -> ProgramResult }

#[allow(dead_code)]
type _Keep = (Address, ProgramResult);
