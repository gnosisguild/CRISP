mod util;

use std::{env, error::Error, process::exit, sync::Arc, fs, path::Path, str};
use chrono::{DateTime, TimeZone, Utc};
use console::style;
use fhe::{
    bfv::{BfvParametersBuilder, Ciphertext, Encoding, Plaintext, PublicKey, SecretKey},
    mbfv::{AggregateIter, CommonRandomPoly, DecryptionShare, PublicKeyShare},
};
use fhe_traits::{FheDecoder, FheEncoder, FheEncrypter, Serialize as FheSerialize}; // TODO: see if we can use serde Serialize in fhe lib
use rand::{Rng, distributions::Uniform, prelude::Distribution, rngs::OsRng, thread_rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use util::timeit::{timeit, timeit_n};
use serde::{Deserialize, Serialize};
//use serde_json::{Result, Value};

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use router::Router;
use std::io::Read;
use std::fs::File;

use walkdir::WalkDir;

use ethers::{
    prelude::{abigen, Abigen},
    providers::{Http, Provider, StreamExt, Middleware},
    middleware::SignerMiddleware,
    signers::{LocalWallet, Signer, Wallet},
    types::{Address, U256, Bytes, TxHash, U64},
    core::k256,
    utils,
};

use sled::Db;
use once_cell::sync::Lazy;

struct Database {
    db: Db,
}

impl Database {
    pub fn new() -> Self {
        let pathdb = env::current_dir().unwrap();
        let mut pathdbst = pathdb.display().to_string();
        pathdbst.push_str("/database");
        let db = sled::open(pathdbst.clone()).unwrap();
        Self { db }
    }
}

static GLOBAL_DB: Lazy<Db> = Lazy::new(|| {
    let pathdb = env::current_dir().unwrap();
    let mut pathdbst = pathdb.display().to_string();
    pathdbst.push_str("/database/enclave_server");
    sled::open(pathdbst.clone()).unwrap()
});

//static open_db: Database = Database::new();

// static pathdb: String = env::current_dir().unwrap();
// static mut pathdbst: String = pathdb.display().to_string();
// pathdbst.push_str("/database");
// static db = sled::open(pathdbst.clone()).unwrap();
//static db: Db = sled::open("/home/ubuntu/guild/CRISP/packages/rust/database").unwrap();

// pick a string at random
fn pick_response() -> String {
    "Test".to_string()
}

#[derive(Debug, Deserialize, Serialize)]
struct JsonResponse {
    response: String
}

#[derive(Debug, Deserialize, Serialize)]
struct JsonResponseTxHash {
    response: String,
    tx_hash: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct JsonRequest {
    response: String,
    pk_share: Vec<u8>,
    id: u32,
    round_id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct CrispConfig {
    round_id: u32,
    poll_length: u32,
    chain_id: u32,
    voting_address: String,
    ciphernode_count: u32,
    enclave_address: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct RoundCount {
    round_count: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct PKShareCount {
    round_id: u32,
    share_id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct PKRequest {
    round_id: u32,
    pk_bytes: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
struct CRPRequest {
    round_id: u32,
    crp_bytes: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
struct TimestampRequest {
    round_id: u32,
    timestamp: i64,
}

#[derive(Debug, Deserialize, Serialize)]
struct PollLengthRequest {
    round_id: u32,
    poll_length: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct VoteCountRequest {
    round_id: u32,
    vote_count: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct SKSShareRequest {
    response: String,
    sks_share: Vec<u8>,
    id: u32,
    round_id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct EncryptedVote {
    round_id: u32,
    enc_vote_bytes: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
struct GetRoundRequest {
    round_id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct GetEmojisRequest {
    round_id: u32,
    emojis: [String; 2],
}

#[derive(Debug, Deserialize, Serialize)]
struct SKSSharePoll {
    response: String,
    round_id: u32,
    ciphernode_count: u32, //TODO: dont need this
}

#[derive(Debug, Deserialize, Serialize)]
struct SKSShareResponse {
    response: String,
    round_id: u32,
    sks_shares: Vec<Vec<u8>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ReportTallyRequest {
    round_id: u32,
    option_1: u32,
    option_2: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct WebResultRequest {
    round_id: u32,
    option_1_tally: u32,
    option_2_tally: u32,
    option_1_emoji: String,
    option_2_emoji: String,
    end_time: i64
}

#[derive(Debug, Deserialize, Serialize)]
struct StateWeb {
    id: u32,
    status: String,
    poll_length: u32,
    voting_address: String,
    chain_id: u32,
    ciphernode_count: u32,
    pk_share_count: u32,
    sks_share_count: u32,
    vote_count: u32,
    start_time: i64,
    ciphernode_total:  u32,
    emojis: [String; 2],
}

#[derive(Debug, Deserialize, Serialize)]
struct StateLite {
    id: u32,
    status: String,
    poll_length: u32,
    voting_address: String,
    chain_id: u32,
    ciphernode_count: u32,
    pk_share_count: u32,
    sks_share_count: u32,
    vote_count: u32,
    crp: Vec<u8>,
    pk: Vec<u8>,
    start_time: i64,
    block_start: U64,
    ciphernode_total:  u32,
    emojis: [String; 2],
}

#[derive(Debug, Deserialize, Serialize)]
struct Round {
    id: u32,
    status: String,
    poll_length: u32,
    voting_address: String,
    chain_id: u32,
    ciphernode_count: u32,
    pk_share_count: u32,
    sks_share_count: u32,
    vote_count: u32,
    crp: Vec<u8>,
    pk: Vec<u8>,
    start_time: i64,
    block_start: U64,
    ciphernode_total:  u32,
    emojis: [String; 2],
    votes_option_1: u32,
    votes_option_2: u32,
    ciphernodes: Vec<Ciphernode>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Ciphernode {
    id: u32,
    pk_share: Vec<u8>,
    sks_share: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
struct GetCiphernode {
    round_id: u32,
    ciphernode_id: u32,
}

fn generate_emoji() -> (String, String) {
    let emojis = [
        "🍇","🍈","🍉","🍊","🍋","🍌","🍍","🥭","🍎","🍏",
        "🍐","🍑","🍒","🍓","🫐","🥝","🍅","🫒","🥥","🥑",
        "🍆","🥔","🥕","🌽","🌶️","🫑","🥒","🥬","🥦","🧄",
        "🧅","🍄","🥜","🫘","🌰","🍞","🥐","🥖","🫓","🥨",
        "🥯","🥞","🧇","🧀","🍖","🍗","🥩","🥓","🍔","🍟",
        "🍕","🌭","🥪","🌮","🌯","🫔","🥙","🧆","🥚","🍳",
        "🥘","🍲","🫕","🥣","🥗","🍿","🧈","🧂","🥫","🍱",
        "🍘","🍙","🍚","🍛","🍜","🍝","🍠","🍢","🍣","🍤",
        "🍥","🥮","🍡","🥟","🥠","🥡","🦀","🦞","🦐","🦑",
        "🦪","🍦","🍧","🍨","🍩","🍪","🎂","🍰","🧁","🥧",
        "🍫","🍬","🍭","🍮","🍯","🍼","🥛","☕","🍵","🍾",
        "🍷","🍸","🍹","🍺","🍻","🥂","🥃",
    ];
    let index1 = rand::thread_rng().gen_range(0..emojis.len());
    let index2 = rand::thread_rng().gen_range(0..emojis.len());
    (emojis[index1].to_string(), emojis[index2].to_string())
}

fn get_state(round_id: u32) -> (Round, String) {
    let mut round_key = round_id.to_string();
    round_key.push_str("-storage");
    println!("Database key is {:?}", round_key);
    let state_out = GLOBAL_DB.get(round_key.clone()).unwrap().unwrap();
    let state_out_str = str::from_utf8(&state_out).unwrap();
    let state_out_struct: Round = serde_json::from_str(&state_out_str).unwrap();
    (state_out_struct, round_key)
}

#[tokio::main]
async fn broadcast_enc_vote(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    // read the POST body
    req.body.read_to_string(&mut payload).unwrap();
    let mut incoming: EncryptedVote = serde_json::from_str(&payload).unwrap();

    let (mut state, key) = get_state(incoming.round_id);

    let sol_vote = Bytes::from(incoming.enc_vote_bytes);
    let tx_hash = call_contract(sol_vote, state.voting_address.clone()).await.unwrap();
    let mut converter = "0x".to_string();
    for i in 0..32 {
        if(tx_hash[i] <= 16) {
            converter.push_str("0");
            converter.push_str(&format!("{:x}", tx_hash[i]));
        } else {
            converter.push_str(&format!("{:x}", tx_hash[i]));
        }
    }

    state.vote_count = state.vote_count + 1;
    let state_str = serde_json::to_string(&state).unwrap();
    let state_bytes = state_str.into_bytes();
    GLOBAL_DB.insert(key, state_bytes).unwrap();

    let response = JsonResponseTxHash { response: "tx_sent".to_string(), tx_hash: converter };
    let out = serde_json::to_string(&response).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    println!("Request for round {:?} send vote tx", incoming.round_id);
    Ok(Response::with((content_type, status::Ok, out)))
}

async fn call_contract(enc_vote: Bytes, address: String) -> Result<TxHash, Box<dyn std::error::Error + Send + Sync>> {
    println!("calling voting contract");

    let infura_key = "INFURAKEY";
    let infura_val = env::var(infura_key).unwrap();
    let mut RPC_URL = "https://sepolia.infura.io/v3/".to_string();
    RPC_URL.push_str(&infura_val);

    let provider = Provider::<Http>::try_from(RPC_URL.clone())?;
    // let block_number: U64 = provider.get_block_number().await?;
    // println!("{block_number}");
    abigen!(
        IVOTE,
        r#"[
            function voteEncrypted(bytes memory _encVote) public
            function getVote(address id) public returns(bytes memory)
            event Transfer(address indexed from, address indexed to, uint256 value)
        ]"#,
    );

    //const RPC_URL: &str = "https://eth.llamarpc.com";
    let VOTE_ADDRESS: &str = &address;

    let eth_key = "PRIVATEKEY";
    let eth_val = env::var(eth_key).unwrap();
    let wallet: LocalWallet = eth_val
        .parse::<LocalWallet>().unwrap()
        .with_chain_id(11155111 as u64);

    // 6. Wrap the provider and wallet together to create a signer client
    let client = SignerMiddleware::new(provider.clone(), wallet.clone());
    //let client = Arc::new(provider);
    let address: Address = VOTE_ADDRESS.parse()?;
    let contract = IVOTE::new(address, Arc::new(client.clone()));

    let test = contract.vote_encrypted(enc_vote).send().await?.clone();
    println!("{:?}", test);
    Ok(test)
}

// fn register_cyphernode(req: &mut Request) -> IronResult<Response> {
    // register ip address or some way to contact nodes when a computation request comes in

// }

fn get_node_by_round(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    // read the POST body
    req.body.read_to_string(&mut payload).unwrap();
    let mut incoming: GetCiphernode = serde_json::from_str(&payload).unwrap();
    println!("Request node data for round {:?}", incoming.round_id);

    let (state, key) = get_state(incoming.round_id);
    let mut cnode = Ciphernode {
        id: 0,
        pk_share: vec![0],
        sks_share: vec![0],
    };

    for i in 0..state.ciphernodes.len() {
        if(state.ciphernodes[i as usize].id == incoming.ciphernode_id){
            cnode.id = state.ciphernodes[i as usize].id;
            cnode.pk_share = state.ciphernodes[i as usize].pk_share.clone();
            cnode.sks_share = state.ciphernodes[i as usize].sks_share.clone();
        };
    };

    if(cnode.id != 0){
        let out = serde_json::to_string(&cnode).unwrap();

        let content_type = "application/json".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, out)))
    } else {
        let response = JsonResponse { response: "Ciphernode Not Registered".to_string() };
        let out = serde_json::to_string(&response).unwrap();

        let content_type = "application/json".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, out)))
    }

    // let response = JsonResponse { response: "Ciphernode Not Registered".to_string() };
    // let out = serde_json::to_string(&response).unwrap();

    // let content_type = "application/json".parse::<Mime>().unwrap();
    // Ok(Response::with((content_type, status::Ok, out)))
}

fn report_tally(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    // read the POST body
    req.body.read_to_string(&mut payload).unwrap();
    let mut incoming: ReportTallyRequest = serde_json::from_str(&payload).unwrap();
    println!("Request report tally for round {:?}", incoming.round_id);

    let (mut state, key) = get_state(incoming.round_id);
    if(state.votes_option_1 == 0 && state.votes_option_2 == 0) {
        state.votes_option_1 = incoming.option_1;
        state.votes_option_2 = incoming.option_2;

        let state_str = serde_json::to_string(&state).unwrap();
        let state_bytes = state_str.into_bytes();
        GLOBAL_DB.insert(key, state_bytes).unwrap();
    }
    let response = JsonResponse { response: "Tally Reported".to_string() };
    let out = serde_json::to_string(&response).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}

fn get_web_result(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    // read the POST body
    req.body.read_to_string(&mut payload).unwrap();
    let mut incoming: GetRoundRequest = serde_json::from_str(&payload).unwrap();
    println!("Request emojis for round {:?}", incoming.round_id);

    let (state, key) = get_state(incoming.round_id);
    
    let response = WebResultRequest {
        round_id: incoming.round_id,
        option_1_tally: state.votes_option_1,
        option_2_tally: state.votes_option_2,
        option_1_emoji: state.emojis[0].clone(),
        option_2_emoji: state.emojis[1].clone(),
        end_time: state.start_time + state.poll_length as i64
    };

    let out = serde_json::to_string(&response).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}

fn get_poll_length_by_round(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    // read the POST body
    req.body.read_to_string(&mut payload).unwrap();
    let mut incoming: PollLengthRequest = serde_json::from_str(&payload).unwrap();
    println!("Request poll length for round {:?}", incoming.round_id);

    let (state, key) = get_state(incoming.round_id);
    incoming.poll_length = state.poll_length;
    let out = serde_json::to_string(&incoming).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}

fn get_emojis_by_round(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    // read the POST body
    req.body.read_to_string(&mut payload).unwrap();
    let mut incoming: GetEmojisRequest = serde_json::from_str(&payload).unwrap();
    println!("Request emojis for round {:?}", incoming.round_id);

    let (state, key) = get_state(incoming.round_id);
    incoming.emojis = state.emojis;
    let out = serde_json::to_string(&incoming).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}

fn get_round_state(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    // read the POST body
    req.body.read_to_string(&mut payload).unwrap();
    let mut incoming: GetRoundRequest = serde_json::from_str(&payload).unwrap();
    println!("Request state for round {:?}", incoming.round_id);

    let (state, key) = get_state(incoming.round_id);
    let out = serde_json::to_string(&state).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}

fn get_round_state_web(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    // read the POST body
    req.body.read_to_string(&mut payload).unwrap();
    let mut incoming: GetRoundRequest = serde_json::from_str(&payload).unwrap();
    println!("Request state for round {:?}", incoming.round_id);

    let (state, key) = get_state(incoming.round_id);
    let state_lite = StateWeb {
        id: state.id,
        status: state.status,
        poll_length: state.poll_length,
        voting_address: state.voting_address,
        chain_id: state.chain_id,
        ciphernode_count: state.ciphernode_count,
        pk_share_count: state.pk_share_count,
        sks_share_count: state.sks_share_count,
        vote_count: state.vote_count,
        start_time: state.start_time,
        ciphernode_total:  state.ciphernode_total,
        emojis: state.emojis,
    };

    let out = serde_json::to_string(&state_lite).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}

fn get_round_state_lite(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    // read the POST body
    req.body.read_to_string(&mut payload).unwrap();
    let mut incoming: GetRoundRequest = serde_json::from_str(&payload).unwrap();
    println!("Request state for round {:?}", incoming.round_id);

    let (state, key) = get_state(incoming.round_id);
    let state_lite = StateLite {
        id: state.id,
        status: state.status,
        poll_length: state.poll_length,
        voting_address: state.voting_address,
        chain_id: state.chain_id,
        ciphernode_count: state.ciphernode_count,
        pk_share_count: state.pk_share_count,
        sks_share_count: state.sks_share_count,
        vote_count: state.vote_count,
        crp: state.crp,
        pk: state.pk,
        start_time: state.start_time,
        block_start: state.block_start,
        ciphernode_total:  state.ciphernode_total,
        emojis: state.emojis,
    };

    let out = serde_json::to_string(&state_lite).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}

fn get_vote_count_by_round(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    // read the POST body
    req.body.read_to_string(&mut payload).unwrap();
    let mut incoming: VoteCountRequest = serde_json::from_str(&payload).unwrap();
    println!("Request vote count for round {:?}", incoming.round_id);

    let (state, key) = get_state(incoming.round_id);
    incoming.vote_count = state.vote_count;
    let out = serde_json::to_string(&incoming).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}

fn get_start_time_by_round(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    // read the POST body
    req.body.read_to_string(&mut payload).unwrap();
    let mut incoming: TimestampRequest = serde_json::from_str(&payload).unwrap();
    println!("Request start time for round {:?}", incoming.round_id);

    let (state, key) = get_state(incoming.round_id);
    incoming.timestamp = state.start_time;
    let out = serde_json::to_string(&incoming).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}

fn get_crp_by_round(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    // read the POST body
    req.body.read_to_string(&mut payload).unwrap();
    let mut incoming: CRPRequest = serde_json::from_str(&payload).unwrap();
    println!("Request crp for round {:?}", incoming.round_id);

    let (state, key) = get_state(incoming.round_id);
    incoming.crp_bytes = state.crp;
    let out = serde_json::to_string(&incoming).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}

fn get_pk_by_round(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    // read the POST body
    req.body.read_to_string(&mut payload).unwrap();
    let mut incoming: PKRequest = serde_json::from_str(&payload).unwrap();

    let (state, key) = get_state(incoming.round_id);
    incoming.pk_bytes = state.pk;
    let out = serde_json::to_string(&incoming).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    println!("Request for round {:?} public key", incoming.round_id);
    Ok(Response::with((content_type, status::Ok, out)))
}

fn get_pk_share_count(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    // read the POST body
    req.body.read_to_string(&mut payload).unwrap();

    let mut incoming: PKShareCount = serde_json::from_str(&payload).unwrap();

    let (state, key) = get_state(incoming.round_id);
    incoming.share_id = state.pk_share_count;
    let out = serde_json::to_string(&incoming).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}

fn get_rounds(req: &mut Request) -> IronResult<Response> {
    let key = "round_count";
    let mut round = GLOBAL_DB.get(key).unwrap();
    if(round == None) {
        println!("initializing first round in db");
        GLOBAL_DB.insert(key, b"0".to_vec()).unwrap();
        round = GLOBAL_DB.get(key).unwrap();
    }
    let mut round_key = std::str::from_utf8(round.unwrap().as_ref()).unwrap().to_string();
    let mut round_int = round_key.parse::<u32>().unwrap();

    let count = RoundCount {round_count: round_int};
    println!("round_count: {:?}", count.round_count);

    let response = JsonResponse { response: "Round Count Retrieved".to_string() };
    let out = serde_json::to_string(&count).unwrap();
    println!("get rounds hit");

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}

#[tokio::main]
async fn init_crisp_round(req: &mut Request) -> IronResult<Response> {
    println!("generating round crp");

    let infura_key = "INFURAKEY";
    let infura_val = env::var(infura_key).unwrap();
    let mut RPC_URL = "https://sepolia.infura.io/v3/".to_string();
    RPC_URL.push_str(&infura_val);

    let provider = Provider::<Http>::try_from(RPC_URL.clone()).unwrap();
    let block_number: U64 = provider.get_block_number().await.unwrap();    

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
    let crp = CommonRandomPoly::new(&params, &mut thread_rng()).unwrap();
    let crp_bytes = crp.to_bytes();

    let mut payload = String::new();

    // read the POST body
    req.body.read_to_string(&mut payload).unwrap();

    // we're expecting the POST to match the format of our JsonRequest struct
    let incoming: CrispConfig = serde_json::from_str(&payload).unwrap();
    println!("ID: {:?}", incoming.round_id); // TODO: check that client sent the expected next round_id
    println!("Address: {:?}", incoming.voting_address);

    // --------------
    let key = "round_count";
    //db.remove(key)?;
    let round = GLOBAL_DB.get(key).unwrap();
    if(round == None) {
        println!("initializing first round in db");
        GLOBAL_DB.insert(key, b"0".to_vec()).unwrap();
    }
    let mut round_key = std::str::from_utf8(round.unwrap().as_ref()).unwrap().to_string();
    let mut round_int = round_key.parse::<u32>().unwrap();
    round_int = round_int + 1;
    let mut inc_round_key = round_int.to_string();
    inc_round_key.push_str("-storage");
    println!("Database key is {:?} and round int is {:?}", inc_round_key, round_int);

    let init_time = Utc::now();
    let timestamp = init_time.timestamp();
    println!("timestamp {:?}", timestamp);

    let (emoji1, emoji2) = generate_emoji();

    let state = Round {
        id: round_int,
        status: "Active".to_string(),
        poll_length: incoming.poll_length,
        voting_address: incoming.voting_address,
        chain_id: incoming.chain_id,
        ciphernode_count: 0,
        pk_share_count: 0,
        sks_share_count: 0,
        vote_count: 0,
        crp: crp_bytes,
        pk: vec![0],
        start_time: timestamp,
        block_start: block_number,
        ciphernode_total: incoming.ciphernode_count,
        emojis: [emoji1, emoji2],
        votes_option_1: 0,
        votes_option_2: 0,
        ciphernodes: vec![
            Ciphernode {
                id: 0,
                pk_share: vec![0],
                sks_share: vec![0],
            }
        ],
    };

    let state_str = serde_json::to_string(&state).unwrap();
    let state_bytes = state_str.into_bytes();
    let key2 = round_int.to_string();
    GLOBAL_DB.insert(inc_round_key, state_bytes).unwrap();

    let new_round_bytes = key2.into_bytes();
    GLOBAL_DB.insert(key, new_round_bytes).unwrap();

    // create a response with our random string, and pass in the string from the POST body
    let response = JsonResponse { response: "CRISP Initiated".to_string() };
    let out = serde_json::to_string(&response).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}


async fn aggregate_pk_shares(round_id: u32) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("aggregating validator keyshare");

    let degree = 4096;
    let plaintext_modulus: u64 = 4096;
    let moduli = vec![0xffffee001, 0xffffc4001, 0x1ffffe0001];

    // Generate a deterministic seed for the Common Poly
    //let mut seed = <ChaCha8Rng as SeedableRng>::Seed::default();

    // Let's generate the BFV parameters structure.
    let params = timeit!(
        "Parameters generation",
        BfvParametersBuilder::new()
            .set_degree(degree)
            .set_plaintext_modulus(plaintext_modulus)
            .set_moduli(&moduli)
            .build_arc()?
    );

    let mut round_key = round_id.to_string();
    round_key.push_str("-storage");
    println!("Database key is {:?}", round_key);

    let state_out = GLOBAL_DB.get(round_key.clone()).unwrap().unwrap();
    let state_out_str = str::from_utf8(&state_out).unwrap();
    let mut state: Round = serde_json::from_str(&state_out_str).unwrap();
    println!("checking db after drop {:?}", state.ciphernode_count);
    println!("{:?}", state.ciphernodes[0].id);
    //println!("{:?}", state.ciphernodes[0].pk_share);

    //let crp = CommonRandomPoly::new_deterministic(&params, seed)?;
    let crp = CommonRandomPoly::deserialize(&state.crp, &params)?;

    // Party setup: each party generates a secret key and shares of a collective
    // public key.
    struct Party {
        pk_share: PublicKeyShare,
    }

    let mut parties :Vec<Party> = Vec::new();
    for i in 1..state.ciphernode_total + 1 { // todo fix init code that causes offset
        // read in pk_shares from storage
        println!("Aggregating PKShare... id {}", i);
        let data_des = PublicKeyShare::deserialize(&state.ciphernodes[i as usize].pk_share, &params, crp.clone()).unwrap();
        // let pk_share = PublicKeyShare::new(&sk_share, crp.clone(), &mut thread_rng())?;
        parties.push(Party { pk_share: data_des });
    }

    // Aggregation: this could be one of the parties or a separate entity. Or the
    // parties can aggregate cooperatively, in a tree-like fashion.
    let pk = timeit!("Public key aggregation", {
        let pk: PublicKey = parties.iter().map(|p| p.pk_share.clone()).aggregate()?;
        pk
    });
    //println!("{:?}", pk);
    println!("Multiparty Public Key Generated");
    let store_pk = pk.to_bytes();
    state.pk = store_pk;
    let state_str = serde_json::to_string(&state).unwrap();
    let state_bytes = state_str.into_bytes();
    GLOBAL_DB.insert(round_key, state_bytes).unwrap();
    println!("aggregate pk stored for round {:?}", round_id);
    Ok(())
}

fn handler(req: &mut Request) -> IronResult<Response> {
    for (key, value) in env::vars() {
        println!("{key}: {value}");
    }
    let response = JsonResponse { response: pick_response() };
    let out = serde_json::to_string(&response).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}

fn health_handler(req: &mut Request) -> IronResult<Response> {
    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok)))
}

// polling endpoint for sks shares

fn register_sks_share(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();

    // read the POST body
    req.body.read_to_string(&mut payload).unwrap();

    // we're expecting the POST to match the format of our JsonRequest struct
    let incoming: SKSShareRequest = serde_json::from_str(&payload).unwrap();
    println!("{:?}", incoming.response);
    println!("ID: {:?}", incoming.id); // cipher node id (based on first upload of pk share)
    println!("Round ID: {:?}", incoming.round_id);


    let mut round_key = incoming.round_id.to_string();
    round_key.push_str("-storage");
    println!("Database key is {:?}", round_key);

    let state_out = GLOBAL_DB.get(round_key.clone()).unwrap().unwrap();
    let state_out_str = str::from_utf8(&state_out).unwrap();
    let mut state_out_struct: Round = serde_json::from_str(&state_out_str).unwrap();
    state_out_struct.sks_share_count = state_out_struct.sks_share_count + 1;

    let index = incoming.id + 1; // offset from vec push
    state_out_struct.ciphernodes[index as usize].sks_share = incoming.sks_share;
    let state_str = serde_json::to_string(&state_out_struct).unwrap();
    let state_bytes = state_str.into_bytes();
    GLOBAL_DB.insert(round_key, state_bytes).unwrap();
    println!("sks share stored for node id {:?}", incoming.id);

    // toso get share threshold from client config
    if(state_out_struct.sks_share_count == state_out_struct.ciphernode_total) {
        println!("All sks shares received");
        //aggregate_pk_shares(incoming.round_id).await;
        // TODO: maybe notify cipher nodes
    }

    // create a response with our random string, and pass in the string from the POST body
    let response = JsonResponse { response: pick_response() };
    let out = serde_json::to_string(&response).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}

fn get_sks_shares(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();

    // read the POST body
    req.body.read_to_string(&mut payload).unwrap();

    // we're expecting the POST to match the format of our JsonRequest struct
    let incoming: SKSSharePoll = serde_json::from_str(&payload).unwrap();
    //const length: usize = incoming.cyphernode_count;

    let (mut state, key) = get_state(incoming.round_id);

    let mut shares = Vec::with_capacity(incoming.ciphernode_count as usize);

    // toso get share threshold from client config
    if(state.sks_share_count == state.ciphernode_total) {
        println!("All sks shares received... sending to cipher nodes");
        for i in 1..state.ciphernode_total + 1 {
            println!("reading share {:?}", i);
            shares.push(state.ciphernodes[i as usize].sks_share.clone());
        }
        let response = SKSShareResponse { 
            response: "final".to_string(),
            round_id: incoming.round_id,
            sks_shares: shares,
        };
        state.status = "Finalized".to_string();
        let state_str = serde_json::to_string(&state).unwrap();
        let state_bytes = state_str.into_bytes();
        GLOBAL_DB.insert(key, state_bytes).unwrap();
        let out = serde_json::to_string(&response).unwrap();
        println!("get rounds hit");

        let content_type = "application/json".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, out)))
    } else {
        let response = SKSShareResponse { 
            response: "waiting".to_string(),
            round_id: incoming.round_id,
            sks_shares: shares,
        };
        let out = serde_json::to_string(&response).unwrap();
        println!("get rounds hit");

        let content_type = "application/json".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, out)))
    }
}

