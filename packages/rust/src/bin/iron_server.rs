mod util;

use std::{env, error::Error, process::exit, sync::Arc, fs, path::Path};
use console::style;
use fhe::{
    bfv::{BfvParametersBuilder, Ciphertext, Encoding, Plaintext, PublicKey, SecretKey},
    mbfv::{AggregateIter, CommonRandomPoly, DecryptionShare, PublicKeyShare},
};
use fhe_traits::{FheDecoder, FheEncoder, FheEncrypter, Serialize};
use rand::{distributions::Uniform, prelude::Distribution, rngs::OsRng, thread_rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use util::timeit::{timeit, timeit_n};
//use serde::{Deserialize};
//use serde_json::{Result, Value};

// use std::convert::Infallible;
// use std::net::SocketAddr;

// use http_body_util::Full;
// use hyper::body::Bytes;
// use hyper::server::conn::http1;
// use hyper::service::service_fn;
// use hyper::{Request, Response};
// use hyper_util::rt::TokioIo;
// use tokio::net::TcpListener;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use rustc_serialize::json;

// pick a string at random
fn pick_response() -> String {
    "Test".to_string()
}

#[derive(RustcEncodable)]
struct JsonResponse {
    response: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("generating validator keyshare");

    let mut num_parties = 10;
    let mut num_voters = 2;

    let degree = 4096;
    let plaintext_modulus: u64 = 4096;
    let moduli = vec![0xffffee001, 0xffffc4001, 0x1ffffe0001];

    // Generate a deterministic seed for the Common Poly
    let mut seed = <ChaCha8Rng as SeedableRng>::Seed::default();

    // Let's generate the BFV parameters structure.
    let params = timeit!(
        "Parameters generation",
        BfvParametersBuilder::new()
            .set_degree(degree)
            .set_plaintext_modulus(plaintext_modulus)
            .set_moduli(&moduli)
            .build_arc()?
    );

    let crp = CommonRandomPoly::new_deterministic(&params, seed)?;
    println!("{:?}", crp);

    // Party setup: each party generates a secret key and shares of a collective
    // public key.
    struct Party {
        sk_share: SecretKey,
        pk_share: PublicKeyShare,
    }
    let mut parties = Vec::with_capacity(num_parties);
    timeit_n!("Party setup (per party)", num_parties as u32, {
        let sk_share = SecretKey::random(&params, &mut OsRng);
        let pk_share = PublicKeyShare::new(&sk_share, crp.clone(), &mut thread_rng())?;
        parties.push(Party { sk_share, pk_share });
    });

    // Aggregation: this could be one of the parties or a separate entity. Or the
    // parties can aggregate cooperatively, in a tree-like fashion.
    let pk = timeit!("Public key aggregation", {
        let pk: PublicKey = parties.iter().map(|p| p.pk_share.clone()).aggregate()?;
        pk
    });
    //println!("{:?}", pk);
    //let test = pk_share.to_bytes();
    //println!("{:?}", pk.c);
    let test = pk.to_bytes();

    //let path: &Path = ...;
    //fs::write(path, file_contents_base64).unwrap();

    // Server Code
    Iron::new(|_: &mut Request| {
        let content_type = "application/json".parse::<Mime>().unwrap();

        // create the response
        let response = JsonResponse { response: pick_response() };

        // convert the response struct to JSON
        let out = json::encode(&response).unwrap();

        Ok(Response::with((content_type, status::Ok, out)))
    }).http("localhost:3000").unwrap();

    Ok(())
}
