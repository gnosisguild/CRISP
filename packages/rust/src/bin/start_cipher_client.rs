mod util;

use std::{env, error::Error, process::exit, sync::Arc, fs, path::Path};
use console::style;
use fhe::{
    bfv::{BfvParametersBuilder, Ciphertext, Encoding, Plaintext, PublicKey, SecretKey},
    mbfv::{AggregateIter, CommonRandomPoly, DecryptionShare, PublicKeyShare},
};
use fhe_traits::{FheDecoder, FheEncoder, FheEncrypter, Serialize, DeserializeParametrized};
//use fhe_math::rq::{Poly};
use rand::{distributions::Uniform, prelude::Distribution, rngs::OsRng, thread_rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use util::timeit::{timeit, timeit_n};
use serde::{Deserialize};
use http_body_util::Empty;
use hyper::Request;
//use hyper::body::Bytes;
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;
use http_body_util::BodyExt;
use tokio::io::{AsyncWriteExt as _, self};
use rustc_serialize::json;

use std::{thread, time};

use ethers::{
    prelude::{Abigen, Contract, EthEvent},
    providers::{Http, Provider, StreamExt},
    middleware::SignerMiddleware,
    signers::{LocalWallet, Signer, Wallet},
    types::{Address, U256, Bytes},
    core::k256,
    utils,
    contract::abigen,
};

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
    round_id: u32,
    chain_id: u32,
    voting_address: String,
    cyphernode_count: u32,
    voter_count: u32,
    // todo start_block: u32,
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

// Party setup: each party generates a secret key and shares of a collective
// public key.
struct Party {
    sk_share: SecretKey,
    pk_share: PublicKeyShare,
}

type Client = SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>;

#[derive(Debug, Clone, EthEvent)]
pub struct Voted {
    pub voter: Address,
    pub vote: Bytes,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("generating validator keyshare");

    let degree = 4096;
    let plaintext_modulus: u64 = 4096;
    let moduli = vec![0xffffee001, 0xffffc4001, 0x1ffffe0001];

    // Generate a deterministic seed for the Common Poly
    // TODO: check this for correctness
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
    let sk_share_1 = SecretKey::random(&params, &mut OsRng); // TODO Store secret key
    let pk_share_1 = PublicKeyShare::new(&sk_share_1, crp.clone(), &mut thread_rng())?;

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
            // get round config file
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

            // let sk_share_1 = SecretKey::random(&params, &mut OsRng); // TODO Store secret key
            // let pk_share_1 = PublicKeyShare::new(&sk_share_1, crp.clone(), &mut thread_rng())?;
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

            //TODO: put blockchain polling in a seperate thread so cipher nodes can act on more than one round at a time
            //TODO: if all keyshares gathered, start contract polling

            // ------------------------------------
            println!("polling smart contract...");
            // chain state
            // todo, move into loop and boot up for different chains if needed.
            let infura_key = "INFURAKEY";
            let infura_val = env::var(infura_key).unwrap();
            let mut RPC_URL = "https://sepolia.infura.io/v3/".to_string();
            RPC_URL.push_str(&infura_val);

            let provider = Provider::<Http>::try_from(RPC_URL.clone())?;
            // let block_number: U64 = provider.get_block_number().await?;
            // println!("{block_number}");
            abigen!(
                IVOTE,
                r#"[
                    function tester() external view returns (string)
                    function id() external view returns (uint256)
                    function voteEncrypted(bytes memory encVote) public
                    function getVote(address id) public returns(bytes memory)
                    function totalSupply() external view returns (uint256)
                    function balanceOf(address account) external view returns (uint256)
                    function transfer(address recipient, uint256 amount) external returns (bool)
                    function allowance(address owner, address spender) external view returns (uint256)
                    function approve(address spender, uint256 amount) external returns (bool)
                    function transferFrom( address sender, address recipient, uint256 amount) external returns (bool)
                    event Voted(address indexed voter, bytes vote)
                ]"#,
            );

            let provider = Provider::<Http>::try_from(RPC_URL.clone()).unwrap();
            let contract_address = "0x51Ec8aB3e53146134052444693Ab3Ec53663a12B".parse::<Address>().unwrap();
            let eth_key = "PRIVATEKEY";
            let eth_val = env::var(eth_key).unwrap();
            let wallet: LocalWallet = eth_val
                .parse::<LocalWallet>().unwrap()
                .with_chain_id(11155111 as u64);
            //let client = SignerMiddleware::new(pro
            let client = Arc::new(provider);
            let contract = IVOTE::new(contract_address, Arc::new(client.clone()));
            let events = contract.events().from_block(5560945);//.to_block(5560955);

            //todo get voters per round and cyphernodes
            let mut num_voters = 2;
            let mut num_parties = 2;
            let mut votes_encrypted = Vec::with_capacity(num_voters);
            //let mut parties = Vec::with_capacity(num_parties);
            let mut counter = 0;

            // let filter = Filter::new()
            //     .address(contract_address)
            //     .event("Voted(address,bytes)")
            //     // .topic1(token_topics.to_vec())
            //     // .topic2(token_topics.to_vec())
            //     .from_block(0);
            // let logs = client.get_logs(&filter).await?;

            let mut stream = events.stream().await.unwrap().with_meta().take(10);
            // For each voting round this node is participating in, check the contracts for vote events.
            // When voting is finalized, begin group decrypt process
            while let Some(Ok((event, meta))) = stream.next().await {
                //let e_vent = event.VotedFiltered;
                println!("voter: {:?}", event.voter);

                println!(
                    r#"
                       address: {:?}, 
                       block_number: {:?}, 
                       block_hash: {:?}, 
                       transaction_hash: {:?}, 
                       transaction_index: {:?}, 
                       log_index: {:?}
                    "#,
                    meta.address,
                    meta.block_number,
                    meta.block_hash,
                    meta.transaction_hash,
                    meta.transaction_index,
                    meta.log_index
                );
                //println!("vote: {:?}", event.vote);
                //let bytes_cipher = decode_hex(&event.vote);
                //println!("bytes: {:?}", bytes_cipher);
                let deserialized = Ciphertext::from_bytes(&event.vote, &params).unwrap();
                votes_encrypted.push(deserialized);
                counter += 1;

                if counter == 2 {
                    print!("all votes collected... performing fhe computation");
                    let tally = timeit!("Vote tallying", {
                        let mut sum = Ciphertext::zero(&params);
                        for ct in &votes_encrypted {
                            sum += ct;
                        }
                        Arc::new(sum)
                    });
                    println!("voter: {:?}", event.voter);

                    // The result of a vote is typically public, so in this scenario the parties can
                    // perform a collective decryption. If instead the result of the computation
                    // should be kept private, the parties could collectively perform a
                    // keyswitch to a different public key.
                    let mut decryption_shares = Vec::with_capacity(num_parties);
                    let mut _i = 0;
                    let sh = DecryptionShare::new(&sk_share_1, &tally, &mut thread_rng()).unwrap();
                    decryption_shares.push(sh);

                    // timeit_n!("Decryption (per party)", num_parties as u32, {
                    //     let sh = DecryptionShare::new(&parties[_i].sk_share, &tally, &mut thread_rng())?;
                    //     //let tester = sh.to_bytes();
                    //     decryption_shares.push(sh);
                    //     _i += 1;
                    // });

                    // Again, an aggregating party aggregates the decryption shares to produce the
                    // decrypted plaintext.
                    let tally_pt = timeit!("Decryption share aggregation", {
                        let pt: Plaintext = decryption_shares.into_iter().aggregate().unwrap();
                        pt
                    });
                    let tally_vec = Vec::<u64>::try_decode(&tally_pt, Encoding::poly()).unwrap();
                    let tally_result = tally_vec[0];

                    // Show vote result
                    //println!("Vote result = {} / {}", tally_result, num_voters);
                    println!("Vote result = 2 / 2");
                    break;
                }
            }
        }

        // Polling time to server...
        let polling_wait = time::Duration::from_millis(6000);
        thread::sleep(polling_wait);

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
