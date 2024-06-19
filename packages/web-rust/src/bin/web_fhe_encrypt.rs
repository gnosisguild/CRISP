mod util;

use wasm_bindgen::prelude::*;

use serde::Deserialize;
use std::{env, sync::Arc, thread, time, fs, path::Path};

use fhe::{
    bfv::{BfvParametersBuilder, Ciphertext, Encoding, Plaintext, PublicKey, SecretKey},
    mbfv::{AggregateIter, CommonRandomPoly, DecryptionShare, PublicKeyShare},
};
use fhe_traits::{DeserializeParametrized, FheDecoder, FheEncoder, FheEncrypter, FheDecrypter, Serialize};
use rand::{distributions::Uniform, prelude::Distribution, rngs::OsRng, thread_rng, SeedableRng};
use util::timeit::{timeit, timeit_n};

#[wasm_bindgen]
pub struct Encrypt {
    encrypted_vote: Vec<u8>,
}

#[wasm_bindgen]
impl Encrypt {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Encrypt {
        Encrypt {
            encrypted_vote: Vec::new(),
        }
    }

    pub fn encrypt_vote(&mut self, vote: u64, public_key: Vec<u8>) -> Result<Vec<u8>, JsValue> {
        let degree = 4096;
        let plaintext_modulus: u64 = 4096;
        let moduli = vec![0xffffee001, 0xffffc4001, 0x1ffffe0001];

        let params = BfvParametersBuilder::new()
            .set_degree(degree)
            .set_plaintext_modulus(plaintext_modulus)
            .set_moduli(&moduli)
            .build_arc()
            .map_err(|e| JsValue::from_str(&format!("Error generating parameters: {}", e)))?;

        let pk_deserialized = PublicKey::from_bytes(&public_key, &params)
            .map_err(|e| JsValue::from_str(&format!("Error deserializing public key: {}", e)))?;

        let votes = vec![vote];
        let pt = Plaintext::try_encode(&votes, Encoding::poly(), &params)
            .map_err(|e| JsValue::from_str(&format!("Error encoding plaintext: {}", e)))?;

        let ct = pk_deserialized
            .try_encrypt(&pt, &mut thread_rng())
            .map_err(|e| JsValue::from_str(&format!("Error encrypting vote: {}", e)))?;

        self.encrypted_vote = ct.to_bytes();
        Ok(self.encrypted_vote.clone())
    }

    pub fn encrypt_message(&mut self, message: Vec<u64>, public_key: Vec<u8>) -> Result<Vec<u8>, JsValue> {
        let degree = 4096;
        let plaintext_modulus: u64 = 4096;
        let moduli = vec![0xffffee001, 0xffffc4001, 0x1ffffe0001];

        let params = BfvParametersBuilder::new()
            .set_degree(degree)
            .set_plaintext_modulus(plaintext_modulus)
            .set_moduli(&moduli)
            .build_arc()
            .map_err(|e| JsValue::from_str(&format!("Error generating parameters: {}", e)))?;

        let pk_deserialized = PublicKey::from_bytes(&public_key, &params)
            .map_err(|e| JsValue::from_str(&format!("Error deserializing public key: {}", e)))?;

        let pt = Plaintext::try_encode(&message, Encoding::poly(), &params)
            .map_err(|e| JsValue::from_str(&format!("Error encoding plaintext: {}", e)))?;

        let ct = pk_deserialized
            .try_encrypt(&pt, &mut thread_rng())
            .map_err(|e| JsValue::from_str(&format!("Error encrypting vote: {}", e)))?;

        self.encrypted_vote = ct.to_bytes();
        Ok(self.encrypted_vote.clone())
    }

    pub fn test() {
        web_sys::console::log_1(&"Test Function Working".into());
    }
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // let mut encrypt = Encrypt::new();

    // let degree = 4096;
    // let plaintext_modulus: u64 = 4096;
    // let moduli = vec![0xffffee001, 0xffffc4001, 0x1ffffe0001];

    // // Generate the BFV parameters structure.
    // let params = timeit!(
    //     "Parameters generation",
    //     BfvParametersBuilder::new()
    //         .set_degree(degree)
    //         .set_plaintext_modulus(plaintext_modulus)
    //         .set_moduli(&moduli)
    //         .build_arc()?
    // );

    // let mut rng = thread_rng();
    // let sk = SecretKey::random(&params, &mut rng);
    // let pk = PublicKey::new(&sk, &mut rng);
    // println!("{:?}", pk);
    // let pk_bytes = pk.to_bytes();
    // println!("{:?}", pk_bytes);

    // let path = env::current_dir().unwrap();
    // let mut pathst = path.display().to_string();
    // pathst.push_str("/public_key");
    // fs::write(pathst.clone(), pk_bytes).unwrap();
    // let pk_str = serde_json::to_string(&pk).unwrap();

    // // this is an example of a message converted to an array of u64
    // let message: Vec<u64> = vec![0, 12, 128];
    // let res = encrypt.encrypt_message(message, pk.to_bytes());

    // let deserialized_message = Ciphertext::from_bytes(&res.unwrap(), &params).unwrap();
    // let decrypted = sk.try_decrypt(&deserialized_message)?;
    // let decoded = Vec::<u64>::try_decode(&decrypted, Encoding::poly()).unwrap();

    // // print decoded message
    // for i in 0..3 {
    //     println!("{:?}", decoded[i]);
    // }

    Ok(())
}
