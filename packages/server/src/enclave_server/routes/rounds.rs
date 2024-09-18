use chrono::Utc;
use log::info;
use actix_web::{web, HttpResponse, Responder};


use crate::enclave_server::models::{CTRequest, PKRequest};
use crate::enclave_server::database::{generate_emoji, get_e3, get_e3_round};
use crate::enclave_server::models::{
    AppState, CrispConfig, JsonResponse, ReportTallyRequest, RoundCount
};

pub fn setup_routes(config: &mut web::ServiceConfig) {
    config
        .route("/get_rounds", web::get().to(get_rounds))
        .route("/get_pk_by_round", web::post().to(get_pk_by_round));
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
    let (state_data, _) = get_e3(incoming.round_id).await.unwrap();
    incoming.pk_bytes = state_data.committee_public_key;

    HttpResponse::Ok().json(incoming)
}
async fn get_ct_by_round(
    data: web::Json<CTRequest>,
) -> impl Responder {
    let mut incoming = data.into_inner();
    info!("Request for round {:?} public key", incoming.round_id);
    let (state_data, _) = get_e3(incoming.round_id).await.unwrap();
    incoming.ct_bytes = state_data.ciphertext_output;

    HttpResponse::Ok().json(incoming)
}
