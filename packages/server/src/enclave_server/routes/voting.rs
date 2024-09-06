
use std::{env, sync::Arc, str};
use std::io::Read;
use ethers::{
    prelude::abigen,
    providers::{Http, Provider, Middleware},
    middleware::{SignerMiddleware, MiddlewareBuilder},
    signers::{LocalWallet, Signer},
    types::{Address, Bytes, TxHash, BlockNumber},
};
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
        let tx_hash = call_contract(sol_vote, state_data.voting_address.clone()).await.unwrap();

        converter = "0x".to_string();
        for i in 0..32 {
            if tx_hash[i] <= 16 {
                converter.push_str("0");
            }
            converter.push_str(&format!("{:x}", tx_hash[i]));
        }

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

// Call Contract Function
async fn call_contract(
    enc_vote: Bytes,
    address: String,
) -> Result<TxHash, Box<dyn std::error::Error + Send + Sync>> {
    info!("calling voting contract");

    let rpc_url = "http://0.0.0.0:8545".to_string();
    let provider = Provider::<Http>::try_from(rpc_url.clone())?;

    abigen!(
        IVOTE,
        r#"[
            function voteEncrypted(bytes memory _encVote) public
            function getVote(address id) public returns(bytes memory)
            event Transfer(address indexed from, address indexed to, uint256 value)
        ]"#,
    );

    let vote_address: &str = &address;
    let eth_val = env!("PRIVATEKEY");
    let wallet: LocalWallet = eth_val
        .parse::<LocalWallet>()?
        .with_chain_id(31337 as u64);

    let nonce_manager = provider.clone().nonce_manager(wallet.address());
    let curr_nonce = nonce_manager
        .get_transaction_count(wallet.address(), Some(BlockNumber::Pending.into()))
        .await?
        .as_u64();

    let client = SignerMiddleware::new(provider.clone(), wallet.clone());
    let address: Address = vote_address.parse()?;
    let contract = IVOTE::new(address, Arc::new(client.clone()));

    let tx = contract.vote_encrypted(enc_vote).nonce(curr_nonce).send().await?.clone();
    info!("{:?}", tx);

    Ok(tx)
}