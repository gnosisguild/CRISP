use alloy::primitives::{Bytes, B256, U256};
use std::{env, str};

use eyre::Result;
use log::info;

use actix_web::{web, HttpResponse, Responder};

use crate::enclave_server::database::get_e3;
use crate::enclave_server::{
    blockchain::relayer::EnclaveContract,
    models::{AppState, EncryptedVote, GetEmojisRequest, JsonResponseTxHash, VoteCountRequest},
};

pub fn setup_routes(config: &mut web::ServiceConfig) {
    config
        .route("/broadcast_enc_vote", web::post().to(broadcast_enc_vote))
        .route(
            "/get_vote_count_by_round",
            web::post().to(get_vote_count_by_round),
        )
        .route("/get_emojis_by_round", web::post().to(get_emojis_by_round));
}

async fn broadcast_enc_vote(
    data: web::Json<EncryptedVote>,
    state: web::Data<AppState>,
) -> impl Responder {
    let vote: EncryptedVote = data.into_inner();
    let (mut state_data, key) = get_e3(vote.round_id as u64).await.unwrap();

    if state_data.has_voted.contains(&vote.postId) {
        return HttpResponse::BadRequest().json(JsonResponseTxHash {
            response: "User has already voted".to_string(),
            tx_hash: "".to_string(),
        });
    }

    let sol_vote = Bytes::from(vote.enc_vote_bytes);
    let e3_id = U256::from(vote.round_id);
    let contract = EnclaveContract::new().await.unwrap();
    let tx_hash = match contract.publish_input(e3_id, sol_vote).await {
        Ok(hash) => hash.transaction_hash.to_string(),
        Err(e) => {
            info!("Error while sending vote transaction: {:?}", e);
            return HttpResponse::InternalServerError().body("Failed to broadcast vote");
        }
    };

    state_data.has_voted.push(vote.postId);
    // Lock the database for writing
    let db = state.db.write().await;
    if let Err(e) = db.insert(key, serde_json::to_vec(&state_data).unwrap()) {
        info!("Error updating state: {:?}", e);
    }

    info!(
        "Vote broadcast for round {}: tx_hash {}",
        vote.round_id, tx_hash
    );
    HttpResponse::Ok().json(JsonResponseTxHash {
        response: "Vote successful".to_string(),
        tx_hash,
    })
}

// Get Emojis by Round Handler
async fn get_emojis_by_round(data: web::Json<GetEmojisRequest>) -> impl Responder {
    let mut incoming = data.into_inner();
    info!("Request emojis for round {:?}", incoming.round_id);

    let (state_data, _) = get_e3(incoming.round_id as u64).await.unwrap();
    incoming.emojis = state_data.emojis;

    HttpResponse::Ok().json(incoming)
}

// Get Vote Count by Round Handler
async fn get_vote_count_by_round(data: web::Json<VoteCountRequest>) -> impl Responder {
    let mut incoming = data.into_inner();
    info!("Request vote count for round {:?}", incoming.round_id);

    let (state_data, _) = get_e3(incoming.round_id as u64).await.unwrap();
    incoming.vote_count = state_data.vote_count as u32;

    HttpResponse::Ok().json(incoming)
}
