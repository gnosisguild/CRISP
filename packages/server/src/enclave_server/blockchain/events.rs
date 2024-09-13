use alloy::{
    primitives::{Address, Bytes, U256}, providers::Provider, sol, sol_types::{SolCall, SolEvent},
    rpc::types::Log
};

use eyre::Result;

use super::listener::ContractEvent;
use super::handlers::{handle_e3, handle_input_published, handle_plaintext_output_published};

sol! {
    #[derive(Debug)]
    event E3Activated(uint256 e3Id, uint256 expiration, bytes committeePublicKey);

    #[derive(Debug)]
    event InputPublished(uint256 indexed e3Id, bytes data, uint256 inputHash, uint256 index);

    #[derive(Debug)]
    event PlaintextOutputPublished(uint256 indexed e3Id, bytes plaintextOutput);
}


impl ContractEvent for E3Activated {
    fn process(&self, log: Log) -> Result<()> {
        println!("Processing E3 request: {:?}", self);

        let event_clone = self.clone();

        tokio::spawn(async move {
            if let Err(e) = handle_e3(event_clone, log).await {
                eprintln!("Error handling E3 request: {:?}", e);
            }
        });


        Ok(())
    }
}

impl ContractEvent for InputPublished {
    fn process(&self, log: Log) -> Result<()> {
        println!("Processing input published: {:?}", self);
        // let event_clone = self.clone();
        // if let Err(e) = handle_input_published(event_clone) {
        //     eprintln!("Error handling input published: {:?}", e);
        // }
        Ok(())
    }
}

impl ContractEvent for PlaintextOutputPublished {
    fn process(&self, log: Log) -> Result<()> {
        println!("Processing public key published: {:?}", self);

        // let event_clone = self.clone();
        // if let Err(e) = handle_plaintext_output_published(event_clone) {
        //     eprintln!("Error handling public key published: {:?}", e);
        // }

        Ok(())
    }
}
