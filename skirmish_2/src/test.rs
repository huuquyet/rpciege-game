#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env, String};

use crate::{Skirmish2, Skirmish2Client};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Skirmish2);
    let client = Skirmish2Client::new(&env, &contract_id);

    let word = client.game_2(&Address::random(&env));
    assert_eq!(word, String::from_slice(&env, "1694-1727"));
}
