//! On-chain Campaign account.
//!
//! WHEN: you need durable state (authority, goal, raised, deadline, bump).
//! HOW:  #[repr(C)] struct + implement Discriminator / Versioned / AccountSize /
//!       AccountDeserialize / PdaSeeds / PdaAccount.
//! WHY:  zero-copy reads are cheap; fixed layout is predictable.
//!
//! Account bytes layout:
//!   [0] disc  [1] version  [2..] Campaign payload
//!
//! Suggested fields (order largest → smallest alignment):
//!   authority: [u8; 32]
//!   goal: u64
//!   raised: u64
//!   deadline: i64   // 0 = no deadline
//!   bump: u8
//!   _reserved: [u8; 7]
//!
//! DATA_LEN should be 64. Use assert_no_padding!(Campaign, 64).
//!
//! PDA seeds: [CAMPAIGN_SEED, authority]
//! Also add: seeds_for(authority: &[u8; 32]) helper for use *before* state exists.

// TODO: impl Campaign state + traits
