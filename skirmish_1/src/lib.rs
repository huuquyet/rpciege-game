#![no_std]

use soroban_sdk::{contractimpl, Address, Env};

pub struct Skirmish1;

#[contractimpl]
impl Skirmish1 {
    pub fn game_1(_env: Env, _nft_dest: Address) {}
}

#[cfg(test)]
mod test;
