use soroban_sdk::{Address, BytesN, Env};

use crate::types::DataKey;

pub fn has_wasm_hash(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::WasmHash)
}

pub fn write_wasm_hash(env: &Env, hash: BytesN<32>) {
    env.storage().instance().set(&DataKey::WasmHash, &hash);
}

pub fn read_wasm_hash(env: &Env) -> BytesN<32> {
    env.storage()
        .instance()
        .get::<DataKey, BytesN<32>>(&DataKey::WasmHash)
        .unwrap()
}

pub fn write_deployed_liqpool(env: &Env, id: Address) {
    env.storage().instance().set(&DataKey::Deployed(id), &true);
}
