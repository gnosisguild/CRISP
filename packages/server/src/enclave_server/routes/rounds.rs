use chrono::Utc;
use log::info;
use actix_web::{web, HttpResponse, Responder};


use crate::enclave_server::models::PKRequest;
use crate::enclave_server::database::{generate_emoji, get_e3, get_e3_round};
use crate::enclave_server::models::{
    AppState, CrispConfig, JsonResponse, ReportTallyRequest, RoundCount
};

pub fn setup_routes(config: &mut web::ServiceConfig) {
    config
        .route("/get_rounds", web::get().to(get_rounds))
        .route("/get_pk_by_round", web::post().to(get_pk_by_round))
        .route("/report_tally", web::post().to(report_tally));
}

async fn get_rounds()-> impl Responder {
    match get_e3_round().await {
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

async fn get_pk_by_round(
    data: web::Json<PKRequest>,
) -> impl Responder {
    let mut incoming = data.into_inner();
    info!("Request for round {:?} public key", incoming.round_id);
    let (state_data, _) = get_e3(incoming.round_id as u64).await.unwrap();
    incoming.pk_bytes = state_data.committee_public_key;

    HttpResponse::Ok().json(incoming)
}

// Report Tally Handler
async fn report_tally(
    data: web::Json<ReportTallyRequest>,
    state: web::Data<AppState>,
) -> impl Responder {
    let incoming = data.into_inner();
    info!("Request report tally for round {:?}", incoming.round_id);

    let (mut state_data, key) = get_e3(incoming.round_id as u64).await.unwrap();

    if state_data.votes_option_1 == 0 && state_data.votes_option_2 == 0 {
        state_data.votes_option_1 = incoming.option_1 as u64;
        state_data.votes_option_2 = incoming.option_2 as u64;

        let state_str = serde_json::to_string(&state_data).unwrap();
        let db = state.db.write().await;
        db.insert(key, state_str.into_bytes()).unwrap();
    }

    let response = JsonResponse {
        response: "Tally Reported".to_string(),
    };

    HttpResponse::Ok().json(response)
}