#[tokio::main]
async fn register_ciphernode(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();

    // read the POST body
    req.body.read_to_string(&mut payload).unwrap();

    // we're expecting the POST to match the format of our JsonRequest struct
    let incoming: JsonRequest = serde_json::from_str(&payload).unwrap();
    println!("{:?}", incoming.response);
    println!("ID: {:?}", incoming.id);
    println!("Round ID: {:?}", incoming.round_id);

    let (mut state, key) = get_state(incoming.round_id);

    state.pk_share_count = state.pk_share_count + 1;
    state.ciphernode_count = state.ciphernode_count + 1;
    let cnode = Ciphernode {
        id: incoming.id,
        pk_share: incoming.pk_share,
        sks_share: vec![0],
    };
    state.ciphernodes.push(cnode);
    let state_str = serde_json::to_string(&state).unwrap();
    let state_bytes = state_str.into_bytes();
    GLOBAL_DB.insert(key, state_bytes).unwrap();

    println!("pk share store for node id {:?}", incoming.id);
    println!("ciphernode count {:?}", state.ciphernode_count);
    println!("ciphernode total {:?}", state.ciphernode_total);
    println!("pk share count {:?}", state.pk_share_count);

    if(state.ciphernode_count == state.ciphernode_total) {
        println!("All shares received");
        aggregate_pk_shares(incoming.round_id).await;
    }

    // create a response with our random string, and pass in the string from the POST body
    let response = JsonResponse { response: pick_response() };
    let out = serde_json::to_string(&response).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Server Code
    let mut router = Router::new();
    router.get("/", handler, "index");
    router.get("/health", health_handler, "health");
    router.get("/get_rounds", get_rounds, "get_rounds");
    router.post("/get_pk_share_count", get_pk_share_count, "get_pk_share_count");
    router.post("/register_ciphernode", register_ciphernode, "register_ciphernode");
    router.post("/init_crisp_round", init_crisp_round, "init_crisp_round");
    router.post("/get_pk_by_round", get_pk_by_round, "get_pk_by_round");
    router.post("/register_sks_share", register_sks_share, "register_sks_share");
    router.post("/get_sks_shares", get_sks_shares, "get_sks_shares");
    router.post("/get_crp_by_round", get_crp_by_round, "get_crp_by_round");
    router.post("/broadcast_enc_vote", broadcast_enc_vote, "broadcast_enc_vote");
    router.post("/get_vote_count_by_round", get_vote_count_by_round, "get_vote_count_by_round");
    router.post("/get_start_time_by_round", get_start_time_by_round, "get_start_time_by_round");
    router.post("/get_emojis_by_round", get_emojis_by_round, "get_emojis_by_round");
    router.post("/get_poll_length_by_round", get_poll_length_by_round, "get_poll_length_by_round");
    router.post("/get_round_state_lite", get_round_state_lite, "get_round_state_lite");
    router.post("/report_tally", report_tally, "report_tally");
    router.post("/get_web_result", get_web_result, "get_web_result");
    router.post("/get_round_state_web", get_web_result, "get_round_state_web");
    router.post("/get_node_by_round", get_node_by_round, "get_node_by_round");

    Iron::new(router).http("0.0.0.0:4000").unwrap();

    Ok(())
}
