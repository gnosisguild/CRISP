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
use serde::{Deserialize};
use http_body_util::Empty;
use hyper::Request;
use hyper::body::Bytes;
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;
use http_body_util::BodyExt;
use tokio::io::{AsyncWriteExt as _, self};
use rustc_serialize::json;

use std::{thread, time};

#[derive(RustcEncodable, RustcDecodable)]
struct JsonRequestGetRounds {
    response: String,
}

#[derive(RustcEncodable, RustcDecodable)]
struct PKShareRequest {
    response: String,
    pk_share: Vec<u8>,
    id: u32,
    round_id: u32,
}

#[derive(RustcEncodable, RustcDecodable)]
struct CrispConfig {
    response: String,
    round_id: u32,
    chain_id: u32,
    voting_address: String,
    cyphernode_count: u32,
}

#[derive(Debug, Deserialize, RustcEncodable)]
struct RoundCount {
    round_count: u32,
}

#[derive(Debug, Deserialize, RustcEncodable)]
struct PKShareCount {
    round_id: u32,
    share_id: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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

    //let mut rounds: Vec<u32> = Vec::with_capacity(1);
    // set the expected CRISP rounds
    let mut internal_round_count = RoundCount { round_count: 0 };

    loop {
        println!("Polling CRISP server...");

        // Client Code
        // Parse our URL for registering keyshare...
        //let url_register_keyshare = "http://127.0.0.1/register_keyshare".parse::<hyper::Uri>()?;
        let url_get_rounds = "http://127.0.0.1/get_rounds".parse::<hyper::Uri>()?;
        // Get the host and the port
        let host = url_get_rounds.host().expect("uri has no host");
        let port = url_get_rounds.port_u16().unwrap_or(3000);
        let address = format!("{}:{}", host, port);
        // Open a TCP connection to the remote host
        let stream = TcpStream::connect(address).await?;
        // Use an adapter to access something implementing `tokio::io` traits as if they implement
        // `hyper::rt` IO traits.
        let io = TokioIo::new(stream);
        // Create the Hyper client
        let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;
        // Spawn a task to poll the connection, driving the HTTP state
        tokio::task::spawn(async move {
            if let Err(err) = conn.await {
                println!("Connection failed: {:?}", err);
            }
        });
        // The authority of our URL will be the hostname of the httpbin remote
        let authority = url_get_rounds.authority().unwrap().clone();
        //let authority_key = url_register_keyshare.authority().unwrap().clone();

        let response = JsonRequestGetRounds { response: "Test".to_string() };
        let out = json::encode(&response).unwrap();
        let req = Request::get("http://127.0.0.1/")
            .uri(url_get_rounds.clone())
            .header(hyper::header::HOST, authority.as_str())
            .body(out)?;

        let mut res = sender.send_request(req).await?;

        println!("Response status: {}", res.status());

        let body_bytes = res.collect().await?.to_bytes();
        let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
        let count: RoundCount = serde_json::from_str(&body_str).expect("JSON was not well-formatted");
        println!("Server Round Count: {:?}", count.round_count);
        println!("Internal Round Count: {:?}", internal_round_count.round_count);

        if(count.round_count > internal_round_count.round_count) {
            println!("Getting latest PK share ID.");
            // --------------------------------------

            // Client Code
            let url_get_shareid = "http://127.0.0.1/get_pk_share_count".parse::<hyper::Uri>()?;
            // Get the host and the port
            let host_get_shareid = url_get_shareid.host().expect("uri has no host");
            let port_get_shareid = url_get_shareid.port_u16().unwrap_or(3000);
            let address_get_shareid = format!("{}:{}", host_get_shareid, port_get_shareid);
            // Open a TCP connection to the remote host
            let stream_get_shareid = TcpStream::connect(address_get_shareid).await?;
            // Use an adapter to access something implementing `tokio::io` traits as if they implement
            // `hyper::rt` IO traits.
            let io_get_shareid = TokioIo::new(stream_get_shareid);
            // Create the Hyper client
            let (mut sender_get_shareid, conn_get_shareid) = hyper::client::conn::http1::handshake(io_get_shareid).await?;
            // Spawn a task to poll the connection, driving the HTTP state
            tokio::task::spawn(async move {
                if let Err(err) = conn_get_shareid.await {
                    println!("Connection failed: {:?}", err);
                }
            });
            // The authority of our URL will be the hostname of the httpbin remote
            let authority_get_shareid = url_get_shareid.authority().unwrap().clone();

            let response_get_shareid = PKShareCount { round_id: count.round_count, share_id: 0 };
            let out_get_shareid = json::encode(&response_get_shareid).unwrap();
            let req_get_shareid = Request::post("http://127.0.0.1/")
                .uri(url_get_shareid.clone())
                .header(hyper::header::HOST, authority_get_shareid.as_str())
                .body(out_get_shareid)?;

            let mut res_get_shareid = sender_get_shareid.send_request(req_get_shareid).await?;

            println!("Response status: {}", res_get_shareid.status());

            let body_bytes = res_get_shareid.collect().await?.to_bytes();
            let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
            let share_count: PKShareCount = serde_json::from_str(&body_str).expect("JSON was not well-formatted");

            // --------------------------------------
            println!("Generating share and serializing.");

            let sk_share_1 = SecretKey::random(&params, &mut OsRng);
            let pk_share_1 = PublicKeyShare::new(&sk_share_1, crp.clone(), &mut thread_rng())?;
            let test_1 = pk_share_1.to_bytes();

            // Client Code
            // Parse our URL for registering keyshare...
            let url_register_keyshare = "http://127.0.0.1/register_keyshare".parse::<hyper::Uri>()?;
            // Get the host and the port
            let host_key = url_register_keyshare.host().expect("uri has no host");
            let port_key = url_register_keyshare.port_u16().unwrap_or(3000);
            let address_key = format!("{}:{}", host_key, port_key);
            // Open a TCP connection to the remote host
            let stream_key = TcpStream::connect(address_key).await?;
            // Use an adapter to access something implementing `tokio::io` traits as if they implement
            // `hyper::rt` IO traits.
            let io_key = TokioIo::new(stream_key);
            // Create the Hyper client
            let (mut sender_key, conn_key) = hyper::client::conn::http1::handshake(io_key).await?;
            // Spawn a task to poll the connection, driving the HTTP state
            tokio::task::spawn(async move {
                if let Err(err) = conn_key.await {
                    println!("Connection failed: {:?}", err);
                }
            });
            // The authority of our URL will be the hostname of the httpbin remote
            let authority_key = url_register_keyshare.authority().unwrap().clone();
            // -------
            // todo: get id from the number of other shares already stored on iron server
            let response_key = PKShareRequest { response: "Test".to_string(), pk_share: test_1, id: share_count.share_id, round_id: count.round_count };
            let out_key = json::encode(&response_key).unwrap();
            let req_key = Request::post("http://127.0.0.1/")
                .uri(url_register_keyshare.clone())
                .header(hyper::header::HOST, authority_key.as_str())
                .body(out_key)?;

            let mut res_key = sender_key.send_request(req_key).await?;

            println!("Response status: {}", res_key.status());

            // Stream the body, writing each frame to stdout as it arrives
            while let Some(next) = res_key.frame().await {
                let frame = next?;
                if let Some(chunk) = frame.data_ref() {
                    io::stdout().write_all(chunk).await?;
                }
            }
            internal_round_count.round_count += 1;
        }

        // Stream the body, writing each frame to stdout as it arrives
        // while let Some(next) = res.frame().await {
        //     let frame = next?;
        //     if let Some(chunk) = frame.data_ref() {
        //         io::stdout().write_all(chunk).await?;
        //     }
        // }
        // Await the response...
        let three_seconds = time::Duration::from_millis(6000);
        thread::sleep(three_seconds);

        // // Aggregation: this could be one of the parties or a separate entity. Or the
        // // parties can aggregate cooperatively, in a tree-like fashion.
        // let pk = timeit!("Public key aggregation", {
        //     let pk: PublicKey = parties.iter().map(|p| p.pk_share.clone()).aggregate()?;
        //     pk
        // });

        // let pk_test = timeit!("Public key aggregation after serialize", {
        //     let pk_test: PublicKey = parties_test.iter().map(|p| p.pk_share.clone()).aggregate()?;
        //     pk_test
        // });

        // println!("{:?}", pk);
    }
    
    Ok(())
}
