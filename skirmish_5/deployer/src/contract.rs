use soroban_sdk::{contract, contractimpl, Address, BytesN, Env};

use crate::{
    interface::DeployerTrait,
    liqpool,
    storage::{has_wasm_hash, read_wasm_hash, write_deployed_liqpool, write_wasm_hash},
    types::Error,
};

#[contract]
pub struct Skirmish5Deployer;

#[contractimpl]
impl DeployerTrait for Skirmish5Deployer {
    fn initialize(env: Env, liqpool_wasm_hash: BytesN<32>) -> Result<(), Error> {
        if has_wasm_hash(&env) {
            return Err(Error::AlreadyInitialized);
        }

        write_wasm_hash(&env, liqpool_wasm_hash);
        Ok(())
    }

    fn new_liqpool(
        env: Env,
        salt: BytesN<32>,
        token_a: Address,
        token_b: Address,
    ) -> Result<Address, Error> {
        if !has_wasm_hash(&env) {
            return Err(Error::NotInitialized);
        }

        let liqpool_id = env
            .deployer()
            .with_current_contract(salt)
            .deploy(read_wasm_hash(&env));

        let liqpool_client = liqpool::Client::new(&env, &liqpool_id);
        liqpool_client.initialize(&token_a, &token_b);

        write_deployed_liqpool(&env, liqpool_id.clone());

        Ok(liqpool_id)
    }
}
