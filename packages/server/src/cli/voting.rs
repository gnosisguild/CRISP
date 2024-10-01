use chrono::Utc;
use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input};
use http_body_util::BodyExt;
use hyper::{body::Incoming, Method, Request, Response};
use log::info;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use super::CONFIG;

use alloy::primitives::{Address, Bytes, U256};

use crate::enclave_server::blockchain::relayer::EnclaveContract;

use crate::cli::{AuthenticationResponse, HyperClientPost, GLOBAL_DB};
use crate::util::timeit::timeit;
use fhe::bfv::{BfvParameters, BfvParametersBuilder, Encoding, Plaintext, PublicKey, SecretKey, Ciphertext};
use fhe_traits::{DeserializeParametrized, FheDecoder,
    FheDecrypter, FheEncoder, FheEncrypter, Serialize as FheSerialize,
};
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

async fn get_response_body(
    resp: Response<Incoming>,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let body_bytes = resp.collect().await?.to_bytes();
    Ok(String::from_utf8(body_bytes.to_vec())?)
}

pub async fn initialize_crisp_round() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Starting new CRISP round!");
    info!("Initializing Keyshare nodes...");
    let contract = EnclaveContract::new(CONFIG.enclave_address.clone()).await?;
    let e3_program: Address = CONFIG.e3_program_address.parse()?;

    info!("Enabling E3 Program...");
    match contract.enable_e3_program(e3_program).await {
        Ok(res) => println!("E3 Program enabled. TxHash: {:?}", res.transaction_hash),
        Err(e) => println!("Error enabling E3 Program: {:?}", e),
    };
    info!("Generating parameters...");
    let params = generate_bfv_parameters().unwrap().to_bytes();

    info!("Requesting E3...");
    let filter: Address = CONFIG.naive_registry_filter_address.parse()?;
    let threshold: [u32; 2] = [1, 2];
    let start_window: [U256; 2] = [U256::from(Utc::now().timestamp()), U256::from(Utc::now().timestamp() + 600)];
    let duration: U256 = U256::from(150);
    let e3_params = Bytes::from(params);
    let compute_provider_params = Bytes::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let res = contract.request_e3(filter, threshold, start_window, duration, e3_program, e3_params, compute_provider_params).await?;
    println!("E3 request sent. TxHash: {:?}", res.transaction_hash);

    Ok(())
}

pub async fn activate_e3_round() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let input_e3_id: u64 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter CRISP round ID.")
        .interact_text()?;
    info!("Voting state Initialized");

    let params = generate_bfv_parameters().unwrap();
    let (sk, pk) = generate_keys(&params);
    let contract = EnclaveContract::new(CONFIG.enclave_address.clone()).await?;
    println!("Public key Len: {:?}", pk.to_bytes().len());
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
    let key = format!("e3:{}", input_e3_id);
    println!("Key: {:?}", key);
    db.insert(
        key.clone(),
        serde_json::to_vec(&e3_params)?,
    )?;
    db.flush()?;
    println!("E3 parameters stored in database.");
    println!("Public key Len: {:?}", pk.to_bytes().len());


    Ok(())
}

pub async fn participate_in_existing_round(
    client: &HyperClientPost,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let input_crisp_id: u64 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter CRISP round ID.")
        .interact_text()?;
    info!("Voting state Initialized");

    let response_pk = PKRequest {
        round_id: input_crisp_id,
        pk_bytes: vec![0],
    };

    let url = format!("{}/get_pk_by_round", CONFIG.enclave_server_url);
    let req = Request::builder()
        .header("Content-Type", "application/json")
        .method(Method::POST)
        .uri(url)
        .body(serde_json::to_string(&response_pk)?)?;

    let resp = client.request(req).await?;
    info!("Response status: {}", resp.status());

    let body_str = get_response_body(resp).await?;
    let pk_res: PKRequest = serde_json::from_str(&body_str)?;
    info!(
        "Shared Public Key for CRISP round {:?} collected.",
        pk_res.round_id
    );
    info!("PK Key: {:?}", pk_res.pk_bytes);

    let params = timeit!("Parameters generation", generate_bfv_parameters()?);
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

    info!("Enclave Address: {:?}", CONFIG.enclave_address);

    let contract = EnclaveContract::new(CONFIG.enclave_address.clone()).await?;
    let res = contract
        .publish_input(U256::from(input_crisp_id), Bytes::from(ct.to_bytes()))
        .await?;
    println!("Vote broadcast. TxHash: {:?}", res.transaction_hash);

    Ok(())
}


pub async fn decrypt_and_publish_result(
    config: &super::CrispConfig,
    client: &HyperClientPost,
    _auth_res: &AuthenticationResponse,
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
    info!(
        "Shared Public Key for CRISP round {:?} collected.",
        ct_res.round_id
    );
    info!("CT Key: {:?}", ct_res.ct_bytes);

    let db = GLOBAL_DB.read().await;
    let params_bytes = db.get(format!("e3:{}", input_crisp_id))?.ok_or("Key not found")?;
    let e3_params: FHEParams = serde_json::from_slice(&params_bytes)?;
    let params = timeit!("Parameters generation", generate_bfv_parameters()?);
    let sk_deserialized = SecretKey::new(e3_params.sk, &params);
    info!("Secret key deserialized.");

    let ct = Ciphertext::from_bytes(&ct_res.ct_bytes, &params)?;
    info!("Ciphertext deserialized.");

    let pt = sk_deserialized.try_decrypt(&ct)?;
    let votes = Vec::<u64>::try_decode(&pt, Encoding::poly())?[0];
    println!("Vote count: {:?}", votes);

    info!("Calling contract with plaintext output.");
    let contract = EnclaveContract::new(CONFIG.enclave_address.clone()).await?;
    let res = contract
        .publish_plaintext_output(U256::from(input_crisp_id), Bytes::from(votes.to_be_bytes()))
        .await?;
    println!("Vote broadcast. TxHash: {:?}", res.transaction_hash);

    Ok(())
}


fn generate_bfv_parameters(
) -> Result<std::sync::Arc<fhe::bfv::BfvParameters>, Box<dyn std::error::Error + Send + Sync>> {
    let degree = 2048;
    let plaintext_modulus: u64 = 1032193;
    let moduli = vec![0x3FFFFFFF000001];

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
