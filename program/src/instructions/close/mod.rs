mod accounts;
mod processor;

pub use accounts::*;

// Close has no extra args — only the discriminator byte.
// TODO: pub struct CloseData; + TryFrom<&[u8]> that ignores leftover / accepts empty
// TODO: define_instruction!(Close, CloseAccounts<'a>, CloseData);
pub mod disc {
    pub const INITIALIZE: u8 = 0;
    pub const DONATE: u8 = 1;
    pub const WITHDRAW: u8 = 2;
    pub const CLOSE: u8 = 3;
}