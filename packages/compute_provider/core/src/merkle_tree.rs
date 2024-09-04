use sha3::{Digest, Keccak256};
use num_bigint::BigUint;
use num_traits::Num;
use ark_bn254::Fr;
use ark_ff::{BigInt, BigInteger};
use light_poseidon::{Poseidon, PoseidonHasher};
use zk_kit_imt::imt::IMT;
use std::str::FromStr;

pub struct MerkleTree {
    pub leaf_hashes: Vec<String>,
    pub tree_depth: usize,
    pub zero_node: String,
    pub arity: usize,
}

impl MerkleTree {
    pub fn new(tree_depth: usize, zero_node: String, arity: usize) -> Self {
        Self {
            leaf_hashes: Vec::new(),
            tree_depth,
            zero_node,
            arity,
        }
    }

    pub fn compute_leaf_hashes(&mut self, data: &[Vec<u8>]) {
        for item in data {
            let mut keccak_hasher = Keccak256::new();
            keccak_hasher.update(item);
            let hex_output = hex::encode(keccak_hasher.finalize());
            let sanitized_hex = hex_output.trim_start_matches("0x");
            let numeric_value = BigUint::from_str_radix(sanitized_hex, 16)
                .unwrap()
                .to_string();
            let fr_element = Fr::from_str(&numeric_value).unwrap();
            let zero_element = Fr::from_str("0").unwrap();
            let mut poseidon_instance = Poseidon::<Fr>::new_circom(2).unwrap();
            let hash_bigint: BigInt<4> = poseidon_instance
                .hash(&[fr_element, zero_element])
                .unwrap()
                .into();
            let hash = hex::encode(hash_bigint.to_bytes_be());
            self.leaf_hashes.push(hash);
        }
    }

    fn poseidon_hash(nodes: Vec<String>) -> String {
        let mut poseidon = Poseidon::<Fr>::new_circom(2).unwrap();
        let mut field_elements = Vec::new();

        for node in nodes {
            let sanitized_node = node.trim_start_matches("0x");
            let numeric_str = BigUint::from_str_radix(sanitized_node, 16)
                .unwrap()
                .to_string();
            let field_repr = Fr::from_str(&numeric_str).unwrap();
            field_elements.push(field_repr);
        }

        let result_hash: BigInt<4> = poseidon.hash(&field_elements).unwrap().into();
        hex::encode(result_hash.to_bytes_be())
    }

    pub fn zeroes(&self) -> Vec<String> {
        let mut zeroes = Vec::new();
        let mut current_zero = self.zero_node.clone();
        for _ in 0..self.tree_depth {
            zeroes.push(current_zero.clone());
            current_zero = Self::poseidon_hash(vec![current_zero; self.arity]);
        }
        zeroes
    }

    pub fn build_tree(&self) -> IMT {
        IMT::new(
            Self::poseidon_hash,
            self.tree_depth,
            self.zero_node.clone(),
            self.arity,
            self.leaf_hashes.clone(),
        )
        .unwrap()
    }
}
