pub mod close;
pub mod donate;
pub mod initialize;
pub mod withdraw;

pub mod impl_instructions;

/// TODO: 1-byte instruction discriminators.
///
/// WHEN: routing in lib.rs + building client instruction data.
/// HOW:  pub mod disc { pub const INITIALIZE: u8 = 0; ... }
/// WHY:  one byte selects the handler; keeps wire format tiny.
/// Analogy: remote-control button IDs.
pub mod disc {
    // TODO
}

// After you define_instruction! in each folder, re-export:
// pub use close::*;
// pub use donate::*;
// pub use initialize::*;
// pub use withdraw::*;
