use compute_provider_core::{
    merkle_tree::MerkleTree, FHEInputs, ComputationInput,
    ComputationResult,
};
use rayon::prelude::*;
use risc0_ethereum_contracts::groth16;
use risc0_zkvm::{default_prover, ExecutorEnv, ProverOpts, VerifierContext};
use std::sync::Arc;

pub struct ComputeProvider {
    input: ComputationInput,
    use_parallel: bool,
    batch_size: Option<usize>,
}

impl ComputeProvider {
    pub fn new(fhe_inputs: FHEInputs, use_parallel: bool, batch_size: Option<usize>) -> Self {
        Self {
            input: ComputationInput {
                fhe_inputs,
                leaf_hashes: Vec::new(),
                tree_depth: 10,
                zero_node: String::from("0"),
                arity: 2,
            },
            use_parallel,
            batch_size,
        }
    }

    pub fn start(&mut self, elf: &[u8]) -> (ComputationResult, Vec<u8>) {
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
            .init();

        if self.use_parallel {
            self.start_parallel(elf)
        } else {
            self.start_sequential(elf)
        }
    }

    fn start_sequential(&mut self, elf: &[u8]) -> (ComputationResult, Vec<u8>) {
        let mut tree_handler = MerkleTree::new(
            self.input.tree_depth,
            self.input.zero_node.clone(),
            self.input.arity,
        );
        tree_handler.compute_leaf_hashes(&self.input.fhe_inputs.ciphertexts);
        self.input.leaf_hashes = tree_handler.leaf_hashes.clone();

        let env = ExecutorEnv::builder()
            .write(&self.input)
            .unwrap()
            .build()
            .unwrap();

        let receipt = default_prover()
            .prove_with_ctx(
                env,
                &VerifierContext::default(),
                elf,
                &ProverOpts::groth16(),
            )
            .unwrap()
            .receipt;

        let seal = groth16::encode(receipt.inner.groth16().unwrap().seal.clone()).unwrap();

        (receipt.journal.decode().unwrap(), seal)
    }

    fn start_parallel(&self, elf: &[u8]) -> (ComputationResult, Vec<u8>) {
        let batch_size = self.batch_size.unwrap_or(1);
        let parallel_tree_depth = (batch_size as f64).log2().ceil() as usize;

        let ciphertexts = Arc::new(self.input.fhe_inputs.ciphertexts.clone());
        let params = Arc::new(self.input.fhe_inputs.params.clone());

        let chunks: Vec<Vec<Vec<u8>>> = ciphertexts
            .chunks(batch_size)
            .map(|chunk| chunk.to_vec())
            .collect();

        let tally_results: Vec<ComputationResult> = chunks
            .into_par_iter()
            .map(|chunk| {
                let mut tree_handler = MerkleTree::new(parallel_tree_depth, "0".to_string(), 2);
                tree_handler.compute_leaf_hashes(&chunk);

                let input = ComputationInput {
                    fhe_inputs: FHEInputs {
                        ciphertexts: chunk.clone(),
                        params: params.to_vec(), // Params are shared across chunks
                    },
                    leaf_hashes: tree_handler.leaf_hashes.clone(),
                    tree_depth: parallel_tree_depth,
                    zero_node: "0".to_string(),
                    arity: 2,
                };

                let env = ExecutorEnv::builder()
                    .write(&input)
                    .unwrap()
                    .build()
                    .unwrap();

                let receipt = default_prover()
                    .prove_with_ctx(
                        env,
                        &VerifierContext::default(),
                        elf,
                        &ProverOpts::groth16(),
                    )
                    .unwrap()
                    .receipt;

                receipt.journal.decode().unwrap()
            })
            .collect();

        // Combine the sorted results for final computation
        let final_depth = self.input.tree_depth - parallel_tree_depth;
        let mut final_input = ComputationInput {
            fhe_inputs: FHEInputs {
                ciphertexts: tally_results
                    .iter()
                    .map(|result| result.ciphertext.clone())
                    .collect(),
                params: params.to_vec(),
            },
            leaf_hashes: tally_results
                .iter()
                .map(|result| result.merkle_root.clone())
                .collect(),
            tree_depth: final_depth,
            zero_node: String::from("0"),
            arity: 2,
        };

        let final_tree_handler = MerkleTree::new(
            final_depth,
            final_input.zero_node.clone(),
            final_input.arity,
        );
        final_input.zero_node = final_tree_handler.zeroes()[parallel_tree_depth].clone();

        let env = ExecutorEnv::builder()
            .write(&final_input)
            .unwrap()
            .build()
            .unwrap();

        let receipt = default_prover()
            .prove_with_ctx(
                env,
                &VerifierContext::default(),
                elf,
                &ProverOpts::groth16(),
            )
            .unwrap()
            .receipt;

        let combined_seal = groth16::encode(receipt.inner.groth16().unwrap().seal.clone()).unwrap();
        (receipt.journal.decode().unwrap(), combined_seal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use compute_provider_methods::COMPUTE_PROVIDER_ELF;
    use fhe::bfv::{
        BfvParameters, BfvParametersBuilder, Ciphertext, Encoding, Plaintext, PublicKey, SecretKey,
    };
    use fhe_traits::{
        DeserializeParametrized, FheDecoder, FheDecrypter, FheEncoder, FheEncrypter, Serialize,
    };
    use rand::thread_rng;
    use std::sync::Arc;

    #[test]
    fn test_compute_provider() {
        let params = create_params();
        let (sk, pk) = generate_keys(&params);
        let inputs = vec![1, 1, 0, 1];
        let ciphertexts = encrypt_inputs(&inputs, &pk, &params);

        let ciphertext_inputs = FHEInputs {
            ciphertexts: ciphertexts.iter().map(|c| c.to_bytes()).collect(),
            params: params.to_bytes(),
        };

        let mut provider = ComputeProvider::new(ciphertext_inputs, false, None);
        let (result, _seal) = provider.start(COMPUTE_PROVIDER_ELF);

        let tally = decrypt_result(&result, &sk, &params);

        assert_eq!(tally, inputs.iter().sum::<u64>());
    }

    fn create_params() -> Arc<BfvParameters> {
        BfvParametersBuilder::new()
            .set_degree(1024)
            .set_plaintext_modulus(65537)
            .set_moduli(&[1152921504606584833])
            .build_arc()
            .expect("Failed to build parameters")
    }

    fn generate_keys(params: &Arc<BfvParameters>) -> (SecretKey, PublicKey) {
        let mut rng = thread_rng();
        let sk = SecretKey::random(params, &mut rng);
        let pk = PublicKey::new(&sk, &mut rng);
        (sk, pk)
    }

    fn encrypt_inputs(
        inputs: &[u64],
        pk: &PublicKey,
        params: &Arc<BfvParameters>,
    ) -> Vec<Ciphertext> {
        let mut rng = thread_rng();
        inputs
            .iter()
            .map(|&input| {
                let pt = Plaintext::try_encode(&[input], Encoding::poly(), params)
                    .expect("Failed to encode plaintext");
                pk.try_encrypt(&pt, &mut rng).expect("Failed to encrypt")
            })
            .collect()
    }

    fn decrypt_result(
        result: &ComputationResult,
        sk: &SecretKey,
        params: &Arc<BfvParameters>,
    ) -> u64 {
        let ct = Ciphertext::from_bytes(&result.ciphertext, params)
            .expect("Failed to deserialize ciphertext");
        let decrypted = sk.try_decrypt(&ct).expect("Failed to decrypt");
        Vec::<u64>::try_decode(&decrypted, Encoding::poly()).expect("Failed to decode")[0]
    }
}
