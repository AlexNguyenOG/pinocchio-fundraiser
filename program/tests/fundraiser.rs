//! ★ STEP 10 — Mollusk flow tests (after program + SBF build work).
//!
//! Rebuild first:
//!   cargo build-sbf --manifest-path program/Cargo.toml --features bpf-entrypoint
//!
//! Cover: init, donate+withdraw, close on goal, close after deadline, rejects.

#[test]
fn tests_come_last() {
    // TODO: rewrite fundraiser integration tests
}
