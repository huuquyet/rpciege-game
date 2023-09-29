use soroban_sdk::{contracterror, contracttype, Address};

#[contracterror]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u32)]
pub enum Error {
    WrongGuess = 0,
}

#[contracttype]
pub enum DataKey {
    Balance(Address),
}
