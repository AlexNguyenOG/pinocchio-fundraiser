//! PDA seed prefix.
//!
//! WHEN: any time you derive/sign a campaign PDA.
//! HOW:  `pub const CAMPAIGN_SEED: &[u8] = b"campaign";`
//! WHY:  seeds make PDA addresses reproducible and unique per authority.
//!
//! Analogy: the printed word on every locker label before the owner's name.

// TODO: define CAMPAIGN_SEED
pub const CAMPAIGN_SEED: &[u8] = b"campaign";