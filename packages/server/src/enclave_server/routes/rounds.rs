use chrono::Utc;
use fhe::{bfv::BfvParametersBuilder, mbfv::CommonRandomPoly};
use fhe_traits::Serialize;
use log::info;
use rand::thread_rng;
use sled::IVec;
use bincode;

use actix_web::{web, HttpResponse, Responder};

use ethers::{
    providers::{Http, Middleware, Provider},
    types::U64,
};

use crate::util::timeit::timeit;

use crate::enclave_server::database::{generate_emoji, get_state, get_e3_round};
use crate::enclave_server::models::{
    AppState, Ciphernode, CrispConfig, JsonResponse, PollLengthRequest, ReportTallyRequest, Round,
    RoundCount, TimestampRequest,
};

pub fn setup_routes(config: &mut web::ServiceConfig) {
    config
        .route("/get_rounds", web::get().to(get_rounds))
        .route("/report_tally", web::post().to(report_tally));
}

async fn get_rounds()-> impl Responder {
    match get_e3_round() {
        Ok(round_count) => {
            let count = RoundCount { round_count: round_count as u32 };
            info!("round_count: {}", count.round_count);
            HttpResponse::Ok().json(count)
        }
        Err(e) => {
            info!("Failed to retrieve round count: {}", e);
            HttpResponse::InternalServerError().body(format!("Error: {}", e))
        }
    }
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
