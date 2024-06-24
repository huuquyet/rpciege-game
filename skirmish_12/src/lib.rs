#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, BytesN, Env};

#[contracttype]
pub enum Station {
    Persistent,
    Temporary,
    Instance,
}

#[contract]
pub struct RestoreContract;

#[contractimpl]
impl RestoreContract {
    pub fn init(
        env: Env,
        user: Address,
        value: Option<BytesN<32>>,
        _nft_dest: Option<Address>,
    ) -> BytesN<32> {
        user.require_auth();

        let account: [u8; 32] = [
            244, 51, 85, 215, 230, 83, 144, 31, 217, 169, 189, 185, 9, 175, 202, 37, 236, 65, 58,
            245, 14, 149, 223, 234, 125, 86, 34, 46, 81, 155, 119, 78,
        ];

        let pilot: BytesN<32> = value.unwrap_or(BytesN::from_array(&env, &account));

        env.storage().persistent().set(&Station::Persistent, &pilot);
        env.storage().temporary().set(&Station::Temporary, &pilot);
        env.storage().instance().set(&Station::Instance, &pilot);

        env.storage()
            .temporary()
            .get::<Station, BytesN<32>>(&Station::Temporary)
            .unwrap()
    }

    pub fn run(env: Env, time: Option<u32>, _nft_dest: Option<Address>) -> BytesN<32> {
        let _expiration: u32 = time.unwrap_or(200);

        env.storage()
            .persistent()
            .extend_ttl(&Station::Persistent, 100, 100);
        env.storage()
            .temporary()
            .extend_ttl(&Station::Temporary, 100, 100);
        env.storage().instance().extend_ttl(100, 100);

        env.storage()
            .persistent()
            .get::<Station, BytesN<32>>(&Station::Persistent)
            .unwrap();
        env.storage()
            .temporary()
            .get::<Station, BytesN<32>>(&Station::Temporary)
            .unwrap();

        env.storage()
            .instance()
            .get::<Station, BytesN<32>>(&Station::Instance)
            .unwrap()
    }
}
