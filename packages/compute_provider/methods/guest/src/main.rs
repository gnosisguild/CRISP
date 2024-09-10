use risc0_zkvm::guest::env;
use compute_provider_core::{ComputationInput, CiphertextInputs, ComputationResult};


fn main() {
    let input: ComputationInput<CiphertextInputs> = env::read();
    
    let result: ComputationResult = input.process();

    env::commit(&result);
}
