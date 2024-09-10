pub mod merkle_tree;

use fhe::bfv::{BfvParameters, Ciphertext};
use fhe_traits::{Deserialize, DeserializeParametrized, Serialize};
use merkle_tree::MerkleTree;
use std::sync::Arc;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ComputationResult {
    pub ciphertext: Vec<u8>,
    pub merkle_root: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CiphertextInputs {
    pub ciphertexts: Vec<Vec<u8>>,
    pub params: Vec<u8>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ComputationInput<C: CiphertextProcessor + serde::ser::Serialize> {
    pub ciphertexts: C,
    pub leaf_hashes: Vec<String>,
    pub tree_depth: usize,
    pub zero_node: String,
    pub arity: usize,
}

pub trait CiphertextProcessor {
    fn process_ciphertexts(&self) -> Vec<u8>;

    fn get_ciphertexts(&self) -> &[Vec<u8>];

    fn get_params(&self) -> &[u8];
}

impl<C: CiphertextProcessor + serde::ser::Serialize> ComputationInput<C> {
    pub fn process(&self) -> ComputationResult {
        let processed_ciphertext = self.ciphertexts.process_ciphertexts();

        let merkle_root = MerkleTree {
            leaf_hashes: self.leaf_hashes.clone(),
            tree_depth: self.tree_depth,
            zero_node: self.zero_node.clone(),
            arity: self.arity,
        }
        .build_tree()
        .root()
        .unwrap();

        ComputationResult {
            ciphertext: processed_ciphertext,
            merkle_root,
        }
    }

    pub fn get_ciphertexts(&self) -> &[Vec<u8>] {
        self.ciphertexts.get_ciphertexts()
    }

    pub fn get_params(&self) -> &[u8] {
        self.ciphertexts.get_params()
    }
}

impl CiphertextProcessor for CiphertextInputs {
    /// Default implementation of the process_ciphertexts method
    fn process_ciphertexts(&self) -> Vec<u8> {
        let params = Arc::new(BfvParameters::try_deserialize(&self.params).unwrap());

        let mut sum = Ciphertext::zero(&params);
        for ciphertext_bytes in &self.ciphertexts {
            let ciphertext = Ciphertext::from_bytes(ciphertext_bytes, &params).unwrap();
            sum += &ciphertext;
        }

        sum.to_bytes()
    }

    fn get_ciphertexts(&self) -> &[Vec<u8>] {
        &self.ciphertexts
    }

    fn get_params(&self) -> &[u8] {
        &self.params
    }
}