use soroban_sdk::{testutils::BytesN as _, Bytes, BytesN, Env};
use std::println;

extern crate std;

use crate::contract::{Skirmish7, Skirmish7Client};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Skirmish7);
    let client = Skirmish7Client::new(&env, &contract_id);

    let rand: BytesN<1000> = BytesN::random(&env);
    let bytes = &Bytes::from_array(&env, &rand.to_array());
    let purified_bytes = client.purify(&bytes);

    println!("{:?}", purified_bytes);
    println!("{:?}", env.budget().print());

    assert_eq!(purified_bytes, client.filter(&bytes));
}
