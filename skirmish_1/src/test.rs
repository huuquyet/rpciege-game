#![cfg(test)]

use super::{Contract, ContractClient};
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    let result = client.game_1(&Address::random(&env));
    assert_eq!(result, ());
}
