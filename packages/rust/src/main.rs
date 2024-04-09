// Implementation of multiparty voting using the `fhe` crate.

//mod util;

use std::{env, error::Error, process::exit, sync::Arc};

use console::style;
use fhe::{
    bfv::{self, Ciphertext, Encoding, Plaintext, PublicKey, SecretKey},
    mbfv::{AggregateIter, CommonRandomPoly, DecryptionShare, PublicKeyShare},
};
use fhe_traits::{FheDecoder, FheEncoder, FheEncrypter};
use rand::{distributions::Uniform, prelude::Distribution, rngs::OsRng, thread_rng};

fn main() -> Result<(), Box<dyn Error>> {

    Ok(())
}
