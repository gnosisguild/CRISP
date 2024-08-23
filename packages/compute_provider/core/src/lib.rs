use risc0_zkp::core::digest::Digest;
use risc0_zkvm::sha::{Impl, Sha256};
use fhe::bfv::{Ciphertext, BfvParameters};
use fhe_traits::{DeserializeParametrized, Serialize, Deserialize};
use std::sync::Arc;

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TallyResult {
    pub tallied_ciphertext: Vec<u8>,
    pub ciphertexts_digest: Digest
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CiphertextInput {
    pub ciphertexts: Vec<Vec<u8>>,
    pub params: Vec<u8>,
}

impl CiphertextInput {
    pub fn process(&self) -> TallyResult {
        // Deserialize the parameters
        let params = Arc::new(BfvParameters::try_deserialize(&self.params).unwrap());

        // Tally the ciphertexts
        let mut sum = Ciphertext::zero(&params);
        for ciphertext_bytes in &self.ciphertexts {
            let ciphertext = Ciphertext::from_bytes(ciphertext_bytes, &params).unwrap();
            sum += &ciphertext;
        }
        let tally: Arc<Ciphertext> = Arc::new(sum);

        // Compute the digest of the ciphertexts
        let digest = *Impl::hash_bytes(&self.ciphertexts.concat());

        TallyResult {
            tallied_ciphertext: tally.to_bytes(),
            ciphertexts_digest: digest
        }
    }
    
}