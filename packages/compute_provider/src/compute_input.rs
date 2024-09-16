use crate::merkle_tree::MerkleTree;
use sha3::{Digest, Keccak256};

use crate::provider::ComputeResult;

pub type FHEProcessor = fn(&FHEInputs) -> Vec<u8>;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FHEInputs {
    pub ciphertexts: Vec<Vec<u8>>,
    pub params: Vec<u8>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ComputeInput {
    pub fhe_inputs: FHEInputs,
    pub leaf_hashes: Vec<String>,
    pub tree_depth: usize,
    pub zero_node: String,
    pub arity: usize,
}

impl ComputeInput {
    pub fn process(&self, fhe_processor: FHEProcessor) -> ComputeResult {
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

        ComputeResult {
            ciphertext: processed_ciphertext,
            params_hash,
            merkle_root,
        }
    }
}
