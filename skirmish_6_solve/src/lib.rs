#![no_std]
#![feature(iter_array_chunks)]

mod contract;
mod interface;

pub mod skirmish_6 {
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/skirmish_6.wasm "
    );
}

#[cfg(test)]
mod test;
