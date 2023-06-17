use soroban_sdk::{Address, BytesN, Env};

use crate::types::Error;

pub trait DeployerTrait {
    fn initialize(env: Env, liqpool_wasm_hash: BytesN<32>) -> Result<(), Error>;

    fn new_liqpool(
        env: Env,
        salt: BytesN<32>,
        token_a: Address,
        token_b: Address,
    ) -> Result<Address, Error>;
}
