//! Pinocchio Fundraiser — on-chain SOL crowdfunding campaign.

#![cfg_attr(target_os = "solana", no_std)]

#[macro_use]
pub mod utils;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod traits;

use instructions::{disc, Close, Donate, Initialize, Withdraw};
use pinocchio::{
    account::AccountView,
    address::Address,
    error::ProgramError,
    ProgramResult,
};

/// Program ID — matches `target/deploy/pinocchio_fundraiser-keypair.json`.
/// Must not be the all-zero address (that collides with the System Program).
pub const ID: Address = Address::new_from_array([
    183, 6, 218, 73, 206, 82, 28, 197, 79, 131, 209, 50, 75, 61, 33, 79, 123, 189, 124, 122, 189,
    145, 26, 86, 125, 130, 111, 244, 178, 67, 8, 80,
]);

/// On-chain entrypoint (only when building with `--features bpf-entrypoint`).
#[cfg(feature = "bpf-entrypoint")]
mod entrypoint {
    use super::*;

    pinocchio::program_entrypoint!(process_instruction);
    pinocchio::default_allocator!();
    pinocchio::nostd_panic_handler!();
}

/// Front door — Solana calls this once per instruction.
#[cfg_attr(not(feature = "bpf-entrypoint"), allow(dead_code))]
fn process_instruction(
    program_id: &Address,
    accounts: &mut [AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data.split_first() {
        Some((&disc::INITIALIZE, data)) => {
            let mut ix = Initialize::try_from((data, accounts))?;
            ix.process(program_id)
        }
        Some((&disc::DONATE, data)) => {
            let mut ix = Donate::try_from((data, accounts))?;
            ix.process(program_id)
        }
        Some((&disc::WITHDRAW, data)) => {
            let mut ix = Withdraw::try_from((data, accounts))?;
            ix.process(program_id)
        }
        Some((&disc::CLOSE, data)) => {
            let mut ix = Close::try_from((data, accounts))?;
            ix.process(program_id)
        }
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
