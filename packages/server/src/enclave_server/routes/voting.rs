use alloy::{
    network::{AnyNetwork, EthereumWallet},
    primitives::{Address, Bytes, B256, U256},
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
    sol,
};
use std::{env, str};

use eyre::Result;
use log::info;

use actix_web::{web, HttpResponse, Responder};

use crate::enclave_server::database::{get_e3, get_state};
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
    let (mut state_data, key) = get_e3(vote.round_id as u64).unwrap();
    println!("Has Voted: {:?}", state_data.has_voted);
    if state_data.has_voted.contains(&vote.postId) {
        return HttpResponse::BadRequest().json(JsonResponseTxHash {
            response: "User has already voted".to_string(),
            tx_hash: "".to_string(),
        });
    }

    let sol_vote = Bytes::from(vote.enc_vote_bytes);
    let e3_id = U256::from(vote.round_id);
    let tx_hash = match call_contract(e3_id, sol_vote, state_data.enclave_address.clone()).await {
        Ok(hash) => hash.to_string(),
        Err(e) => {
            info!("Error while sending vote transaction: {:?}", e);
            return HttpResponse::InternalServerError().body("Failed to broadcast vote");
        }
    };

    state_data.has_voted.push(vote.postId);

    if let Err(e) = state
        .db
        .insert(key, serde_json::to_vec(&state_data).unwrap())
    {
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

    let (state_data, _) = get_state(incoming.round_id);
    incoming.emojis = state_data.emojis;

    HttpResponse::Ok().json(incoming)
}

// Get Vote Count by Round Handler
async fn get_vote_count_by_round(data: web::Json<VoteCountRequest>) -> impl Responder {
    let mut incoming = data.into_inner();
    info!("Request vote count for round {:?}", incoming.round_id);

    let (state_data, _) = get_state(incoming.round_id);
    incoming.vote_count = state_data.vote_count;

    HttpResponse::Ok().json(incoming)
}

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    contract IVOTE {
        function voteEncrypted(bytes memory _encVote) public;
        function getVote(address id) public returns (bytes memory);
        event Transfer(address indexed from, address indexed to, uint256 value);
    }
}

pub async fn call_contract(
    e3_id: U256,
    enc_vote: Bytes,
    address: String,
) -> Result<B256, Box<dyn std::error::Error + Send + Sync>> {
    info!("Calling voting contract");

    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set in the environment");
    let rpc_url = "http://0.0.0.0:8545";
    let contract = EnclaveContract::new(rpc_url, &address, &private_key).await?;
    let receipt = contract.publish_input(e3_id, enc_vote).await?;

    // Log the transaction hash
    let tx_hash = receipt.transaction_hash;
    info!("Transaction hash: {:?}", tx_hash);

    Ok(tx_hash)
}
