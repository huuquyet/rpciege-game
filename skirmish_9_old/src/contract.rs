use soroban_sdk::{contract, contractimpl, BytesN, Env};

use crate::{interface::Skirmish9OldTrait, types::Error};

#[contract]
pub struct Skirmish9Old;

#[contractimpl]
impl Skirmish9OldTrait for Skirmish9Old {
    fn version() -> u32 {
        1
    }

    fn upgrade(env: Env, new_wasm_hash: BytesN<32>) -> Result<(), Error>{
        env.deployer().update_current_contract_wasm(new_wasm_hash);
        Ok(())
    }
}
