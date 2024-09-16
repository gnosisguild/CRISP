mod compute_input;
mod compute_manager;
mod merkle_tree;
mod provider;

pub use compute_manager::*;
pub use compute_input::*;
pub use provider::*;

use fhe::bfv::{BfvParameters, Ciphertext};
use fhe_traits::{Deserialize, DeserializeParametrized, Serialize};
use std::sync::Arc;


// Example Implementation of the CiphertextProcessor function
pub fn default_fhe_processor(fhe_inputs: &FHEInputs) -> Vec<u8> {
    let params = Arc::new(BfvParameters::try_deserialize(&fhe_inputs.params).unwrap());

    let mut sum = Ciphertext::zero(&params);
    for ciphertext_bytes in &fhe_inputs.ciphertexts {
        let ciphertext = Ciphertext::from_bytes(ciphertext_bytes, &params).unwrap();
        sum += &ciphertext;
    }

    sum.to_bytes()
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use compute_provider_methods::COMPUTE_PROVIDER_ELF;
//     use fhe::bfv::{
//         BfvParameters, BfvParametersBuilder, Ciphertext, Encoding, Plaintext, PublicKey, SecretKey,
//     };
//     use fhe_traits::{
//         DeserializeParametrized, FheDecoder, FheDecrypter, FheEncoder, FheEncrypter, Serialize,
//     };
//     use rand::thread_rng;
//     use std::sync::Arc;

//     #[test]
//     fn test_compute_provider() {
//         let params = create_params();
//         let (sk, pk) = generate_keys(&params);
//         let inputs = vec![1, 1, 0, 1];
//         let ciphertexts = encrypt_inputs(&inputs, &pk, &params);

//         let ciphertext_inputs = FHEInputs {
//             ciphertexts: ciphertexts.iter().map(|c| c.to_bytes()).collect(),
//             params: params.to_bytes(),
//         };

//         let mut provider = ComputeProvider::new(ciphertext_inputs, false, None);
//         let (result, _seal) = provider.start(COMPUTE_PROVIDER_ELF);

//         let tally = decrypt_result(&result, &sk, &params);

//         assert_eq!(tally, inputs.iter().sum::<u64>());
//     }

//     fn create_params() -> Arc<BfvParameters> {
//         BfvParametersBuilder::new()
//             .set_degree(1024)
//             .set_plaintext_modulus(65537)
//             .set_moduli(&[1152921504606584833])
//             .build_arc()
//             .expect("Failed to build parameters")
//     }

//     fn generate_keys(params: &Arc<BfvParameters>) -> (SecretKey, PublicKey) {
//         let mut rng = thread_rng();
//         let sk = SecretKey::random(params, &mut rng);
//         let pk = PublicKey::new(&sk, &mut rng);
//         (sk, pk)
//     }

//     fn encrypt_inputs(
//         inputs: &[u64],
//         pk: &PublicKey,
//         params: &Arc<BfvParameters>,
//     ) -> Vec<Ciphertext> {
//         let mut rng = thread_rng();
//         inputs
//             .iter()
//             .map(|&input| {
//                 let pt = Plaintext::try_encode(&[input], Encoding::poly(), params)
//                     .expect("Failed to encode plaintext");
//                 pk.try_encrypt(&pt, &mut rng).expect("Failed to encrypt")
//             })
//             .collect()
//     }

//     fn decrypt_result(
//         result: &ComputationResult,
//         sk: &SecretKey,
//         params: &Arc<BfvParameters>,
//     ) -> u64 {
//         let ct = Ciphertext::from_bytes(&result.ciphertext, params)
//             .expect("Failed to deserialize ciphertext");
//         let decrypted = sk.try_decrypt(&ct).expect("Failed to decrypt");
//         Vec::<u64>::try_decode(&decrypted, Encoding::poly()).expect("Failed to decode")[0]
//     }
// }
