#![cfg(test)]

extern crate std;

use core::ops::AddAssign;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use soroban_sdk::{
    testutils::{Address as _, Ledger, LedgerInfo},
    Address, BytesN, Env,
};
use std::println;

use crate::{
    contract::{get_entropy, Skirmish6, Skirmish6Client},
    types::Error,
};

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

    let contract_id = env.register_contract(None, Skirmish6);
    let client = Skirmish6Client::new(&env, &contract_id);

    let array = [
        8, 200, 60, 97, 90, 91, 113, 94, 198, 240, 251, 30, 183, 149, 10, 61, 135, 234, 87, 118,
        134, 202, 206, 129, 159, 136, 107, 161, 252, 47, 148, 88,
    ];
    let source = Address::from_contract_id(&BytesN::from_array(&env, &array));
    let nft_dest = Address::random(&env);

    assert_eq!(
        client.try_nightfall(&source, &nft_dest),
        Err(Ok(Error::TrapNotSet))
    );

    client.set_trap(&0, &0, &source);
    assert_eq!(
        client.try_nightfall(&source, &nft_dest),
        Err(Ok(Error::YouDied))
    );

    let mut rng = SmallRng::seed_from_u64(get_entropy(&env, &source));
    let monster_xy: (u32, u32) = (rng.gen(), rng.gen());

    client.set_trap(&monster_xy.0, &monster_xy.1, &source);
    println!(
        "{:?} {} {:?}",
        &monster_xy,
        &env.ledger().sequence(),
        &source
    );
    assert_eq!(client.try_nightfall(&source, &nft_dest), Ok(Ok(())));
}
