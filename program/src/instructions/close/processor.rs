//! Close — business logic (END).
//!
//! Steps (END):
//!   1. Enforce — auth match; allow if raised >= goal OR (deadline != 0 && now >= deadline)
//!   2. Nudge   — recipient.set_lamports(recipient + campaign); campaign.set_lamports(0)
//!   3. Destroy — campaign.close()  // zeroes owner + data_len; stops revival attacks
//!
//! WHEN account.close(): after draining lamports from a program-owned account.
//! WHY useful: reclaim rent + prevent someone from reincarnating a "closed" account
//!             with leftover data bits.
//!
//! Analogy: empty the tip jar into the organizer's bag, then melt the jar.

// TODO: impl Close::process
