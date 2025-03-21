use alloy::primitives::{Bytes, U256};
use actix_web::{web, HttpResponse, Responder};
use log::info;
use eyre::Error;
use crate::server::{
    config::CONFIG,
    database::{get_e3, GLOBAL_DB},
    blockchain::relayer::EnclaveContract,
    models::{EncryptedVote, JsonResponseTxHash, E3},
};

pub fn setup_routes(config: &mut web::ServiceConfig) {
    config
        .service(
            web::scope("/voting")
                .route("/broadcast", web::post().to(broadcast_encrypted_vote))
        );
}

/// Broadcast an encrypted vote to the blockchain
/// 
/// # Arguments
/// 
/// * `EncryptedVote` - The vote data to be broadcast
/// 
/// # Returns
/// 
/// * A JSON response indicating the success or failure of the operation
async fn broadcast_encrypted_vote(data: web::Json<EncryptedVote>) -> impl Responder {
    let vote = data.into_inner();
    
    // Validate and update vote status
    let (mut state_data, key) = match validate_and_update_vote_status(&vote).await {
        Ok(result) => result,
        Err(response) => return response,
    };

    // Prepare vote data for blockchain
    let e3_id = U256::from(vote.round_id);
    let sol_vote = Bytes::from(vote.enc_vote_bytes);

    // Broadcast vote to blockchain
    let contract = EnclaveContract::new(CONFIG.enclave_address.clone()).await.unwrap();
    match contract.publish_input(e3_id, sol_vote).await {
        Ok(hash) => HttpResponse::Ok().json(JsonResponseTxHash {
            response: "Vote Successful".to_string(),
            tx_hash: hash.transaction_hash.to_string(),
        }),
        Err(e) => handle_vote_error(e, &mut state_data, &key, &vote.postId).await,
    }
}

/// Validate and update the vote status
/// 
/// # Arguments
/// 
/// * `vote` - The vote data to be validated and updated
/// 
/// # Returns
/// 
/// * A tuple containing the state data and the key
async fn validate_and_update_vote_status(vote: &EncryptedVote) -> Result<(E3, String), HttpResponse> {
    let (mut state_data, key) = get_e3(vote.round_id).await.unwrap();
    
    if state_data.has_voted.contains(&vote.postId) {
        return Err(HttpResponse::BadRequest().json(JsonResponseTxHash {
            response: "User Has Already Voted".to_string(),
            tx_hash: "".to_string(),
        }));
    }
    
    state_data.has_voted.push(vote.postId.clone());
    GLOBAL_DB.insert(&key, &state_data).await.unwrap();
    
    Ok((state_data, key.to_string()))
}

/// Handle the vote error
/// 
/// # Arguments
/// 
/// * `e` - The error that occurred
/// * `state_data` - The state data to be rolled back
/// * `key` - The key for the state data
/// * `post_id` - The post ID for the vote
async fn handle_vote_error(e: Error, state_data: &mut E3, key: &str, post_id: &str) -> HttpResponse {
    info!("Error while sending vote transaction: {:?}", e);

    // Rollback the vote
    if let Some(pos) = state_data.has_voted.iter().position(|x| x == post_id) {
        state_data.has_voted.remove(pos);
        GLOBAL_DB.insert(key, state_data).await.unwrap();
    }

    HttpResponse::InternalServerError().body("Failed to broadcast vote")
}