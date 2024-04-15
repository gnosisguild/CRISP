mod util;

use wasm_bindgen::prelude::*;

use std::{thread, time, env, sync::Arc};
use serde::Deserialize;

use fhe::{
    bfv::{BfvParametersBuilder, Ciphertext, Encoding, Plaintext, PublicKey, SecretKey},
    mbfv::{AggregateIter, CommonRandomPoly, DecryptionShare, PublicKeyShare},
};
use fhe_traits::{FheDecoder, FheEncoder, FheEncrypter, Serialize, DeserializeParametrized};
use rand::{distributions::Uniform, prelude::Distribution, rngs::OsRng, thread_rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use util::timeit::{timeit, timeit_n};
use rustc_serialize::json;

#[wasm_bindgen]
pub struct Encrypt {
    encrypted_vote: Vec<u8>,
}

#[wasm_bindgen]
impl Encrypt {
    pub fn new(&mut self) {
        self.encrypted_vote = Vec::new();
    }

    pub fn encrypt_vote(&mut self, vote: u64, public_key: Vec<u8>) -> Vec<u8> {
        let degree = 4096;
        let plaintext_modulus: u64 = 4096;
        let moduli = vec![0xffffee001, 0xffffc4001, 0x1ffffe0001];
        // Let's generate the BFV parameters structure.
        let params = timeit!(
            "Parameters generation",
            BfvParametersBuilder::new()
                .set_degree(degree)
                .set_plaintext_modulus(plaintext_modulus)
                .set_moduli(&moduli)
                .build_arc().unwrap()
        );
        let pk_deserialized = PublicKey::from_bytes(&public_key, &params).unwrap();
        let votes: Vec<u64> = [vote].to_vec();
        let pt = Plaintext::try_encode(&[votes[0]], Encoding::poly(), &params).unwrap();
        let ct = pk_deserialized.try_encrypt(&pt, &mut thread_rng()).unwrap();
        //let sol_vote = Bytes::from(ct.to_bytes());
        self.encrypted_vote = ct.to_bytes();
        self.encrypted_vote.clone()
    }

    pub fn test() {
        println!("Test Function Working");
    }
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Ok(())
}