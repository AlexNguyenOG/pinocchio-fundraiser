//! Withdraw — business logic (SIGN).
//!
//! Steps (SIGN):
//!   1. Secure    — from_bytes + validate_self; authority.address == campaign.authority
//!   2. Inventory — amount = MAX ? raised : requested; reject if 0 or > raised
//!   3. Give      — Transfer from campaign → recipient with invoke_signed
//!                  seeds: [CAMPAIGN_SEED, authority, &[bump]]
//!   4. Note      — raised = raised.checked_sub(amount)
//!
//! WHEN invoke_signed: the *from* account is a PDA (no private key).
//! HOW useful: only your program can empty the jar — humans can't forge PDA sigs.
//!
//! Keep rent in the account: only withdraw `raised`, not the rent-exempt reserve.

// TODO: impl Withdraw::process
