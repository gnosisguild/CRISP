
use std::{env, sync::Arc, str};
use std::io::Read;
use alloy::{
    network::{AnyNetwork, EthereumWallet},
    primitives::{Address, Bytes, U256, B256},
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
    sol,
};

use eyre::Result;
use log::info;

use actix_web::{web, HttpResponse, Responder};

use crate::enclave_server::models::{EncryptedVote, JsonResponseTxHash, AppState, GetEmojisRequest, VoteCountRequest};
use crate::enclave_server::database::{GLOBAL_DB, get_state};

pub fn setup_routes(config: &mut web::ServiceConfig) {
    config
        .route("/broadcast_enc_vote", web::post().to(broadcast_enc_vote))
        .route("/get_vote_count_by_round", web::post().to(get_vote_count_by_round))
        .route("/get_emojis_by_round", web::post().to(get_emojis_by_round));
}

async fn broadcast_enc_vote(
    data: web::Json<EncryptedVote>,
    state: web::Data<AppState>,  // Access shared state
) -> impl Responder {
    let incoming = data.into_inner();
    let mut response_str = "";
    let mut converter = "".to_string();
    let (mut state_data, key) = get_state(incoming.round_id);

    for voted in &state_data.has_voted {
        if *voted == incoming.postId {
            response_str = "User Has Already Voted";
            break;
        }
    }

    if response_str == "" {
        response_str = "Vote Successful";
        let sol_vote = Bytes::from(incoming.enc_vote_bytes);
        let tx_hash = match call_contract(sol_vote, state_data.voting_address.clone()).await {
            Ok(hash) => hash,
            Err(e) => {
                info!("Error while sending vote transaction: {:?}", e);
                return HttpResponse::InternalServerError().body("Failed to broadcast vote");
            }
        };
        converter = tx_hash.to_string();
        state_data.vote_count += 1;
        state_data.has_voted.push(incoming.postId.clone());
        let state_str = serde_json::to_string(&state_data).unwrap();
        state.db.insert(key, state_str.into_bytes()).unwrap();
    }

    let response = JsonResponseTxHash {
        response: response_str.to_string(),
        tx_hash: converter,
    };

    info!("Request for round {:?} send vote tx", incoming.round_id);
    HttpResponse::Ok().json(response)
}

// Get Emojis by Round Handler
async fn get_emojis_by_round(
    data: web::Json<GetEmojisRequest>,
) -> impl Responder {
    let mut incoming = data.into_inner();
    info!("Request emojis for round {:?}", incoming.round_id);

    let (state_data, _) = get_state(incoming.round_id);
    incoming.emojis = state_data.emojis;

    HttpResponse::Ok().json(incoming)
}

// Get Vote Count by Round Handler
async fn get_vote_count_by_round(
    data: web::Json<VoteCountRequest>,
) -> impl Responder {
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
    enc_vote: Bytes,
    address: String,
) -> Result<B256, Box<dyn std::error::Error + Send + Sync>> {
    info!("Calling voting contract");

    // Set up the signer from a private key
    let eth_val = env::var("PRIVATEKEY").expect("PRIVATEKEY must be set in the environment");
    let signer: PrivateKeySigner = eth_val.parse()?;
    let wallet = EthereumWallet::from(signer);

    // Set up the provider using the Alloy library
    let rpc_url = "http://0.0.0.0:8545".parse()?;
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .network::<AnyNetwork>()
        .wallet(wallet)
        .on_http(rpc_url);

    // Parse the address of the contract
    let vote_address: Address = address.parse()?;

    // Create the contract instance
    let contract = IVOTE::new(vote_address, &provider);

    // Send the voteEncrypted transaction
    let builder = contract.voteEncrypted(enc_vote);
    let receipt = builder.send().await?.get_receipt().await?;

    // Log the transaction hash
    let tx_hash = receipt.transaction_hash;
    info!("Transaction hash: {:?}", tx_hash);

    Ok(tx_hash)
}