#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env, String};

#[contract]
pub struct Skirmish2;

#[contractimpl]
impl Skirmish2 {
    pub fn game_2(_env: Env, _nft_dest: Address) -> String {
        String::from_slice(&_env, "1694-1727")
    }
}

#[cfg(test)]
mod test;
