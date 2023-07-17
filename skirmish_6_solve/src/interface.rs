use soroban_sdk::{Address, Env};

use crate::skirmish_6::Error;

pub trait Skirmish6SolveTrait {
    fn solve(
        env: Env,
        skirmish_6_address: Address,
        source: Address,
        nft_dest: Address,
    ) -> Result<(), Error>;

    fn find_monster(env: Env, source: Address, sequence: u64) -> (u32, u32);
}
