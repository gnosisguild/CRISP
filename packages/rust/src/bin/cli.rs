mod util;

use dialoguer::{theme::ColorfulTheme, Input, FuzzySelect};
use std::{thread, time, env, sync::Arc};
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

use fhe::{
    bfv::{BfvParametersBuilder, Ciphertext, Encoding, Plaintext, PublicKey, SecretKey},
    mbfv::{AggregateIter, CommonRandomPoly, DecryptionShare, PublicKeyShare},
};
use fhe_traits::{FheDecoder, FheEncoder, FheEncrypter, Serialize, DeserializeParametrized};
//use fhe_math::rq::{Poly};
use rand::{distributions::Uniform, prelude::Distribution, rngs::OsRng, thread_rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use util::timeit::{timeit, timeit_n};

use http_body_util::Empty;
use hyper::Request;
//use hyper::body::Bytes;
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;
use http_body_util::BodyExt;
use tokio::io::{AsyncWriteExt as _, self};
use rustc_serialize::json;

use ethers::{
    prelude::{abigen, Abigen},
    providers::{Http, Provider},
    middleware::SignerMiddleware,
    signers::{LocalWallet, Signer, Wallet},
    types::{Address, U256, Bytes},
    core::k256,
    utils,
};

type Client = SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>;

#[derive(Deserialize, RustcEncodable, RustcDecodable)]
struct JsonResponse {
    response: String
}

#[derive(Deserialize, RustcEncodable, RustcDecodable)]
struct JsonResponseTxHash {
    response: String,
    tx_hash: String,
}

#[derive(RustcEncodable, RustcDecodable)]
struct JsonRequestGetRounds {
    response: String,
}

#[derive(Debug, Deserialize, RustcEncodable)]
struct RoundCount {
    round_count: u32,
}

#[derive(RustcEncodable, RustcDecodable)]
struct JsonRequest {
    response: String,
    pk_share: u32,
    id: u32,
    round_id: u32,
}

#[derive(Debug, Deserialize, RustcEncodable)]
struct CrispConfig {
    round_id: u32,
    chain_id: u32,
    voting_address: String,
    ciphernode_count: u32,
    voter_count: u32,
}

#[derive(Debug, Deserialize, RustcEncodable, RustcDecodable)]
struct PKRequest {
    round_id: u32,
    pk_bytes: Vec<u8>,
}

#[derive(Debug, Deserialize, RustcEncodable, RustcDecodable)]
struct EncryptedVote {
    round_id: u32,
    enc_vote_bytes: Vec<u8>,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

	print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let selections = &[
        "CRISP: Voting Protocol (ETH)",
        "More Coming Soon!"
    ];

    let selections_2 = &[
        "Initialize new CRISP round.",
        "Continue Existing CRISP round."
    ];

    let selections_3 = &[
        "Abstain.",
        "Vote yes.",
        "Vote no."
    ];

    let selection_1 = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Enclave (EEEE): Please choose the private execution environment you would like to run!")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    if(selection_1 == 0){
    	print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    	//println!("Encrypted Protocol Selected {}!", selections[selection_1]);
	    let selection_2 = FuzzySelect::with_theme(&ColorfulTheme::default())
	        .with_prompt("Create a new CRISP round or particpate in an existing round.")
	        .default(0)
	        .items(&selections_2[..])
	        .interact()
	        .unwrap();

	    if(selection_2 == 0){
	    	print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
	    	println!("Starting new CRISP round!");
		    // let input_token: String = Input::with_theme(&ColorfulTheme::default())
		    //     .with_prompt("Enter Proposal Registration Token")
		    //     .interact_text()
		    //     .unwrap();
		    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
		    println!("Reading proposal details from config.");
            let path = env::current_dir().unwrap();
            let mut pathst = path.display().to_string();
            pathst.push_str("/example_config.json");
            let mut file = File::open(pathst).unwrap();
            let mut data = String::new();
            file.read_to_string(&mut data).unwrap();
            let config: CrispConfig = serde_json::from_str(&data).expect("JSON was not well-formatted");
            println!("round id: {:?}", config.round_id); // get new round id from current id in server
            println!("chain id: {:?}", config.chain_id);
            println!("voting contract: {:?}", config.voting_address);
            println!("ciphernode count: {:?}", config.ciphernode_count);
            println!("voter count: {:?}", config.voter_count);

            println!("Calling contract to initialize onchain proposal...");
	        let three_seconds = time::Duration::from_millis(1000);
	        thread::sleep(three_seconds);

            println!("Initializing Keyshare nodes...");
            // call init on server
            // have nodes poll

            // Todo: pull client code into function that takes endpoint url and body as input 
            // Client Code
            // Parse our URL for registering keyshare...
            let url_id = "http://127.0.0.1/get_rounds".parse::<hyper::Uri>()?;
            // Get the host and the port
            let host_id = url_id.host().expect("uri has no host");
            let port_id = url_id.port_u16().unwrap_or(4000);
            let address_id = format!("{}:{}", host_id, port_id);
            // Open a TCP connection to the remote host
            let stream_id = TcpStream::connect(address_id).await?;
            // Use an adapter to access something implementing `tokio::io` traits as if they implement
            // `hyper::rt` IO traits.
            let io_id = TokioIo::new(stream_id);
            // Create the Hyper client
            let (mut sender_id, conn_id) = hyper::client::conn::http1::handshake(io_id).await?;
            // Spawn a task to poll the connection, driving the HTTP state
            tokio::task::spawn(async move {
                if let Err(err) = conn_id.await {
                    println!("Connection failed: {:?}", err);
                }
            });
            // The authority of our URL will be the hostname of the httpbin remote
            let authority_id = url_id.authority().unwrap().clone();
            let response_id = JsonRequestGetRounds { response: "Test".to_string() };
            let out_id = json::encode(&response_id).unwrap();
            let req_id = Request::get("http://127.0.0.1/")
                .uri(url_id.clone())
                .header(hyper::header::HOST, authority_id.as_str())
                .body(out_id)?;
            let mut res_id = sender_id.send_request(req_id).await?;

            println!("Response status: {}", res_id.status());

            let body_bytes = res_id.collect().await?.to_bytes();
            let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
            let count: RoundCount = serde_json::from_str(&body_str).expect("JSON was not well-formatted");
            println!("Server Round Count: {:?}", count.round_count);


            // Client Code --------------------------------
            // Parse our URL for registering keyshare...
            let url = "http://127.0.0.1/init_crisp_round".parse::<hyper::Uri>()?;
            // Get the host and the port
            let host = url.host().expect("uri has no host");
            let port = url.port_u16().unwrap_or(4000);
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
            let authority = url.authority().unwrap().clone();
            let round_id = count.round_count + 1;
            let response = CrispConfig { round_id: round_id, chain_id: 5, voting_address: config.voting_address, ciphernode_count: config.ciphernode_count, voter_count: config.voter_count };
            //let response = JsonRequest { response: "Test".to_string(), pk_share: 0, id: 0, round_id: 0 };
            let out = json::encode(&response).unwrap();
            let req = Request::post("http://127.0.0.1/")
                .uri(url.clone())
                .header(hyper::header::HOST, authority.as_str())
                .body(out)?;

            let mut res = sender.send_request(req).await?;

            println!("Response status: {}", res.status());

            // Stream the body, writing each frame to stdout as it arrives
            while let Some(next) = res.frame().await {
                let frame = next?;
                if let Some(chunk) = frame.data_ref() {
                    io::stdout().write_all(chunk).await?;
                }
            }
            println!("Round Initialized.");
	    	println!("Gathering Keyshare nodes for execution environment...");
            thread::sleep(three_seconds);
            println!("\nYou can now vote Encrypted with Round ID: {:?}", round_id);

	    }
	    if(selection_2 == 1){
	    	print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
		    let input_crisp_id: u32 = Input::with_theme(&ColorfulTheme::default())
		        .with_prompt("Enter CRISP round ID.")
		        .interact_text()
		        .unwrap();
            let path = env::current_dir().unwrap();
            let mut pathst = path.display().to_string();
            pathst.push_str("/example_config.json");
            let mut file = File::open(pathst).unwrap();
            let mut data = String::new();
            file.read_to_string(&mut data).unwrap();
            let config: CrispConfig = serde_json::from_str(&data).expect("JSON was not well-formatted");
            println!("Voting state Initialized");

            // Client Code
            // Parse our URL for registering keyshare...
            let url_pk = "http://127.0.0.1/get_pk_by_round".parse::<hyper::Uri>()?;
            // Get the host and the port
            let host_pk = url_pk.host().expect("uri has no host");
            let port_pk = url_pk.port_u16().unwrap_or(4000);
            let address_pk = format!("{}:{}", host_pk, port_pk);
            // Open a TCP connection to the remote host
            let stream_pk = TcpStream::connect(address_pk).await?;
            // Use an adapter to access something implementing `tokio::io` traits as if they implement
            // `hyper::rt` IO traits.
            let io_pk = TokioIo::new(stream_pk);
            // Create the Hyper client
            let (mut sender_pk, conn_pk) = hyper::client::conn::http1::handshake(io_pk).await?;
            // Spawn a task to poll the connection, driving the HTTP state
            tokio::task::spawn(async move {
                if let Err(err) = conn_pk.await {
                    println!("Connection failed: {:?}", err);
                }
            });
            // The authority of our URL will be the hostname of the httpbin remote
            let authority_pk = url_pk.authority().unwrap().clone();
            let v: Vec<u8> = vec! [0];
            let response_pk = PKRequest { round_id: input_crisp_id, pk_bytes: v };
            let out_pk = json::encode(&response_pk).unwrap();
            let req_pk = Request::post("http://127.0.0.1/")
                .uri(url_pk.clone())
                .header(hyper::header::HOST, authority_pk.as_str())
                .body(out_pk)?;
            let mut res_pk = sender_pk.send_request(req_pk).await?;

            println!("Response status: {}", res_pk.status());

            let body_bytes = res_pk.collect().await?.to_bytes();
            let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
            let pk_res: PKRequest = serde_json::from_str(&body_str).expect("JSON was not well-formatted");
            // println!("Server Round Count: {:?}", pk_res.round_id);
            // println!("PK: {:?}", pk_res.pk_bytes);
            println!("Shared Public Key for CRISP round {:?} collected.", pk_res.round_id);
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
            let pk_deserialized = PublicKey::from_bytes(&pk_res.pk_bytes, &params).unwrap();
            // todo: validate that this user can vote
            let selection_3 = FuzzySelect::with_theme(&ColorfulTheme::default())
                .with_prompt("Please select your voting option.")
                .default(0)
                .items(&selections_3[..])
                .interact()
                .unwrap();

            let mut vote_choice: u64 = 0;
            if(selection_3 == 0){
                println!("Exiting voting system. You may choose to vote later.");
                vote_choice = 0;
            }
            if(selection_3 == 1){
                vote_choice = 1;
            }
            if(selection_3 == 2){
                vote_choice = 0;
            }
            println!("Encrypting vote.");
            let votes: Vec<u64> = [vote_choice].to_vec();
            let pt = Plaintext::try_encode(&[votes[0]], Encoding::poly(), &params)?;
            let ct = pk_deserialized.try_encrypt(&pt, &mut thread_rng())?;
            println!("Vote encrypted.");
            println!("Calling voting contract with encrypted vote.");
            // contact server to broadcast vote
            // Client Code
            let url_contract = "http://127.0.0.1/broadcast_enc_vote".parse::<hyper::Uri>()?;
            let host_contract = url_contract.host().expect("uri has no host");
            let port_contract = url_contract.port_u16().unwrap_or(4000);
            let address_contract = format!("{}:{}", host_contract, port_contract);
            let stream_contract = TcpStream::connect(address_contract).await?;
            let io_contract = TokioIo::new(stream_contract);
            let (mut sender_contract, conn_contract) = hyper::client::conn::http1::handshake(io_contract).await?;
            tokio::task::spawn(async move {
                if let Err(err) = conn_contract.await {
                    println!("Connection failed: {:?}", err);
                }
            });
            let authority_contract = url_contract.authority().unwrap().clone();
            let request_contract = EncryptedVote { round_id: input_crisp_id, enc_vote_bytes: ct.to_bytes()};
            let out_contract = json::encode(&request_contract).unwrap();
            let req_contract = Request::post("http://127.0.0.1/")
                .uri(url_contract.clone())
                .header(hyper::header::HOST, authority_contract.as_str())
                .body(out_contract)?;
            let mut res_contract = sender_contract.send_request(req_contract).await?;

            println!("Response status: {}", res_contract.status());

            let body_bytes = res_contract.collect().await?.to_bytes();
            let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
            let contract_res: JsonResponseTxHash = serde_json::from_str(&body_str).expect("JSON was not well-formatted");
            println!("Contract call: {:?}", contract_res.response);
            println!("TxHash is {:?}", contract_res.tx_hash);

            // let sol_vote = Bytes::from(ct.to_bytes());
            // //println!("{:?}", votes_encrypted[0].to_bytes());
            // //println!("{:?}", sol_vote);
            // let infura_key = "9a9193c8c1604e0c8f85b44c7674b33f";
            // let infura_val = env::var(infura_key).unwrap();
            // let mut RPC_URL = "https://sepolia.infura.io/v3/".to_string();
            // RPC_URL.push_str(&infura_val);

            // let provider = Provider::<Http>::try_from(RPC_URL.clone())?;
            // // let block_number: U64 = provider.get_block_number().await?;
            // // println!("{block_number}");
            // abigen!(
            //     IVOTE,
            //     r#"[
            //         function tester() external view returns (string)
            //         function id() external view returns (uint256)
            //         function voteEncrypted(bytes memory _encVote) public
            //         function getVote(address id) public returns(bytes memory)
            //         function totalSupply() external view returns (uint256)
            //         function balanceOf(address account) external view returns (uint256)
            //         function transfer(address recipient, uint256 amount) external returns (bool)
            //         function allowance(address owner, address spender) external view returns (uint256)
            //         function approve(address spender, uint256 amount) external returns (bool)
            //         function transferFrom( address sender, address recipient, uint256 amount) external returns (bool)
            //         event Transfer(address indexed from, address indexed to, uint256 value)
            //         event Approval(address indexed owner, address indexed spender, uint256 value)
            //     ]"#,
            // );

            // //const RPC_URL: &str = "https://eth.llamarpc.com";
            // const VOTE_ADDRESS: &str = "0x51Ec8aB3e53146134052444693Ab3Ec53663a12B";

            // let eth_key = "PRIVATEKEY";
            // let eth_val = env::var(eth_key).unwrap();
            // let wallet: LocalWallet = eth_val
            //     .parse::<LocalWallet>().unwrap()
            //     .with_chain_id(11155111 as u64);

            // // 6. Wrap the provider and wallet together to create a signer client
            // let client = SignerMiddleware::new(provider.clone(), wallet.clone());
            // //let client = Arc::new(provider);
            // let address: Address = VOTE_ADDRESS.parse()?;
            // let contract = IVOTE::new(address, Arc::new(client.clone()));

            // contract.vote_encrypted(sol_vote).send().await?;
	    }

    }
    if(selection_1 == 1){
    	println!("Check back soon!");
    	std::process::exit(1);
    }

    // println!("Hello {}!", input);

    // let mail: String = Input::with_theme(&ColorfulTheme::default())
    //     .with_prompt("Your email")
    //     .validate_with({
    //         let mut force = None;
    //         move |input: &String| -> Result<(), &str> {
    //             if input.contains('@') || force.as_ref().map_or(false, |old| old == input) {
    //                 Ok(())
    //             } else {
    //                 force = Some(input.clone());
    //                 Err("This is not a mail address; type the same value again to force use")
    //             }
    //         }
    //     })
    //     .interact_text()
    //     .unwrap();

    // println!("Email: {}", mail);

    // let mail: String = Input::with_theme(&ColorfulTheme::default())
    //     .with_prompt("Your planet")
    //     .default("Earth".to_string())
    //     .interact_text()
    //     .unwrap();

    // println!("Planet: {}", mail);

    // let mail: String = Input::with_theme(&ColorfulTheme::default())
    //     .with_prompt("Your galaxy")
    //     .with_initial_text("Milky Way".to_string())
    //     .interact_text()
    //     .unwrap();

    // println!("Galaxy: {}", mail);
    Ok(())
}
