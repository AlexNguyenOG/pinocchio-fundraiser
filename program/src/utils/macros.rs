
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
        const _: ()  = assert!(
            core::mem::size_of::<$t>() == $expected,
            "Struct size mistmatch - check filed order / padding"
        )
    
    };
}