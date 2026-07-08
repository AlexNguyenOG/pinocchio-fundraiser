# Pinocchio Fundraiser — Practice Workbook

The source files are **empty stubs with teaching comments**. Type the code yourself.
This guide tells you **when** to reach for a pattern, **how** it works, and **why** it’s useful.

---

## What you’re building

A SOL crowdfunding tip jar:

| Disc | Instruction | Who | Job |
|------|-------------|-----|-----|
| `0` | initialize | Authority | Create campaign PDA |
| `1` | donate | Anyone | Send SOL in |
| `2` | withdraw | Authority | Pull raised SOL out (PDA signs) |
| `3` | close | Authority | Drain + close when done |

---

## How to study (don’t skip this)

For every pattern, ask three questions out loud:

1. **When?** What problem am I solving right now?
2. **How?** What’s the smallest correct shape of the code?
3. **Why useful?** What breaks if I leave it out?

Then type it. Compile often: `cargo check`.

---

## Suggested build order

Work bottom-up so each layer can compile before the next needs it.

1. `constants.rs`
2. `utils/macros.rs`
3. `error.rs`
4. `traits/mod.rs`
5. `state/campaign.rs`
6. `instructions/impl_instructions.rs`
7. `initialize/` (accounts → data → processor → wire `define_instruction!`)
8. Wire disc `0` in `lib.rs` + entrypoint
9. `donate/` → disc `1`
10. `withdraw/` → disc `2`
11. `close/` → disc `3`

---

## Pattern cheat sheet (memorize this)

### 1. Constant seed — `b"campaign"`

| | |
|--|--|
| **When** | Deriving or signing a PDA |
| **How** | `pub const CAMPAIGN_SEED: &[u8] = b"campaign";` |
| **Useful** | Same seeds → same address; program can prove ownership later |
| **Skip it?** | Addresses become random or forgeable |

**Analogy:** printed word on every locker label.

---

### 2. Macros — `require_len!`, etc.

| Macro | When | Error |
|-------|------|-------|
| `require_len!` | Before parsing ix data | `InvalidInstructionData` |
| `require_account_len!` | Before reading account bytes | `InvalidAccountData` |
| `validate_discriminator!` | Before treating bytes as a typed account | `InvalidAccountData` |
| `assert_no_padding!` | After defining `#[repr(C)]` state | compile fail if size wrong |

**Useful:** fail fast; don’t panic mid-parse.  
**Analogy:** measure the envelope before opening it.

---

### 3. Custom errors

| | |
|--|--|
| **When** | Business rules (deadline, unauthorized, zero goal) |
| **How** | `#[repr(u32)] enum` + `From` → `ProgramError::Custom(n)` |
| **Useful** | Clients show clear messages; on-chain stays a number |

---

### 4. Traits — Discriminator / Versioned / AccountSize / AccountDeserialize / PDA

| Trait | When | Useful |
|-------|------|--------|
| `Discriminator` | Every program-owned account type | Stop type confusion (byte 0 sticker) |
| `Versioned` | Layout might evolve | Branch on old vs new (byte 1) |
| `AccountSize` | Create / length-check accounts | Know exact rent size |
| `AccountDeserialize` | Read/write state | Zero-copy = faster, smaller than Borsh |
| `PdaSeeds` / `PdaAccount` | Program-controlled accounts | PDA can “sign” via seeds |

**Account layout to remember:**

```
[disc:1][version:1][payload: DATA_LEN]
```

Pinocchio disc = **1 byte**. Anchor disc = **8 bytes**. Don’t mix them up.

**Analogy:** clear plastic overlay on a paper form (not photocopying the form).

---

### 5. `TryFrom` for accounts (the bouncer)

| | |
|--|--|
| **When** | Start of every instruction |
| **How** | Destructure `&mut [AccountView]` by **index order**, then SOW checks |
| **Useful** | Attackers can’t swap fake accounts past you |

**SOW checklist (say it every time):**

1. **S**igner? — `is_signer()` when approval / payment is required  
2. **O**wner / program ID? — `owned_by`, `address() == expected`  
3. **W**ritable? — if you’ll change lamports or data  

**Order matters.** Pinocchio does not match accounts by name like Anchor.

---

### 6. `TryFrom` for instruction data

| | |
|--|--|
| **When** | Client sent args after the disc byte |
| **How** | `require_len!` → `from_le_bytes` → business checks |
| **Useful** | Typed values instead of magic byte indexes everywhere |

**Remember:** Solana = **little-endian**.

---

### 7. `define_instruction!`

| | |
|--|--|
| **When** | You have both Accounts + Data parsers |
| **How** | Macro builds `struct Ix { accounts, data }` + `TryFrom<(data, accounts)>` |
| **Useful** | Less boilerplate; router stays clean |

Use **fully qualified** `pinocchio::AccountView` inside the macro.

---

### 8. Entrypoint + router (`lib.rs`)

| | |
|--|--|
| **When** | Always — one front door per program |
| **How** | `instruction_data.split_first()` → match disc → `try_from` → `process` |
| **Useful** | One place to add new instructions |

For `no_std` Solana builds:

```rust
pinocchio::program_entrypoint!(process_instruction);
pinocchio::default_allocator!();
pinocchio::nostd_panic_handler!();
```

---

### 9. CPI: `invoke` vs `invoke_signed`

| Call | When | Example |
|------|------|---------|
| `.invoke()` | A real wallet already signed | Donor → campaign |
| `.invoke_signed(&[Signer])` | A **PDA** must act as signer | Create campaign; campaign → recipient |

**Seeds for signing:** `[CAMPAIGN_SEED, authority_bytes, &[bump]]`

**Analogy:** human key vs robot combination.

---

### 10. Borrow → CPI → borrow again

| | |
|--|--|
| **When** | You need to read state, then CPI, then write state |
| **How** | Short `{ borrow; read; }` block → CPI → `try_borrow_mut` |
| **Useful** | Avoids `AccountBorrowFailed`; CPI needs unborrowed accounts |

---

### 11. Clock sysvar

| | |
|--|--|
| **When** | Deadlines / time locks |
| **How** | `Clock::get()?.unix_timestamp` (don’t trust a client-passed clock) |
| **Useful** | Fair, cluster-agreed time |

---

### 12. Closing accounts

| | |
|--|--|
| **When** | Account is finished; reclaim rent |
| **How** | Move all lamports out → `account.close()` |
| **Useful** | Stops revival (old data reused as a live account) |

---

## Instruction memory hooks

| Ix | Hook | Core idea |
|----|------|-----------|
| initialize | **OPEN** | Obtain bump → Provision PDA → Engrave state → Note bump |
| donate | **TIP** | Type-check → Inspect deadline → Pay + bump raised |
| withdraw | **SIGN** | Secure auth → Inventory amount → Give via `invoke_signed` → Note raised |
| close | **END** | Enforce rules → Nudge lamports → Destroy with `close()` |

---

## Owner checks by instruction (easy to mix up)

| Instruction | Campaign owner should be |
|-------------|--------------------------|
| initialize | **System** (empty / not init yet) |
| donate / withdraw / close | **Your program `ID`** |

---

## Build

```bash
cargo check
cargo build-sbf --features bpf-entrypoint
```

After first SBF build, paste the program pubkey into `lib.rs` → `ID`.

---

## If you get stuck

Open the matching stub file — the `WHEN` / `HOW` / `WHY` comments are the lesson for that file.  
Type until `cargo check` is clean for that layer, then move on.

Want a live coaching turn? Say which file you’re on (e.g. `initialize/accounts.rs`) and paste what you wrote; I’ll nudge the next line without dumping the full solution.
