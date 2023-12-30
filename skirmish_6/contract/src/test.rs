#![cfg(test)]

use rand::{rngs::SmallRng, Rng, SeedableRng};
use soroban_sdk::{testutils::Address as _, Address, Env};

use crate::{
    contract::{get_entropy, Skirmish6, Skirmish6Client},
    types::Error,
};

#[test]
fn test() {
    let env = Env::default();

    env.mock_all_auths();

    let contract_id = env.register_contract(None, Skirmish6);
    let client = Skirmish6Client::new(&env, &contract_id);

    let source = Address::generate(&env);
    let nft_dest = Address::generate(&env);

    assert_eq!(
        client.try_nightfall(&source, &Some(nft_dest.clone())),
        Err(Ok(Error::TrapNotSet))
    );

    client.set_trap(&0, &0, &source);
    assert_eq!(
        client.try_nightfall(&source, &Some(nft_dest.clone())),
        Err(Ok(Error::YouDied))
    );

    let mut rng = SmallRng::seed_from_u64(get_entropy(&env, &source));
    let monster_xy: (u32, u32) = (rng.gen(), rng.gen());

    client.set_trap(&monster_xy.0, &monster_xy.1, &source);

    assert_eq!(client.try_nightfall(&source, &Some(nft_dest)), Ok(Ok(())));
}
