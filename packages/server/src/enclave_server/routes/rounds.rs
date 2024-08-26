use chrono::Utc;
use fhe::{bfv::BfvParametersBuilder, mbfv::CommonRandomPoly};
use fhe_traits::Serialize;
use iron::mime::Mime;
use iron::prelude::*;
use iron::status;
use rand::thread_rng;
use router::Router;
use std::env;
use std::io::Read;

use ethers::{
    providers::{Http, Middleware, Provider},
    types::U64,
};

use crate::util::timeit::timeit;

use crate::enclave_server::database::{generate_emoji, get_state, GLOBAL_DB};
use crate::enclave_server::models::{
    Ciphernode, CrispConfig, JsonResponse, PollLengthRequest, ReportTallyRequest, Round,
    RoundCount, TimestampRequest,
};

pub fn setup_routes(router: &mut Router) {
    router.get("/get_rounds", get_rounds, "get_rounds");
    router.post("/init_crisp_round", init_crisp_round, "init_crisp_round");
    router.post(
        "/get_start_time_by_round",
        get_start_time_by_round,
        "get_start_time_by_round",
    );
    router.post(
        "/get_poll_length_by_round",
        get_poll_length_by_round,
        "get_poll_length_by_round",
    );
    router.post("/report_tally", report_tally, "report_tally");
}

fn get_rounds(_req: &mut Request) -> IronResult<Response> {
    //let test = _req.headers.get::<iron::headers::ContentType>().unwrap();
    //println!("content_type: {:?}", test);

    // let test3 = _req.headers.get::<iron::headers::Authorization<Bearer>>().unwrap();
    // println!("auth: {:?}", test3.token);
    // let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
    // let claims: BTreeMap<String, String> = test3.token.verify_with_key(&key).unwrap();
    // println!("decoded hmac {:?}", claims);

    //let test2 = _req.headers.get::<iron::headers::UserAgent>();
    //println!("user agent: {:?}", test2);

    let key = "round_count";
    let mut round = GLOBAL_DB.get(key).unwrap();
    if round == None {
        println!("initializing first round in db");
        GLOBAL_DB.insert(key, b"0".to_vec()).unwrap();
        round = GLOBAL_DB.get(key).unwrap();
    }
    let round_key = std::str::from_utf8(round.unwrap().as_ref())
        .unwrap()
        .to_string();
    let round_int = round_key.parse::<u32>().unwrap();

    let count = RoundCount {
        round_count: round_int,
    };
    println!("round_count: {:?}", count.round_count);

    let out = serde_json::to_string(&count).unwrap();
    println!("get rounds hit");

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}

#[tokio::main]
async fn init_crisp_round(req: &mut Request) -> IronResult<Response> {
    // let auth = _req.headers.get::<iron::headers::Authorization<Bearer>>().unwrap();
    // if auth.token != env {

    // }
    println!("generating round crp");

    let infura_val = env!("INFURAKEY");
    let mut rpc_url = "https://sepolia.infura.io/v3/".to_string();
    rpc_url.push_str(&infura_val);

    let provider = Provider::<Http>::try_from(rpc_url.clone()).unwrap();
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
            .build_arc()
            .unwrap()
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
    if round == None {
        println!("initializing first round in db");
        GLOBAL_DB.insert(key, b"0".to_vec()).unwrap();
    }
    let round_key = std::str::from_utf8(round.unwrap().as_ref())
        .unwrap()
        .to_string();
    let mut round_int = round_key.parse::<u32>().unwrap();
    round_int = round_int + 1;
    let mut inc_round_key = round_int.to_string();
    inc_round_key.push_str("-storage");
    println!(
        "Database key is {:?} and round int is {:?}",
        inc_round_key, round_int
    );

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
        ciphernodes: vec![Ciphernode {
            id: 0,
            pk_share: vec![0],
            sks_share: vec![0],
        }],
        has_voted: vec!["".to_string()],
    };

    let state_str = serde_json::to_string(&state).unwrap();
    let state_bytes = state_str.into_bytes();
    let key2 = round_int.to_string();
    GLOBAL_DB.insert(inc_round_key, state_bytes).unwrap();

    let new_round_bytes = key2.into_bytes();
    GLOBAL_DB.insert(key, new_round_bytes).unwrap();

    // create a response with our random string, and pass in the string from the POST body
    let response = JsonResponse {
        response: "CRISP Initiated".to_string(),
    };
    let out = serde_json::to_string(&response).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}

fn get_start_time_by_round(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    // read the POST body
    req.body.read_to_string(&mut payload).unwrap();
    let mut incoming: TimestampRequest = serde_json::from_str(&payload).unwrap();
    println!("Request start time for round {:?}", incoming.round_id);

    let (state, _key) = get_state(incoming.round_id);
    incoming.timestamp = state.start_time;
    let out = serde_json::to_string(&incoming).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}

fn get_poll_length_by_round(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    // read the POST body
    req.body.read_to_string(&mut payload).unwrap();
    let mut incoming: PollLengthRequest = serde_json::from_str(&payload).unwrap();
    println!("Request poll length for round {:?}", incoming.round_id);

    let (state, _key) = get_state(incoming.round_id);
    incoming.poll_length = state.poll_length;
    let out = serde_json::to_string(&incoming).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}

fn report_tally(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    // read the POST body
    req.body.read_to_string(&mut payload).unwrap();
    let incoming: ReportTallyRequest = serde_json::from_str(&payload).unwrap();
    println!("Request report tally for round {:?}", incoming.round_id);

    let (mut state, key) = get_state(incoming.round_id);
    if state.votes_option_1 == 0 && state.votes_option_2 == 0 {
        state.votes_option_1 = incoming.option_1;
        state.votes_option_2 = incoming.option_2;

        let state_str = serde_json::to_string(&state).unwrap();
        let state_bytes = state_str.into_bytes();
        GLOBAL_DB.insert(key, state_bytes).unwrap();
    }
    let response = JsonResponse {
        response: "Tally Reported".to_string(),
    };
    let out = serde_json::to_string(&response).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}
