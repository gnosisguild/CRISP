use std::{env, sync::Arc, fs, str, thread, time};
use std::fs::File;
use std::io::{Read, Write};
use chrono::Utc;
use fhe::{
    bfv::{BfvParametersBuilder, Ciphertext, Encoding, Plaintext, SecretKey},
    mbfv::{AggregateIter, CommonRandomPoly, DecryptionShare, PublicKeyShare},
};
use fhe_traits::{FheDecoder, Serialize as FheSerialize, DeserializeParametrized};
use rand::{Rng, rngs::OsRng, thread_rng};
use serde::{Deserialize, Serialize};
use http_body_util::{Empty, BodyExt};
use hyper::{Request, Method, body::Bytes};
use hyper_tls::HttpsConnector;
use hyper_util::{client::legacy::{Client as HyperClient, connect::HttpConnector}, rt::TokioExecutor};
use tokio::io::{AsyncWriteExt, self};
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
use env_logger::{Builder, Target};
use log::{LevelFilter, info};

// Constants
const DEGREE: usize = 4096;
const PLAINTEXT_MODULUS: u64 = 4096;
const MODULI: [u64; 3] = [0xffffee001, 0xffffc4001, 0x1ffffe0001];
const POLLING_INTERVAL: u64 = 6000;

type HyperClientGet = HyperClient<HttpsConnector<HttpConnector>, Empty<Bytes>>;
type HyperClientPost = HyperClient<HttpsConnector<HttpConnector>, String>;
// Structs
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
    ciphernode_total: u32,
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
    index: Vec<u32>,
    pk_shares: Vec<Vec<u8>>,
    sk_shares: Vec<Vec<i64>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct RegisterNodeResponse {
    response: String,
    node_index: u32,
}

// Global database
static GLOBAL_DB: Lazy<Db> = Lazy::new(|| {
    let path = env::current_dir().unwrap();
    let config_path = format!("{}/example_ciphernode_config.json", path.display());
    let mut file = File::open(&config_path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let args: Vec<String> = env::args().collect();
    let cnode_selector = args.get(1).map(|arg| arg.parse::<usize>().unwrap()).unwrap_or(0);

    let mut config: CiphernodeConfig = serde_json::from_str(&data).expect("JSON was not well-formatted");
    let node_id = if config.ids.len() <= cnode_selector {
        info!("Generating new ciphernode...");
        let new_id = rand::thread_rng().gen_range(0..100000);
        config.ids.push(new_id);
        new_id
    } else if config.ids[cnode_selector] == 0 {
        info!("Generating initial ciphernode id...");
        let new_id = rand::thread_rng().gen_range(0..100000);
        config.ids[cnode_selector] = new_id;
        new_id
    } else {
        info!("Using ciphernode id {:?}", config.ids[cnode_selector]);
        config.ids[cnode_selector]
    };

    let config_file = serde_json::to_string(&config).unwrap();
    fs::write(&config_path, config_file).unwrap();

    let db_path = format!("{}/database/ciphernode-{}", path.display(), node_id);
    info!("Node database path {:?}", db_path);
    sled::open(db_path).unwrap()
});

fn init_logger() {
    Builder::new()
        .target(Target::Stdout)
        .filter(None, LevelFilter::Info)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}:{}] - {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_logger();

    let config = load_config()?;
    let node_id = get_node_id(&config);

    let params = generate_bfv_params();

    let mut internal_round_count = RoundCount { round_count: 0 };

    loop {
        info!("Polling Enclave server.");
        let https = HttpsConnector::new();
        let client_get = HyperClient::builder(TokioExecutor::new()).build::<_, Empty<Bytes>>(https.clone());
        let client = HyperClient::builder(TokioExecutor::new()).build::<_, String>(https);

        let count = get_round_count(&client_get, &config).await?;

        if count.round_count > internal_round_count.round_count {
            handle_new_round(&client, &config, &count, node_id, &params).await?;
        }

        thread::sleep(time::Duration::from_millis(POLLING_INTERVAL));
    }
}

