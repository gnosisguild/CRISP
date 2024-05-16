mod util;

use std::{env, sync::Arc, fs, str};
use std::fs::File;
use std::io::Read;
use chrono::{Utc};
use fhe::{
    bfv::{BfvParametersBuilder, Ciphertext, Encoding, Plaintext, SecretKey},
    mbfv::{AggregateIter, CommonRandomPoly, DecryptionShare, PublicKeyShare},
};
use fhe_traits::{FheDecoder, Serialize as FheSerialize, DeserializeParametrized};
use rand::{Rng, rngs::OsRng, thread_rng};
use util::timeit::{timeit};
use serde::{Deserialize, Serialize};
use http_body_util::Empty;
use hyper::Request;
use hyper::Method;

use hyper_tls::HttpsConnector;
use hyper_util::{client::legacy::Client as HyperClient, rt::TokioExecutor};
use bytes::Bytes;
use headers::Authorization;

use http_body_util::BodyExt;
use tokio::io::{AsyncWriteExt as _, self};

use std::{thread, time};

use ethers::{
    providers::{Http, Provider},
    types::{Address, U64},
    contract::abigen,
};

use sled::Db;
use once_cell::sync::Lazy;

use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;

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
    index: u32,
    round_id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct CrispConfig {
    round_id: u32,
    poll_length: u32,
    chain_id: u32,
    voting_address: String,
    ciphernode_count: u32,
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
struct CiphernodeConfig {
    ids: Vec<u32>,
    enclave_address: String,
    enclave_port: u16,
}

#[derive(Debug, Deserialize, Serialize)]
struct GetCiphernode {
    round_id: u32,
    ciphernode_id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct GetEligibilityRequest {
    round_id: u32,
    node_id: u32,
    is_eligible: bool,
    reason: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Ciphernode {
    id: u32,
    index : Vec<u32>,
    pk_shares: Vec<Vec<u8>>,
    sk_shares: Vec<Vec<i64>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct RegisterNodeResponse {
    response: String,
    node_index: u32,
}

static GLOBAL_DB: Lazy<Db> = Lazy::new(|| {
    let path = env::current_dir().unwrap();
    let mut pathst = path.display().to_string();
    pathst.push_str("/example_ciphernode_config.json");
    let mut file = File::open(pathst.clone()).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let args: Vec<String> = env::args().collect();
    let mut cnode_selector = 0;
    if args.len() != 1 {
        cnode_selector = args[1].parse::<usize>().unwrap();
    };

    let mut config: CiphernodeConfig = serde_json::from_str(&data).expect("JSON was not well-formatted");
    let node_id: u32;
    if(config.ids.len() - 1) < cnode_selector {
        println!("generating new ciphernode...");
        node_id = rand::thread_rng().gen_range(0..100000);
        config.ids.push(node_id);

        let configfile = serde_json::to_string(&config).unwrap();
        fs::write(pathst.clone(), configfile).unwrap();
    } else if config.ids[cnode_selector] == 0 {
        println!("generating initial ciphernode id...");
        node_id = rand::thread_rng().gen_range(0..100000);
        config.ids[cnode_selector as usize] = node_id;
        
        let configfile = serde_json::to_string(&config).unwrap();
        fs::write(pathst.clone(), configfile).unwrap();
    } else {
        println!("Using ciphernode id {:?}", config.ids[cnode_selector]);
        node_id = config.ids[cnode_selector];
    };

    let pathdb = env::current_dir().unwrap();
    let mut pathdbst = pathdb.display().to_string();
    pathdbst.push_str("/database/ciphernode-");
    pathdbst.push_str(&node_id.to_string());
    println!("Node database path {:?}", pathdbst);
    sled::open(pathdbst.clone()).unwrap()
});

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Getting configuration file.");
    let path = env::current_dir().unwrap();
    let mut pathst = path.display().to_string();
    pathst.push_str("/example_ciphernode_config.json");
    let mut file = File::open(pathst.clone()).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let mut config: CiphernodeConfig = serde_json::from_str(&data).expect("JSON was not well-formatted");
    let args: Vec<String> = env::args().collect();
    let mut cnode_selector = 0;
    if args.len() != 1 {
        cnode_selector = args[1].parse::<usize>().unwrap();
    };

    if(config.ids.len() - 1) < cnode_selector {
        println!("generating new ciphernode...");
        let new_id = rand::thread_rng().gen_range(0..100000);
        config.ids.push(new_id);

        let configfile = serde_json::to_string(&config).unwrap();
        fs::write(pathst.clone(), configfile).unwrap();
    }

    let node_id: u32 = config.ids[cnode_selector as usize];
    println!("Node ID: {:?} selected.", node_id);
    let pathdb = env::current_dir().unwrap();
    let mut pathdbst = pathdb.display().to_string();
    pathdbst.push_str("/database/ciphernode-");
    pathdbst.push_str(&node_id.to_string());
    pathdbst.push_str("-state");

    let node_state_bytes = GLOBAL_DB.get(pathdbst.clone()).unwrap();
    if node_state_bytes == None {
        println!("Initializing node state in database.");
        let state = Ciphernode {
            id: node_id,
            index: vec![0],
            pk_shares: vec![vec![0]], // indexed by round id - 1
            sk_shares: vec![vec![0]],
        };

        let state_str = serde_json::to_string(&state).unwrap();
        let state_bytes = state_str.into_bytes();
        GLOBAL_DB.insert(pathdbst.clone(), state_bytes).unwrap();
    }

    let degree = 4096;
    let plaintext_modulus: u64 = 4096;
    let moduli = vec![0xffffee001, 0xffffc4001, 0x1ffffe0001];

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
        println!("Polling Enclave server.");
        let https = HttpsConnector::new();
        let client_get = HyperClient::builder(TokioExecutor::new()).build::<_, Empty<Bytes>>(https.clone());
        let client = HyperClient::builder(TokioExecutor::new()).build::<_, String>(https);

        let mut url_get_rounds_str = config.enclave_address.clone();
        url_get_rounds_str.push_str("/get_rounds");

        let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret")?;
        let mut claims = BTreeMap::new();
        claims.insert("sub", "someone");
        let mut bearer_str = "Bearer ".to_string();
        let token_str = claims.sign_with_key(&key)?;
        bearer_str.push_str(&token_str);
        println!("{:?}", bearer_str);

        let req = Request::builder()
            //.header("authorization", bearer_str)
            .method(Method::GET)
            .uri(url_get_rounds_str)
            .body(Empty::<Bytes>::new())?;

        let resp = client_get.request(req).await?;

        let body_bytes = resp.collect().await?.to_bytes();
        let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
        println!("Get Round Response {:?}", body_str);
        let count: RoundCount = serde_json::from_str(&body_str).expect("JSON was not well-formatted");
        println!("Server Round Count: {:?}", count.round_count);
        println!("Internal Round Count: {:?}", internal_round_count.round_count);

        // Check to see if the server reported a new round
        if count.round_count > internal_round_count.round_count {
            println!("Getting New Round State.");

            let response_get_state = GetRoundRequest { round_id: count.round_count };
            let out = serde_json::to_string(&response_get_state).unwrap();
            let mut url_get_state = config.enclave_address.clone();
            url_get_state.push_str("/get_round_state_lite");
            let req = Request::builder()
                .method(Method::POST)
                .uri(url_get_state)
                .body(out)?;

            let resp = client.request(req).await?;
            println!("Get Round State Response status: {}", resp.status());
            let body_bytes = resp.collect().await?.to_bytes();
            let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();

            let state: StateLite = serde_json::from_str(&body_str).expect("JSON was not well-formatted");

            let get_eligibility = GetEligibilityRequest {
                round_id: count.round_count,
                node_id: node_id,
                is_eligible: false,
                reason: "".to_string(),
            };
            let out = serde_json::to_string(&get_eligibility).unwrap();
            let mut url_get_eligibility = config.enclave_address.clone();
            url_get_eligibility.push_str("/get_round_eligibility");
            let req = Request::builder()
                .method(Method::POST)
                .uri(url_get_eligibility)
                .body(out)?;

            let resp = client.request(req).await?;
            println!("Get Eligibility Response status: {}", resp.status());
            let body_bytes = resp.collect().await?.to_bytes();
            let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();

            let eligibility: GetEligibilityRequest = serde_json::from_str(&body_str).expect("JSON was not well-formatted");
            println!("Ciphernode eligibility: {:?}", eligibility.reason);

            if eligibility.is_eligible == false && eligibility.reason == "Waiting For New Round" {
                internal_round_count.round_count = count.round_count;
                continue
            };

            if eligibility.is_eligible == true && eligibility.reason == "Open Node Spot" {
                // do registration
                println!("Generating PK share and serializing.");

                // deserialize crp_bytes
                let crp = CommonRandomPoly::deserialize(&state.crp, &params).unwrap();
                let sk_share_1 = SecretKey::random(&params, &mut OsRng);
                let pk_share_1 = PublicKeyShare::new(&sk_share_1, crp.clone(), &mut thread_rng())?;
                // serialize pk_share
                let pk_share_bytes = pk_share_1.to_bytes();
                let sk_share_bytes = sk_share_1.coeffs.into_vec();
                let (mut node_state, db_key) = get_state(node_id);

                // overwrite old shares each new round
                node_state.pk_shares[0] = pk_share_bytes.clone();
                node_state.sk_shares[0] = sk_share_bytes;

                let response_key = PKShareRequest {
                    response: "Register Ciphernode Key".to_string(),
                    pk_share: pk_share_bytes,
                    id: node_id,
                    round_id: state.id
                };
                let out = serde_json::to_string(&response_key).unwrap();
                let mut url_register_keyshare = config.enclave_address.clone();
                url_register_keyshare.push_str("/register_ciphernode");
                let req = Request::builder()
                    .method(Method::POST)
                    .uri(url_register_keyshare)
                    .body(out)?;

                let resp = client.request(req).await?;
                println!("Register Node Response status: {}", resp.status());
                let body_bytes = resp.collect().await?.to_bytes();
                let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();

                let registration_res: RegisterNodeResponse = serde_json::from_str(&body_str).expect("JSON was not well-formatted");
                println!("Ciphernode index: {:?}", registration_res.node_index);
 
                // index is the order in which this node registered with the server
                node_state.index[0] = registration_res.node_index;

                let state_str = serde_json::to_string(&node_state).unwrap();
                let state_bytes = state_str.into_bytes();
                let _ = GLOBAL_DB.insert(db_key, state_bytes);
                internal_round_count.round_count = count.round_count;
                start_contract_watch(&state, node_id, &config).await;
            };

            if eligibility.is_eligible == true && eligibility.reason == "Previously Registered" {
                println!("Server reported to resume watching.");
                internal_round_count.round_count = count.round_count;
                start_contract_watch(&state, node_id, &config).await;
            };
            if eligibility.is_eligible == false && eligibility.reason == "Round Full" {
                println!("Server reported round full, wait for next round.");
                internal_round_count.round_count = count.round_count;
                continue
            };
        }

        // Polling time to server...
        let polling_wait = time::Duration::from_millis(6000);
        thread::sleep(polling_wait);
    }
}

fn get_state(node_id: u32) -> (Ciphernode, String) {
    let pathdb = env::current_dir().unwrap();
    let mut pathdbst = pathdb.display().to_string();
    pathdbst.push_str("/database/ciphernode-");
    pathdbst.push_str(&node_id.to_string());
    pathdbst.push_str("-state");
    println!("Database key is {:?}", pathdbst);
    let state_out = GLOBAL_DB.get(pathdbst.clone()).unwrap().unwrap();
    let state_out_str = str::from_utf8(&state_out).unwrap();
    let state_out_struct: Ciphernode = serde_json::from_str(&state_out_str).unwrap();
    (state_out_struct, pathdbst)
}

async fn get_votes_contract(block_start: U64, address: String, _chain_id: u32) -> Vec<Vec<u8>> {
    println!("Filtering contract for votes");
    // chain state
    let infura_key = "INFURAKEY";
    let infura_val = env::var(infura_key).unwrap();
    let mut rpc_url = "https://sepolia.infura.io/v3/".to_string();
    rpc_url.push_str(&infura_val);

    abigen!(
        IVOTE,
        r#"[
            function tester() external view returns (string)
            function id() external view returns (uint256)
            function voteEncrypted(bytes memory encVote) public
            event Voted(address indexed voter, bytes vote)
        ]"#,
    );
    let provider = Provider::<Http>::try_from(rpc_url.clone()).unwrap();
    let contract_address = address.parse::<Address>().unwrap();
    let client = Arc::new(provider);
    let contract = IVOTE::new(contract_address, Arc::new(client.clone()));

    let events = contract.events().from_block(block_start).query().await.unwrap();

    let mut votes_encrypted = Vec::with_capacity(events.len());
    for event in events.iter() {
        votes_encrypted.push(event.vote.to_vec());
    }
    votes_encrypted
}

async fn start_contract_watch(state: &StateLite, node_id: u32, config: &CiphernodeConfig) {
    let degree = 4096;
    let plaintext_modulus: u64 = 4096;
    let moduli = vec![0xffffee001, 0xffffc4001, 0x1ffffe0001];

    // Generate the BFV parameters structure.
    let params = timeit!(
        "Parameters generation",
        BfvParametersBuilder::new()
            .set_degree(degree)
            .set_plaintext_modulus(plaintext_modulus)
            .set_moduli(&moduli)
            .build_arc().unwrap()
    );

    let https = HttpsConnector::new();
    let client = HyperClient::builder(TokioExecutor::new()).build::<_, String>(https);

    let num_parties = state.ciphernode_total;

    // For each voting round this node is participating in, check the contracts for vote events.
    // When voting is finalized, begin group decrypt process

    // let (tx, rx) = mpsc::channel::<()>();
    // let rt = Runtime::new().unwrap();
    // //thread::spawn(move || {
    //     rt.spawn(async move { poll_contract(state.id, node_id, rx).await });
    // //});

    // TODO: move to thread so main loop can continue to look for more work
    loop {
        println!("Waiting for round {:?} poll to end.", state.id);
        let now = Utc::now();
        let internal_time = now.timestamp();
        if (state.start_time + state.poll_length as i64) < internal_time {
            print!("poll time ended... performing fhe computation");

            let response_get_voters = VoteCountRequest { round_id: state.id, vote_count: 0 };
            let out = serde_json::to_string(&response_get_voters).unwrap();
            let mut url_get_voters = config.enclave_address.clone();
            url_get_voters.push_str("/get_vote_count_by_round");
            let req = Request::builder()
                .method(Method::POST)
                .uri(url_get_voters)
                .body(out).unwrap();

            let resp = client.request(req).await.unwrap();
            println!("Get Vote Count Response status: {}", resp.status());

            let body_bytes_get_voters = resp.collect().await.unwrap().to_bytes();
            let body_str_get_voters = String::from_utf8(body_bytes_get_voters.to_vec()).unwrap();
            let num_voters: VoteCountRequest = serde_json::from_str(&body_str_get_voters).expect("JSON was not well-formatted");

            let votes_collected = get_votes_contract(state.block_start, state.voting_address.clone(), state.chain_id).await;
            println!("All votes collected? {:?}", num_voters.vote_count == votes_collected.len() as u32);

            if votes_collected.len() == 0 {
                println!("Vote result = {} / {}", 0, num_voters.vote_count);

                let response_report = ReportTallyRequest {
                       round_id: state.id,
                       option_1: 0,
                       option_2: 0
                };
                let out = serde_json::to_string(&response_report).unwrap();
                let mut url_report = config.enclave_address.clone();
                url_report.push_str("/report_tally");
                let req = Request::builder()
                    .method(Method::POST)
                    .uri(url_report)
                    .body(out).unwrap();

                let resp = client.request(req).await.unwrap();
                println!("Tally Reported Response status: {}", resp.status());
                break;
            }

            let tally = timeit!("Vote tallying", {
                let mut sum = Ciphertext::zero(&params);
                for i in 0..(votes_collected.len()) {
                    let deserialized_vote = Ciphertext::from_bytes(&votes_collected[i as usize], &params).unwrap();
                    sum += &deserialized_vote;
                }
                Arc::new(sum)
            });

            // The result of a vote is typically public, so in this scenario the parties can
            // perform a collective decryption. If instead the result of the computation
            // should be kept private, the parties could collectively perform a
            // keyswitch to a different public key.
            let mut decryption_shares = Vec::with_capacity(state.ciphernode_total as usize);
            let (node_state, _db_key) = get_state(node_id);
            let sk_share_coeff_bytes = node_state.sk_shares[0].clone();
            let sk_share_1 = SecretKey::new(sk_share_coeff_bytes, &params);

            let sh = DecryptionShare::new(&sk_share_1, &tally, &mut thread_rng()).unwrap();
            let sks_bytes = sh.to_bytes();

            let response_sks = SKSShareRequest {
                response: "Register_SKS_Share".to_string(),
                sks_share: sks_bytes,
                index: node_state.index[0], // index of stored pk shares on server
                round_id: state.id
            };
            let out = serde_json::to_string(&response_sks).unwrap();
            let mut url_register_sks = config.enclave_address.clone();
            url_register_sks.push_str("/register_sks_share");
            let req = Request::builder()
                .method(Method::POST)
                .uri(url_register_sks)
                .body(out).unwrap();

            let mut resp = client.request(req).await.unwrap();
            println!("Register SKS Response status: {}", resp.status());

            // Stream the body, writing each frame to stdout as it arrives
            while let Some(next) = resp.frame().await {
                let frame = next.unwrap();
                if let Some(chunk) = frame.data_ref() {
                    io::stdout().write_all(chunk).await.unwrap();
                }
            }

            // poll the enclave server to get all sks shares.
            loop {
                let response_get_sks = SKSSharePoll { 
                    response: "Get_All_SKS_Shares".to_string(),
                    round_id: state.id,
                    ciphernode_count: num_parties as u32
                };
                let out = serde_json::to_string(&response_get_sks).unwrap();
                let mut url_register_get_sks = config.enclave_address.clone();
                url_register_get_sks.push_str("/get_sks_shares");
                let req = Request::builder()
                    .method(Method::POST)
                    .uri(url_register_get_sks)
                    .body(out).unwrap();

                let resp = client.request(req).await.unwrap();
                println!("Get All SKS Response status: {}", resp.status());

                if resp.status().to_string() == "500 Internal Server Error" {
                    println!("enclave resource failed, trying to poll for sks shares again...");
                    continue;
                }

                let body_bytes = resp.collect().await.unwrap().to_bytes();
                let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
                let shares: SKSShareResponse = serde_json::from_str(&body_str).expect("JSON was not well-formatted");

                if shares.response == "final" {
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

                    let response_report = ReportTallyRequest {
                           round_id: state.id,
                           option_1: option_1_total as u32,
                           option_2: option_2_total as u32
                    };
                    let out = serde_json::to_string(&response_report).unwrap();
                    let mut url_report = config.enclave_address.clone();
                    url_report.push_str("/report_tally");
                    let req = Request::builder()
                        .method(Method::POST)
                        .uri(url_report)
                        .body(out).unwrap();

                    let resp = client.request(req).await.unwrap();
                    println!("Tally Reported Response status: {}", resp.status());
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
