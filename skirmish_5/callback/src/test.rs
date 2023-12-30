#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env};

use crate::contract::{SwapCallback, SwapCallbackClient};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, SwapCallback);
    let client = SwapCallbackClient::new(&env, &contract_id);

    let source = Address::generate(&env);

    assert_eq!(
        client.swap_callback(&source, &source, &i128::from(100), &Some(source.clone())),
        ()
    );
}
