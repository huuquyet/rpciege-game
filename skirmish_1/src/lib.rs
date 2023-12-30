#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, Address, Env};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    Unknown = 0,
}

#[contract]
pub struct Skirmish1;

#[contractimpl]
impl Skirmish1 {
    pub fn game_1(_env: Env, _nft_dest: Option<Address>) -> Result<(), Error> {
        Ok(())
    }
}

#[cfg(test)]
mod test;
