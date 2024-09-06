use chrono::Utc;
use fhe::{bfv::BfvParametersBuilder, mbfv::CommonRandomPoly};
use fhe_traits::Serialize;
use iron::status;
use rand::thread_rng;
use std::env;
use std::io::Read;
use log::info;

use actix_web::{web, HttpResponse, Responder};

use ethers::{
    providers::{Http, Middleware, Provider},
    types::U64,
};

use crate::util::timeit::timeit;

use crate::enclave_server::database::{generate_emoji, get_state, GLOBAL_DB};
use crate::enclave_server::models::{
    Ciphernode, CrispConfig, JsonResponse, PollLengthRequest, ReportTallyRequest, Round,
    RoundCount, TimestampRequest, AppState
};

pub fn setup_routes(config: &mut web::ServiceConfig) {
    config
        .route("/get_rounds", web::get().to(get_rounds))
        .route("/init_crisp_round", web::post().to(init_crisp_round))
        .route("/get_start_time_by_round", web::post().to(get_start_time_by_round))
        .route("/get_poll_length_by_round", web::post().to(get_poll_length_by_round))
        .route("/report_tally", web::post().to(report_tally));
}
async fn get_rounds(state: web::Data<AppState>) -> impl Responder {
    let key = "round_count";
    let mut round = state.db.get(key).unwrap();

    if round.is_none() {
        info!("initializing first round in db");
        state.db.insert(key, b"0".to_vec()).unwrap();
        round = state.db.get(key).unwrap();
    }

    let round_key = std::str::from_utf8(round.unwrap().as_ref()).unwrap().to_string();
    let round_int = round_key.parse::<u32>().unwrap();

    let count = RoundCount {
        round_count: round_int,
    };

    info!("round_count: {:?}", count.round_count);

    HttpResponse::Ok().json(count)
}

// Initialize CRISP Round Handler
async fn init_crisp_round(
    data: web::Json<CrispConfig>,
    state: web::Data<AppState>,  // Access shared state
) -> impl Responder {
    info!("generating round crp");

    let rpc_url = "http://0.0.0.0:8545".to_string();
    let provider = Provider::<Http>::try_from(rpc_url).unwrap();
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

    let incoming = data.into_inner();
    info!("ID: {:?}", incoming.round_id); // TODO: Check that client sent the expected next round_id
    info!("Address: {:?}", incoming.voting_address);

    // Initialize or increment round count
    let key = "round_count";
    let round = state.db.get(key).unwrap();
    if round.is_none() {
        info!("initializing first round in db");
        state.db.insert(key, b"0".to_vec()).unwrap();
    }

    let round_key = std::str::from_utf8(round.unwrap().as_ref()).unwrap().to_string();
    let mut round_int = round_key.parse::<u32>().unwrap();
    round_int += 1;

    let inc_round_key = format!("{}-storage", round_int);
    info!("Database key is {:?} and round int is {:?}", inc_round_key, round_int);

    let init_time = Utc::now();
    let timestamp = init_time.timestamp();
    info!("timestamp {:?}", timestamp);

    let (emoji1, emoji2) = generate_emoji();

    let state_data = Round {
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

    let state_str = serde_json::to_string(&state_data).unwrap();
    state.db.insert(inc_round_key, state_str.into_bytes()).unwrap();

    let new_round_bytes = round_int.to_string().into_bytes();
    state.db.insert(key, new_round_bytes).unwrap();

    let response = JsonResponse {
        response: "CRISP Initiated".to_string(),
    };

    HttpResponse::Ok().json(response)
}

// Get Start Time by Round Handler
async fn get_start_time_by_round(
    data: web::Json<TimestampRequest>,
) -> impl Responder {
    let mut incoming = data.into_inner();
    info!("Request start time for round {:?}", incoming.round_id);

    let (state_data, _) = get_state(incoming.round_id);
    incoming.timestamp = state_data.start_time;

    HttpResponse::Ok().json(incoming)
}

// Get Poll Length by Round Handler
async fn get_poll_length_by_round(
    data: web::Json<PollLengthRequest>,
) -> impl Responder {
    let mut incoming = data.into_inner();
    info!("Request poll length for round {:?}", incoming.round_id);

    let (state_data, _) = get_state(incoming.round_id);
    incoming.poll_length = state_data.poll_length;

    HttpResponse::Ok().json(incoming)
}

// Report Tally Handler
async fn report_tally(
    data: web::Json<ReportTallyRequest>,
    state: web::Data<AppState>,  
) -> impl Responder {
    let incoming = data.into_inner();
    info!("Request report tally for round {:?}", incoming.round_id);

    let (mut state_data, key) = get_state(incoming.round_id);

    if state_data.votes_option_1 == 0 && state_data.votes_option_2 == 0 {
        state_data.votes_option_1 = incoming.option_1;
        state_data.votes_option_2 = incoming.option_2;

        let state_str = serde_json::to_string(&state_data).unwrap();
        state.db.insert(key, state_str.into_bytes()).unwrap();
    }

    let response = JsonResponse {
        response: "Tally Reported".to_string(),
    };

    HttpResponse::Ok().json(response)
}