use soroban_sdk::{contract, contractimpl, Address, BytesN, Env};

use crate::{
    interface::Skirmish9OldTrait,
    types::{DataKey, Error},
};

#[contract]
pub struct Skirmish9Old;

#[contractimpl]
impl Skirmish9OldTrait for Skirmish9Old {
    fn init(env: Env, admin: Address) -> Result<(), Error> {
        env.storage().instance().set(&DataKey::Admin, &admin);
        Ok(())
    }

    fn version() -> Result<u32, Error> {
        Ok(1)
    }

    fn upgrade(env: Env, new_wasm_hash: BytesN<32>) -> Result<(), Error> {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        env.deployer().update_current_contract_wasm(new_wasm_hash);
        Ok(())
    }
}
