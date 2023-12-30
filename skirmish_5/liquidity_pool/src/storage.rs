use soroban_sdk::{Address, Env};

use crate::{types::DataKey, WEEKS_IN_LEDGERS};

pub fn has_token_a(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::TokenA)
}

pub fn read_token_a(env: &Env) -> Address {
    env.storage().instance().get(&DataKey::TokenA).unwrap()
}

pub fn read_token_b(env: &Env) -> Address {
    env.storage().instance().get(&DataKey::TokenB).unwrap()
}

pub fn write_token_a(env: &Env, a: Address) {
    env.storage().instance().set(&DataKey::TokenA, &a)
}

pub fn write_token_b(env: &Env, b: Address) {
    env.storage().instance().set(&DataKey::TokenB, &b)
}

pub fn read_reserve_a(env: &Env) -> i128 {
    env.storage()
        .instance()
        .get(&DataKey::ReserveA)
        .unwrap_or(0)
}

pub fn read_reserve_b(env: &Env) -> i128 {
    env.storage()
        .instance()
        .get(&DataKey::ReserveB)
        .unwrap_or(0)
}

pub fn write_reserve_a(env: &Env, reserve: i128) {
    env.storage().instance().set(&DataKey::ReserveA, &reserve)
}

pub fn write_reserve_b(env: &Env, reserve: i128) {
    env.storage().instance().set(&DataKey::ReserveB, &reserve)
}

pub fn bump_instance(env: &Env) {
    env.storage()
        .instance()
        .extend_ttl(WEEKS_IN_LEDGERS, WEEKS_IN_LEDGERS);
}
