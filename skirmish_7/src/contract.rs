use soroban_sdk::{contract, contractimpl, Bytes, Env};

use crate::{interface::Skirmish7Trait, types::Error};

#[contract]
pub struct Skirmish7;

pub(crate) const PRIMES: [u8; 54] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
    197, 199, 211, 223, 227, 229, 233, 239, 241, 251,
];

#[contractimpl]
impl Skirmish7Trait for Skirmish7 {
    fn purify(env: Env, bytes: Bytes) -> Result<Bytes, Error> {
        let mut purified_bytes = Bytes::new(&env);
        let mut prime_counts = [0u8; PRIMES.len()];
        let mut prime_index = 0;

        for item in bytes.iter() {
            match PRIMES.binary_search(&item) {
                Ok(index) => prime_counts[index] += 1,
                _ => {}
            }
        }

        for prime_count in prime_counts {
            for _ in 0..prime_count {
                purified_bytes.push_back(PRIMES[prime_index]);
            }
            prime_index += 1;
        }

        Ok(purified_bytes)
    }
}
