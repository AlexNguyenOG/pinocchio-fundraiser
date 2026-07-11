//! Pinocchio Fundraiser — on-chain SOL crowdfunding campaign.

#![cfg_attr(target_os = "solana", no_std)]

#[macro_use]
pub mod utils;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod traits;

use instructions::{disc, Initialize};
use pinocchio::{
    account::AccountView,
    address::Address,
    error::ProgramError,
    ProgramResult,
};

/// Program ID — replace with your deployed keypair pubkey after first build.
pub const ID: Address = Address::new_from_array([0u8; 32]);

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
        // TODO: DONATE, WITHDRAW, CLOSE
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
