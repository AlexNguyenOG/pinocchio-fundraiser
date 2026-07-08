use pinocchio::{
    account::AccountView,
    address::Address,
    error::ProgramError,
    ProgramResult,
};

pub trait Discriminator {
    const DISCRIMINATOR: u8;

}

pub trait Versioned {
    const VERSION: u8;
}

pub trait AccountSize {
    const DATA_LEN: usize;
    const LEN: usize =  1 + 1 + Self::DATA_LEN;
}

pub trait AccountDeserialize: Sized + Discriminator + AccountSize + Versioned {

}

pub trait PdaSeeds {
    const PREFIX: &'static [u8];
    
    fn seeds(&self) -> [&[u8]; 2];

    fn derive_address(&self, program_id: &Address) -> (Address, u8) {
        let [a, b] = self.seeds();
        Address::derive_program_address(&[a, b], program_id)
        .expect("Valid PDA bump seeds")
    }
    fn derive_address_with_bump(&self, program_id: &Address, bump: u8) -> Address {
        let [a, b] = self.seeds();
        Address::derive_address(&[a, b], Some(bump), program_id)
    }
    fn validate_pda(&self, account: &AccountView, program_id:  &Address, bump: u8) ->ProgramResult<()>
}

pub trait PdaAccount: PdaSeeds {
    fn bump(&self) -> u8;

    fn validate_self(&self, account: &AccountView, program_id: &Address) -> ProgramResult {
        self.validate_pda(account, program_id, self.bump())/
    }
    }
}