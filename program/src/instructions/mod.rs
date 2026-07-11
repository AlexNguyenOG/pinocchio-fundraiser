pub mod close;
pub mod donate;
pub mod initialize;
pub mod withdraw;

pub mod impl_instructions;

/// 1-byte instruction discriminators — remote-control button IDs.
pub mod disc {
    pub const INITIALIZE: u8 = 0;
    pub const DONATE: u8 = 1;
    pub const WITHDRAW: u8 = 2;
    pub const CLOSE: u8 = 3;
}

pub use initialize::*;
// pub use donate::*;
// pub use withdraw::*;
// pub use close::*;
