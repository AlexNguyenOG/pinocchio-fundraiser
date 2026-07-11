//! Glue: accounts + data → one Instruction type.

macro_rules! define_instruction {
    ($name:ident, $accounts:ty, $data:ty) => {
        pub struct $name<'a> {
            pub accounts: $accounts,
            pub data: $data,
        }

        impl<'a> From<($accounts, $data)> for $name<'a> {
            fn from((accounts, data): ($accounts, $data)) -> Self {
                Self { accounts, data }
            }
        }

        impl<'a> TryFrom<(&'a [u8], &'a mut [pinocchio::AccountView])> for $name<'a> {
            type Error = pinocchio::error::ProgramError;

            fn try_from(
                (data, accounts): (&'a [u8], &'a mut [pinocchio::AccountView]),
            ) -> Result<Self, Self::Error> {
                Ok(Self {
                    accounts: <$accounts>::try_from(accounts)?,
                    data: <$data>::try_from(data)?,
                })
            }
        }
    };
}

pub(crate) use define_instruction;
