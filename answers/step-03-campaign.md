# Answers — Step 3 — Campaign state

> Practice stubs live under `program/src/`. Type first; peek here only when stuck.

## Step 3 — `program/src/state/campaign.rs`

```rust
use crate::constants::CAMPAIGN_SEED;
use crate::traits::{
    AccountDeserialize, AccountSize, Discriminator, PdaAccount, PdaSeeds, Versioned,
};

#[repr(C)] pub struct Campaign {
    pub authority: [u8; 32],
    pub goal: u64,
    pub raised: u64,
    pub deadline: i64,
    pub bump: u8,
    pub _reserved: [u8; 7]
}

impl AccountSize for Campaign {
    const DATA_LEN: usize = 64;
}

impl Discriminator for Campaign {
    const DISCRIMINATOR: u8 = 1;
}

impl Versioned for Campaign {
    const VERSION: u8 = 1;
}

impl AccountDeserialize for Campaign {
}

assert_no_padding!(Campaign, Campaign::DATA_LEN);

impl PdaSeeds for Campaign {
    const PREFIX: &'static [u8] = CAMPAIGN_SEED;

    fn seeds(&self) -> [&[u8]; 2] {
        [Self::PREFIX, self.authority.as_ref()]
    }
}

impl PdaAccount for Campaign {
    fn bump(&self) -> u8 {
        self.bump
    }
}

impl Campaign {
    /// PDA seeds before the campaign account exists (used in initialize).
    pub fn seeds_for(authority: &[u8; 32]) -> [&[u8]; 2] {
        [CAMPAIGN_SEED, authority.as_ref()]
    }
}
```

