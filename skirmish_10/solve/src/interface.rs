use soroban_sdk::{Address, Env};

use crate::skirmish_10::Error;

pub trait Skirmish10SolveTrait {
    fn solve(
        env: Env,
        contract: Address,
        addr: Address,
        _nft_dest: Option<Address>,
    ) -> Result<(), Error>;
}
