use bytes::Bytes;
use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input};
use http_body_util::BodyExt;
use http_body_util::Empty;
use hyper::{Method, Request};
use serde::{Deserialize, Serialize};
use std::{thread, time};
use tokio::io::{self, AsyncWriteExt as _};
use log::info;

use crate::cli::AuthenticationResponse;
use crate::cli::{HyperClientGet, HyperClientPost};
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
    #[allow(non_snake_case)]
    postId: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct JsonResponseTxHash {
    response: String,
    tx_hash: String,
}

pub async fn initialize_crisp_round(
    config: &super::CrispConfig,
    client_get: &HyperClientGet,
    client: &HyperClientPost,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Starting new CRISP round!");

    info!("Initializing Keyshare nodes...");

    let response_id = JsonRequestGetRounds {
        response: "Test".to_string(),
    };
    let _out = serde_json::to_string(&response_id).unwrap();
    let mut url_id = config.enclave_address.clone();
    url_id.push_str("/get_rounds");

    let req = Request::builder()
        .method(Method::GET)
        .uri(url_id)
        .body(Empty::<Bytes>::new())?;

    let resp = client_get.request(req).await?;

    info!("Response status: {}", resp.status());

    let body_bytes = resp.collect().await?.to_bytes();
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
    let count: RoundCount = serde_json::from_str(&body_str).expect("JSON was not well-formatted");
    info!("Server Round Count: {:?}", count.round_count);

    let round_id = count.round_count + 1;
    let response = super::CrispConfig {
        round_id: round_id,
        poll_length: config.poll_length,
        chain_id: config.chain_id,
        voting_address: config.voting_address.clone(),
        ciphernode_count: config.ciphernode_count,
        enclave_address: config.enclave_address.clone(),
        authentication_id: config.authentication_id.clone(),
    };
    let out = serde_json::to_string(&response).unwrap();
    let mut url = config.enclave_address.clone();
    url.push_str("/init_crisp_round");
    let req = Request::builder()
        .header("authorization", "Bearer fpKL54jvWmEGVoRdCNjG")
        .header("Content-Type", "application/json")
        .method(Method::POST)
        .uri(url)
        .body(out)?;

    let mut resp = client.request(req).await?;

    info!("Response status: {}", resp.status());

    while let Some(next) = resp.frame().await {
        let frame = next?;
        if let Some(chunk) = frame.data_ref() {
            tokio::io::stdout().write_all(chunk).await?;
        }
    }
    info!("Round Initialized.");
    info!("Gathering Keyshare nodes for execution environment...");
    let three_seconds = time::Duration::from_millis(1000);
    thread::sleep(three_seconds);
    info!("\nYou can now vote Encrypted with Round ID: {:?}", round_id);

    Ok(())
}

pub async fn participate_in_existing_round(
    config: &super::CrispConfig,
    client: &HyperClientPost,
    auth_res: &AuthenticationResponse,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let input_crisp_id: u32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter CRISP round ID.")
        .interact_text()
        .unwrap();
    info!("Voting state Initialized");

    // get public encrypt key
    let v: Vec<u8> = vec![0];
    let response_pk = PKRequest {
        round_id: input_crisp_id,
        pk_bytes: v,
    };
    let out = serde_json::to_string(&response_pk).unwrap();
    let mut url = config.enclave_address.clone();
    url.push_str("/get_pk_by_round");
    let req = Request::builder().header("Content-Type", "application/json").method(Method::POST).uri(url).body(out)?;

    let resp = client.request(req).await?;

    info!("Response status: {}", resp.status());

    let body_bytes = resp.collect().await?.to_bytes();
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
    let pk_res: PKRequest = serde_json::from_str(&body_str).expect("JSON was not well-formatted");
    info!(
        "Shared Public Key for CRISP round {:?} collected.",
        pk_res.round_id
    );
    info!("Public Key: {:?}", pk_res.pk_bytes);

    let degree = 4096;
    let plaintext_modulus: u64 = 4096;
    let moduli = vec![0xffffee001, 0xffffc4001, 0x1ffffe0001];
    // Let's generate the BFV parameters structure.
    let params = timeit!(
        "Parameters generation",
        BfvParametersBuilder::new()
            .set_degree(degree)
            .set_plaintext_modulus(plaintext_modulus)
            .set_moduli(&moduli)
            .build_arc()?
    );
    let pk_deserialized = PublicKey::from_bytes(&pk_res.pk_bytes, &params).unwrap();

    // Select voting option
    let selections_3 = &["Abstain.", "Vote yes.", "Vote no."];

    let selection_3 = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Please select your voting option.")
        .default(0)
        .items(&selections_3[..])
        .interact()
        .unwrap();

    let mut vote_choice: u64 = 0;
    if selection_3 == 0 {
        info!("Exiting voting system. You may choose to vote later.");
        return Ok(());
    } else if selection_3 == 1 {
        vote_choice = 1;
    } else if selection_3 == 2 {
        vote_choice = 0;
    }
    info!("Encrypting vote.");
    let votes: Vec<u64> = [vote_choice].to_vec();
    let pt = Plaintext::try_encode(&[votes[0]], Encoding::poly(), &params)?;
    let ct = pk_deserialized.try_encrypt(&pt, &mut thread_rng())?;
    info!("Vote encrypted.");
    info!("Calling voting contract with encrypted vote.");

    let request_contract = EncryptedVote {
        round_id: input_crisp_id,
        enc_vote_bytes: ct.to_bytes(),
        postId: auth_res.jwt_token.clone(),
    };
    let out = serde_json::to_string(&request_contract).unwrap();
    let mut url = config.enclave_address.clone();
    url.push_str("/broadcast_enc_vote");
    let req = Request::builder().header("Content-Type", "application/json").method(Method::POST).uri(url).body(out)?;

    let resp = client.request(req).await?;

    info!("Response status: {}", resp.status());

    let body_bytes = resp.collect().await?.to_bytes();
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
    let contract_res: JsonResponseTxHash =
        serde_json::from_str(&body_str).expect("JSON was not well-formatted");
    info!("Contract call: {:?}", contract_res.response);
    info!("TxHash is {:?}", contract_res.tx_hash);

    Ok(())
}
