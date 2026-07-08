//! Donate — account validation.
//!
//! WHEN: anyone can send SOL in (no authority required).
//!
//! Accounts:
//!   0. donor           — signer, writable
//!   1. campaign        — writable, owned by OUR program (ID)
//!   2. system_program  — pinocchio_system::ID
//!
//! Contrast with initialize:
//!   init  → campaign owned by System + empty
//!   donate → campaign owned by ID + already has Campaign data
//!
//! WHY: wrong owner means you're not writing to a real campaign.

// TODO: DonateAccounts + TryFrom
