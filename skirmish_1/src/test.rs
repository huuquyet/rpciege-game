#![cfg(test)]

use super::{Skirmish1, Skirmish1Client};
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Skirmish1);
    let client = Skirmish1Client::new(&env, &contract_id);

    let result = client.game_1(&Address::random(&env));
    assert_eq!(result, ());
}
