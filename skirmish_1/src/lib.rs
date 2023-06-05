#![no_std]

use soroban_sdk::{contractimpl, Address, Env};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn game_1(_env: Env, _nft_dest: Address) {}
}

#[cfg(test)]
mod test;
