use std::{thread, time::Duration};
use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input};
use http_body_util::{BodyExt, Empty};
use hyper::{body::Incoming, Method, Request, Response};
use serde::{Deserialize, Serialize};
use tokio::io::{self, AsyncWriteExt};
use log::{info, error};
use std::env;
use chrono::Utc;

use alloy::primitives::{Bytes, U256};

use crate::enclave_server::blockchain::relayer::EnclaveContract;

use crate::cli::{AuthenticationResponse, HyperClientGet, HyperClientPost};
use crate::util::timeit::timeit;
use fhe::bfv::{BfvParametersBuilder, Encoding, Plaintext, PublicKey};
use fhe_traits::{DeserializeParametrized, FheEncoder, FheEncrypter, Serialize as FheSerialize};
use rand::thread_rng;

#[derive(Debug, Deserialize, Serialize)]
struct JsonRequestGetRounds {
    response: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct RoundCount {
    round_count: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct PKRequest {
    round_id: u32,
    pk_bytes: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
struct EncryptedVote {
    round_id: u32,
    enc_vote_bytes: Vec<u8>,
    #[serde(rename = "postId")]
    post_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct JsonResponseTxHash {
    response: String,
    tx_hash: String,
}


async fn get_response_body(resp: Response<Incoming>) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let body_bytes = resp.collect().await?.to_bytes();
    Ok(String::from_utf8(body_bytes.to_vec())?)
}

pub async fn initialize_crisp_round(
    config: &super::CrispConfig,
    client_get: &HyperClientGet,
    client: &HyperClientPost,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Starting new CRISP round!");
    info!("Initializing Keyshare nodes...");
    
    let private_key = env::var("PRIVATEKEY").expect("PRIVATEKEY must be set in the environment");
    let rpc_url = "http://0.0.0.0:8545";
    let contract = EnclaveContract::new(rpc_url, &config.voting_address, &private_key).await?;
    // Current time as start time
    // let start_time = U256::from(Utc::now().timestamp());
    // let e3_params = Bytes::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    // let duration = U256::from(config.poll_length);
    // let res = contract.request_e3(start_time,duration, e3_params).await?;

    // println!("E3 request sent. TxHash: {:?}", res.transaction_hash);


    // let url_id = format!("{}/get_rounds", config.enclave_address);
    // let req = Request::builder()
    //     .method(Method::GET)
    //     .uri(url_id)
    //     .body(Empty::<Bytes>::new())?;

    // let resp = client_get.request(req).await?;
    // info!("Response status: {}", resp.status());

    // let body_str = get_response_body(resp).await?;
    // let count: RoundCount = serde_json::from_str(&body_str)?;
    // info!("Server Round Count: {:?}", count.round_count);

    // let round_id = count.round_count + 1;
    // let response = super::CrispConfig {
    //     round_id,
    //     poll_length: config.poll_length,
    //     chain_id: config.chain_id,
    //     voting_address: config.voting_address.clone(),
    //     ciphernode_count: config.ciphernode_count,
    //     enclave_address: config.enclave_address.clone(),
    //     authentication_id: config.authentication_id.clone(),
    // };

    // let url = format!("{}/init_crisp_round", config.enclave_address);
    // let req = Request::builder()
    //     .header("authorization", "Bearer fpKL54jvWmEGVoRdCNjG")
    //     .header("Content-Type", "application/json")
    //     .method(Method::POST)
    //     .uri(url)
    //     .body(serde_json::to_string(&response)?)?;

    // let mut resp = client.request(req).await?;
    // info!("Response status: {}", resp.status());

    // while let Some(frame) = resp.frame().await {
    //     if let Some(chunk) = frame?.data_ref() {
    //         io::stdout().write_all(chunk).await?;
    //     }
    // }

    // info!("Round Initialized.");
    // info!("Gathering Keyshare nodes for execution environment...");
    // thread::sleep(Duration::from_secs(1));
    // info!("\nYou can now vote Encrypted with Round ID: {:?}", round_id);

    Ok(())
}

pub async fn participate_in_existing_round(
    config: &super::CrispConfig,
    client: &HyperClientPost,
    auth_res: &AuthenticationResponse,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let input_crisp_id: u32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter CRISP round ID.")
        .interact_text()?;
    info!("Voting state Initialized");

    // Get public encrypt key
    let response_pk = PKRequest {
        round_id: input_crisp_id,
        pk_bytes: vec![0],
    };

    let url = format!("{}/get_pk_by_round", config.enclave_address);
    let req = Request::builder()
        .header("Content-Type", "application/json")
        .method(Method::POST)
        .uri(url)
        .body(serde_json::to_string(&response_pk)?)?;

    let resp = client.request(req).await?;
    info!("Response status: {}", resp.status());

    let body_str = get_response_body(resp).await?;
    let pk_res: PKRequest = serde_json::from_str(&body_str)?;
    info!("Shared Public Key for CRISP round {:?} collected.", pk_res.round_id);
    info!("Public Key: {:?}", pk_res.pk_bytes);

    let params = timeit!(
        "Parameters generation",
        generate_bfv_parameters()?
    );
    let pk_deserialized = PublicKey::from_bytes(&pk_res.pk_bytes, &params)?;

    let vote_choice = get_user_vote()?;
    if vote_choice.is_none() {
        info!("Exiting voting system. You may choose to vote later.");
        return Ok(());
    }

    info!("Encrypting vote.");
    let ct = encrypt_vote(vote_choice.unwrap(), &pk_deserialized, &params)?;
    info!("Vote encrypted.");
    info!("Calling voting contract with encrypted vote.");

    let request_contract = EncryptedVote {
        round_id: input_crisp_id,
        enc_vote_bytes: ct.to_bytes(),
        post_id: auth_res.jwt_token.clone(),
    };

    let url = format!("{}/broadcast_enc_vote", config.enclave_address);
    let req = Request::builder()
        .header("Content-Type", "application/json")
        .method(Method::POST)
        .uri(url)
        .body(serde_json::to_string(&request_contract)?)?;

    let resp = client.request(req).await?;
    info!("Response status: {}", resp.status());

    let body_str = get_response_body(resp).await?;
    let contract_res: JsonResponseTxHash = serde_json::from_str(&body_str)?;
    info!("Contract call: {:?}", contract_res.response);
    info!("TxHash is {:?}", contract_res.tx_hash);

    Ok(())
}

fn generate_bfv_parameters() -> Result<std::sync::Arc<fhe::bfv::BfvParameters>, Box<dyn std::error::Error + Send + Sync>> {
    let degree = 4096;
    let plaintext_modulus: u64 = 4096;
    let moduli = vec![0xffffee001, 0xffffc4001, 0x1ffffe0001];
    
    Ok(BfvParametersBuilder::new()
        .set_degree(degree)
        .set_plaintext_modulus(plaintext_modulus)
        .set_moduli(&moduli)
        .build_arc()?)
}

fn get_user_vote() -> Result<Option<u64>, Box<dyn std::error::Error + Send + Sync>> {
    let selections = &["Abstain.", "Vote yes.", "Vote no."];
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Please select your voting option.")
        .default(0)
        .items(&selections[..])
        .interact()?;

    match selection {
        0 => Ok(None),
        1 => Ok(Some(1)),
        2 => Ok(Some(0)),
        _ => Err("Invalid selection".into()),
    }
}

fn encrypt_vote(
    vote: u64,
    public_key: &PublicKey,
    params: &std::sync::Arc<fhe::bfv::BfvParameters>,
) -> Result<fhe::bfv::Ciphertext, Box<dyn std::error::Error + Send + Sync>> {
    let pt = Plaintext::try_encode(&[vote], Encoding::poly(), params)?;
    Ok(public_key.try_encrypt(&pt, &mut thread_rng())?)
}