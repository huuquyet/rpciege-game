use soroban_sdk::{contracterror, contracttype, Address};

#[contracterror]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u32)]
pub enum Error {
    TrapNotSet = 1,
    YouDied = 2,
}

#[contracttype]
pub enum DataKey {
    TrapXY(Address),
}
