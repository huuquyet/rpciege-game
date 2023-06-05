#![cfg(test)]

use super::{Contract, ContractClient};
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    let word = client.game_2(&Address::random(&env));
    assert_eq!(word, String::from_slice(&env, "1694-1727"));
}
