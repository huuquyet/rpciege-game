use rand::{rngs::SmallRng, Rng, SeedableRng};
use soroban_sdk::{contract, contractimpl, panic_with_error, xdr::ToXdr, Address, Env, Vec};

use crate::{
    interface::Skirmish6Trait,
    types::{DataKey, Error},
};

#[contract]
pub struct Skirmish6;

#[contractimpl]
impl Skirmish6Trait for Skirmish6 {
    fn set_trap(env: Env, x: u32, y: u32, source: Address) -> Result<(), Error> {
        source.require_auth();
        let trap_xy = (x, y);
        env.storage()
            .instance()
            .set(&DataKey::TrapXY(source), &trap_xy);
        Ok(())
    }

    fn nightfall(env: Env, source: Address, _nft_dest: Address) -> Result<(), Error> {
        source.require_auth();
        let mut rng = SmallRng::seed_from_u64(get_entropy(&env, &source));
        let monster_xy = (rng.gen(), rng.gen());
        let trap_xy = env
            .storage()
            .instance()
            .get::<DataKey, (u32, u32)>(&DataKey::TrapXY(source))
            .unwrap_or_else(|| panic_with_error!(&env, Error::TrapNotSet));
        if monster_xy != trap_xy {
            panic_with_error!(&env, Error::YouDied);
        }
        Ok(())
    }
}

pub(crate) fn get_entropy(env: &Env, source: &Address) -> u64 {
    let sequence = u64::from(env.ledger().sequence());
    let mut entropy: u64 = u64::MIN;
    for [a, b, c, d] in source.to_xdr(env).iter().array_chunks() {
        entropy = entropy.wrapping_add(u64::from_be_bytes([a, 0, b, 0, c, 0, d, 0]));
    }
    // modulo 6 tolerance to account for the time delay
    // between simulation and submission
    entropy = entropy.wrapping_add(sequence - sequence % 6);
    entropy
}
