use soroban_sdk::{contracterror, contracttype, Address};

#[derive(Copy, Clone, Debug)]
#[contracterror]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 0,
    NotInitialized = 1,
}

#[contracttype]
pub enum DataKey {
    WasmHash,
    Deployed(Address),
}