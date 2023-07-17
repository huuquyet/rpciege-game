use soroban_sdk::{contracterror, contracttype};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    Unknown = 0,
}

#[contracttype]
pub enum DataKey {
    LiquidityPool,
}
