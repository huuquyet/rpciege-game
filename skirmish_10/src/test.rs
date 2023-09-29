#![cfg(test)]

extern crate std;

use rand::{rngs::SmallRng, Rng, SeedableRng};
use soroban_sdk::{
    testutils::{Address as _, Ledger, LedgerInfo},
    Address, Env,
};

use crate::contract::{Skirmish10, Skirmish10Client};

#[test]
fn test() {
    let env = Env::default();

    env.mock_all_auths();
    env.ledger().set(LedgerInfo {
        timestamp: Default::default(),
        protocol_version: Default::default(),
        sequence_number: 100,
        network_id: Default::default(),
        base_reserve: Default::default(),
        max_entry_expiration: Default::default(),
        min_persistent_entry_expiration: Default::default(),
        min_temp_entry_expiration: Default::default(),
    });

    let contract_id = env.register_contract(None, Skirmish10);
    let client = Skirmish10Client::new(&env, &contract_id);

    let addr = Address::random(&env);

    // assert_eq!(client.shuffle(&addr, &u32::MAX, &Some(addr.clone())), Err(Error::WrongGuess));

    let state = u64::MIN
        .wrapping_add(env.ledger().timestamp())
        .wrapping_add(env.ledger().sequence() as u64);
    let mut rng = SmallRng::seed_from_u64(state);
    let guess = rng.gen_range(0..1_000_000_000);

    assert_eq!(client.shuffle(&addr, &guess, &Some(addr.clone())), ());
}
