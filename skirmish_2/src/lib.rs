#![no_std]

use soroban_sdk::{contractimpl, Address, Env, String};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn game_2(_env: Env, _nft_dest: Address) -> String {
        String::from_slice(&_env, "1694-1727")
    }
}

#[cfg(test)]
mod test;
