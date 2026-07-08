//! Withdraw — instruction data.
//!
//! Layout: [0..8] amount (u64 LE), must be > 0.
//! Convention: amount == u64::MAX means "withdraw all raised".
//!
//! WHEN: you want flexible partial or full withdrawals.
//! WHY:  avoid forcing clients to know exact raised balance for a full drain.

// TODO: WithdrawData + TryFrom
