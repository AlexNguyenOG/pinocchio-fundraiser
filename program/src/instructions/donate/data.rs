//! Donate — instruction data.
//!
//! Layout: [0..8] amount (u64 LE), must be > 0.
//!
//! WHEN: parsing the rest of instruction_data after disc byte 1.
//! HOW:  require_len! → from_le_bytes → reject zero.
//! WHY:  zero donations are noise / potential griefing.

// TODO: DonateData + TryFrom
