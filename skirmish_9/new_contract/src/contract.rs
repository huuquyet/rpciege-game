use soroban_sdk::{contract, contractimpl, Address, BytesN, Env};

use crate::{
    interface::Skirmish9NewTrait,
    types::{DataKey, Error},
};

#[contract]
pub struct Skirmish9New;

#[contractimpl]
impl Skirmish9NewTrait for Skirmish9New {
    fn init(e: Env, admin: Address) -> Result<(), Error> {
        e.storage().instance().set(&DataKey::Admin, &admin);
        Ok(())
    }

    fn version() -> Result<u32, Error> {
        Ok(2)
    }

    fn upgrade_contract(env: Env, hash: BytesN<32>) -> Result<(), Error> {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        env.deployer().update_current_contract_wasm(hash);
        Ok(())
    }

    fn run(_env: Env, _nft_dest: Option<Address>) -> Result<(), Error> {
        Ok(())
    }
}
