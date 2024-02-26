mod util;

use std::{env, error::Error, process::exit, sync::Arc, fs, path::Path};
use console::style;
use fhe::{
    bfv::{BfvParametersBuilder, Ciphertext, Encoding, Plaintext, PublicKey, SecretKey},
    mbfv::{AggregateIter, CommonRandomPoly, DecryptionShare, PublicKeyShare},
};
use fhe_traits::{FheDecoder, FheEncoder, FheEncrypter, Serialize};
use rand::{distributions::Uniform, prelude::Distribution, rngs::OsRng, thread_rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use util::timeit::{timeit, timeit_n};
//use serde::{Deserialize, Serialize};

fn main() -> Result<(), Box<dyn Error>> {
    println!("generating validator keyshare");

    let mut num_parties = 10;
    let mut num_voters = 2;

    let degree = 4096;
    let plaintext_modulus: u64 = 4096;
    let moduli = vec![0xffffee001, 0xffffc4001, 0x1ffffe0001];

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
    let crp = CommonRandomPoly::new(&params, &mut thread_rng())?;
    let mut rng = thread_rng();
    println!("{:?}", rng);
    //let test = crp.to_bytes();

    // Party setup: each party generates a secret key and shares of a collective
    // public key.
    //#[derive(Serialize, Deserialize)]
    struct Party {
        sk_share: SecretKey,
        pk_share: PublicKeyShare,
    }
    let mut parties = Vec::with_capacity(num_parties);
    timeit_n!("Party setup (per party)", num_parties as u32, {
        let sk_share = SecretKey::random(&params, &mut OsRng);
        let pk_share = PublicKeyShare::new(&sk_share, crp.clone(), &mut thread_rng())?;
        //let bytes = bincode::serialize(&pk_share).unwrap();
        //let test = pk_share.to_bytes();
        parties.push(Party { sk_share, pk_share });
    });

    // Aggregation: this could be one of the parties or a separate entity. Or the
    // parties can aggregate cooperatively, in a tree-like fashion.
    let pk = timeit!("Public key aggregation", {
        let pk: PublicKey = parties.iter().map(|p| p.pk_share.clone()).aggregate()?;
        pk
    });
    //println!("{:?}", pk);
    //let () = pk;
    //let test = pk_share.to_bytes();
    //println!("{:?}", test);
    //let test = pk.to_bytes();

    //let path: &Path = ...;
    //fs::write(path, file_contents_base64).unwrap();

    Ok(())
}
