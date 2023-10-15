use soroban_sdk::{Address, BytesN, Env};

use crate::types::Error;

pub trait Skirmish9OldTrait {
    fn init(env: Env, admin: Address) -> Result<(), Error>;

    fn version() -> Result<u32, Error>;

    fn upgrade(env: Env, new_wasm_hash: BytesN<32>) -> Result<(), Error>;
}
