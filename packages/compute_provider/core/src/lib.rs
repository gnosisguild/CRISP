pub mod merkle_tree;

use merkle_tree::MerkleTree;
use std::sync::Arc;
use fhe::bfv::{BfvParameters, Ciphertext};
use fhe_traits::{Deserialize, DeserializeParametrized, Serialize};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ComputationResult {
    pub ciphertext: Vec<u8>,
    pub merkle_root: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ComputationInput {
    pub ciphertexts: Vec<Vec<u8>>,
    pub params: Vec<u8>,
    pub leaf_hashes: Vec<String>,
    pub tree_depth: usize,
    pub zero_node: String,
    pub arity: usize,
}

impl ComputationInput {
    pub fn process(&self) -> ComputationResult {
        // Deserialize the parameters
        let params = Arc::new(BfvParameters::try_deserialize(&self.params).unwrap());

        // Tally the ciphertexts
        let mut sum = Ciphertext::zero(&params);
        for ciphertext_bytes in &self.ciphertexts {
            let ciphertext = Ciphertext::from_bytes(ciphertext_bytes, &params).unwrap();
            sum += &ciphertext;
        }
        let tally: Arc<Ciphertext> = Arc::new(sum);

        let merkle_root = MerkleTree {
            leaf_hashes: self.leaf_hashes.clone(),
            tree_depth: self.tree_depth,
            zero_node: self.zero_node.clone(),
            arity: self.arity,
        }.build_tree().root().unwrap();

        ComputationResult {
            ciphertext: tally.to_bytes(),
            merkle_root
        }
    }
}
