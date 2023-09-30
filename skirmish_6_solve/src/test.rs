#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env};

use crate::contract::{Skirmish6Solve, Skirmish6SolveClient};

#[test]
#[should_panic]
fn test() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, Skirmish6Solve);
    let client = Skirmish6SolveClient::new(&env, &contract_id);

    let contract = Address::random(&env);
    let source = Address::random(&env);

    assert_eq!(client.solve(&contract, &source, &Some(source.clone())), ());
}
