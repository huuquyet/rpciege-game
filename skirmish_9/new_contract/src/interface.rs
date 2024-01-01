use soroban_sdk::{Address, BytesN, Env};

use crate::types::Error;

pub trait Skirmish9NewTrait {
    fn init(env: Env, admin: Address) -> Result<(), Error>;

    fn version() -> Result<u32, Error>;

    fn upgrade_contract(env: Env, hash: BytesN<32>) -> Result<(), Error>;

    fn run(env: Env, _nft_dest: Option<Address>) -> Result<(), Error>;
}
