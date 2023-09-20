#![no_std]

mod contract;
mod interface;
mod storage;
mod types;

pub mod liqpool {
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/skirmish_5_liquidity_pool.wasm"
    );
}
