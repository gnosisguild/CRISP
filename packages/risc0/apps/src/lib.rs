// src/lib.rs
use anyhow::Result;
use methods::VOTING_ELF;
use compute_provider_host::ComputeProvider;
use compute_provider_core::{FHEInputs, ComputationResult};


pub fn run_compute(params: FHEInputs) -> Result<(ComputationResult, Vec<u8>)> {

    let mut provider = ComputeProvider::new(params, true, None);

    let (result, seal) = provider.start(VOTING_ELF);

    Ok((result, seal))
}