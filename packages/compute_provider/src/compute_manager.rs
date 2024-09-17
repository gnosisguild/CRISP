use crate::ciphertext_output::ComputeProvider;
use crate::compute_input::{ComputeInput, FHEInputs};
use crate::merkle_tree::MerkleTree;
use crate::FHEProcessor;
use rayon::prelude::*;
use sha3::{Digest, Keccak256};
use std::sync::Arc;

pub struct ComputeManager<P>
where
    P: ComputeProvider + Send + Sync,
{
    input: ComputeInput,
    provider: P,
    processor: FHEProcessor,
    use_parallel: bool,
    batch_size: Option<usize>,
}

impl<P> ComputeManager<P>
where
    P: ComputeProvider + Send + Sync,
{
    pub fn new(
        provider: P,
        fhe_inputs: FHEInputs,
        fhe_processor: FHEProcessor,
        use_parallel: bool,
        batch_size: Option<usize>,
    ) -> Self {
        Self {
            provider,
            input: ComputeInput {
                fhe_inputs,
                ciphertext_hash: Vec::new(),
                leaf_hashes: Vec::new(),
                tree_depth: 10,
                zero_node: String::from("0"),
                arity: 2,
            },
            processor: fhe_processor,
            use_parallel,
            batch_size,
        }
    }

    pub fn start(&mut self) -> (P::Output, Vec<u8>) {
        if self.use_parallel {
            self.start_parallel()
        } else {
            self.start_sequential()
        }
    }

    fn start_sequential(&mut self) -> (P::Output, Vec<u8>) {
        let mut tree_handler = MerkleTree::new(
            self.input.tree_depth,
            self.input.zero_node.clone(),
            self.input.arity,
        );
        tree_handler.compute_leaf_hashes(&self.input.fhe_inputs.ciphertexts);
        self.input.leaf_hashes = tree_handler.leaf_hashes.clone();

        // Compute the ciphertext
        let ciphertext = (self.processor)(&self.input.fhe_inputs);

        // Compute the hash of the ciphertext
        self.input.ciphertext_hash = Keccak256::digest(&ciphertext).to_vec();

        (self.provider.prove(&self.input), ciphertext)
    }

    fn start_parallel(&self) -> (P::Output, Vec<u8>) {
        let batch_size = self.batch_size.unwrap_or(1);
        let parallel_tree_depth = (batch_size as f64).log2().ceil() as usize;

        let ciphertexts = Arc::new(self.input.fhe_inputs.ciphertexts.clone());
        let params = Arc::new(self.input.fhe_inputs.params.clone());

        let chunks: Vec<Vec<Vec<u8>>> = ciphertexts
            .chunks(batch_size)
            .map(|chunk| chunk.to_vec())
            .collect();

        let tally_results: Vec<(P::Output, Vec<u8>, String)> = chunks
            .into_par_iter()
            .map(|chunk| {
                let mut tree_handler = MerkleTree::new(parallel_tree_depth, "0".to_string(), 2);
                tree_handler.compute_leaf_hashes(&chunk);
                let merkle_root = tree_handler.build_tree().root().unwrap();

                let fhe_inputs = FHEInputs {
                    ciphertexts: chunk.clone(),
                    params: params.to_vec(),
                };

                let ciphertext = (self.processor)(&fhe_inputs);
                let ciphertext_hash = Keccak256::digest(&ciphertext).to_vec();

                let input = ComputeInput {
                    fhe_inputs,
                    ciphertext_hash,
                    leaf_hashes: tree_handler.leaf_hashes.clone(),
                    tree_depth: parallel_tree_depth,
                    zero_node: "0".to_string(),
                    arity: 2,
                };

                (self.provider.prove(&input), ciphertext, merkle_root)
            })
            .collect();

        // Combine the sorted results for final computation
        // The final depth is the input tree depth minus the parallel tree depth
        let final_depth = self.input.tree_depth - parallel_tree_depth;
        // The leaf hashes are the hashes of the merkle roots of the parallel trees
        let leaf_hashes: Vec<String> = tally_results
            .iter()
            .map(|result| result.2.clone())
            .collect();
        // The params are the same for all parallel trees
        let fhe_inputs = FHEInputs {
            // The ciphertexts are the final ciphertexts of the parallel trees
            ciphertexts: tally_results
                .iter()
                .map(|result| result.1.clone())
                .collect(),
            params: params.to_vec(),
        };

        let ciphertext = (self.processor)(&fhe_inputs);
        let ciphertext_hash = Keccak256::digest(&ciphertext).to_vec();

        let mut final_input = ComputeInput {
            fhe_inputs,
            ciphertext_hash,
            leaf_hashes: leaf_hashes.clone(),
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

        (self.provider.prove(&final_input), ciphertext)
    }
}
