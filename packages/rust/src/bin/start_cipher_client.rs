mod util;

use std::{env, error::Error, process::exit, sync::Arc, fs, path::Path, process, str};
use std::sync::mpsc::{self, TryRecvError};
use chrono::{DateTime, TimeZone, Utc};
use console::style;
use fhe::{
    bfv::{BfvParametersBuilder, Ciphertext, Encoding, Plaintext, PublicKey, SecretKey},
    mbfv::{AggregateIter, CommonRandomPoly, DecryptionShare, PublicKeyShare, SecretKeySwitchShare},
};
use fhe_traits::{FheDecoder, FheEncoder, FheEncrypter, Serialize as FheSerialize, DeserializeParametrized};
//use fhe_math::rq::{Poly};
use rand::{Rng, distributions::Uniform, prelude::Distribution, rngs::OsRng, thread_rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use util::timeit::{timeit, timeit_n};
use serde::{Deserialize, Serialize};
use http_body_util::Empty;
use hyper::Request;
//use hyper::body::Bytes;
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;
use http_body_util::BodyExt;
use tokio::io::{AsyncWriteExt as _, self};
use tokio::runtime::Runtime;

use std::{thread, time};

use ethers::{
    prelude::{Abigen, Contract, EthEvent},
    providers::{Http, Provider, StreamExt, Middleware},
    middleware::SignerMiddleware,
    signers::{LocalWallet, Signer, Wallet},
    types::{Address, U256, Bytes, U64, Filter, H256},
    core::k256,
    utils,
    contract::abigen,
};

use sled::Db;
use once_cell::sync::Lazy;

#[derive(Debug, Deserialize, Serialize)]
struct JsonRequest {
    response: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct GetRoundRequest {
    round_id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct PKShareRequest {
    response: String,
    pk_share: Vec<u8>,
    id: u32,
    round_id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct SKSShareRequest {
    response: String,
    sks_share: Vec<u8>,
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
    // todo start_block: u32,
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
struct CRPRequest {
    round_id: u32,
    crp_bytes: Vec<u8>,
}

// Party setup: each party generates a secret key and shares of a collective
// public key.
struct Party {
    sk_share: SecretKey,
    pk_share: PublicKeyShare,
}

type Client = SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>;

#[derive(Debug, Clone, EthEvent)]
pub struct Voted {
    pub voter: Address,
    pub vote: Bytes,
}

#[derive(Debug, Deserialize, Serialize)]
struct SKSShareResponse {
    response: String,
    round_id: u32,
    sks_shares: Vec<Vec<u8>>,
}
#[derive(Debug, Deserialize, Serialize)]
struct SKSSharePoll {
    response: String,
    round_id: u32,
    ciphernode_count: u32
}

#[derive(Debug, Deserialize, Serialize)]
struct VoteCountRequest {
    round_id: u32,
    vote_count: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct ReportTallyRequest {
    round_id: u32,
    option_1: u32,
    option_2: u32,
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
struct CiphernodeDB {
    id: u32,
    round_storage: Vec<RoundData>,
}

#[derive(Debug, Deserialize, Serialize)]
struct RoundData {
    round_id: u32,
    encrypted_votes: Vec<Vec<u8>>,
}

// #[derive(Debug, Deserialize, Serialize)]
// struct Round {
//     id: u32,
//     voting_address: String,
//     chain_id: u32,
//     ciphernode_count: u32,
//     pk_share_count: u32,
//     sks_share_count: u32,
//     vote_count: u32,
//     crp: Vec<u8>,
//     pk: Vec<u8>,
//     start_time: i64,
//     ciphernode_total:  u32,
//     emojis: [String; 2],
//     ciphernodes: Vec<Ciphernode>,
// }

// #[derive(Debug, Deserialize, Serialize)]
// struct Ciphernode {
//     id: u32,
//     pk_share: Vec<u8>,
//     sks_share: Vec<u8>,
// }

static ID: Lazy<i64> = Lazy::new(|| {
    rand::thread_rng().gen_range(0..1000)
});

static GLOBAL_DB: Lazy<Db> = Lazy::new(|| {
    let pathdb = env::current_dir().unwrap();
    let mut pathdbst = pathdb.display().to_string();
    pathdbst.push_str("/database/ciphernode-");
    pathdbst.push_str(&ID.to_string());
    sled::open(pathdbst.clone()).unwrap()
});

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Initializing parameters.");

    let degree = 4096;
    let plaintext_modulus: u64 = 4096;
    let moduli = vec![0xffffee001, 0xffffc4001, 0x1ffffe0001];

    // Generate a deterministic seed for the Common Poly
    //let mut seed = <ChaCha8Rng as SeedableRng>::Seed::default();

    // Generate the BFV parameters structure.
    let params = timeit!(
        "Parameters generation",
        BfvParametersBuilder::new()
            .set_degree(degree)
            .set_plaintext_modulus(plaintext_modulus)
            .set_moduli(&moduli)
            .build_arc()?
    );

    // set the expected CRISP rounds
    let mut internal_round_count = RoundCount { round_count: 0 };

    loop {
        println!("Polling CRISP server...");

        // Client Code Get Rounds
        let url_get_rounds = "http://127.0.0.1/get_rounds".parse::<hyper::Uri>()?;
        let host = url_get_rounds.host().expect("uri has no host");
        let port = url_get_rounds.port_u16().unwrap_or(4000);
        let address = format!("{}:{}", host, port);
        let stream = TcpStream::connect(address).await?;
        let io = TokioIo::new(stream);
        let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;
        tokio::task::spawn(async move {
            if let Err(err) = conn.await {
                println!("Connection failed: {:?}", err);
            }
        });
        let authority = url_get_rounds.authority().unwrap().clone();

        let response = JsonRequest { response: "get_rounds".to_string() };
        let out = serde_json::to_string(&response).unwrap();
        let req = Request::get("http://127.0.0.1/")
            .uri(url_get_rounds.clone())
            .header(hyper::header::HOST, authority.as_str())
            .body(out)?;

        let mut res = sender.send_request(req).await?;
        println!("Response status: {}", res.status());

        let body_bytes = res.collect().await?.to_bytes();
        let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
        let count: RoundCount = serde_json::from_str(&body_str).expect("JSON was not well-formatted");
        println!("Server Round Count: {:?}", count.round_count);
        println!("Internal Round Count: {:?}", internal_round_count.round_count);

        // Check to see if the server reported a new round
        // TODO: also check timestamp to be sure round isnt over, or already registered
        if(count.round_count > internal_round_count.round_count) {
            println!("Getting Ciphernode ID."); // This is the current pk share count for now.
            // Client Code get the number of pk_shares on the server.
            // Currently the number of shares becomes the cipher client ID for the round.
            let url_get_state = "http://127.0.0.1/get_round_state_lite".parse::<hyper::Uri>()?;
            let host_get_state = url_get_state.host().expect("uri has no host");
            let port_get_state = url_get_state.port_u16().unwrap_or(4000);
            let address_get_state = format!("{}:{}", host_get_state, port_get_state);
            let stream_get_state = TcpStream::connect(address_get_state).await?;
            let io_get_state = TokioIo::new(stream_get_state);
            let (mut sender_get_state, conn_get_state) = hyper::client::conn::http1::handshake(io_get_state).await?;
            tokio::task::spawn(async move {
                if let Err(err) = conn_get_state.await {
                    println!("Connection failed: {:?}", err);
                }
            });
            let authority_get_state = url_get_state.authority().unwrap().clone();

            let response_get_state = GetRoundRequest { round_id: count.round_count };
            let out_get_state = serde_json::to_string(&response_get_state).unwrap();
            let req_get_state = Request::post("http://127.0.0.1/")
                .uri(url_get_state.clone())
                .header(hyper::header::HOST, authority_get_state.as_str())
                .body(out_get_state)?;

            let mut res_get_state = sender_get_state.send_request(req_get_state).await?;

            println!("Response status: {}", res_get_state.status());

            let body_bytes = res_get_state.collect().await?.to_bytes();
            let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
            // static state: Lazy<StateLite> = Lazy::new(|| {
            //     serde_json::from_str(&body_str).expect("JSON was not well-formatted")
            // });
            let state: StateLite = serde_json::from_str(&body_str).expect("JSON was not well-formatted");
            //let share_count: PKShareCount = serde_json::from_str(&body_str).expect("JSON was not well-formatted");
            println!("database round count {:?}", state.id);
            println!("database pk share id {:?}", state.pk_share_count);
            // TODO: store this to come back after crash with same id
            let node_id = state.pk_share_count;

            // TODO: if(state.poll_length + state.start_time > internal_time) { skip; }
            // TODO: if(state.ciphernode_count == state.ciphernode_total) { skip; } 
            // TODO: create storage for pk and allow for re-entering a round if client crashes

            // --------------------------------------
            println!("Generating share and serializing.");

            // deserialize crp_bytes
            let crp = CommonRandomPoly::deserialize(&state.crp, &params).unwrap();
            let sk_share_1 = SecretKey::random(&params, &mut OsRng); // TODO Store secret key
            let pk_share_1 = PublicKeyShare::new(&sk_share_1, crp.clone(), &mut thread_rng())?;
            // serialize pk_share
            let pk_share_bytes = pk_share_1.to_bytes();

            // --------------------------------------
            // Client Code Register PK Share on Enclave server
            let url_register_keyshare = "http://127.0.0.1/register_ciphernode".parse::<hyper::Uri>()?;
            let host_key = url_register_keyshare.host().expect("uri has no host");
            let port_key = url_register_keyshare.port_u16().unwrap_or(4000);
            let address_key = format!("{}:{}", host_key, port_key);
            let stream_key = TcpStream::connect(address_key).await?;
            let io_key = TokioIo::new(stream_key);
            let (mut sender_key, conn_key) = hyper::client::conn::http1::handshake(io_key).await?;
            tokio::task::spawn(async move {
                if let Err(err) = conn_key.await {
                    println!("Connection failed: {:?}", err);
                }
            });
            let authority_key = url_register_keyshare.authority().unwrap().clone();

            let response_key = PKShareRequest {
                response: "Test".to_string(),
                pk_share: pk_share_bytes,
                id: node_id,
                round_id: state.id
            };
            let out_key = serde_json::to_string(&response_key).unwrap();
            let req_key = Request::post("http://127.0.0.1/")
                .uri(url_register_keyshare.clone())
                .header(hyper::header::HOST, authority_key.as_str())
                .body(out_key)?;

            let mut res_key = sender_key.send_request(req_key).await?;
            println!("Response status: {}", res_key.status());
            // Stream the body, writing each frame to stdout as it arrives
            while let Some(next) = res_key.frame().await {
                let frame = next?;
                if let Some(chunk) = frame.data_ref() {
                    io::stdout().write_all(chunk).await?;
                }
            }

            internal_round_count.round_count += 1;

            //todo get voters per round and cyphernodes
            //let mut num_voters = 2;
            let mut num_parties = state.ciphernode_total;
            //let mut votes_encrypted = Vec::with_capacity(1000); // todo: store votes 
            let mut counter = 0;

            // For each voting round this node is participating in, check the contracts for vote events.
            // When voting is finalized, begin group decrypt process

            // let (tx, rx) = mpsc::channel::<()>();
            // let rt = Runtime::new().unwrap();
            // //thread::spawn(move || {
            //     rt.spawn(async move { poll_contract(state.id, node_id, rx).await });
            // //});

            // TODO: move to thread so main loop can continue to look for more work
            loop {
                println!("Looping to check for poll end.");
                let now = Utc::now();
                let internal_time = now.timestamp();
                if (state.start_time + state.poll_length as i64) < internal_time {
                    print!("poll time ended... performing fhe computation");

                    let url_get_voters = "http://127.0.0.1/get_vote_count_by_round".parse::<hyper::Uri>()?;
                    let host_get_voters = url_get_voters.host().expect("uri has no host");
                    let port_get_voters = url_get_voters.port_u16().unwrap_or(4000);
                    let address_get_voters = format!("{}:{}", host_get_voters, port_get_voters);
                    let stream_get_voters = TcpStream::connect(address_get_voters).await?;
                    let io_get_voters = TokioIo::new(stream_get_voters);
                    let (mut sender_get_voters, conn_get_voters) = hyper::client::conn::http1::handshake(io_get_voters).await?;
                    tokio::task::spawn(async move {
                        if let Err(err) = conn_get_voters.await {
                            println!("Connection failed: {:?}", err);
                        }
                    });
                    let authority_get_voters = url_get_voters.authority().unwrap().clone();

                    let response_get_voters = VoteCountRequest { round_id: state.id, vote_count: 0 };
                    let out_get_voters = serde_json::to_string(&response_get_voters).unwrap();
                    let req_get_voters = Request::post("http://127.0.0.1/")
                        .uri(url_get_voters.clone())
                        .header(hyper::header::HOST, authority_get_voters.as_str())
                        .body(out_get_voters)?;

                    let mut res_get_voters = sender_get_voters.send_request(req_get_voters).await?;
                    println!("Response status: {}", res_get_voters.status());

                    let body_bytes_get_voters = res_get_voters.collect().await?.to_bytes();
                    let body_str_get_voters = String::from_utf8(body_bytes_get_voters.to_vec()).unwrap();
                    let num_voters: VoteCountRequest = serde_json::from_str(&body_str_get_voters).expect("JSON was not well-formatted");


                    // get votes from db for round 
                    // let mut key = state.id.to_string();
                    // key.push_str("-");
                    // key.push_str(&node_id.to_string());
                    // key.push_str("-ciphernode-storage");
                    // let votes_db = GLOBAL_DB.get(key).unwrap().unwrap();
                    // let votes_out_str = str::from_utf8(&votes_db).unwrap();
                    // let votes_out_struct: RoundData = serde_json::from_str(&votes_out_str).unwrap();

                    let mut votes_collected = get_votes_contract(state.id, state.block_start, state.voting_address, state.chain_id).await;

                    println!("number of votes from filter {:?}", votes_collected.len());
                    println!("all votes collected? {:?}", num_voters.vote_count == votes_collected.len() as u32);

                    let tally = timeit!("Vote tallying", {
                        let mut sum = Ciphertext::zero(&params);
                        for i in 0..(votes_collected.len()) {
                            println!("index {:?}", i);
                            let deserialized_vote = Ciphertext::from_bytes(&votes_collected[i as usize], &params).unwrap();
                            sum += &deserialized_vote;
                        }
                        // for ct in &votes_encrypted {
                        //     sum += ct;
                        // }
                        Arc::new(sum)
                    });

                    // The result of a vote is typically public, so in this scenario the parties can
                    // perform a collective decryption. If instead the result of the computation
                    // should be kept private, the parties could collectively perform a
                    // keyswitch to a different public key.
                    let mut decryption_shares = Vec::with_capacity(state.ciphernode_total as usize);
                    let mut _i = 0;
                    let sh = DecryptionShare::new(&sk_share_1, &tally, &mut thread_rng()).unwrap();
                    let sks_bytes = sh.to_bytes();

                    // ------------------------------------
                    // Client Code register sks share with chrys server
                    let url_register_sks = "http://127.0.0.1/register_sks_share".parse::<hyper::Uri>()?;
                    let host_sks = url_register_sks.host().expect("uri has no host");
                    let port_sks = url_register_sks.port_u16().unwrap_or(4000);
                    let address_sks = format!("{}:{}", host_sks, port_sks);
                    let stream_sks = TcpStream::connect(address_sks).await?;
                    let io_sks = TokioIo::new(stream_sks);
                    let (mut sender_sks, conn_sks) = hyper::client::conn::http1::handshake(io_sks).await?;
                    tokio::task::spawn(async move {
                        if let Err(err) = conn_sks.await {
                            println!("Connection failed: {:?}", err);
                        }
                    });
                    let authority_sks = url_register_sks.authority().unwrap().clone();
                    let response_sks = SKSShareRequest {
                        response: "Register_SKS_Share".to_string(),
                        sks_share: sks_bytes,
                        id: node_id,
                        round_id: state.id
                    };
                    let out_sks = serde_json::to_string(&response_sks).unwrap();
                    let req_sks = Request::post("http://127.0.0.1/")
                        .uri(url_register_sks.clone())
                        .header(hyper::header::HOST, authority_sks.as_str())
                        .body(out_sks)?;

                    let mut res_sks = sender_sks.send_request(req_sks).await?;
                    println!("Response status: {}", res_sks.status());

                    // Stream the body, writing each frame to stdout as it arrives
                    while let Some(next) = res_key.frame().await {
                        let frame = next?;
                        if let Some(chunk) = frame.data_ref() {
                            io::stdout().write_all(chunk).await?;
                        }
                    }

                    // poll the chrys server to get all sks shares.
                    loop {
                        // Client Code Get all sks shares
                        let url_register_get_sks = "http://127.0.0.1/get_sks_shares".parse::<hyper::Uri>()?;
                        let host_get_sks = url_register_get_sks.host().expect("uri has no host");
                        let port_get_sks = url_register_get_sks.port_u16().unwrap_or(4000);
                        let address_get_sks = format!("{}:{}", host_get_sks, port_get_sks);
                        let stream_get_sks = TcpStream::connect(address_get_sks).await?;
                        let io_get_sks = TokioIo::new(stream_get_sks);
                        let (mut sender_get_sks, conn_get_sks) = hyper::client::conn::http1::handshake(io_get_sks).await?;
                        tokio::task::spawn(async move {
                            if let Err(err) = conn_get_sks.await {
                                println!("Connection failed: {:?}", err);
                            }
                        });
                        let authority_get_sks = url_register_get_sks.authority().unwrap().clone();
                        let response_get_sks = SKSSharePoll { response: "Get_All_SKS_Shares".to_string(), round_id: count.round_count, ciphernode_count: num_parties as u32};
                        let out_get_sks = serde_json::to_string(&response_get_sks).unwrap();
                        let req_get_sks = Request::post("http://127.0.0.1/")
                            .uri(url_register_get_sks.clone())
                            .header(hyper::header::HOST, authority_get_sks.as_str())
                            .body(out_get_sks)?;

                        let mut res_get_sks = sender_get_sks.send_request(req_get_sks).await?;
                        println!("Response status: {}", res_get_sks.status());

                        if(res_get_sks.status().to_string() == "500 Internal Server Error") {
                            println!("enclave resource failed, trying to poll for sks shares again...");
                            continue;
                        }

                        let body_bytes = res_get_sks.collect().await?.to_bytes();
                        let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
                        let shares: SKSShareResponse = serde_json::from_str(&body_str).expect("JSON was not well-formatted");

                        if(shares.response == "final") {
                            // do decrypt
                            println!("collected all of the decrypt shares!");
                            for i in 0..state.ciphernode_total {
                                decryption_shares.push(DecryptionShare::deserialize(&shares.sks_shares[i as usize], &params, tally.clone()));
                            }

                            // Again, an aggregating party aggregates the decryption shares to produce the
                            // decrypted plaintext.
                            let tally_pt = timeit!("Decryption share aggregation", {
                                let pt: Plaintext = decryption_shares.into_iter().aggregate().unwrap();
                                pt
                            });
                            let tally_vec = Vec::<u64>::try_decode(&tally_pt, Encoding::poly()).unwrap();
                            let tally_result = tally_vec[0];

                            // Show vote result
                            println!("Vote result = {} / {}", tally_result, num_voters.vote_count);

                            // report result to server
                            let option_1_total = tally_result;
                            let option_2_total = num_voters.vote_count - tally_result as u32;
                            println!("option 1 total {:?}", option_1_total);
                            println!("option 2 total {:?}", option_2_total);

                            let url_report = "http://127.0.0.1/report_tally".parse::<hyper::Uri>()?;
                            let host_report = url_report.host().expect("uri has no host");
                            let port_report = url_report.port_u16().unwrap_or(4000);
                            let address_report = format!("{}:{}", host_report, port_report);
                            let stream_report = TcpStream::connect(address_report).await?;
                            let io_report = TokioIo::new(stream_report);
                            let (mut sender_report, conn_report) = hyper::client::conn::http1::handshake(io_report).await?;
                            tokio::task::spawn(async move {
                                if let Err(err) = conn_report.await {
                                    println!("Connection failed: {:?}", err);
                                }
                            });
                            let authority_report = url_report.authority().unwrap().clone();
                            let response_report = ReportTallyRequest {
                                   round_id: state.id,
                                   option_1: option_1_total as u32,
                                   option_2: option_2_total as u32
                            };
                            let out_report = serde_json::to_string(&response_report).unwrap();
                            let req_report = Request::post("http://127.0.0.1/")
                                .uri(url_report.clone())
                                .header(hyper::header::HOST, authority_report.as_str())
                                .body(out_report)?;

                            let mut res_report = sender_report.send_request(req_report).await?;
                            println!("Response status: {}", res_report.status());
                            println!("Tally reported to enclave server");
                            //let _ = tx.send(());
                            break;
                        }

                        let polling_sks = time::Duration::from_millis(3000);
                        thread::sleep(polling_sks);
                    }
                    break;
                }
                let polling_end_round = time::Duration::from_millis(6000);
                thread::sleep(polling_end_round);           
            }
        }

        // Polling time to server...
        let polling_wait = time::Duration::from_millis(6000);
        thread::sleep(polling_wait);
    }
    Ok(())
}

async fn get_votes_contract(round_id: u32, block_start: U64, address: String, chain_id: u32) -> Vec<Vec<u8>> {
    println!("Filtering contract for votes");
    // chain state
    let infura_key = "INFURAKEY";
    let infura_val = env::var(infura_key).unwrap();
    let mut RPC_URL = "https://sepolia.infura.io/v3/".to_string();
    RPC_URL.push_str(&infura_val);
    let provider = Provider::<Http>::try_from(RPC_URL.clone()).unwrap();
    //let block_number: U64 = provider.get_block_number().await.unwrap();
    //println!("Current block height is {:?}", block_number);
    abigen!(
        IVOTE,
        r#"[
            function tester() external view returns (string)
            function id() external view returns (uint256)
            function voteEncrypted(bytes memory encVote) public
            event Voted(address indexed voter, bytes vote)
        ]"#,
    );
    let provider = Provider::<Http>::try_from(RPC_URL.clone()).unwrap();
    let contract_address = address.parse::<Address>().unwrap();
    let eth_key = "PRIVATEKEY";
    let eth_val = env::var(eth_key).unwrap();
    let wallet: LocalWallet = eth_val
        .parse::<LocalWallet>().unwrap()
        .with_chain_id(chain_id as u64);
    let client = Arc::new(provider);
    let contract = IVOTE::new(contract_address, Arc::new(client.clone()));

    // let event = contract.event::<ValueChanged>()?;

    // let watcher = event.watcher().from_block(5).to_block(10);

    let events = contract.events().from_block(block_start).query().await.unwrap();
    //println!("{:?}", events);

    // let filter = Filter::new()
    //     .address(contract_address)
    //     .event("Voted(address,bytes)")
    //     .from_block(block_start);
    // let logs = client.get_logs(&filter).await.unwrap();
    let mut votes_encrypted = Vec::with_capacity(events.len());
    for event in events.iter() {
        votes_encrypted.push(event.vote.to_vec());
    }
    votes_encrypted
}

async fn poll_contract(round_id: u32, cnode_id: u32, rx: std::sync::mpsc::Receiver<()>) {
    println!("Polling contract for votes");
    // chain state
    let infura_key = "INFURAKEY";
    let infura_val = env::var(infura_key).unwrap();
    let mut RPC_URL = "https://sepolia.infura.io/v3/".to_string();
    RPC_URL.push_str(&infura_val);
    let provider = Provider::<Http>::try_from(RPC_URL.clone()).unwrap();
    let block_number: U64 = provider.get_block_number().await.unwrap();
    println!("Current block height is {:?}", block_number);
    abigen!(
        IVOTE,
        r#"[
            function tester() external view returns (string)
            function id() external view returns (uint256)
            function voteEncrypted(bytes memory encVote) public
            event Voted(address indexed voter, bytes vote)
        ]"#,
    );
    let provider = Provider::<Http>::try_from(RPC_URL.clone()).unwrap();
    let contract_address = "0x51Ec8aB3e53146134052444693Ab3Ec53663a12B".parse::<Address>().unwrap();
    let eth_key = "PRIVATEKEY";
    let eth_val = env::var(eth_key).unwrap();
    let wallet: LocalWallet = eth_val
        .parse::<LocalWallet>().unwrap()
        .with_chain_id(11155111 as u64);
    let client = Arc::new(provider);
    let contract = IVOTE::new(contract_address, Arc::new(client.clone()));
    let events = contract.events().from_block(5560945);//.to_block(5560955);

    // let token_topics = [
    //     H256::from(USDC_ADDRESS.parse::<H160>()?),
    //     H256::from(USDT_ADDRESS.parse::<H160>()?),
    //     H256::from(DAI_ADDRESS.parse::<H160>()?),
    // ];
    let filter = Filter::new()
        .address(contract_address)
        .event("Voted(address,bytes)")
        // .topic1(H256::from("address indexed voter".parse::<H160>().unwrap()))
        // .topic2(token_topics.to_vec())
        .from_block(5777125);
    let logs = client.get_logs(&filter).await.unwrap();
    for log in logs.iter() {
        let token0 = log.topics.clone();
        //let token1 = Address::from(log.topics[2]);
        //let fee_tier = U256::from_big_endian(&log.topics[3].as_bytes()[29..32]);
        //let tick_spacing = U256::from_big_endian(&log.data[29..32]);
        //let pool = Address::from(&log.data[44..64].try_into()?);
        println!("{:?}", log.data);
    }

    //TODO: scan blocks since round start block to get any votes missed if crashed
    // let mut stream = events.stream().await.unwrap().with_meta().take(10);

    // while let Some(Ok((event, meta))) = stream.next().await {
    //     println!("New vote event received");
    //     println!("voter: {:?}", event.voter);
    //     println!(
    //         r#"
    //            address: {:?}, 
    //            block_number: {:?}, 
    //            block_hash: {:?}, 
    //            transaction_hash: {:?}, 
    //            transaction_index: {:?}, 
    //            log_index: {:?}
    //         "#,
    //         meta.address,
    //         meta.block_number,
    //         meta.block_hash,
    //         meta.transaction_hash,
    //         meta.transaction_index,
    //         meta.log_index
    //     );

    //     let mut key = round_id.to_string();
    //     key.push_str("-");
    //     key.push_str(&cnode_id.to_string());
    //     key.push_str("-ciphernode-storage");
    //     let votes = GLOBAL_DB.get(key.clone()).unwrap();
    //     if(votes == None) {
    //         println!("initializing first vote in db");
    //         let data = RoundData {
    //             round_id: round_id,
    //             encrypted_votes: vec![event.vote.to_vec()],
    //         };
    //         let data_str = serde_json::to_string(&data).unwrap();
    //         let data_bytes = data_str.into_bytes();
    //         GLOBAL_DB.insert(key, data_bytes).unwrap();
    //     } else {
    //         let votes_str = votes.unwrap();
    //         let votes_out_str = str::from_utf8(&votes_str).unwrap();
    //         let mut votes_out_struct: RoundData = serde_json::from_str(&votes_out_str).unwrap();
    //         votes_out_struct.encrypted_votes.push(event.vote.to_vec());
    //         let votes_in_str = serde_json::to_string(&votes_out_struct).unwrap();
    //         let votes_in_bytes = votes_in_str.into_bytes();
    //         GLOBAL_DB.insert(key, votes_in_bytes);
    //     }
    //     match rx.try_recv() {
    //         Ok(_) | Err(TryRecvError::Disconnected) => {
    //             println!("Terminating.");
    //             break;
    //         }
    //         Err(TryRecvError::Empty) => {}
    //     }
    //     // votes_encrypted.push(deserialized_vote);
    // }
}
