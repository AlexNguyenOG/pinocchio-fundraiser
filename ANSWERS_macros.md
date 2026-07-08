# Macros.rs — Answer Sheet

Write your answers under each question. This file is markdown only — it does **not** get compiled into the program.

Tip: keep answers short (1–3 sentences). Depth beats length.

---

## Q1. Error types

What’s the difference between `InvalidInstructionData` and `InvalidAccountData`?  
When would you use each?

**Your answer:**



---

## Q2. Length before parse

Why check length **before** doing something like `data[0..8].try_into()` / `from_le_bytes`?

**Your answer:**



---

## Q3. Empty discriminator

Why does `validate_discriminator!` also need to reject **empty** data (not only “wrong `data[0]`”)?

**Your answer:**



---

## Q4. Disc size

Pinocchio account discriminators are **1 byte**. Anchor’s are **8 bytes**.  
Why does that matter when writing this macro / reading account data?

**Your answer:**



---

## Q5. Compile-time assert

`assert_no_padding!` fails at **compile time**.  
How is that more useful than finding a size mismatch only at runtime on-chain?

**Your answer:**



---

## Q6. Macro vs function

These are `macro_rules!`, not normal functions.  
Why is a macro a better fit for “`return Err(...)` out of the **caller**”?

**Your answer:**



---

## Optional: what you’ll write

After answering, implement in `program/src/utils/macros.rs`:

1. `require_len!($data, $len)` → `InvalidInstructionData`
2. `require_account_len!($data, $len)` → `InvalidAccountData`
3. `validate_discriminator!($data, $disc)` → empty or wrong byte 0 → `InvalidAccountData`
4. `assert_no_padding!($t, $expected)` → compile-time `size_of` check

When you’re done answering (and typing macros), say so here or in chat and we’ll review.
