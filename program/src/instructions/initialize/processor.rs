//! Initialize — business logic (OPEN).
//!
//! WHEN: accounts + data already validated; now mutate chain state.
//! HOW:  impl Initialize { fn process(&mut self, program_id) -> ProgramResult }
//!
//! Steps (OPEN):
//!   1. Obtain bump  — Address::derive_program_address(&[b"campaign", authority], &ID)
//!   2. Provision    — CreateAccount::with_minimum_balance(...).invoke_signed(&[Signer])
//!   3. Engrave      — campaign.try_borrow_mut → Campaign::init_bytes → fill fields
//!   4. Note bump    — store bump in state for later withdraw/close
//!
//! WHEN invoke_signed vs invoke:
//!   invoke_signed — PDA must appear as signer (create PDA, PDA sends SOL)
//!   invoke        — a real keypair already signed the tx (donor transfer)
//!
//! WHY useful: create a program-owned tip jar that only THIS program can unlock later.

// TODO: impl Initialize::process
