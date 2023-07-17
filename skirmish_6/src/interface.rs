use soroban_sdk::{Address, Env, Vec};

use crate::types::Error;

pub trait Skirmish6Trait {
    fn set_trap(env: Env, x: u32, y: u32, source: Address) -> Result<(), Error>;

    fn nightfall(env: Env, source: Address, _nft_dest: Address) -> Result<(), Error>;
}