fn load_config() -> Result<CiphernodeConfig, Box<dyn std::error::Error + Send + Sync>>{
    let path = env::current_dir()?;
    let config_path = format!("{}/example_ciphernode_config.json", path.display());
    let mut file = File::open(config_path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    let config: CiphernodeConfig = serde_json::from_str(&data)?;
    Ok(config)
}

fn get_node_id(config: &CiphernodeConfig) -> u32 {
    let args: Vec<String> = env::args().collect();
    let cnode_selector = args.get(1).map(|arg| arg.parse::<usize>().unwrap()).unwrap_or(0);
    config.ids[cnode_selector]
}

fn generate_bfv_params() -> Arc<fhe::bfv::BfvParameters> {
    BfvParametersBuilder::new()
        .set_degree(DEGREE)
        .set_plaintext_modulus(PLAINTEXT_MODULUS)
        .set_moduli(&MODULI)
        .build_arc()
        .unwrap()
}

async fn get_round_count(client: &HyperClientGet, config: &CiphernodeConfig) -> Result<RoundCount, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!("{}/get_rounds", config.enclave_address);
    let req = Request::builder()
        .method(Method::GET)
        .uri(url)
        .body(Empty::<Bytes>::new())?;

    let resp = client.request(req).await?;
    let body_bytes = resp.collect().await?.to_bytes();
    let body_str = String::from_utf8(body_bytes.to_vec())?;
    let count: RoundCount = serde_json::from_str(&body_str)?;
    info!("Server Round Count: {:?}", count.round_count);
    Ok(count)
}

async fn handle_new_round(
    client: &HyperClientPost,
    config: &CiphernodeConfig,
    count: &RoundCount,
    node_id: u32,
    params: &Arc<fhe::bfv::BfvParameters>
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Getting New Round State.");
    let state = get_round_state(client, config, count.round_count).await?;
    let eligibility = get_round_eligibility(client, config, count.round_count, node_id).await?;

    match (eligibility.is_eligible, eligibility.reason.as_str()) {
        (false, "Waiting For New Round") => {
            // Do nothing
        },
        (true, "Open Node Spot") => {
            register_ciphernode(client, config, &state, node_id, params).await?;
            start_contract_watch(&state, node_id, config).await;
        },
        (true, "Previously Registered") => {
            start_contract_watch(&state, node_id, config).await;
        },
        (false, "Round Full") => {
            info!("Server reported round full, wait for next round.");
        },
        _ => {
            info!("Unknown eligibility status: {:?}", eligibility);
        }
    }

    Ok(())
}

async fn get_round_state(
    client: &HyperClientPost,
    config: &CiphernodeConfig,
    round_id: u32
) -> Result<StateLite, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!("{}/get_round_state_lite", config.enclave_address);
    let body = serde_json::to_string(&GetRoundRequest { round_id })?;
    let req = Request::builder()
        .header("Content-Type", "application/json")
        .method(Method::POST)
        .uri(url)
        .body(body)?;

    let resp = client.request(req).await?;
    let body_bytes = resp.collect().await?.to_bytes();
    let body_str = String::from_utf8(body_bytes.to_vec())?;
    let state: StateLite = serde_json::from_str(&body_str)?;
    Ok(state)
}

async fn get_round_eligibility(
    client: &HyperClientPost,
    config: &CiphernodeConfig,
    round_id: u32,
    node_id: u32
) -> Result<GetEligibilityRequest, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!("{}/get_round_eligibility", config.enclave_address);
    let body = serde_json::to_string(&GetEligibilityRequest {
        round_id,
        node_id,
        is_eligible: false,
        reason: "".to_string(),
    })?;
    let req = Request::builder()
        .header("Content-Type", "application/json")
        .method(Method::POST)
        .uri(url)
        .body(body)?;

    let resp = client.request(req).await?;
    let body_bytes = resp.collect().await?.to_bytes();
    let body_str = String::from_utf8(body_bytes.to_vec())?;
    let eligibility: GetEligibilityRequest = serde_json::from_str(&body_str)?;
    info!("Ciphernode eligibility: {:?}", eligibility.reason);
    Ok(eligibility)
}

async fn register_ciphernode(
    client: &HyperClientPost,
    config: &CiphernodeConfig,
    state: &StateLite,
    node_id: u32,
    params: &Arc<fhe::bfv::BfvParameters>
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Generating PK share and serializing.");
    let crp = CommonRandomPoly::deserialize(&state.crp,params).unwrap();
    let sk_share = SecretKey::random(params, &mut OsRng);
    let pk_share = PublicKeyShare::new(&sk_share, crp.clone(), &mut thread_rng())?;
    
    let pk_share_bytes = pk_share.to_bytes();
    let sk_share_bytes = sk_share.coeffs.into_vec();
    
    let (mut node_state, db_key) = get_state(node_id);
    
    node_state.pk_shares[0] = pk_share_bytes.clone();
    node_state.sk_shares[0] = sk_share_bytes;

    let response_key = PKShareRequest {
        response: "Register Ciphernode Key".to_string(),
        pk_share: pk_share_bytes,
        id: node_id,
        round_id: state.id
    };
    
    let url = format!("{}/register_ciphernode", config.enclave_address);
    let body = serde_json::to_string(&response_key)?;
    let req = Request::builder()
        .header("Content-Type", "application/json")
        .method(Method::POST)
        .uri(url)
        .body(body)?;

    let resp = client.request(req).await?;
    let body_bytes = resp.collect().await?.to_bytes();
    let body_str = String::from_utf8(body_bytes.to_vec())?;
    let registration_res: RegisterNodeResponse = serde_json::from_str(&body_str)?;
    
    info!("Ciphernode index: {:?}", registration_res.node_index);
    
    node_state.index[0] = registration_res.node_index;
    
    let state_str = serde_json::to_string(&node_state)?;
    let state_bytes = state_str.into_bytes();
    GLOBAL_DB.insert(db_key, state_bytes)?;
    
    Ok(())
}

