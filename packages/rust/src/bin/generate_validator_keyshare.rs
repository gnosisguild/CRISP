mod util;

use std::{env, error::Error, process::exit, sync::Arc, fs, path::Path};
use console::style;
use fhe::{
    bfv::{BfvParametersBuilder, Ciphertext, Encoding, Plaintext, PublicKey, SecretKey},
    mbfv::{AggregateIter, CommonRandomPoly, DecryptionShare, PublicKeyShare},
};
use fhe_traits::{FheDecoder, FheEncoder, FheEncrypter, Serialize};
//use fhe_math::rq::{Poly};
use rand::{distributions::Uniform, prelude::Distribution, rngs::OsRng, thread_rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use util::timeit::{timeit, timeit_n};
//use serde::{Deserialize, Serialize};

fn main() -> Result<(), Box<dyn Error>> {
    println!("generating validator keyshare");

    let degree = 4096;
    let plaintext_modulus: u64 = 4096;
    let moduli = vec![0xffffee001, 0xffffc4001, 0x1ffffe0001];

    // Generate a deterministic seed for the Common Poly
    let mut seed = <ChaCha8Rng as SeedableRng>::Seed::default();

    // Let's generate the BFV parameters structure.
    let params = timeit!(
        "Parameters generation",
        BfvParametersBuilder::new()
            .set_degree(degree)
            .set_plaintext_modulus(plaintext_modulus)
            .set_moduli(&moduli)
            .build_arc()?
    );

    let crp = CommonRandomPoly::new_deterministic(&params, seed)?;
    //println!("{:?}", crp);

    // Party setup: each party generates a secret key and shares of a collective
    // public key.
    struct Party {
        sk_share: SecretKey,
        pk_share: PublicKeyShare,
    }
    let mut parties = Vec::with_capacity(1);

    let sk_share = SecretKey::random(&params, &mut OsRng);
    let pk_share = PublicKeyShare::new(&sk_share, crp.clone(), &mut thread_rng())?;
    print!("{:?}", pk_share.to_bytes());
    let test = pk_share.to_bytes();
    let crp_des = PublicKeyShare::deserialize(&test, &params, crp).unwrap();
    println!("-------------------------");
    print!("{:?}", crp_des.to_bytes());
    //let ctx = params.ctx_at_level(0).to_bytes();
    //let test2 = 
    //let pk_2 = PublicKeyShare { par: params, crp: crp, p0_share: pk_share.p0_share };
    parties.push(Party { sk_share, pk_share });

    Ok(())
}
