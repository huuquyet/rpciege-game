use soroban_sdk::{Bytes, Env};

use crate::types::Error;

pub trait Skirmish7Trait {
    fn purify(env: Env, bytes: Bytes) -> Result<Bytes, Error>;
    fn filter(env: Env, bytes: Bytes) -> Result<Bytes, Error>;
}
