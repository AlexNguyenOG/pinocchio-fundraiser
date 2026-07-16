# Practice rebuild — current step

Branch: `practice-from-scratch`  
Answers (per step — no scrolling): **[answers/](./answers/)** · index **[ANSWERS.md](./ANSWERS.md)**

## Build order

| Step | Files | Status |
|------|--------|--------|
| 1 | `constants.rs` → `utils/macros.rs` → `error.rs` | ✅ done |
| 2 | `traits/mod.rs` | ✅ done |
| 3 | `state/campaign.rs` | ✅ done |
| 4 | `instructions/impl_instructions.rs` + discs | ✅ done |
| **5** | `initialize/` | **← you are here** → [`answers/step-05-initialize.md`](./answers/step-05-initialize.md) |
| 4 | `instructions/impl_instructions.rs` + discs | [`answers/step-04-instruction-glue.md`](./answers/step-04-instruction-glue.md) |
| 5 | `initialize/` | [`answers/step-05-initialize.md`](./answers/step-05-initialize.md) |
| 6 | Wire `lib.rs` | [`answers/step-06-lib-router.md`](./answers/step-06-lib-router.md) |
| 7 | `donate/` | [`answers/step-07-donate.md`](./answers/step-07-donate.md) |
| 8 | `withdraw/` | [`answers/step-08-withdraw.md`](./answers/step-08-withdraw.md) |
| 9 | `close/` | [`answers/step-09-close.md`](./answers/step-09-close.md) |
| 10 | Mollusk tests | [`answers/step-10-tests.md`](./answers/step-10-tests.md) |

After each step: `cargo check`.

---

## Fix from step 1 before continuing (`macros.rs`)

`constants` + `error` look good. Macros need a clean pass — `cargo check` fails today.

Shape of each macro (no extra `;` after the outer `}`):

```rust
macro_rules! require_len {
    ($data:expr, $len:expr) => {
        if $data.len() < $len {
            return Err(pinocchio::error::ProgramError::InvalidInstructionData);
        }
    };
}
```

Watch for:
1. Path is `pinocchio::error::ProgramError`, not `Error`
2. Typo: `pinocchio` not `pinoccio`
3. Each arm ends with `};` inside the macro; the macro itself ends with a single `}`
4. `assert_no_padding!` message is a plain string (no `{}` placeholders), matching ANSWERS step 1b

---

## Step 2 — `program/src/traits/mod.rs`

**When:** every typed account + every PDA needs shared read/validate helpers  
**Why:** zero-copy (no Borsh alloc) + reusable PDA checks  

Implement (in order):

1. `Discriminator` — `const DISCRIMINATOR: u8`
2. `Versioned` — `const VERSION: u8`
3. `AccountSize` — `DATA_LEN` + default `LEN = 1 + 1 + DATA_LEN`
4. `AccountDeserialize` — `from_bytes` / `from_bytes_mut` / `init_bytes` (use your macros; `unsafe` cast from `data[2..]`)
5. `PdaSeeds` — `PREFIX`, `seeds()`, `derive_address`, `derive_address_with_bump`, `validate_pda`
6. `PdaAccount` — `bump()` + `validate_self`

PRACTICE.md §4 · stuck → **[`answers/step-02-traits.md`](./answers/step-02-traits.md)** only.

When done: say **“step 2 done”**.

