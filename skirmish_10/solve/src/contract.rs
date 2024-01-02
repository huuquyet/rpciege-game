use rand::{rngs::SmallRng, Rng, SeedableRng};
use soroban_sdk::{contract, contractimpl, Address, Env, IntoVal};

use crate::{
    interface::Skirmish10SolveTrait,
    skirmish_10::{Client, Error},
};

#[contract]
#[allow(dead_code)]
pub struct Skirmish10Solve;

#[contractimpl]
#[allow(dead_code)]
impl Skirmish10SolveTrait for Skirmish10Solve {
    fn solve(
        env: Env,
        contract: Address,
        addr: Address,
        _nft_dest: Option<Address>,
    ) -> Result<(), Error> {
        addr.require_auth_for_args((&addr,).into_val(&env));

        let state = u64::MIN
            .wrapping_add(env.ledger().timestamp())
            .wrapping_add(env.ledger().sequence() as u64);
        let mut rng = SmallRng::seed_from_u64(state);
        let guess = rng.gen_range(0..1_000_000_000);

        let skirmish_10_client = Client::new(&env, &contract);
        skirmish_10_client.shuffle(&addr, &guess, &_nft_dest);

        Ok(())
    }
}
