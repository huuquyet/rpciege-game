use soroban_sdk::{contracterror, contracttype};

#[contracterror]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u32)]
pub enum Error {
    Unknown = 0,
}

#[contracttype]
#[derive(Clone)]
enum DataKey {
    Admin,
}
