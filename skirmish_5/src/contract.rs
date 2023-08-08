use soroban_sdk::{contract, contractimpl, token, Address, Env};

use crate::{interface::SwapCallbackTrait, types::Error};

#[contract]
pub struct SwapCallback;

#[allow(unused_variables)]
#[contractimpl]
impl SwapCallbackTrait for SwapCallback {
    fn swap_callback(
        env: Env,
        liqpool: Address,
        token_id: Address,
        amount: i128,
        initiator: Option<Address>,
    ) -> Result<(), Error> {
        // let token_client = token::Client::new(&env, &token_id);
        // token_client.transfer(&initiator.unwrap(), &liqpool, &amount);

        Ok(())
    }
}
