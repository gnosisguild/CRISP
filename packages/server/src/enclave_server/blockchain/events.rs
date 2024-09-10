use alloy::{
    sol,
    primitives::{Address, Bytes, U256},
    providers::Provider,
    sol_types::{SolCall, SolEvent},
};
use eyre::Result;

use super::listener::ContractEvent;

sol! {
    #[derive(Debug)]
    event E3Requested(uint256 indexed e3Id, uint256 startTime, uint256 endTime, bytes e3Params);

    #[derive(Debug)]
    event VoteCast(uint256 indexed e3Id, bytes vote);

    #[derive(Debug)]
    event PublicKeyPublished(uint256 indexed e3Id, bytes committeePublicKey);

    #[derive(Debug)]
    event CiphertextSubmitted(uint256 indexed e3Id, bytes ciphertextOutput);

    #[derive(Debug)]
    event PlaintextSubmitted(uint256 indexed e3Id, bytes plaintextOutput);
}

impl ContractEvent for E3Requested {
    fn process(&self) -> Result<()> {
        println!("Processing E3 request: {:?}", self);
        Ok(())
    }
}

impl ContractEvent for VoteCast {
    fn process(&self) -> Result<()> {
        println!("Processing vote cast: {:?}", self);
        Ok(())
    }
}

impl ContractEvent for PublicKeyPublished {
    fn process(&self) -> Result<()> {
        println!("Processing public key published: {:?}", self);
        Ok(())
    }
}

impl ContractEvent for CiphertextSubmitted {
    fn process(&self) -> Result<()> {
        println!("Processing ciphertext submitted: {:?}", self);
        Ok(())
    }
}

impl ContractEvent for PlaintextSubmitted {
    fn process(&self) -> Result<()> {
        println!("Processing plaintext submitted: {:?}", self);
        Ok(())
    }
}