async fn start_contract_watch(state: &StateLite, node_id: u32, config: &CiphernodeConfig) {
    let params = generate_bfv_params();
    let https = HttpsConnector::new();
    let client = HyperClient::builder(TokioExecutor::new()).build::<_, String>(https);

    loop {
        info!("Waiting for round {:?} poll to end.", state.id);
        let now = Utc::now();
        let internal_time = now.timestamp();
        
        if (state.start_time + state.poll_length as i64) < internal_time {
            info!("Poll time ended... performing FHE computation");
            
            match process_votes(&client, state, node_id, config, &params).await {
                Ok(_) => break,
                Err(e) => {
                    info!("Error processing votes: {:?}", e);
                    // Implement appropriate error handling or retry logic here
                }
            }
        }
        
        thread::sleep(time::Duration::from_millis(POLLING_INTERVAL));
    }
}

async fn process_votes(
    client: &HyperClientPost,
    state: &StateLite,
    node_id: u32,
    config: &CiphernodeConfig,
    params: &Arc<fhe::bfv::BfvParameters>
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let num_voters = get_vote_count(client, config, state.id).await?;
    let votes_collected = get_votes_contract(state.block_start, state.voting_address.clone(), state.chain_id).await?;
    
    info!("Votes Collected Len: {:?}", votes_collected.len());
    info!("All votes collected? {:?}", num_voters.vote_count == votes_collected.len() as u32);

    if votes_collected.is_empty() {
        report_tally(client, config, state.id, 0, 0).await?;
        return Ok(());
    }

    let tally = tally_votes(&votes_collected, params);
    let decryption_shares = collect_decryption_shares(client, config, state, node_id, &tally, params).await?;
    let tally_result = decrypt_tally(decryption_shares, &tally, params)?;

    let option_1_total = tally_result;
    let option_2_total = num_voters.vote_count - tally_result as u32;
    
    info!("Vote result = {} / {}", tally_result, num_voters.vote_count);
    info!("Option 1 total: {:?}", option_1_total);
    info!("Option 2 total: {:?}", option_2_total);

    report_tally(client, config, state.id, option_1_total as u32, option_2_total).await?;

    Ok(())
}

async fn get_vote_count(
    client: &HyperClientPost,
    config: &CiphernodeConfig,
    round_id: u32
) -> Result<VoteCountRequest, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!("{}/get_vote_count_by_round", config.enclave_address);
    let body = serde_json::to_string(&VoteCountRequest { round_id, vote_count: 0 })?;
    let req = Request::builder()
        .header("Content-Type", "application/json")
        .method(Method::POST)
        .uri(url)
        .body(body)?;

    let resp = client.request(req).await?;
    let body_bytes = resp.collect().await?.to_bytes();
    let body_str = String::from_utf8(body_bytes.to_vec())?;
    let num_voters: VoteCountRequest = serde_json::from_str(&body_str)?;
    
    info!("VoteCountRequest: {:?}", num_voters);
    Ok(num_voters)
}

async fn get_votes_contract(block_start: U64, address: String, _chain_id: u32) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error + Send + Sync>> {
    info!("Filtering contract for votes");
    
    let rpc_url = "http://127.0.0.1:8545".to_string();

    abigen!(
        IVOTE,
        r#"[
            function tester() external view returns (string)
            function id() external view returns (uint256)
            function voteEncrypted(bytes memory encVote) public
            event Voted(address indexed voter, bytes vote)
        ]"#,
    );
    
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let contract_address = address.parse::<Address>()?;
    let client = Arc::new(provider);
    let contract = IVOTE::new(contract_address, Arc::new(client.clone()));

    let events = contract.events().from_block(block_start).query().await?;

    let votes_encrypted: Vec<Vec<u8>> = events.iter().map(|event| event.vote.to_vec()).collect();
    Ok(votes_encrypted)
}

