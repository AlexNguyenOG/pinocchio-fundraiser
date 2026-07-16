# Macro answers (short)

Step 1 full answers (constants + macros + error): **[answers/step-01-foundation.md](./answers/step-01-foundation.md)**

Index of all steps: **[ANSWERS.md](./ANSWERS.md)**

## `program/src/utils/macros.rs`

```rust
macro_rules! require_len {
    ($data:expr, $len:expr) => {
        if $data.len() < $len {
            return Err(pinocchio::error::ProgramError::InvalidInstructionData);
        }
    };
}

macro_rules! require_account_len {
    ($data:expr, $len:expr) => {
        if $data.len() < $len {
            return Err(pinocchio::error::ProgramError::InvalidAccountData);
        }
    };
}

macro_rules! validate_discriminator {
    ($data:expr, $disc:expr) => {
        if $data.is_empty() || $data[0] != $disc {
            return Err(pinocchio::error::ProgramError::InvalidAccountData);
        }
    };
}

macro_rules! assert_no_padding {
    ($t:ty, $expected:expr) => {
        const _: () = assert!(
            core::mem::size_of::<$t>() == $expected,
            "struct size mismatch — check field order / padding"
        );
    };
}
```
