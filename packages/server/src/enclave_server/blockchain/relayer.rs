use alloy::{
    network::{Ethereum, EthereumWallet},
    primitives::{Address, Bytes, U256},
    providers::fillers::{
        ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, WalletFiller,
    },
    providers::{Identity, Provider, ProviderBuilder, RootProvider},
    rpc::types::TransactionReceipt,
    signers::local::PrivateKeySigner,
    sol,
    transports::BoxTransport,
};
use eyre::Result;
use std::sync::Arc;
use crate::enclave_server::CONFIG;

sol! {
    #[derive(Debug)]
    struct E3 {
        uint256 seed;
        uint32[2] threshold;
        uint256[2] startWindow;
        uint256 duration;
        uint256 expiration;
        address e3Program;
        bytes e3ProgramParams;
        address inputValidator;
        address decryptionVerifier;
        bytes committeePublicKey;
        bytes ciphertextOutput;
        bytes plaintextOutput;
    }

    #[derive(Debug)]
    #[sol(rpc)]
    contract Enclave {
        uint256 public nexte3Id = 0;

        function request(address filter, uint32[2] calldata threshold, uint256[2] calldata startWindow, uint256 duration, address e3Program, bytes memory e3ProgramParams, bytes memory computeProviderParams) external payable returns (uint256 e3Id, E3 memory e3);

        function activate(uint256 e3Id, bytes memory pubKey) external returns (bool success);

        function publishInput(uint256 e3Id, bytes memory data ) external returns (bool success);

        function publishCiphertextOutput(uint256 e3Id, bytes calldata data, bytes memory proof) external returns (bool success);

        function publishPlaintextOutput(uint256 e3Id, bytes memory data) external returns (bool success);

        function getE3(uint256 e3Id) external view returns (E3 memory e3);
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

pub struct EnclaveContract {
    provider: Arc<CRISPProvider>,
    contract_address: Address,
}

impl EnclaveContract {
    pub async fn new() -> Result<Self> {
        let signer: PrivateKeySigner = CONFIG.private_key.parse()?;
        let wallet = EthereumWallet::from(signer.clone());
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_builtin(&CONFIG.http_rpc_url)
            .await?;

        Ok(Self {
            provider: Arc::new(provider),
            contract_address: CONFIG.contract_address.parse()?,
        })
    }

    pub async fn request_e3(
        &self,
        filter: Address,
        threshold: [u32; 2],
        start_window: [U256; 2],
        duration: U256,
        e3_program: Address,
        e3_params: Bytes,
        compute_provider_params: Bytes,
    ) -> Result<TransactionReceipt> {
        let contract = Enclave::new(self.contract_address, &self.provider);
        let builder = contract.request(
            filter,
            threshold,
            start_window,
            duration,
            e3_program,
            e3_params,
            compute_provider_params,
        );
        let receipt = builder.send().await?.get_receipt().await?;
        Ok(receipt)
    }

    pub async fn activate_e3(&self, e3_id: U256, pub_key: Bytes) -> Result<TransactionReceipt> {
        let contract = Enclave::new(self.contract_address, &self.provider);
        let builder = contract.activate(e3_id, pub_key);
        let receipt = builder.send().await?.get_receipt().await?;
        Ok(receipt)
    }

    pub async fn publish_input(&self, e3_id: U256, data: Bytes) -> Result<TransactionReceipt> {
        let contract = Enclave::new(self.contract_address, &self.provider);
        let builder = contract.publishInput(e3_id, data);
        let receipt = builder.send().await?.get_receipt().await?;
        Ok(receipt)
    }

    pub async fn publish_ciphertext_output(
        &self,
        e3_id: U256,
        data: Bytes,
        proof: Bytes,
    ) -> Result<TransactionReceipt> {
        let contract = Enclave::new(self.contract_address, &self.provider);
        let builder = contract.publishCiphertextOutput(e3_id, data, proof);
        let receipt = builder.send().await?.get_receipt().await?;
        Ok(receipt)
    }

    pub async fn publish_plaintext_output(
        &self,
        e3_id: U256,
        data: Bytes,
    ) -> Result<TransactionReceipt> {
        let contract = Enclave::new(self.contract_address, &self.provider);
        let builder = contract.publishPlaintextOutput(e3_id, data);
        let receipt = builder.send().await?.get_receipt().await?;
        Ok(receipt)
    }

    pub async fn get_e3_id(&self) -> Result<U256> {
        let contract = Enclave::new(self.contract_address, &self.provider);
        let e3_id = contract.nexte3Id().call().await?;
        Ok(e3_id.nexte3Id)
    }

    pub async fn get_e3(&self, e3_id: U256) -> Result<E3> {
        let contract = Enclave::new(self.contract_address, &self.provider);
        let e3_return = contract.getE3(e3_id).call().await?;
        Ok(e3_return.e3)
    }

    pub async fn get_latest_block(&self) -> Result<u64> {
        let block = self.provider.get_block_number().await?;
        Ok(block)
    }
}
