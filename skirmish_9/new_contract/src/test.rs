#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, BytesN, Env};

mod old_contract {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/skirmish_9_old_contract.wasm"
    );
}

mod new_contract {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/skirmish_9_new_contract.wasm"
    );
}

fn install_new_wasm(e: &Env) -> BytesN<32> {
    e.deployer().upload_contract_wasm(new_contract::WASM)
}

#[test]
fn test() {
    let env = Env::default();
    env.mock_all_auths();

    // Note that we use register_contract_wasm instead of register_contract
    // because the old contracts WASM is expected to exist in storage.
    let contract_id = env.register_contract_wasm(None, old_contract::WASM);

    let client = old_contract::Client::new(&env, &contract_id);
    let admin = Address::generate(&env);

    assert_eq!(client.init(&admin), ());
    assert_eq!(client.version(), 1);

    let new_wasm_hash = install_new_wasm(&env);

    client.upgrade(&new_wasm_hash);
    assert_eq!(client.version(), 2);

    // new_v2_fn was added in the new contract, so the existing
    // client is out of date. Generate a new one.
    let client = new_contract::Client::new(&env, &contract_id);
    assert_eq!(client.run(&Some(admin)), ());
}
