use core::ops::AddAssign;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use soroban_sdk::{
    contract, contractimpl, panic_with_error, xdr::ToXdr, Address, Env, IntoVal, Vec,
};

use crate::{
    interface::Skirmish6Trait,
    types::{DataKey, Error},
};

#[contract]
pub struct Skirmish6;

const WEEKS_IN_LEDGERS: u32 = 151200 * 4;

#[contractimpl]
impl Skirmish6Trait for Skirmish6 {
    fn set_trap(env: Env, x: u32, y: u32, source: Address) -> Result<(), Error> {
        source.require_auth_for_args((&source,).into_val(&env));
        let trap_xy = (x, y);

        env.storage()
            .temporary()
            .set(&DataKey::TrapXY(source.clone()), &trap_xy);
        env.storage().temporary().bump(
            &DataKey::TrapXY(source),
            WEEKS_IN_LEDGERS,
            WEEKS_IN_LEDGERS,
        );
        Ok(())
    }

    fn nightfall(env: Env, source: Address, _nft_dest: Option<Address>) -> Result<(), Error> {
        source.require_auth_for_args((&source,).into_val(&env));
        let mut rng = SmallRng::seed_from_u64(get_entropy(&env, &source));
        let monster_xy = (rng.gen(), rng.gen());

        let trap_xy = env
            .storage()
            .temporary()
            .get::<DataKey, (u32, u32)>(&DataKey::TrapXY(source))
            .unwrap_or_else(|| panic_with_error!(&env, Error::TrapNotSet));
        if monster_xy != trap_xy {
            panic_with_error!(&env, Error::YouDied);
        }
        Ok(())
    }

    fn print(env: Env, source: Address) -> Result<Vec<u32>, Error> {
        source.require_auth_for_args((&source,).into_val(&env));
        let mut sequence = u64::from(env.ledger().sequence());
        let mut out = Vec::new(&env);
        for _i in 0..5 {
            let mut rng =
                SmallRng::seed_from_u64(get_entropy_by_sequence(&env, &source, &sequence));
            let monster_xy: (u32, u32) = (rng.gen(), rng.gen());
            out.push_back(monster_xy.0);
            out.push_back(monster_xy.1);
            sequence.add_assign(6u64);
        }
        Ok(out)
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

fn get_entropy_by_sequence(env: &Env, source: &Address, sequence: &u64) -> u64 {
    let mut entropy: u64 = u64::MIN;
    for [a, b, c, d] in source.to_xdr(env).iter().array_chunks() {
        entropy = entropy.wrapping_add(u64::from_be_bytes([a, 0, b, 0, c, 0, d, 0]));
    }
    // modulo 6 tolerance to account for the time delay
    // between simulation and submission
    entropy = entropy.wrapping_add(sequence - sequence % 6);
    entropy
}
