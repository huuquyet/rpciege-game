#![cfg(test)]

extern crate std;

use rand::{rngs::SmallRng, Rng, SeedableRng};
use soroban_sdk::{testutils::Address as _, Address, Env};

use crate::contract::{Skirmish10, Skirmish10Client};

#[test]
#[should_panic]
fn test_panic() {
    let env = Env::default();

    env.mock_all_auths();

    let contract_id = env.register_contract(None, Skirmish10);
    let client = Skirmish10Client::new(&env, &contract_id);

    let addr = Address::random(&env);
    client.shuffle(&addr, &u32::MAX, &Some(addr.clone()));
}

#[test]
fn test() {
    let env = Env::default();

    env.mock_all_auths();

    let contract_id = env.register_contract(None, Skirmish10);
    let client = Skirmish10Client::new(&env, &contract_id);

    let addr = Address::random(&env);
    let state = u64::MIN
        .wrapping_add(env.ledger().timestamp())
        .wrapping_add(env.ledger().sequence() as u64);
    let mut rng = SmallRng::seed_from_u64(state);
    let guess = rng.gen_range(0..1_000_000_000);

    assert_eq!(client.shuffle(&addr, &guess, &Some(addr.clone())), ());
}
