use rand::{rngs::SmallRng, Rng, SeedableRng};
use soroban_sdk::{contract, contractimpl, xdr::ToXdr, Address, Env};

use crate::{
    interface::Skirmish6SolveTrait,
    skirmish_6::{Client, Error},
};

#[contract]
pub struct Skirmish6Solve;

#[contractimpl]
impl Skirmish6SolveTrait for Skirmish6Solve {
    fn solve(
        env: Env,
        skirmish_6_address: Address,
        source: Address,
        nft_dest: Option<Address>,
    ) -> Result<(), Error> {
        source.require_auth();
        let mut rng = SmallRng::seed_from_u64(get_entropy(
            &env,
            &source,
            &u64::from(env.ledger().sequence()),
        ));
        let monster_xy: (u32, u32) = (rng.gen(), rng.gen());

        let skirmish_6_client = Client::new(&env, &skirmish_6_address);
        skirmish_6_client.set_trap(&monster_xy.0, &monster_xy.1, &source);
        skirmish_6_client.nightfall(&source, &nft_dest);

        Ok(())
    }

    fn find_monster(env: Env, source: Address, sequence: u64) -> (u32, u32) {
        let mut rng = SmallRng::seed_from_u64(get_entropy(&env, &source, &sequence));
        let monster_xy: (u32, u32) = (rng.gen(), rng.gen());

        monster_xy
    }
}

fn get_entropy(env: &Env, source: &Address, sequence: &u64) -> u64 {
    let mut entropy: u64 = u64::MIN;
    for [a, b, c, d] in source.to_xdr(env).iter().array_chunks() {
        entropy = entropy.wrapping_add(u64::from_be_bytes([a, 0, b, 0, c, 0, d, 0]));
    }
    // modulo 6 tolerance to account for the time delay
    // between simulation and submission
    entropy = entropy.wrapping_add(sequence - sequence % 6);
    entropy
}
