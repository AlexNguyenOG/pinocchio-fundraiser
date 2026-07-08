//! Initialize — instruction data parser.
//!
//! WHEN: client sent args after the discriminator byte.
//! HOW:  TryFrom<&[u8]> → struct { goal: u64, deadline: i64 }
//! WHY:  turn raw bytes into typed values with validation.
//!
//! Layout AFTER disc byte is peeled in lib.rs:
//!   [0..8]  goal     (u64 LE, must be > 0)
//!   [8..16] deadline (i64 LE, 0 = no deadline)
//!
//! Pattern to memorize:
//!   1. require_len!(data, N)
//!   2. from_le_bytes on fixed slices
//!   3. business-rule checks (goal != 0)
//!
//! Analogy: packing slip — measure, decode little-endian stamps, reject bad goals.

// TODO: InitializeData + TryFrom
