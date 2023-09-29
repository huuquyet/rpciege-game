use rand::{rngs::SmallRng, Rng, SeedableRng};
use soroban_sdk::{contract, contractimpl, Address, Env, IntoVal};

use crate::{
    interface::Skirmish10Trait,
    types::{DataKey, Error},
};

#[contract]
#[allow(dead_code)]
pub struct Skirmish10;

#[contractimpl]
#[allow(dead_code)]
impl Skirmish10Trait for Skirmish10 {
    /// Try winning the fund by providing your guess
    fn shuffle(
        env: Env,
        addr: Address,
        guess: u32,
        _nft_dest: Option<Address>,
    ) -> Result<(), Error> {
        addr.require_auth_for_args((&addr,).into_val(&env));
        let state = u64::MIN
            .wrapping_add(env.ledger().timestamp())
            .wrapping_add(env.ledger().sequence() as u64);
        let mut rng = SmallRng::seed_from_u64(state);
        if guess != rng.gen_range(0..1_000_000_000) {
            return Err(Error::WrongGuess);
        }
        // if the guess is correct, mint a new FundWon token
        env.storage().persistent().set::<DataKey, i128>(
            &DataKey::Balance(addr.clone()),
            &(env
                .storage()
                .persistent()
                .get(&DataKey::Balance(addr))
                .unwrap_or(0)
                + 1),
        );
        Ok(())
    }
}
