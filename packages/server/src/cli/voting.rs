use std::{env, sync::Arc};
use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input};
use http_body_util::BodyExt;
use hyper::{body::Incoming, Method, Request, Response};
use serde::{Deserialize, Serialize};
use log::info;
use chrono::Utc;

use alloy::primitives::{Address, Bytes, U256};

use crate::enclave_server::blockchain::relayer::EnclaveContract;

use crate::cli::{AuthenticationResponse, HyperClientPost, GLOBAL_DB};
use crate::util::timeit::timeit;
use fhe::bfv::{BfvParametersBuilder, Encoding, Plaintext, PublicKey, BfvParameters, SecretKey};
use fhe_traits::{Deserialize as FheDeserialize, DeserializeParametrized, DeserializeWithContext, FheDecoder, FheDecrypter, FheEncoder, FheEncrypter, Serialize as FheSerialize};
use fhe_math::rq::Poly;
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
struct FHEParams {
    params: Vec<u8>,
    pk: Vec<u8>,
    sk: Vec<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
struct PKRequest {
    round_id: u64,
    pk_bytes: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
struct CTRequest {
    round_id: u64,
    ct_bytes: Vec<u8>,
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
    config: &super::CrispConfig
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Starting new CRISP round!");
    info!("Initializing Keyshare nodes...");

    let params = generate_bfv_parameters().unwrap().to_bytes();
    
    let contract = EnclaveContract::new().await?;
    
    let filter: Address = "0x95222290dd7278aa3ddd389cc1e1d165cc4bafe5".parse()?;
    let threshold: [u32; 2] = [1, 2];
    let start_window: [U256; 2] = [U256::from(Utc::now().timestamp()), U256::from(Utc::now().timestamp() + 600)];
    let duration: U256 = U256::from(40);
    let e3_program: Address = "0x95222290dd7278aa3ddd389cc1e1d165cc4bafe5".parse()?;
    let e3_params = Bytes::from(params);
    let compute_provider_params = Bytes::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let res = contract.request_e3(filter, threshold, start_window, duration, e3_program, e3_params, compute_provider_params).await?;
    println!("E3 request sent. TxHash: {:?}", res.transaction_hash);

    Ok(())
}

pub async fn activate_e3_round(config: &super::CrispConfig) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let input_e3_id: u64 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter CRISP round ID.")
        .interact_text()?;
    info!("Voting state Initialized");

    let params = generate_bfv_parameters().unwrap();
    let (sk, pk) = generate_keys(&params);

    let contract = EnclaveContract::new().await?;

    let pk_bytes = Bytes::from(pk.to_bytes());
    // Print how many bytes are in the public key
    println!("Public key bytes: {:?}", pk_bytes.len());
    let e3_id = U256::from(input_e3_id);
    let res = contract.activate_e3(e3_id, pk_bytes).await?;
    println!("E3 activated. TxHash: {:?}", res.transaction_hash);

    let e3_params = FHEParams {
        params: params.to_bytes(),
        pk: pk.to_bytes(),
        sk: sk.coeffs.into_vec(),
    };

    let db = GLOBAL_DB.write().await;
    db.insert(format!("e3:{}", input_e3_id), serde_json::to_vec(&e3_params)?)?;
    println!("E3 parameters stored in database.");

    Ok(())
}


pub async fn decrypt_and_publish_result(
    config: &super::CrispConfig,
    client: &HyperClientPost,
    auth_res: &AuthenticationResponse,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let input_crisp_id: u64 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter CRISP round ID.")
        .interact_text()?;
    info!("Decryption Initialized");

    // Get final Ciphertext
    let response_pk = CTRequest {
        round_id: input_crisp_id,
        ct_bytes: vec![0],
    };

    let url = format!("{}/get_ct_by_round", config.enclave_address);
    let req = Request::builder()
        .header("Content-Type", "application/json")
        .method(Method::POST)
        .uri(url)
        .body(serde_json::to_string(&response_pk)?)?;

    let resp = client.request(req).await?;
    info!("Response status: {}", resp.status());

    let body_str = get_response_body(resp).await?;
    let ct_res: CTRequest = serde_json::from_str(&body_str)?;
    info!("Shared Public Key for CRISP round {:?} collected.", ct_res.round_id);
    info!("Public Key: {:?}", ct_res.ct_bytes);

    let db = GLOBAL_DB.read().await;
    let params_bytes = db.get(format!("e3:{}", input_crisp_id))?.unwrap();
    let e3_params: FHEParams = serde_json::from_slice(&params_bytes)?;
    let params = timeit!(
        "Parameters generation",
        generate_bfv_parameters()?
    );
    let pk_deserialized = PublicKey::from_bytes(&e3_params.pk, &params)?;
    let sk_deserialized = SecretKey::new(e3_params.sk, &params);

    let ct = fhe::bfv::Ciphertext::from_bytes(&ct_res.ct_bytes, &params)?;

    let pt = sk_deserialized.try_decrypt(&ct)?;    
    let votes = Vec::<u64>::try_decode(&pt, Encoding::poly())?[0];
    println!("Vote count: {:?}", votes);

    info!("Calling contract with plaintext output.");
    let contract = EnclaveContract::new().await?;
    let res = contract.publish_plaintext_output(U256::from(input_crisp_id), Bytes::from(votes.to_be_bytes())).await?;
    println!("Vote broadcast. TxHash: {:?}", res.transaction_hash);
    
    Ok(())
}


pub async fn participate_in_existing_round(
    config: &super::CrispConfig,
    client: &HyperClientPost,
    auth_res: &AuthenticationResponse,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let input_crisp_id: u64 = Input::with_theme(&ColorfulTheme::default())
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

    let contract = EnclaveContract::new().await?;
    let res = contract.publish_input(U256::from(input_crisp_id), Bytes::from(ct.to_bytes())).await?;
    println!("Vote broadcast. TxHash: {:?}", res.transaction_hash);
    
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
fn generate_keys(params: &Arc<BfvParameters>) -> (SecretKey, PublicKey) {
    let mut rng = thread_rng();
    let sk = SecretKey::random(params, &mut rng);
    let pk = PublicKey::new(&sk, &mut rng);
    (sk, pk)
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