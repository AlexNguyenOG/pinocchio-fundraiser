//! Pinocchio Fundraiser — practice stubs.
//!
//! Fill these files yourself. Start with PRACTICE.md for when/why/how.

#![cfg_attr(target_os = "solana", no_std)]

#[macro_use]
pub mod utils;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod traits;

use pinocchio::{
    account::AccountView,
    address::Address,
    error::ProgramError,
    ProgramResult,
};

/// TODO: replace with your deployed program pubkey (32 bytes).
///
/// WHEN: after first `cargo build-sbf` — copy pubkey from the keypair JSON.
/// WHY:  clients and PDA derivation must agree on *which* program owns the PDAs.
pub const ID: Address = Address::new_from_array([0u8; 32]);

/// TODO: emit the on-chain entrypoint when `bpf-entrypoint` is enabled.
///
/// WHEN: only for Solana/SBF builds (not needed for plain `cargo check` of logic).
/// HOW:  for `no_std` Solana, prefer:
///         pinocchio::program_entrypoint!(process_instruction);
///         pinocchio::default_allocator!();
///         pinocchio::nostd_panic_handler!();
/// WHY:  the runtime must know where to jump; panic/allocator are required on-chain.
#[cfg(feature = "bpf-entrypoint")]
mod entrypoint {
    // TODO
}

/// Front door — Solana calls this once per instruction.
///
/// Analogy: a receptionist. They look at your ticket number (first byte)
/// and send you to the right counter.
///
/// WHEN: every program has exactly one of these.
/// USEFUL: keeps routing in one place; instructions don't care about each other.
#[cfg_attr(not(feature = "bpf-entrypoint"), allow(dead_code))]
fn process_instruction(
    _program_id: &Address,
    _accounts: &mut [AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    // TODO: match on instruction_data.split_first()
    //   0 => Initialize
    //   1 => Donate
    //   2 => Withdraw
    //   3 => Close
    //   _ => InvalidInstructionData
    //
    // Pattern:
    //   Some((&disc, data)) => {
    //       let mut ix = MyIx::try_from((data, accounts))?;
    //       ix.process(program_id)
    //   }
    let _ = instruction_data;
    Err(ProgramError::InvalidInstructionData)
}
