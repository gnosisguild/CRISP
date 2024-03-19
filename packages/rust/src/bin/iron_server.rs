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

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use rustc_serialize::json;
use rustc_serialize::json::Json;
use router::Router;
use std::io::Read;

use walkdir::WalkDir;

use ethers::{
    types::{Address, U256, Bytes},
};

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
    round_id: u32,
}

#[derive(RustcEncodable, RustcDecodable)]
struct CrispConfig {
    round_id: u32,
    chain_id: u32,
    voting_address: String,
    cyphernode_count: u32,
}

// fn get_new_crisp_id(req: &mut Request) -> IronResult<Response> {

// }

// fn register_cyphernode(req: &mut Request) -> IronResult<Response> {
    // register ip address or some way to contact nodes when a computation request comes in

// }

fn get_rounds(req: &mut Request) -> IronResult<Response> {
    let path = env::current_dir().unwrap();
    // read round count file in /keyshares

    let response = JsonResponse { response: "weee".to_string() };
    let out = json::encode(&response).unwrap();
    println!("get rounds hit");

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}

fn init_crisp_round(req: &mut Request) -> IronResult<Response> {
    // create a new dir for the round
    // use a round_id to lable dir
    // try to create the keyshares/id/ dir
    // store config file
    // update round count file in /keyshares
    let mut payload = String::new();

    // read the POST body
    req.body.read_to_string(&mut payload).unwrap();

    // we're expecting the POST to match the format of our JsonRequest struct
    let incoming: CrispConfig = json::decode(&payload).unwrap();
    println!("ID: {:?}", incoming.round_id);
    println!("Address: {:?}", incoming.voting_address);

    let path = env::current_dir().unwrap();
    let mut pathst = path.display().to_string();
    pathst.push_str("/keyshares/");
    pathst.push_str(&incoming.round_id.to_string());
    println!("The current directory is {}", pathst);
    fs::create_dir_all(pathst.clone()).unwrap();

    pathst.push_str("/config.json");
    let configfile = json::encode(&incoming).unwrap();
    fs::write(pathst.clone(), configfile).unwrap();

    // create a response with our random string, and pass in the string from the POST body
    let response = JsonResponse { response: "CRISP Initiated".to_string() };
    let out = json::encode(&response).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}


async fn aggregate_pk_shares() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("aggregating validator keyshare");

    let mut num_parties = 2; // todo set this from an init config 

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
        pk_share: PublicKeyShare,
    }
    //let mut parties = Vec::with_capacity(num_parties);
    let mut parties :Vec<Party> = Vec::new();
    for i in 0..num_parties {
        // read in pk_shares from storage
        let path = env::current_dir().unwrap();
        let mut keypath = path.display().to_string();
        keypath.push_str("/keyshares");
        let mut pathst = path.display().to_string();
        pathst.push_str("/keyshares/test-");
        pathst.push_str(&i.to_string());
        println!("The current directory is {}", pathst);
        let data = fs::read(pathst).expect("Unable to read file");
        let data_des = PublicKeyShare::deserialize(&data, &params, crp.clone()).unwrap();
        // let pk_share = PublicKeyShare::new(&sk_share, crp.clone(), &mut thread_rng())?;
        parties.push(Party { pk_share: data_des });
    }

    // Aggregation: this could be one of the parties or a separate entity. Or the
    // parties can aggregate cooperatively, in a tree-like fashion.
    let pk = timeit!("Public key aggregation", {
        let pk: PublicKey = parties.iter().map(|p| p.pk_share.clone()).aggregate()?;
        pk
    });
    println!("{:?}", pk);
    //let store_pk = pk.to_bytes();
    // store the public key
    Ok(())
}

fn handler(req: &mut Request) -> IronResult<Response> {
    let response = JsonResponse { response: pick_response() };
    let out = json::encode(&response).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}

#[tokio::main]
async fn post_handler(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();

    // read the POST body
    req.body.read_to_string(&mut payload).unwrap();

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
        aggregate_pk_shares().await;
    }

    // create a response with our random string, and pass in the string from the POST body
    let response = JsonResponse { response: pick_response() };
    let out = json::encode(&response).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Server Code
    let mut router = Router::new();
    router.get("/", handler, "index");
    router.get("/get_rounds", get_rounds, "get_rounds");
    router.post("/register_keyshare", post_handler, "register_keyshare");
    router.post("/init_crisp_round", init_crisp_round, "init_crisp_round");

    Iron::new(router).http("localhost:3000").unwrap();

    Ok(())
}
