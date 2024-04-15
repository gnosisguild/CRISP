mod util;

use dialoguer::{theme::ColorfulTheme, Input, FuzzySelect};
use wasm_bindgen::prelude::*;

use std::{thread, time, env, sync::Arc};
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

use fhe::{
    bfv::{BfvParametersBuilder, Ciphertext, Encoding, Plaintext, PublicKey, SecretKey},
    mbfv::{AggregateIter, CommonRandomPoly, DecryptionShare, PublicKeyShare},
};
use fhe_traits::{FheDecoder, FheEncoder, FheEncrypter, Serialize, DeserializeParametrized};
//use fhe_math::rq::{Poly};
use rand::{distributions::Uniform, prelude::Distribution, rngs::OsRng, thread_rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use util::timeit::{timeit, timeit_n};

use http_body_util::Empty;
use hyper::Request;
//use hyper::body::Bytes;
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;
use http_body_util::BodyExt;
use tokio::io::{AsyncWriteExt as _, self};
use rustc_serialize::json;

use ethers::{
    prelude::{abigen, Abigen},
    providers::{Http, Provider},
    middleware::SignerMiddleware,
    signers::{LocalWallet, Signer, Wallet},
    types::{Address, U256, Bytes},
    core::k256,
    utils,
};

#[wasm_bindgen]
pub struct Encrypt {
    width: u32,
    height: u32,
}

impl Encrypt {
    fn encrypt_vote(vote: u64, public_key: Vec<u8>) -> Bytes {
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
        Bytes::from(ct.to_bytes())
    }

    fn test() {
        println!("Test Function Working");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Ok(())
}