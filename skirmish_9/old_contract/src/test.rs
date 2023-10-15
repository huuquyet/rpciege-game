#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env};

use crate::contract::{Skirmish9Old, Skirmish9OldClient};

#[test]
fn test() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, Skirmish9Old);
    let client = Skirmish9OldClient::new(&env, &contract_id);

    assert_eq!(client.version(), 1);

    let admin = Address::random(&env);
    assert_eq!(client.init(&admin), ());
}
