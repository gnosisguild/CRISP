mod util;

use std::{env, error::Error, process::exit, sync::Arc};
use console::style;
use fhe::{
    bfv::{BfvParametersBuilder, Ciphertext, Encoding, Plaintext, PublicKey, SecretKey},
    mbfv::{AggregateIter, CommonRandomPoly, DecryptionShare, PublicKeyShare},
};
use fhe_traits::{FheDecoder, FheEncoder, FheEncrypter};
use rand::{distributions::Uniform, prelude::Distribution, rngs::OsRng, thread_rng};
use util::timeit::{timeit, timeit_n};

fn main() -> Result<(), Box<dyn Error>> {
    println!("casting encrypted vote");

    let mut num_parties = 10;
    let mut num_voters = 100;

    let degree = 4096;
    let plaintext_modulus: u64 = 4096;
    let moduli = vec![0xffffee001, 0xffffc4001, 0x1ffffe0001];

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

    // Party setup: each party generates a secret key and shares of a collective
    // public key.
    struct Party {
        sk_share: SecretKey,
        pk_share: PublicKeyShare,
    }
    let mut parties = Vec::with_capacity(num_parties);
    timeit_n!("Party setup (per party)", num_parties as u32, {
        let sk_share = SecretKey::random(&params, &mut OsRng);
        let pk_share = PublicKeyShare::new(&sk_share, crp.clone(), &mut thread_rng())?;
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
    let test = pk.to_bytes();
    // voting	
    let dist = Uniform::new_inclusive(0, 1);
    let votes: Vec<u64> = dist
        .sample_iter(&mut thread_rng())
        .take(num_voters)
        .collect();
    println!("{:?}", votes);
    let mut votes_encrypted = Vec::with_capacity(num_voters);
    let mut _i = 0;
    timeit_n!("Vote casting (per voter)", num_voters as u32, {
        #[allow(unused_assignments)]
        let pt = Plaintext::try_encode(&[votes[_i]], Encoding::poly(), &params)?;
        let ct = pk.try_encrypt(&pt, &mut thread_rng())?;
        votes_encrypted.push(ct);
        _i += 1;
    });

    Ok(())
}
