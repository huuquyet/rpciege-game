#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, BytesN, Env};

use crate::contract::{Skirmish6Solve, Skirmish6SolveClient};

#[test]
#[should_panic]
fn test() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, Skirmish6Solve);
    let client = Skirmish6SolveClient::new(&env, &contract_id);

    let array = [
        8, 200, 60, 97, 90, 91, 113, 94, 198, 240, 251, 30, 183, 149, 10, 61, 135, 234, 87, 118,
        134, 202, 206, 129, 159, 136, 107, 161, 252, 47, 148, 88,
    ];
    let contract = Address::from_contract_id(&BytesN::from_array(&env, &array));
    let source = Address::random(&env);
    let nft_dest = Address::random(&env);

    assert_eq!(client.solve(&contract, &source, &nft_dest), ());
}
