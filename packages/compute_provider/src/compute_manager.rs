use crate::provider::ComputeProvider;
use crate::compute_input::{ComputeInput,FHEInputs};
use crate::merkle_tree::MerkleTree;
use crate::ComputeOutput;
use rayon::prelude::*;
use std::sync::Arc;

pub struct ComputeManager<P>
where
    P: ComputeProvider + Send + Sync,
{
    input: ComputeInput,
    provider: P,
    use_parallel: bool,
    batch_size: Option<usize>,
}

impl<P> ComputeManager<P>
where
    P: ComputeProvider + Send + Sync
{
    pub fn new(provider: P, fhe_inputs: FHEInputs, use_parallel: bool, batch_size: Option<usize>) -> Self {
        Self {
            provider,
            input: ComputeInput {
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

    pub fn start(&mut self) -> P::Output {
        if self.use_parallel {
            self.start_parallel()
        } else {
            self.start_sequential()
        }
    }

    fn start_sequential(&mut self) -> P::Output {
        let mut tree_handler = MerkleTree::new(
            self.input.tree_depth,
            self.input.zero_node.clone(),
            self.input.arity,
        );
        tree_handler.compute_leaf_hashes(&self.input.fhe_inputs.ciphertexts);
        self.input.leaf_hashes = tree_handler.leaf_hashes.clone();

        self.provider.prove(&self.input)
    }

    fn start_parallel(&self) -> P::Output {
        let batch_size = self.batch_size.unwrap_or(1);
        let parallel_tree_depth = (batch_size as f64).log2().ceil() as usize;

        let ciphertexts = Arc::new(self.input.fhe_inputs.ciphertexts.clone());
        let params = Arc::new(self.input.fhe_inputs.params.clone());

        let chunks: Vec<Vec<Vec<u8>>> = ciphertexts
            .chunks(batch_size)
            .map(|chunk| chunk.to_vec())
            .collect();

        let tally_results: Vec<P::Output> = chunks
            .into_par_iter()
            .map(|chunk| {
                let mut tree_handler = MerkleTree::new(parallel_tree_depth, "0".to_string(), 2);
                tree_handler.compute_leaf_hashes(&chunk);

                let input = ComputeInput {
                    fhe_inputs: FHEInputs {
                        ciphertexts: chunk.clone(),
                        params: params.to_vec(), // Params are shared across chunks
                    },
                    leaf_hashes: tree_handler.leaf_hashes.clone(),
                    tree_depth: parallel_tree_depth,
                    zero_node: "0".to_string(),
                    arity: 2,
                };

                self.provider.prove(&input)
            })
            .collect();

        // Combine the sorted results for final computation
        let final_depth = self.input.tree_depth - parallel_tree_depth;
        let mut final_input = ComputeInput {
            fhe_inputs: FHEInputs {
                ciphertexts: tally_results
                    .iter()
                    .map(|result| result.ciphertext().clone())
                    .collect(),
                params: params.to_vec(),
            },
            leaf_hashes: tally_results
                .iter()
                .map(|result| result.merkle_root().clone())
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

        self.provider.prove(&final_input)
    }
}
