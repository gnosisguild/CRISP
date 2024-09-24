use super::relayer::EnclaveContract;
use crate::enclave_server::database::{get_e3, get_e3_round, save_e3, generate_emoji};
use crate::enclave_server::models::E3;
use alloy::primitives::U256;
use chrono::Utc;
use eyre::Result;
use log::info;
pub async fn sync_contracts_db() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Syncing contracts with database");
    let contract = EnclaveContract::new().await?;
    let contract_e3_id = contract.get_e3_id().await?.to::<u64>();
    let db_e3_id = get_e3_round().await?;

    if contract_e3_id == 0 {
        info!("No E3s found in contract, skipping sync");
        return Ok(());
    }

    // Update existing E3 if expired
    if let Ok((mut e3, key)) = get_e3(db_e3_id).await {
        if e3.status != "Finished" && e3.status == "Published" && e3.expiration < Utc::now().timestamp() as u64 {
            let c_e3 = contract.get_e3(U256::from(db_e3_id)).await?;
            let inputs_count = contract.get_input_count(U256::from(db_e3_id)).await?.to::<u64>();

            e3.plaintext_output = c_e3.plaintextOutput.to_vec();
            e3.votes_option_2 = u64::from_be_bytes(e3.plaintext_output.as_slice().try_into()?);
            e3.votes_option_1 = inputs_count - e3.votes_option_2;
            e3.status = "Finished".to_string();

            save_e3(&e3, &key).await?;
        }
    }

    // Sync new E3s
    for e3_id in db_e3_id + 1..=contract_e3_id {
        let e3 = contract.get_e3(U256::from(e3_id)).await?;
        let inputs_count = contract.get_input_count(U256::from(e3_id)).await?.to::<u64>();

        let (status, votes) = if e3.plaintextOutput.is_empty() {
            if e3.ciphertextOutput.is_empty() {
                ("Active", 0)
            } else {
                ("Published", 0)
            }
        } else {
            let votes = u64::from_be_bytes(e3.plaintextOutput.to_vec().as_slice().try_into()?);
            ("Finished", votes)
        };

        let e3_obj = E3 {
            id: e3_id,
            chain_id: 31337, // Hardcoded for testing
            enclave_address: contract.contract_address.to_string(),
            status: status.to_string(),
            has_voted: vec!["".to_string()],
            vote_count: inputs_count,
            votes_option_1: inputs_count - votes,
            votes_option_2: votes,
            start_time: e3.expiration.to::<u64>() - e3.duration.to::<u64>(),
            block_start: 0,
            duration: e3.duration.to::<u64>(),
            expiration: e3.expiration.to::<u64>(),
            e3_params: e3.e3ProgramParams.to_vec(),
            committee_public_key: e3.committeePublicKey.to_vec(),
            ciphertext_output: e3.ciphertextOutput.to_vec(),
            plaintext_output: e3.plaintextOutput.to_vec(),
            ciphertext_inputs: vec![],
            emojis: generate_emoji().into(),
        };

        save_e3(&e3_obj, &format!("e3:{}", e3_id)).await?;
    }

    info!("Contracts synced with database");
    Ok(())
}