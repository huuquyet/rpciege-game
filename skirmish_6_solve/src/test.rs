#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env};

use crate::{contract::{Skirmish6Solve, Skirmish6SolveClient}, skirmish_6};

#[test]
#[should_panic]
fn test() {
    let env = Env::default();
    env.mock_all_auths();
    
    // Register contract skirmish 6 using the imported Wasm.
    let skirmish6 = env.register_contract_wasm(None, skirmish_6::WASM);

    // Register contract to solve skirmish 6 defined in this crate.
    let contract_id = env.register_contract(None, Skirmish6Solve);
    // Create a client for calling contract solve.
    let client = Skirmish6SolveClient::new(&env, &contract_id);

    // Create a random address for source and nft_dest args
    let source = Address::random(&env);

    // Invoke contract solve via its client, then it will invoke contract skirmish 6.
    assert_eq!(client.solve(&skirmish6, &source, &Some(source.clone())), ());
}
