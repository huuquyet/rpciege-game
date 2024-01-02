#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env};

use crate::{
    contract::{Skirmish10Solve, Skirmish10SolveClient},
    skirmish_10,
};

#[test]
#[should_panic]
fn test() {
    let env = Env::default();
    env.mock_all_auths();

    let skirmish10 = env.register_contract_wasm(None, skirmish_10::WASM);
    let contract_id = env.register_contract(None, Skirmish10Solve);
    let client = Skirmish10SolveClient::new(&env, &contract_id);

    let source = Address::generate(&env);

    assert_eq!(
        client.solve(&skirmish10, &source, &Some(source.clone())),
        ()
    );
}
