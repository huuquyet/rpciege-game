#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, panic_with_error, xdr::ToXdr, Address, Env, Symbol,
};

#[contract]
pub struct Skirmish3;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    MissingPew = 1,
    UsedPew = 2,
}

#[contractimpl]
impl Skirmish3 {
    pub fn game_3(env: Env, symbol: Symbol, _nft_dest: Address) -> Result<(), Error> {
        if env.storage().instance().has(&symbol) {
            panic_with_error!(env, Error::UsedPew);
        }

        let bytes = symbol.clone().to_xdr(&env);
        let hash = env.crypto().sha256(&bytes);
        let mut i = 0;
        let mut has_pew = false;

        for v in hash.clone().into_iter() {
            if v == 112
                && hash.get(i + 1).unwrap_or(0) == 101
                && hash.get(i + 2).unwrap_or(0) == 119
            {
                has_pew = true;
            }
            i += 1;
        }

        if !has_pew {
            panic_with_error!(env, Error::MissingPew);
        } else {
            env.storage().instance().set(&symbol, &true);
        }
        Ok(())
    }
}

#[cfg(test)]
mod test;
