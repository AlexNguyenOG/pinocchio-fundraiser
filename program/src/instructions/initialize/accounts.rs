//! Initialize — account validation (the bouncer).
//!
//! WHEN: first step of every instruction — before any CPI or writes.
//! HOW:  struct holding &AccountView / &mut AccountView + TryFrom<&mut [AccountView]>
//! WHY:  if you skip checks, attackers pass fake accounts and steal funds.
//!
//! Expected accounts (ORDER MATTERS — Pinocchio has no name matching):
//!   0. authority       — signer, writable (pays rent)
//!   1. campaign        — writable PDA, still System-owned + empty
//!   2. system_program  — must equal pinocchio_system::ID
//!
//! Checklist to memorize (SOW):
//!   S — Signer?     authority.is_signer()
//!   O — Owner/ID?   campaign.owned_by(System), system_program.address() == System
//!   W — Writable?   authority + campaign must be writable
//!
//! Also:
//!   derive_program_address([CAMPAIGN_SEED, authority], &ID) == campaign.address()
//!   campaign.is_data_empty()  → not already initialized
//!
//! Analogy: apartment lease — only the named signer can rent; unit must be vacant.

// TODO: InitializeAccounts + TryFrom
