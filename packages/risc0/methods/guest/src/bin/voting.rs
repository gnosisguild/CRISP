use risc0_zkvm::guest::env;
use compute_provider::{ComputeInput, ComputeResult, default_fhe_processor};


fn main() {
    let input: ComputeInput = env::read();
    
    let result: ComputeResult = input.process(default_fhe_processor);

    env::commit(&result);
}
