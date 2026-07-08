//! Close — account validation.
//!
//! WHEN: permanently ending a campaign and reclaiming rent.
//!
//! Accounts:
//!   0. authority  — signer
//!   1. campaign   — writable, owned by ID
//!   2. recipient — writable (gets ALL remaining lamports including rent)
//!
//! No system_program needed if you move lamports manually + account.close()
//! (you're not doing a System Transfer CPI for the close path).

// TODO: CloseAccounts + TryFrom
