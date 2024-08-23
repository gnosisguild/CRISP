use risc0_zkvm::guest::env;
use compute_provider_core::{CiphertextInput, TallyResult};

fn main() {
    let input: CiphertextInput = env::read();
    
    let result: TallyResult = input.process();

    env::commit(&result);
}
