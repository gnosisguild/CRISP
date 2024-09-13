use super::{
    events::{E3Activated, InputPublished, PlaintextOutputPublished},
    relayer::EnclaveContract,
};
use crate::enclave_server::database::{generate_emoji, get_e3, GLOBAL_DB};
use crate::enclave_server::models::E3;
use alloy::{
    primitives::{Address, Bytes, U256, FixedBytes},
    providers::Provider,
    rpc::types::Log,
    sol,
    sol_types::{SolCall, SolEvent},
};
use chrono::Utc;
use compute_provider_core::FHEInputs;
use std::env;
use std::error::Error;
use tokio::time::{sleep, Duration};
use voting_risc0::run_compute;
use alloy_sol_types::SolValue;

pub async fn handle_e3(
    e3_activated: E3Activated,
    log: Log,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let e3_id = e3_activated.e3Id.to::<u64>();
    println!("Handling E3 request with id {}", e3_id);

    // Fetch E3 from the contract
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATEKEY must be set in the environment");
    let rpc_url = env::var("RPC_URL").expect("RPC_URL must be set in the environment");
    let contract_address =
        env::var("CONTRACT_ADDRESS").expect("CONTRACT_ADDRESS must be set in the environment");
    let contract = EnclaveContract::new(&rpc_url, &contract_address, &private_key).await?;

    let e3 = contract.get_e3(e3_activated.e3Id).await?;
    println!("Fetched E3 from the contract.");
    println!("E3: {:?}", e3);

    let start_time = Utc::now().timestamp() as u64;

    let block_start = match log.block_number {
        Some(bn) => bn,
        None => contract.get_latest_block().await?,
    };

    let (emoji1, emoji2) = generate_emoji();

    let e3_obj = E3 {
        // Identifiers
        id: e3_id,

        // Status-related
        status: "Active".to_string(),
        has_voted: vec!["".to_string()],
        vote_count: 0,

        // Timing-related
        start_time,
        block_start,
        duration: e3.duration.to::<u64>(),
        expiration: e3.expiration.to::<u64>(),

        // Parameters
        e3_params: e3.e3ProgramParams.to_vec(),
        committee_public_key: e3.committeePublicKey.to_vec(),

        // Outputs
        ciphertext_output: vec![],
        plaintext_output: vec![],

        // Ciphertext Inputs
        ciphertext_inputs: vec![],

        // Emojis
        emojis: [emoji1, emoji2],
    };

    // Save E3 to the database
    let key = format!("e3:{}", e3_id);
    GLOBAL_DB
        .insert(key, serde_json::to_vec(&e3_obj).unwrap())
        .unwrap();

    // Sleep till the E3 expires
    sleep(Duration::from_secs(e3.duration.to::<u64>())).await;

    // Get All Encrypted Votes
    let (e3, _) = get_e3(e3_id).unwrap();
    println!("E3 FROM DB: {:?}", e3);

    let fhe_inputs = FHEInputs {
        params: e3.e3_params,
        ciphertexts: e3.ciphertext_inputs,
    };

    // Call Compute Provider
    // let (compute_result, seal) = run_compute(fhe_inputs).unwrap();

    // let data = (
    //     compute_result.ciphertext,
    //     compute_result.merkle_root,
    //     seal,
    // );

    // let encoded_data = data.abi_encode();

    // // Params will be encoded on chain to create the journal
    // let tx = contract
    //     .publish_ciphertext_output(e3_activated.e3Id, encoded_data.into())
    //     .await?;

    // println!(
    //     "CiphertextOutputPublished event published with tx: {:?}",
    //     tx
    // );
    println!("E3 request handled successfully.");
    Ok(())
}

pub fn handle_input_published(input: InputPublished) -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("Handling VoteCast event...");

    let e3_id = input.e3Id.to::<u64>();
    let data = input.data.to_vec();
    let (mut e3, key) = get_e3(e3_id).unwrap();
    e3.ciphertext_inputs.push(data);
    e3.vote_count += 1;

    GLOBAL_DB
        .insert(key, serde_json::to_vec(&e3).unwrap())
        .unwrap();

    println!("Saved Input with Hash: {:?}", input.inputHash);
    Ok(())
}

pub fn handle_plaintext_output_published(
    plaintext_output: PlaintextOutputPublished,
) -> Result<(), Box<dyn Error>> {
    println!("Handling PlaintextOutputPublished event...");

    let e3_id = plaintext_output.e3Id.to::<u64>();
    let (mut e3, key) = get_e3(e3_id).unwrap();
    e3.plaintext_output = plaintext_output.plaintextOutput.to_vec();

    GLOBAL_DB
        .insert(key, serde_json::to_vec(&e3).unwrap())
        .unwrap();
    println!("PlaintextOutputPublished event handled.");
    Ok(())
}
