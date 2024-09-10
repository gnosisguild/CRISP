// src/lib.rs

use alloy_sol_types::{sol, SolInterface, SolValue};
use alloy_primitives::U256;
use anyhow::{Context, Result};
use ethers::prelude::*;
use methods::VOTING_ELF;
use compute_provider_host::ComputeProvider;
use compute_provider_core::CiphertextInputs;

sol! {
    interface ICRISPRisc0 {
        function verify(uint256 e3Id, bytes memory data);
    }
}

pub struct TxSender {
    chain_id: u64,
    client: SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>,
    contract: Address,
}

impl TxSender {
    pub fn new(chain_id: u64, rpc_url: &str, private_key: &str, contract: &str) -> Result<Self> {
        let provider = Provider::<Http>::try_from(rpc_url)?;
        let wallet: LocalWallet = private_key.parse::<LocalWallet>()?.with_chain_id(chain_id);
        let client = SignerMiddleware::new(provider.clone(), wallet.clone());
        let contract = contract.parse::<Address>()?;

        Ok(TxSender {
            chain_id,
            client,
            contract,
        })
    }

    pub async fn send(&self, calldata: Vec<u8>) -> Result<Option<TransactionReceipt>> {
        let tx = TransactionRequest::new()
            .chain_id(self.chain_id)
            .to(self.contract)
            .from(self.client.address())
            .data(calldata);

        log::info!("Transaction request: {:?}", &tx);

        let tx = self.client.send_transaction(tx, None).await?.await?;

        log::info!("Transaction receipt: {:?}", &tx);

        Ok(tx)
    }
}

pub struct PublisherParams {
    pub chain_id: u64,
    pub eth_wallet_private_key: String,
    pub rpc_url: String,
    pub contract: String,
    pub e3_id: U256,
    pub ciphertexts: Vec<Vec<u8>>,
    pub params: Vec<u8>,
}

pub fn publish_proof(params: PublisherParams) -> Result<()> {
    env_logger::init();

    let tx_sender = TxSender::new(
        params.chain_id,
        &params.rpc_url,
        &params.eth_wallet_private_key,
        &params.contract,
    )?;

    let ciphertext_inputs = CiphertextInputs {
        ciphertexts: params.ciphertexts,
        params: params.params,
    };

    let mut provider = ComputeProvider::new(ciphertext_inputs, false, None);
    let (result, seal) = provider.start(VOTING_ELF);

    // TODO: Fix this call, Encode Result and Seal into a single calldata
    let calldata = ICRISPRisc0::ICRISPRisc0Calls::verify(ICRISPRisc0::verifyCall {
        e3Id: params.e3_id,
        data: seal.into(),
    })
    .abi_encode();

    let runtime = tokio::runtime::Runtime::new()?;

    runtime.block_on(tx_sender.send(calldata))?;

    Ok(())
}
