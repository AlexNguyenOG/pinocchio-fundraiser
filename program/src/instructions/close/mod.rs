mod accounts;
mod processor;

pub use accounts::*;

// Close has no extra args — only the discriminator byte.
// TODO: pub struct CloseData; + TryFrom<&[u8]> that ignores leftover / accepts empty
// TODO: define_instruction!(Close, CloseAccounts<'a>, CloseData);
