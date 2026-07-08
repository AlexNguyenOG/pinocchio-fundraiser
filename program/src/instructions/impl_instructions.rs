//! Glue: accounts + data → one Instruction type.
//!
//! WHEN: every instruction that has both accounts and data parsers.
//! HOW:  macro_rules! define_instruction!($Name, $Accounts, $Data)
//!       expanding to:
//!         struct $Name<'a> { accounts: $Accounts, data: $Data }
//!         TryFrom<(&[u8], &mut [AccountView])>
//! WHY:  avoid copy-pasting the same TryFrom boilerplate 4 times.
//!
//! IMPORTANT: inside the macro, use fully-qualified paths:
//!   pinocchio::AccountView
//!   pinocchio::error::ProgramError
//! (macro hygiene won't see your imports at the call site.)
//!
//! Analogy: sandwich press — bread + filling → lunch.

// TODO: define_instruction! macro
// TODO: pub(crate) use define_instruction;
