use risc0_zkvm::guest::env;
use compute_provider::{ComputeInput, ComputeResult};
use voting_core::fhe_processor;

fn main() {
    let input: ComputeInput = env::read();
    
    let result: ComputeResult = input.process(fhe_processor);

    env::commit(&result);
}
