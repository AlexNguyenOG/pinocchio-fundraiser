//! Close processor — mnemonic END.
//!
//! ★ STEP 9
//! E — enforce (authority + goal met OR deadline passed)  
//! N — nudge all lamports to recipient  
//! D — destroy (campaign.close())  

use pinocchio::{address::Address, ProgramResult};

// TODO: impl Close { pub fn process(...) -> ProgramResult }

#[allow(dead_code)]
type _Keep = (Address, ProgramResult);
