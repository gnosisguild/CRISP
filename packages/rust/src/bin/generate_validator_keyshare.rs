// mod util;

// use std::{env, error::Error, process::exit, sync::Arc, fs, path::Path};
// use console::style;
// use fhe::{
//     bfv::{BfvParametersBuilder, Ciphertext, Encoding, Plaintext, PublicKey, SecretKey},
//     mbfv::{AggregateIter, CommonRandomPoly, DecryptionShare, PublicKeyShare},
// };
// use fhe_traits::{FheDecoder, FheEncoder, FheEncrypter, Serialize};
// //use fhe_math::rq::{Poly};
// use rand::{distributions::Uniform, prelude::Distribution, rngs::OsRng, thread_rng, SeedableRng};
// use rand_chacha::ChaCha8Rng;
// use util::timeit::{timeit, timeit_n};
// //use serde::{Deserialize, Serialize};
// use http_body_util::Empty;
// use hyper::Request;
// use hyper::body::Bytes;
// use hyper_util::rt::TokioIo;
// use tokio::net::TcpStream;
// use http_body_util::BodyExt;
// use tokio::io::{AsyncWriteExt as _, self};
// use rustc_serialize::json;

// use std::{thread, time};

// // use hyper::Client;
// // use hyper::header::Connection;
// // use hyper::header::Basic;
// // use hyper::header::Headers;

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

// #[derive(RustcEncodable, RustcDecodable)]
// struct JsonRequest {
//     response: String,
//     pk_share: Vec<u8>,
//     id: u32,
//     round_id: u32,
// }

// #[derive(RustcEncodable, RustcDecodable)]
// struct CrispConfig {
//     response: String,
//     round_id: u32,
//     chain_id: u32,
//     voting_address: String,
//     cyphernode_count: u32,
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//     println!("generating validator keyshare");

//     let degree = 4096;
//     let plaintext_modulus: u64 = 4096;
//     let moduli = vec![0xffffee001, 0xffffc4001, 0x1ffffe0001];

//     // Generate a deterministic seed for the Common Poly
//     let mut seed = <ChaCha8Rng as SeedableRng>::Seed::default();

//     // Let's generate the BFV parameters structure.
//     let params = timeit!(
//         "Parameters generation",
//         BfvParametersBuilder::new()
//             .set_degree(degree)
//             .set_plaintext_modulus(plaintext_modulus)
//             .set_moduli(&moduli)
//             .build_arc()?
//     );

//     let crp = CommonRandomPoly::new_deterministic(&params, seed)?;
//     let crp_bytes = crp.to_bytes();
//     //println!("{:?}", crp.to_bytes());

//     // Party setup: each party generates a secret key and shares of a collective
//     // public key.
//     struct Party {
//         sk_share: SecretKey,
//         pk_share: PublicKeyShare,
//     }

//     let mut parties = Vec::with_capacity(2);
//     let mut parties_test = Vec::with_capacity(2);

//     // Client Code

//     // Parse our URL for registering keyshare...
//     let url = "http://127.0.0.1/register_keyshare".parse::<hyper::Uri>()?;

//     // Get the host and the port
//     let host = url.host().expect("uri has no host");
//     let port = url.port_u16().unwrap_or(3000);

//     let address = format!("{}:{}", host, port);

//     // Open a TCP connection to the remote host
//     let stream = TcpStream::connect(address).await?;

//     // Use an adapter to access something implementing `tokio::io` traits as if they implement
//     // `hyper::rt` IO traits.
//     let io = TokioIo::new(stream);

//     // Create the Hyper client
//     let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;

//     // Spawn a task to poll the connection, driving the HTTP state
//     tokio::task::spawn(async move {
//         if let Err(err) = conn.await {
//             println!("Connection failed: {:?}", err);
//         }
//     });

//     // The authority of our URL will be the hostname of the httpbin remote
//     let authority = url.authority().unwrap().clone();

//     // testing nodes in one run ------------------------------------
//     for i in 0..2 {
//         println!("{:?}", i);
//         println!("generating share 1 and serialize");
//         let sk_share_1 = SecretKey::random(&params, &mut OsRng);
//         let pk_share_1 = PublicKeyShare::new(&sk_share_1, crp.clone(), &mut thread_rng())?;
//         let test_1 = pk_share_1.to_bytes();
//         //print_type_of(&test_1); // &str
//         let test_1_des = PublicKeyShare::deserialize(&test_1, &params, crp.clone()).unwrap();
//         parties.push(Party { sk_share: sk_share_1.clone(), pk_share: pk_share_1 });
//         parties_test.push(Party { sk_share: sk_share_1, pk_share: test_1_des });

//         let response = JsonRequest { response: "Test".to_string(), pk_share: test_1, id: i, round_id: 0 };
//         let out = json::encode(&response).unwrap();
//         let req = Request::post("http://127.0.0.1/")
//             .uri(url.clone())
//             .header(hyper::header::HOST, authority.as_str())
//             .body(out)?;

//         let mut res = sender.send_request(req).await?;

//         println!("Response status: {}", res.status());

//         // Stream the body, writing each frame to stdout as it arrives
//         while let Some(next) = res.frame().await {
//             let frame = next?;
//             if let Some(chunk) = frame.data_ref() {
//                 io::stdout().write_all(chunk).await?;
//             }
//         }
//         // Await the response...
//         let three_seconds = time::Duration::from_millis(3000);
//         thread::sleep(three_seconds);
//     }

//     // Aggregation: this could be one of the parties or a separate entity. Or the
//     // parties can aggregate cooperatively, in a tree-like fashion.
//     let pk = timeit!("Public key aggregation", {
//         let pk: PublicKey = parties.iter().map(|p| p.pk_share.clone()).aggregate()?;
//         pk
//     });

//     let pk_test = timeit!("Public key aggregation after serialize", {
//         let pk_test: PublicKey = parties_test.iter().map(|p| p.pk_share.clone()).aggregate()?;
//         pk_test
//     });

//     println!("{:?}", pk);
//     // testing nodes in one run ------------------------------------

//     // testing init the dir for a round
    
//     Ok(())
// }
