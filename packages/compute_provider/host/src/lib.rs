use compute_provider_core::{CiphertextInput, TallyResult};
use methods::COMPUTE_PROVIDER_ELF;
use risc0_ethereum_contracts::groth16;
use risc0_zkvm::{default_prover, ExecutorEnv, ProverOpts, VerifierContext};

#[derive(Debug)]
pub struct ComputeProvider {
    input: CiphertextInput,
}

impl ComputeProvider {
    pub fn new(input: CiphertextInput) -> Self {
        Self { input }
    }

    pub fn start(&self) -> (TallyResult, Vec<u8>) {
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
            .init();

        let env = ExecutorEnv::builder()
            .write(&self.input)
            .unwrap()
            .build()
            .unwrap();

        let receipt = default_prover()
            .prove_with_ctx(
                env,
                &VerifierContext::default(),
                COMPUTE_PROVIDER_ELF,
                &ProverOpts::groth16(),
            )
            .unwrap()
            .receipt;

        let seal = groth16::encode(receipt.inner.groth16().unwrap().seal.clone()).unwrap();

        (receipt.journal.decode().unwrap(), seal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use compute_provider_core::CiphertextInput;
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
        let inputs = vec![1, 1, 0];
        let ciphertexts = encrypt_inputs(&inputs, &pk, &params);

        let input = create_input(&ciphertexts, &params);
        let provider = ComputeProvider::new(input);
        let result = provider.input.process();

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

    fn create_input(ciphertexts: &[Ciphertext], params: &Arc<BfvParameters>) -> CiphertextInput {
        CiphertextInput {
            ciphertexts: ciphertexts.iter().map(|c| c.to_bytes()).collect(),
            params: params.to_bytes(),
        }
    }

    fn decrypt_result(result: &TallyResult, sk: &SecretKey, params: &Arc<BfvParameters>) -> u64 {
        let ct = Ciphertext::from_bytes(&result.tallied_ciphertext, params)
            .expect("Failed to deserialize ciphertext");
        let decrypted = sk.try_decrypt(&ct).expect("Failed to decrypt");
        Vec::<u64>::try_decode(&decrypted, Encoding::poly()).expect("Failed to decode")[0]
    }
}
