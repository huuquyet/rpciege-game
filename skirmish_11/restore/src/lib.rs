#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, BytesN, Env, Symbol};

const LEDGER_COUNT_WEEK: u32 = 12 * 60 * 24 * 10;
const SPACESHIP: Symbol = symbol_short!("SPACESHIP");
const STARSHIP: Symbol = symbol_short!("STARSHIP");
const SHIP: Symbol = symbol_short!("SHIP");

#[contract]
pub struct RestoreContract;

#[contractimpl]
impl RestoreContract {
    pub fn set(env: Env, user: Address, value: Option<BytesN<32>>) -> BytesN<32> {
        user.require_auth();

        let account: [u8; 32] = [
            244, 51, 85, 215, 230, 83, 144, 31, 217, 169, 189, 185, 9, 175, 202, 37, 236, 65, 58,
            245, 14, 149, 223, 234, 125, 86, 34, 46, 81, 155, 119, 78,
        ];

        let pilot: BytesN<32> = value.unwrap_or(BytesN::from_array(&env, &account));

        env.storage().persistent().set(&SPACESHIP, &pilot);
        env.storage().temporary().set(&STARSHIP, &pilot);
        env.storage().instance().set(&SHIP, &pilot);

        env.storage()
            .temporary()
            .get::<Symbol, BytesN<32>>(&STARSHIP)
            .unwrap()
    }

    pub fn restore(env: Env, time: Option<u32>, _nft_dest: Option<Address>) -> BytesN<32> {
        let _expiration: u32 = time.unwrap_or(200);

        env.storage()
            .persistent()
            .extend_ttl(&SPACESHIP, LEDGER_COUNT_WEEK, LEDGER_COUNT_WEEK);
        env.storage()
            .temporary()
            .extend_ttl(&STARSHIP, LEDGER_COUNT_WEEK, LEDGER_COUNT_WEEK);
        env.storage()
            .instance()
            .extend_ttl(LEDGER_COUNT_WEEK, LEDGER_COUNT_WEEK);

        env.storage()
            .persistent()
            .get::<Symbol, BytesN<32>>(&SPACESHIP)
            .unwrap();
        env.storage()
            .temporary()
            .get::<Symbol, BytesN<32>>(&STARSHIP)
            .unwrap();

        env.storage()
            .instance()
            .get::<Symbol, BytesN<32>>(&SHIP)
            .unwrap()
    }
}
