use soroban_sdk::{BytesN, Env};

use crate::types::Error;

pub trait Skirmish9OldTrait {
    fn version() -> u32;

    fn upgrade(
        env: Env,
        new_wasm_hash: BytesN<32>,
    ) -> Result<(), Error>;
}
