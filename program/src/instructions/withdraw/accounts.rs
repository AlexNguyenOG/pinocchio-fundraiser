//! Withdraw — account validation.
//!
//! WHEN: moving SOL *out* of the PDA (authority-only).
//!
//! Accounts:
//!   0. authority       — signer (must match campaign.authority later)
//!   1. campaign        — writable, owned by ID
//!   2. recipient       — writable (where SOL goes)
//!   3. system_program
//!
//! Note: authority does NOT need to be writable here — they aren't paying.
//! Campaign must be writable — lamports leave it.
//!
//! WHY separate recipient: authority can send raised funds to a treasury wallet.

// TODO: WithdrawAccounts + TryFrom
