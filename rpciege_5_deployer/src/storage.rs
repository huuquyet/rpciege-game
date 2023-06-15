use soroban_sdk::{vec, Address, BytesN, Env};

use crate::types::DataKey;

pub fn has_wasm_hash(env: &Env) -> bool {
    env.storage().has(&DataKey::WasmHash)
}

pub fn write_wasm_hash(env: &Env, hash: BytesN<32>) {
    env.storage().set(&DataKey::WasmHash, &hash);
}

pub fn read_wasm_hash(env: &Env) -> BytesN<32> {
    env.storage().get(&DataKey::WasmHash).unwrap().unwrap()
}

pub fn write_deployed_liqpool(env: &Env, id: Address) {
    env.storage().set(&DataKey::Deployed(id), &true);
}