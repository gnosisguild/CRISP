use risc0_zkvm::guest::env;
use compute_provider_core::{ComputationInput, ComputationResult};


fn main() {
    let input: ComputationInput = env::read();
    
    let result: ComputationResult = input.process();

    env::commit(&result);
}
