#![no_std]

mod contract;
mod interface;

pub mod skirmish_10 {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/skirmish_10_contract.wasm"
    );
}

#[cfg(test)]
mod test;
