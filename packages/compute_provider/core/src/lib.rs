use std::{str::FromStr, sync::Arc};

use num_bigint::BigUint;
use num_traits::Num;
use sha3::{Digest, Keccak256};

use ark_bn254::Fr;
use ark_ff::{BigInt, BigInteger};
use fhe::bfv::{BfvParameters, Ciphertext};
use fhe_traits::{Deserialize, DeserializeParametrized, Serialize};
use light_poseidon::{Poseidon, PoseidonHasher};
use zk_kit_imt::imt::IMT;


#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TallyResult {
    pub tallied_ciphertext: Vec<u8>,
    pub merkle_root: String,
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

        let merkle_root = self.compute_merkle_root();

        TallyResult {
            tallied_ciphertext: tally.to_bytes(),
            merkle_root,
        }
    }

    fn compute_merkle_root(&self) -> String {
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

        const ZERO: &str = "0";
        const DEPTH: usize = 32;
        const ARITY: usize = 2;

        let mut tree = IMT::new(
            poseidon_hash,
            DEPTH,
            ZERO.to_string(),
            ARITY,
            vec![],
        )
        .unwrap();

        let mut poseidon_instance = Poseidon::<Fr>::new_circom(2).unwrap();

        for ciphertext in &self.ciphertexts {
            let mut keccak_hasher = Keccak256::new();
            keccak_hasher.update(ciphertext);
            let hex_output = hex::encode(keccak_hasher.finalize());
            let sanitized_hex = hex_output.trim_start_matches("0x");
            let numeric_value = BigUint::from_str_radix(sanitized_hex, 16)
                .unwrap()
                .to_string();
            let fr_element = Fr::from_str(&numeric_value).unwrap();
            let zero_element = Fr::from_str("0").unwrap();
            let hash_bigint: BigInt<4> = poseidon_instance
                .hash(&[fr_element, zero_element])
                .unwrap().into();
            let data_hash = hex::encode(hash_bigint.to_bytes_be());
            tree.insert(data_hash).unwrap();
        }

        tree.root().expect("Failed to get root from IMT")
    }
}
