#![cfg(test)]

extern crate alloc;
extern crate std;

use alloc::string::ToString;
use itertools::Itertools;
use soroban_sdk::{testutils::Address as _, xdr::ToXdr, Address, Env, Symbol};
use std::{format, fs::OpenOptions, io::Write, string::String};

use crate::{Skirmish3, Skirmish3Client};

#[test]
#[should_panic]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Skirmish3);
    let client = Skirmish3Client::new(&env, &contract_id);

    // generate_permutations(&env, 3);
    client.game_3(&Symbol::new(&env, "pew"), &Some(Address::random(&env)));
}

fn generate_permutations(env: &Env, length: usize) {
    let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    std::fs::remove_file("output.txt").expect("delete file failed");
    let mut file_ref = OpenOptions::new()
        .append(true)
        .create(true)
        .open("output.txt")
        .unwrap();
    let prefix = String::from("");
    let suffix = String::from("");

    for len in 1..=length {
        for permutation in chars.chars().map(|c| c.to_string()).permutations(len) {
            let word = format!("{}{}{}", prefix, permutation.join(""), suffix);
            let bytes = Symbol::new(&env, &word).to_xdr(&env);
            let hash = env.crypto().sha256(&bytes);
            // let readable = String::from_utf8_lossy(&hash);

            let output = format!("{:?}: {:?}\n", word, hash);
            // println!("{:?}: {:?}", word, hash);
            file_ref
                .write_all(output.as_bytes())
                .expect("write to file failed");
            env.budget().reset_default();
        }
    }
}
