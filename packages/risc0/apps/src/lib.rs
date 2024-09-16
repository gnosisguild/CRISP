// src/lib.rs
use anyhow::Result;
use compute_provider::{
    ComputeInput, ComputeManager, ComputeOutput, ComputeProvider, ComputeResult, FHEInputs,
};
use methods::VOTING_ELF;
use risc0_ethereum_contracts::groth16;
use risc0_zkvm::{default_prover, ExecutorEnv, ProverOpts, VerifierContext};
pub struct Risc0Provider;

pub struct Risc0Output {
    pub result: ComputeResult,
    pub seal: Vec<u8>,
}

impl ComputeOutput for Risc0Output {
    fn ciphertext(&self) -> &Vec<u8> {
        &self.result.ciphertext
    }

    fn merkle_root(&self) -> String {
        self.result.merkle_root.clone()
    }

    fn params_hash(&self) -> String {
        self.result.params_hash.clone()
    }
}

impl ComputeProvider for Risc0Provider {
    type Output = Risc0Output;

    fn prove(&self, input: &ComputeInput) -> Self::Output {
        println!("Proving with RISC0 provider");
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
            seal,
        }
    }
}

pub fn run_compute(params: FHEInputs) -> Result<(ComputeResult, Vec<u8>)> {
    let risc0_provider = Risc0Provider;

    let mut provider = ComputeManager::new(risc0_provider, params, false, None);

    let output = provider.start();

    Ok((output.result, output.seal))
}
