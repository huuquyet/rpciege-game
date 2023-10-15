use soroban_sdk::{Address, Env};

use crate::types::Error;

pub trait Skirmish10Trait {
    fn shuffle(
        env: Env,
        addr: Address,
        guess: u32,
        _nft_dest: Option<Address>,
    ) -> Result<(), Error>;
}
