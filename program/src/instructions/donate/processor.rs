//! Donate — business logic (TIP).
//!
//! Steps (TIP):
//!   1. Type-check — borrow campaign, Campaign::from_bytes, validate_self (PDA)
//!   2. Inspect    — if deadline != 0 and Clock::get()?.unix_timestamp >= deadline → err
//!   3. Pay        — Transfer { from: donor, to: campaign, lamports }.invoke()
//!                   (plain invoke — donor already signed)
//!   4. then       — from_bytes_mut, raised = raised.checked_add(amount)
//!
//! IMPORTANT: drop borrows BEFORE CPI.
//!   try_borrow → read fields → end of block → Transfer → try_borrow_mut
//!
//! WHEN Clock::get: any time-based rule without trusting a client-passed clock account.
//! WHY useful: tip jar that auto-locks after a deadline.

// TODO: impl Donate::process
