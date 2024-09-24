use super::{
    events::{CiphertextOutputPublished, E3Activated, InputPublished, PlaintextOutputPublished},
    relayer::EnclaveContract,
};
use crate::enclave_server::models::E3;
use crate::enclave_server::{
    config::CONFIG,
    database::{generate_emoji, get_e3, increment_e3_round, save_e3},
};
use alloy::rpc::types::Log;
use chrono::Utc;
use compute_provider::FHEInputs;
use std::error::Error;
use tokio::time::{sleep, Duration};
use voting_risc0::run_compute;
use log::info;

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

pub async fn handle_e3(e3_activated: E3Activated, log: Log) -> Result<()> {
    let e3_id = e3_activated.e3Id.to::<u64>();
    info!("Handling E3 request with id {}", e3_id);

    // Fetch E3 from the contract
    let contract = EnclaveContract::new().await?;

    let e3 = contract.get_e3(e3_activated.e3Id).await?;
    info!("Fetched E3 from the contract.");
    info!("E3: {:?}", e3);

    let start_time = Utc::now().timestamp() as u64;

    let block_start = match log.block_number {
        Some(bn) => bn,
        None => contract.get_latest_block().await?,
    };

    let (emoji1, emoji2) = generate_emoji();

    let e3_obj = E3 {
        // Identifiers
        id: e3_id,
        chain_id: 31337 as u64, // Hardcoded for testing
        enclave_address: CONFIG.contract_address.clone(),

        // Status-related
        status: "Active".to_string(),
        has_voted: vec!["".to_string()],
        vote_count: 0,
        votes_option_1: 0,
        votes_option_2: 0,

        // Timing-related
        start_time,
        block_start,
        duration: e3.duration.to::<u64>(),
        expiration: e3.expiration.to::<u64>(),

        // Parameters
        e3_params: e3.e3ProgramParams.to_vec(),
        committee_public_key: e3_activated.committeePublicKey.to_vec(),

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
    save_e3(&e3_obj, &key).await?;
    increment_e3_round().await.unwrap();

    // Sleep till the E3 expires
    sleep(Duration::from_secs(e3.duration.to::<u64>())).await;

    // Get All Encrypted Votes
    let (e3, _) = get_e3(e3_id).await.unwrap();
    if e3.vote_count > 0 {
        info!("E3 FROM DB");
        info!("Vote Count: {:?}", e3.vote_count);

        let fhe_inputs = FHEInputs {
            params: e3.e3_params,
            ciphertexts: e3.ciphertext_inputs,
        };

        // Call Compute Provider in a separate thread
        let (risc0_output, ciphertext) =
            tokio::task::spawn_blocking(move || run_compute(fhe_inputs).unwrap())
                .await
                .unwrap();

        println!("RISC0 Output: {:?}", risc0_output);

        // Params will be encoded on chain to create the journal
        let tx = contract
            .publish_ciphertext_output(
                e3_activated.e3Id,
                ciphertext.into(),
                risc0_output.seal.into(),
            )
            .await?;

        info!(
            "CiphertextOutputPublished event published with tx: {:?}",
            tx.transaction_hash
        );
    }

    info!("E3 request handled successfully.");
    Ok(())
}

pub async fn handle_input_published(input: InputPublished) -> Result<()> {
    info!("Handling VoteCast event...");

    let e3_id = input.e3Id.to::<u64>();
    let (mut e3, key) = get_e3(e3_id).await?;

    e3.ciphertext_inputs
        .push((input.data.to_vec(), input.index.to::<u64>()));
    e3.vote_count += 1;

    save_e3(&e3, &key).await?;

    info!("Saved Input with Hash: {:?}", input.inputHash);
    Ok(())
}

pub async fn handle_ciphertext_output_published(
    ciphertext_output: CiphertextOutputPublished,
) -> Result<()> {
    info!("Handling CiphertextOutputPublished event...");

    let e3_id = ciphertext_output.e3Id.to::<u64>();
    let (mut e3, key) = get_e3(e3_id).await?;

    e3.ciphertext_output = ciphertext_output.ciphertextOutput.to_vec();
    e3.status = "Published".to_string();

    save_e3(&e3, &key).await?;

    info!("CiphertextOutputPublished event handled.");
    Ok(())
}

pub async fn handle_plaintext_output_published(
    plaintext_output: PlaintextOutputPublished,
) -> Result<()> {
    info!("Handling PlaintextOutputPublished event...");

    let e3_id = plaintext_output.e3Id.to::<u64>();
    let (mut e3, key) = get_e3(e3_id).await?;

    e3.plaintext_output = plaintext_output.plaintextOutput.to_vec();
    e3.votes_option_2 = u64::from_be_bytes(e3.plaintext_output.as_slice().try_into().unwrap());
    e3.votes_option_1 = e3.vote_count - e3.votes_option_2;
    e3.status = "Finished".to_string();

    save_e3(&e3, &key).await?;

    info!("PlaintextOutputPublished event handled.");
    Ok(())
}