fn tally_votes(votes_collected: &[Vec<u8>], params: &Arc<fhe::bfv::BfvParameters>) -> Arc<Ciphertext> {
    let mut sum = Ciphertext::zero(params);
    for vote in votes_collected {
        let deserialized_vote = Ciphertext::from_bytes(vote, params).unwrap();
        sum += &deserialized_vote;
    }
    Arc::new(sum)
}

async fn collect_decryption_shares(
    client: &HyperClientPost,
    config: &CiphernodeConfig,
    state: &StateLite,
    node_id: u32,
    tally: &Arc<Ciphertext>,
    params: &Arc<fhe::bfv::BfvParameters>
) -> Result<Vec<DecryptionShare>, Box<dyn std::error::Error + Send + Sync>> {
    let (node_state, _) = get_state(node_id);
    let sk_share_coeff_bytes = node_state.sk_shares[0].clone();
    let sk_share = SecretKey::new(sk_share_coeff_bytes, params);

    let sh = DecryptionShare::new(&sk_share, tally, &mut thread_rng())?;
    let sks_bytes = sh.to_bytes();

    let response_sks = SKSShareRequest {
        response: "Register_SKS_Share".to_string(),
        sks_share: sks_bytes,
        index: node_state.index[0],
        round_id: state.id
    };

    let url = format!("{}/register_sks_share", config.enclave_address);
    let body = serde_json::to_string(&response_sks)?;
    let req = Request::builder()
        .header("Content-Type", "application/json")
        .method(Method::POST)
        .uri(url)
        .body(body)?;

    let mut resp = client.request(req).await?;
    info!("Register SKS Response status: {}", resp.status());

    while let Some(frame) = resp.frame().await {
        if let Some(chunk) = frame?.data_ref() {
            io::stdout().write_all(chunk).await?;
        }
    }

    let mut decryption_shares = Vec::with_capacity(state.ciphernode_total as usize);

    loop {
        let response_get_sks = SKSSharePoll { 
            response: "Get_All_SKS_Shares".to_string(),
            round_id: state.id,
            ciphernode_count: state.ciphernode_total
        };

        let url = format!("{}/get_sks_shares", config.enclave_address);
        let body = serde_json::to_string(&response_get_sks)?;
        let req = Request::builder()
            .header("Content-Type", "application/json")
            .method(Method::POST)
            .uri(url)
            .body(body)?;

        let resp = client.request(req).await?;
        info!("Get All SKS Response status: {}", resp.status());

        if resp.status().as_u16() == 500 {
            info!("Enclave resource failed, trying to poll for sks shares again...");
            continue;
        }

        let body_bytes = resp.collect().await?.to_bytes();
        let body_str = String::from_utf8(body_bytes.to_vec())?;
        let shares: SKSShareResponse = serde_json::from_str(&body_str)?;

        if shares.response == "final" {
            info!("Collected all of the decrypt shares!");
            for sks_share in shares.sks_shares {
                decryption_shares.push(DecryptionShare::deserialize(&sks_share, params, tally.clone())?);
            }
            break;
        }

        thread::sleep(time::Duration::from_millis(3000));
    }

    Ok(decryption_shares)
}

fn decrypt_tally(
    decryption_shares: Vec<DecryptionShare>,
    tally: &Arc<Ciphertext>,
    params: &Arc<fhe::bfv::BfvParameters>
) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
    let tally_pt: Plaintext = decryption_shares.into_iter().aggregate()?;
    let tally_vec = Vec::<u64>::try_decode(&tally_pt, Encoding::poly())?;
    Ok(tally_vec[0])
}

async fn report_tally(
    client: &HyperClientPost,
    config: &CiphernodeConfig,
    round_id: u32,
    option_1: u32,
    option_2: u32
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let response_report = ReportTallyRequest {
        round_id,
        option_1,
        option_2
    };
    
    let url = format!("{}/report_tally", config.enclave_address);
    let body = serde_json::to_string(&response_report)?;
    let req = Request::builder()
        .header("Content-Type", "application/json")
        .method(Method::POST)
        .uri(url)
        .body(body)?;

    let resp = client.request(req).await?;
    info!("Tally Reported Response status: {}", resp.status());
    Ok(())
}

fn get_state(node_id: u32) -> (Ciphernode, String) {
    let pathdb = env::current_dir().unwrap();
    let pathdbst = format!("{}/database/ciphernode-{}-state", pathdb.display(), node_id);
    info!("Database key is {:?}", pathdbst);
    let state_out = GLOBAL_DB.get(pathdbst.clone()).unwrap().unwrap();
    let state_out_str = str::from_utf8(&state_out).unwrap();
    let state_out_struct: Ciphernode = serde_json::from_str(state_out_str).unwrap();
    (state_out_struct, pathdbst)
}