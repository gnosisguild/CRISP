use alloy::{
    network::{EthereumWallet, Ethereum},
    primitives::{address, Address, Bytes, U256},
    providers::fillers::{
        ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, WalletFiller
    },
    providers::{Provider, ProviderBuilder, RootProvider, Identity},
    rpc::types::TransactionReceipt,
    signers::local::PrivateKeySigner,
    sol,
    sol_types::SolCall,
    transports::BoxTransport,
};
use eyre::Result;
use log::info;
use std::sync::Arc;
use tokio::sync::Mutex;

sol! {
    #[derive(Debug)]
    #[sol(rpc)]
    contract CRISPVoting {
        function requestE3(
            uint256 startWindowStart,
            uint256 duration,
            bytes memory e3Params
        ) public;

        function publishPublicKey(uint256 e3Id, bytes memory committeePublicKey) public;

        function castVote(uint256 e3Id, bytes memory vote) public;

        function submitCiphertext(uint256 e3Id, bytes memory ciphertextOutput) public;

        function submitPlaintext(uint256 e3Id, bytes memory plaintextOutput) public;

        function getPublicKey(uint256 e3Id) public view returns (bytes memory);

        function getCiphertextOutput(uint256 e3Id) public view returns (bytes memory);

        function getPlaintextOutput(uint256 e3Id) public view returns (bytes memory);
    }
}

type CRISPProvider = FillProvider<
    JoinFill<
        JoinFill<JoinFill<JoinFill<Identity, GasFiller>, NonceFiller>, ChainIdFiller>,
        WalletFiller<EthereumWallet>,
    >,
    RootProvider<BoxTransport>,
    BoxTransport,
    Ethereum,
>;

pub struct CRISPVotingContract {
    provider: Arc<CRISPProvider>,
    contract_address: Address,
    wallet: PrivateKeySigner,
}

impl CRISPVotingContract {
    pub async fn new(rpc_url: &str, contract_address: &str, private_key: &str) -> Result<Self> {
        let signer: PrivateKeySigner = private_key.parse()?;
        let wallet = EthereumWallet::from(signer.clone());
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_builtin(rpc_url)
            .await?;

        Ok(Self {
            provider: Arc::new(provider),
            contract_address: contract_address.parse()?,
            wallet: signer,
        })
    }

    pub async fn request_e3(
        &self,
        start_window_start: U256,
        duration: U256,
        e3_params: Bytes,
    ) -> Result<TransactionReceipt> {
        let contract = CRISPVoting::new(self.contract_address, &self.provider);
        let builder = contract.requestE3(start_window_start, duration, e3_params);
        let receipt = builder.send().await?.get_receipt().await?;
        Ok(receipt)
    }

    pub async fn publish_public_key(
        &self,
        e3_id: U256,
        committee_public_key: Bytes,
    ) -> Result<TransactionReceipt> {
        let contract = CRISPVoting::new(self.contract_address, &self.provider);
        let builder = contract.publishPublicKey(e3_id, committee_public_key);
        let receipt = builder.send().await?.get_receipt().await?;
        Ok(receipt)
    }

    pub async fn cast_vote(&self, e3_id: U256, vote: Bytes) -> Result<TransactionReceipt> {
        let contract = CRISPVoting::new(self.contract_address, &self.provider);
        let builder = contract.castVote(e3_id, vote);
        let receipt = builder.send().await?.get_receipt().await?;
        Ok(receipt)
    }

    pub async fn submit_ciphertext(
        &self,
        e3_id: U256,
        ciphertext_output: Bytes,
    ) -> Result<TransactionReceipt> {
        let contract = CRISPVoting::new(self.contract_address, &self.provider);
        let builder = contract.submitCiphertext(e3_id, ciphertext_output);
        let receipt = builder.send().await?.get_receipt().await?;
        Ok(receipt)
    }

    pub async fn submit_plaintext(
        &self,
        e3_id: U256,
        plaintext_output: Bytes,
    ) -> Result<TransactionReceipt> {
        let contract = CRISPVoting::new(self.contract_address, &self.provider);
        let builder = contract.submitPlaintext(e3_id, plaintext_output);
        let receipt = builder.send().await?.get_receipt().await?;
        Ok(receipt)
    }

    pub async fn get_public_key(&self, e3_id: U256) -> Result<Bytes> {
        let contract = CRISPVoting::new(self.contract_address, &self.provider);
        let public_key = contract.getPublicKey(e3_id).call().await?;
        Ok(public_key._0)
    }

    pub async fn get_ciphertext_output(&self, e3_id: U256) -> Result<Bytes> {
        let contract = CRISPVoting::new(self.contract_address, &self.provider);
        let ciphertext_output = contract.getCiphertextOutput(e3_id).call().await?;
        Ok(ciphertext_output._0)
    }

    pub async fn get_plaintext_output(&self, e3_id: U256) -> Result<Bytes> {
        let contract = CRISPVoting::new(self.contract_address, &self.provider);
        let plaintext_output = contract.getPlaintextOutput(e3_id).call().await?;
        Ok(plaintext_output._0)
    }
}
