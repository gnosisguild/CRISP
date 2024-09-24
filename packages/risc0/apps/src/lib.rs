use anyhow::Result;
use compute_provider::{ComputeInput, ComputeManager, ComputeProvider, ComputeResult, FHEInputs};
use methods::VOTING_ELF;
use risc0_ethereum_contracts::groth16;
use risc0_zkvm::{default_prover, ExecutorEnv, ProverOpts, VerifierContext};
use voting_core::fhe_processor;
use serde::{Deserialize, Serialize};
pub struct Risc0Provider;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Risc0Output {
    pub result: ComputeResult,
    pub bytes: Vec<u8>,
    pub seal: Vec<u8>,
}

impl ComputeProvider for Risc0Provider {
    type Output = Risc0Output;

    fn prove(&self, input: &ComputeInput) -> Self::Output {   

        let env = ExecutorEnv::builder()
            .write(input)
            .unwrap()
            .build()
            .unwrap();

        let receipt = default_prover()
            .prove_with_ctx(
                env,
                &VerifierContext::default(),
                VOTING_ELF,
                &ProverOpts::groth16(),
            )
            .unwrap()
            .receipt;

        let decoded_journal = receipt.journal.decode().unwrap();

        let seal = groth16::encode(receipt.inner.groth16().unwrap().seal.clone()).unwrap();

        Risc0Output {
            result: decoded_journal,
            bytes: receipt.journal.bytes.clone(),
            seal,
        }
    }
}

pub fn run_compute(params: FHEInputs) -> Result<(Risc0Output, Vec<u8>)> {
    let risc0_provider = Risc0Provider;

    let mut provider = ComputeManager::new(risc0_provider, params, fhe_processor, false, None);

    let output = provider.start();

    Ok(output)
}
