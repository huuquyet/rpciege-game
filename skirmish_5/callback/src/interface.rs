use soroban_sdk::{Address, Env};

use crate::types::Error;

pub trait SwapCallbackTrait {
    fn swap_callback(
        env: Env,
        liqpool: Address,
        token_id: Address,
        amount: i128,
        initiator: Option<Address>,
    ) -> Result<(), Error>;
}
