# Answers — Step 10 — Mollusk tests

> Practice stubs live under `program/src/`. Type first; peek here only when stuck.

## Step 10 — `program/tests/fundraiser.rs`

_Requires `cargo build-sbf --features bpf-entrypoint` first._

```rust
//! Mollusk integration tests for the fundraiser instruction flow.
//!
//! Prerequisites: build the SBF binary first so Mollusk can load the ELF:
//!   cargo build-sbf --manifest-path program/Cargo.toml --features bpf-entrypoint

use {
    mollusk_svm::{program::keyed_account_for_system_program, result::Check, Mollusk},
    pinocchio_fundraiser::{
        constants::CAMPAIGN_SEED, instructions::disc, state::Campaign, traits::AccountSize, ID,
    },
    solana_account::Account,
    solana_instruction::{AccountMeta, Instruction},
    solana_program_error::ProgramError,
    solana_pubkey::Pubkey,
    solana_sdk_ids::system_program,
};

fn program_id() -> Pubkey {
    Pubkey::new_from_array(*ID.as_array())
}

fn setup() -> Mollusk {
    // Prefer an explicit deploy dir; otherwise resolve next to this crate.
    // Cursor agents often set CARGO_TARGET_DIR away from the workspace root.
    let candidates = [
        std::env::var("SBF_OUT_DIR").ok(),
        std::env::var("CARGO_TARGET_DIR")
            .ok()
            .map(|d| format!("{d}/deploy")),
        Some(concat!(env!("CARGO_MANIFEST_DIR"), "/../target/deploy").to_string()),
        Some(concat!(env!("CARGO_MANIFEST_DIR"), "/target/deploy").to_string()),
    ];

    let deploy = candidates
        .into_iter()
        .flatten()
        .find(|dir| {
            std::path::Path::new(dir)
                .join("pinocchio_fundraiser.so")
                .exists()
        })
        .expect(
            "pinocchio_fundraiser.so not found — run: cargo build-sbf --manifest-path program/Cargo.toml --features bpf-entrypoint",
        );

    std::env::set_var("SBF_OUT_DIR", &deploy);
    Mollusk::new(&program_id(), "pinocchio_fundraiser")
}

fn system_account(lamports: u64) -> Account {
    Account::new(lamports, 0, &system_program::id())
}

/// Every instruction list that CPIs to System must include this keyed account.
fn with_system(accounts: &[(Pubkey, Account)]) -> Vec<(Pubkey, Account)> {
    let mut out = accounts.to_vec();
    out.push(keyed_account_for_system_program());
    out
}

fn campaign_pda(authority: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[CAMPAIGN_SEED, authority.as_ref()], &program_id())
}

fn initialize_ix(authority: Pubkey, campaign: Pubkey, goal: u64, deadline: i64) -> Instruction {
    let mut data = vec![disc::INITIALIZE];
    data.extend_from_slice(&goal.to_le_bytes());
    data.extend_from_slice(&deadline.to_le_bytes());
    Instruction::new_with_bytes(
        program_id(),
        &data,
        vec![
            AccountMeta::new(authority, true),
            AccountMeta::new(campaign, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
    )
}

fn donate_ix(donor: Pubkey, campaign: Pubkey, amount: u64) -> Instruction {
    let mut data = vec![disc::DONATE];
    data.extend_from_slice(&amount.to_le_bytes());
    Instruction::new_with_bytes(
        program_id(),
        &data,
        vec![
            AccountMeta::new(donor, true),
            AccountMeta::new(campaign, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
    )
}

fn withdraw_ix(authority: Pubkey, campaign: Pubkey, recipient: Pubkey, amount: u64) -> Instruction {
    let mut data = vec![disc::WITHDRAW];
    data.extend_from_slice(&amount.to_le_bytes());
    Instruction::new_with_bytes(
        program_id(),
        &data,
        vec![
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new(campaign, false),
            AccountMeta::new(recipient, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
    )
}

fn close_ix(authority: Pubkey, campaign: Pubkey, recipient: Pubkey) -> Instruction {
    Instruction::new_with_bytes(
        program_id(),
        &[disc::CLOSE],
        vec![
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new(campaign, false),
            AccountMeta::new(recipient, false),
        ],
    )
}

#[test]
fn initialize_creates_campaign() {
    let mollusk = setup();
    let authority = Pubkey::new_unique();
    let (campaign, _bump) = campaign_pda(&authority);

    let goal = 1_000_000_000u64;
    let deadline = 0i64;
    let rent = mollusk.sysvars.rent.minimum_balance(Campaign::LEN);

    let ix = initialize_ix(authority, campaign, goal, deadline);

    mollusk.process_and_validate_instruction(
        &ix,
        &with_system(&[
            (authority, system_account(10_000_000_000)),
            (campaign, system_account(0)),
        ]),
        &[
            Check::success(),
            Check::account(&campaign)
                .owner(&program_id())
                .space(Campaign::LEN)
                .lamports(rent)
                .data_slice(0, &[1, 1]) // disc + version
                .data_slice(2, authority.as_ref())
                .data_slice(34, &goal.to_le_bytes())
                .data_slice(42, &0u64.to_le_bytes()) // raised
                .data_slice(50, &deadline.to_le_bytes())
                .build(),
        ],
    );
}

#[test]
fn initialize_rejects_zero_goal() {
    let mollusk = setup();
    let authority = Pubkey::new_unique();
    let (campaign, _) = campaign_pda(&authority);

    mollusk.process_and_validate_instruction(
        &initialize_ix(authority, campaign, 0, 0),
        &with_system(&[
            (authority, system_account(10_000_000_000)),
            (campaign, system_account(0)),
        ]),
        &[Check::err(ProgramError::Custom(1))], // InvalidGoal
    );
}

#[test]
fn donate_and_withdraw() {
    let mollusk = setup();
    let authority = Pubkey::new_unique();
    let donor = Pubkey::new_unique();
    let recipient = Pubkey::new_unique();
    let (campaign, _) = campaign_pda(&authority);

    let goal = 2_000_000_000u64;
    let donation = 1_000_000_000u64;
    let withdraw = 400_000_000u64;
    let rent = mollusk.sysvars.rent.minimum_balance(Campaign::LEN);

    let ix_init = initialize_ix(authority, campaign, goal, 0);
    let ix_donate = donate_ix(donor, campaign, donation);
    let ix_withdraw = withdraw_ix(authority, campaign, recipient, withdraw);

    mollusk.process_and_validate_instruction_chain(
        &[
            (
                &ix_init,
                &[
                    Check::success(),
                    Check::account(&campaign).owner(&program_id()).build(),
                ],
            ),
            (
                &ix_donate,
                &[
                    Check::success(),
                    Check::account(&campaign)
                        .lamports(rent + donation)
                        .data_slice(42, &donation.to_le_bytes())
                        .build(),
                ],
            ),
            (
                &ix_withdraw,
                &[
                    Check::success(),
                    Check::account(&campaign)
                        .lamports(rent + donation - withdraw)
                        .data_slice(42, &(donation - withdraw).to_le_bytes())
                        .build(),
                    Check::account(&recipient).lamports(withdraw).build(),
                ],
            ),
        ],
        &with_system(&[
            (authority, system_account(10_000_000_000)),
            (donor, system_account(10_000_000_000)),
            (recipient, system_account(0)),
            (campaign, system_account(0)),
        ]),
    );
}

#[test]
fn close_when_goal_met() {
    let mollusk = setup();
    let authority = Pubkey::new_unique();
    let donor = Pubkey::new_unique();
    let (campaign, _) = campaign_pda(&authority);

    let goal = 500_000_000u64;
    let donation = 500_000_000u64;
    let rent = mollusk.sysvars.rent.minimum_balance(Campaign::LEN);
    let authority_start = 10_000_000_000u64;

    let ix_init = initialize_ix(authority, campaign, goal, 0);
    let ix_donate = donate_ix(donor, campaign, donation);
    let ix_close = close_ix(authority, campaign, authority);

    mollusk.process_and_validate_instruction_chain(
        &[
            (&ix_init, &[Check::success()]),
            (
                &ix_donate,
                &[
                    Check::success(),
                    Check::account(&campaign)
                        .data_slice(42, &donation.to_le_bytes())
                        .build(),
                ],
            ),
            (
                &ix_close,
                &[
                    Check::success(),
                    Check::account(&campaign).closed().build(),
                    // authority paid rent at init, then received campaign lamports (rent + donation)
                    Check::account(&authority)
                        .lamports(authority_start - rent + rent + donation)
                        .build(),
                ],
            ),
        ],
        &with_system(&[
            (authority, system_account(authority_start)),
            (donor, system_account(10_000_000_000)),
            (campaign, system_account(0)),
        ]),
    );
}

#[test]
fn close_rejects_before_goal_or_deadline() {
    let mollusk = setup();
    let authority = Pubkey::new_unique();
    let donor = Pubkey::new_unique();
    let (campaign, _) = campaign_pda(&authority);

    let goal = 10_000_000_000u64;
    let donation = 100_000_000u64;

    let ix_init = initialize_ix(authority, campaign, goal, 0);
    let ix_donate = donate_ix(donor, campaign, donation);
    let ix_close = close_ix(authority, campaign, authority);

    mollusk.process_and_validate_instruction_chain(
        &[
            (&ix_init, &[Check::success()]),
            (&ix_donate, &[Check::success()]),
            (
                &ix_close,
                &[Check::err(ProgramError::Custom(5))], // CannotClose
            ),
        ],
        &with_system(&[
            (authority, system_account(10_000_000_000)),
            (donor, system_account(10_000_000_000)),
            (campaign, system_account(0)),
        ]),
    );
}

#[test]
fn close_after_deadline() {
    let mut mollusk = setup();
    let authority = Pubkey::new_unique();
    let donor = Pubkey::new_unique();
    let (campaign, _) = campaign_pda(&authority);

    let now = 1_700_000_000i64;
    mollusk.sysvars.clock.unix_timestamp = now;

    let goal = 10_000_000_000u64; // not met by the small donation
    let deadline = now + 60;
    let donation = 100_000_000u64;
    let rent = mollusk.sysvars.rent.minimum_balance(Campaign::LEN);
    let authority_start = 10_000_000_000u64;

    let ix_init = initialize_ix(authority, campaign, goal, deadline);
    let ix_donate = donate_ix(donor, campaign, donation);
    let ix_close = close_ix(authority, campaign, authority);

    let result = mollusk.process_and_validate_instruction_chain(
        &[
            (&ix_init, &[Check::success()]),
            (
                &ix_donate,
                &[
                    Check::success(),
                    Check::account(&campaign)
                        .lamports(rent + donation)
                        .build(),
                ],
            ),
        ],
        &with_system(&[
            (authority, system_account(authority_start)),
            (donor, system_account(10_000_000_000)),
            (campaign, system_account(0)),
        ]),
    );

    mollusk.sysvars.clock.unix_timestamp = deadline + 1;

    mollusk.process_and_validate_instruction(
        &ix_close,
        &result.resulting_accounts,
        &[
            Check::success(),
            Check::account(&campaign).closed().build(),
            Check::account(&authority)
                .lamports(authority_start - rent + rent + donation)
                .build(),
        ],
    );
}

#[test]
fn donate_rejects_after_deadline() {
    let mut mollusk = setup();
    let authority = Pubkey::new_unique();
    let donor = Pubkey::new_unique();
    let (campaign, _) = campaign_pda(&authority);

    let now = 1_700_000_000i64;
    mollusk.sysvars.clock.unix_timestamp = now;
    let deadline = now + 10;

    let result = mollusk.process_and_validate_instruction(
        &initialize_ix(authority, campaign, 1_000_000, deadline),
        &with_system(&[
            (authority, system_account(10_000_000_000)),
            (donor, system_account(10_000_000_000)),
            (campaign, system_account(0)),
        ]),
        &[Check::success()],
    );

    mollusk.sysvars.clock.unix_timestamp = deadline;

    mollusk.process_and_validate_instruction(
        &donate_ix(donor, campaign, 100_000),
        &result.resulting_accounts,
        &[Check::err(ProgramError::Custom(3))], // DeadlinePassed
    );
}
```

