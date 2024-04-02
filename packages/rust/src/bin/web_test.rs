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

type Client = SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>;

#[derive(RustcEncodable, RustcDecodable)]
struct JsonRequestGetRounds {
    response: String,
}

#[derive(Debug, Deserialize, RustcEncodable)]
struct RoundCount {
    round_count: u32,
}

#[derive(RustcEncodable, RustcDecodable)]
struct JsonRequest {
    response: String,
    pk_share: u32,
    id: u32,
    round_id: u32,
}

#[derive(Debug, Deserialize, RustcEncodable)]
struct CrispConfig {
    round_id: u32,
    chain_id: u32,
    voting_address: String,
    cyphernode_count: u32,
    voter_count: u32,
}

#[derive(Debug, Deserialize, RustcEncodable, RustcDecodable)]
struct PKRequest {
    round_id: u32,
    pk_bytes: Vec<u8>,
}

#[tokio::main]
//#[wasm_bindgen]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
            .build_arc()?
    );
    println!("let's bring fhe to the web! gg");
    Ok(())
}