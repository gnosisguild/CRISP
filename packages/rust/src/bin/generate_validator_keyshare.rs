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
    let crp_bytes = crp.to_bytes();
    //println!("{:?}", crp.to_bytes());

    // Party setup: each party generates a secret key and shares of a collective
    // public key.
    struct Party {
        sk_share: SecretKey,
        pk_share: PublicKeyShare,
    }

    let mut parties = Vec::with_capacity(3);
    let mut parties_test = Vec::with_capacity(3);

    println!("generating share 1 and serialize");
    let sk_share_1 = SecretKey::random(&params, &mut OsRng);
    let pk_share_1 = PublicKeyShare::new(&sk_share_1, crp.clone(), &mut thread_rng())?;
    //print!("{:?}", pk_share_1.to_bytes());
    let test_1 = pk_share_1.to_bytes();
    let test_1_des = PublicKeyShare::deserialize(&test_1, &params, crp.clone()).unwrap();
    //print!("{:?}", test_1_des.to_bytes());
    //let ctx = params.ctx_at_level(0).to_bytes();
    //let test2 = 
    //let pk_2 = PublicKeyShare { par: params, crp: crp, p0_share: pk_share.p0_share };
    parties.push(Party { sk_share: sk_share_1.clone(), pk_share: pk_share_1 });
    parties_test.push(Party { sk_share: sk_share_1, pk_share: test_1_des });
    
    println!("generating share 2 and serialize");
    let sk_share_2 = SecretKey::random(&params, &mut OsRng);
    let pk_share_2 = PublicKeyShare::new(&sk_share_2, crp.clone(), &mut thread_rng())?;
    //print!("{:?}", pk_share_1.to_bytes());
    let test_2 = pk_share_2.to_bytes();
    let test_2_des = PublicKeyShare::deserialize(&test_2, &params, crp.clone()).unwrap();
    parties.push(Party { sk_share: sk_share_2.clone(), pk_share: pk_share_2 });
    parties_test.push(Party { sk_share: sk_share_2, pk_share: test_2_des });

    println!("generating share 3 and serialize");
    let sk_share_3 = SecretKey::random(&params, &mut OsRng);
    let pk_share_3 = PublicKeyShare::new(&sk_share_3, crp.clone(), &mut thread_rng())?;
    //print!("{:?}", pk_share_1.to_bytes());
    let test_3 = pk_share_3.to_bytes();
    let test_3_des = PublicKeyShare::deserialize(&test_3, &params, crp.clone()).unwrap();
    parties.push(Party { sk_share: sk_share_3.clone(), pk_share: pk_share_3 });
    parties_test.push(Party { sk_share: sk_share_3, pk_share: test_3_des });

    // Aggregation: this could be one of the parties or a separate entity. Or the
    // parties can aggregate cooperatively, in a tree-like fashion.
    let pk = timeit!("Public key aggregation", {
        let pk: PublicKey = parties.iter().map(|p| p.pk_share.clone()).aggregate()?;
        pk
    });

    let pk_test = timeit!("Public key aggregation after serialize", {
        let pk_test: PublicKey = parties_test.iter().map(|p| p.pk_share.clone()).aggregate()?;
        pk_test
    });

    // println!("{:?}", pk);
    // println!("--------------------");
    // println!("{:?}", pk_test);
    Ok(())
}
