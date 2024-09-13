pub mod merkle_tree;

use fhe::bfv::{BfvParameters, Ciphertext};
use fhe_traits::{Deserialize, DeserializeParametrized, Serialize};
use merkle_tree::MerkleTree;
use sha3::{Digest, Keccak256};
use std::sync::Arc;

pub type FHEProcessor = fn(&FHEInputs) -> Vec<u8>;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ComputationResult {
    pub ciphertext: Vec<u8>,
    pub params_hash: String,
    pub merkle_root: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FHEInputs {
    pub ciphertexts: Vec<Vec<u8>>,
    pub params: Vec<u8>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ComputationInput {
    pub fhe_inputs: FHEInputs,
    pub leaf_hashes: Vec<String>,
    pub tree_depth: usize,
    pub zero_node: String,
    pub arity: usize,
}

impl ComputationInput {
    pub fn process(&self, fhe_processor: FHEProcessor) -> ComputationResult {
        let processed_ciphertext = (fhe_processor)(&self.fhe_inputs);

        let merkle_root = MerkleTree {
            leaf_hashes: self.leaf_hashes.clone(),
            tree_depth: self.tree_depth,
            zero_node: self.zero_node.clone(),
            arity: self.arity,
        }
        .build_tree()
        .root()
        .unwrap();

        let params_hash = hex::encode(
            Keccak256::new()
                .chain_update(&self.fhe_inputs.params)
                .finalize(),
        );

        ComputationResult {
            ciphertext: processed_ciphertext,
            params_hash,
            merkle_root,
        }
    }
}


// Example Implementation of the CiphertextProcessor function
pub fn default_fhe_processor(fhe_inputs: &FHEInputs) -> Vec<u8> {
    let params = Arc::new(BfvParameters::try_deserialize(&fhe_inputs.params).unwrap());

    let mut sum = Ciphertext::zero(&params);
    for ciphertext_bytes in &fhe_inputs.ciphertexts {
        let ciphertext = Ciphertext::from_bytes(ciphertext_bytes, &params).unwrap();
        sum += &ciphertext;
    }

    sum.to_bytes()
}