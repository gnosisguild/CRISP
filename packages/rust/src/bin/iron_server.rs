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
use router::Router;
use std::io::Read;

use walkdir::WalkDir;

// pick a string at random
fn pick_response() -> String {
    "Test".to_string()
}

#[derive(RustcEncodable, RustcDecodable)]
struct JsonResponse {
    response: String
}

#[derive(RustcEncodable, RustcDecodable)]
struct JsonRequest {
    response: String,
    pk_share: Vec<u8>,
    id: u32,
}

fn handler(req: &mut Request) -> IronResult<Response> {
    let response = JsonResponse { response: pick_response() };
    let out = json::encode(&response).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}

// async fn gen_pk_shares() -> {

// }

fn post_handler(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();

    // read the POST body
    req.body.read_to_string(&mut payload).unwrap();
    print!("weeee got the payload");
    //println!("{:?}", payload);

    // we're expecting the POST to match the format of our JsonRequest struct
    let incoming: JsonRequest = json::decode(&payload).unwrap();
    println!("{:?}", incoming.response);
    println!("ID: {:?}", incoming.id);

    let path = env::current_dir().unwrap();
    let mut keypath = path.display().to_string();
    keypath.push_str("/keyshares");
    let mut pathst = path.display().to_string();
    pathst.push_str("/keyshares/test-");
    pathst.push_str(&incoming.id.to_string());
    println!("The current directory is {}", pathst);
    fs::write(pathst.clone(), incoming.pk_share).unwrap();

    let share_count = WalkDir::new(keypath.clone()).into_iter().count();
    println!("Files: {}", WalkDir::new(keypath.clone()).into_iter().count());
    if(share_count == 3) {
        println!("All shares received");
    }
    //let data = fs::read(pathst).expect("Unable to read file");
    //let test_1_des = PublicKeyShare::deserialize(&incoming.pk_share, &params, crp.clone()).unwrap();

    // create a response with our random string, and pass in the string from the POST body
    let response = JsonResponse { response: pick_response() };
    let out = json::encode(&response).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
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

    let test = pk.to_bytes();

    //let path = env::current_dir().unwrap();
    //let mut pathst = path.display().to_string();
    //pathst.push_str("/test");
    //println!("The current directory is {}", pathst);
    //let data = fs::read(pathst).expect("Unable to read file");
    //let test_1_des = PublicKeyShare::deserialize(&data, &params, crp.clone()).unwrap();
    //print!("{:?}", test_1_des);

    // Server Code
    let mut router = Router::new();
    router.get("/", handler, "index");
    router.post("/", post_handler, "post_name");

    Iron::new(router).http("localhost:3000").unwrap();

    Ok(())
}
