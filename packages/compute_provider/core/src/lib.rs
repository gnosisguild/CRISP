use fhe::bfv::{BfvParameters, Ciphertext};
use fhe_traits::{Deserialize, DeserializeParametrized, Serialize};
use risc0_zkvm::sha::{Impl, Sha256};
use std::sync::Arc;
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
        fn hash_function(nodes: Vec<String>) -> String {
            let concatenated = nodes.join("");
            let hash = Impl::hash_bytes(concatenated.as_bytes());
            format!("{:?}", hash)
        }

        let num_leaves = self.ciphertexts.len();
        let arity = 2;
        let depth = (num_leaves as f64).log(arity as f64).ceil() as usize;
        let zero = format!("{:?}", Impl::hash_bytes(&[0u8]));

        let mut tree =
            IMT::new(hash_function, depth, zero, arity, vec![]).expect("Failed to create IMT");

        for ciphertext in &self.ciphertexts {
            let hash = format!("{:?}", Impl::hash_bytes(ciphertext));
            tree.insert(hash).expect("Failed to insert into IMT");
        }

        tree.root().expect("Failed to get root from IMT")
    }
}
