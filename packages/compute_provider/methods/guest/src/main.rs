use risc0_zkvm::guest::env;
use compute_provider_core::{ComputationInput, ComputationResult, default_fhe_processor};


fn main() {
    let input: ComputationInput = env::read();
    
    let result: ComputationResult = input.process(default_fhe_processor);

    env::commit(&result);
}
