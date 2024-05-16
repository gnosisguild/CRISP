mod util;

use dialoguer::{theme::ColorfulTheme, Input, FuzzySelect};
use std::{thread, time, env};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

use fhe::{
    bfv::{BfvParametersBuilder, Encoding, Plaintext, PublicKey},
};
use fhe_traits::{FheEncoder, FheEncrypter, Serialize as FheSerialize, DeserializeParametrized};
use rand::{thread_rng};
use util::timeit::{timeit};

use hyper::Request;
use hyper::Method;
use hyper_tls::HttpsConnector;
use hyper_util::{client::legacy::Client as HyperClient, rt::TokioExecutor};
use bytes::Bytes;

use http_body_util::Empty;
use http_body_util::BodyExt;
use tokio::io::{AsyncWriteExt as _, self};


#[derive(Debug, Deserialize, Serialize)]
struct JsonResponse {
    response: String
}

#[derive(Debug, Deserialize, Serialize)]
struct JsonResponseTxHash {
    response: String,
    tx_hash: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct JsonRequestGetRounds {
    response: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct RoundCount {
    round_count: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct JsonRequest {
    response: String,
    pk_share: u32,
    id: u32,
    round_id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct CrispConfig {
    round_id: u32,
    poll_length: u32,
    chain_id: u32,
    voting_address: String,
    ciphernode_count: u32,
    enclave_address: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct PKRequest {
    round_id: u32,
    pk_bytes: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
struct EncryptedVote {
    round_id: u32,
    enc_vote_bytes: Vec<u8>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let https = HttpsConnector::new();
    //let client = HyperClient::builder(TokioExecutor::new()).build::<_, Empty<Bytes>>(https);
    let client_get = HyperClient::builder(TokioExecutor::new()).build::<_, Empty<Bytes>>(https.clone());
    let client = HyperClient::builder(TokioExecutor::new()).build::<_, String>(https);

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

    if selection_1 == 0 {
    	print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    	//println!("Encrypted Protocol Selected {}!", selections[selection_1]);
	    let selection_2 = FuzzySelect::with_theme(&ColorfulTheme::default())
	        .with_prompt("Create a new CRISP round or particpate in an existing round.")
	        .default(0)
	        .items(&selections_2[..])
	        .interact()
	        .unwrap();

	    if selection_2 == 0 {
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
            println!("poll length {:?}", config.poll_length);
            println!("chain id: {:?}", config.chain_id);
            println!("voting contract: {:?}", config.voting_address);
            println!("ciphernode count: {:?}", config.ciphernode_count);

            println!("Initializing Keyshare nodes...");

            let response_id = JsonRequestGetRounds { response: "Test".to_string() };
            let _out = serde_json::to_string(&response_id).unwrap();
            let mut url_id = config.enclave_address.clone();
            url_id.push_str("/get_rounds");

            //let token = Authorization::bearer("some-opaque-token").unwrap();
            //println!("bearer token {:?}", token.token());
            //todo: add auth field to config file to get bearer token
            let req = Request::builder()
                .header("authorization", "Bearer fpKL54jvWmEGVoRdCNjG")
                .method(Method::GET)
                .uri(url_id)
                .body(Empty::<Bytes>::new())?;

            let resp = client_get.request(req).await?;

            println!("Response status: {}", resp.status());

            let body_bytes = resp.collect().await?.to_bytes();
            let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
            let count: RoundCount = serde_json::from_str(&body_str).expect("JSON was not well-formatted");
            println!("Server Round Count: {:?}", count.round_count);

            let round_id = count.round_count + 1;
            let response = CrispConfig { 
                round_id: round_id,
                poll_length: config.poll_length,
                chain_id: config.chain_id,
                voting_address: config.voting_address,
                ciphernode_count: config.ciphernode_count,
                enclave_address: config.enclave_address.clone()
            };
            let out = serde_json::to_string(&response).unwrap();
            let mut url = config.enclave_address.clone();
            url.push_str("/init_crisp_round");
            let req = Request::builder()
                .method(Method::POST)
                .uri(url)
                .body(out)?;

            let mut resp = client.request(req).await?;

            println!("Response status: {}", resp.status());

            while let Some(next) = resp.frame().await {
                let frame = next?;
                if let Some(chunk) = frame.data_ref() {
                    io::stdout().write_all(chunk).await?;
                }
            }
            println!("Round Initialized.");
	    	println!("Gathering Keyshare nodes for execution environment...");
            let three_seconds = time::Duration::from_millis(1000);
            thread::sleep(three_seconds);
            println!("\nYou can now vote Encrypted with Round ID: {:?}", round_id);

	    }
	    if selection_2 == 1 {
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

            let v: Vec<u8> = vec! [0];
            let response_pk = PKRequest { round_id: input_crisp_id, pk_bytes: v };
            let out = serde_json::to_string(&response_pk).unwrap();
            let mut url = config.enclave_address.clone();
            url.push_str("/get_pk_by_round");
            let req = Request::builder()
                .method(Method::POST)
                .uri(url)
                .body(out)?;

            let resp = client.request(req).await?;

            println!("Response status: {}", resp.status());

            let body_bytes = resp.collect().await?.to_bytes();
            let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
            let pk_res: PKRequest = serde_json::from_str(&body_str).expect("JSON was not well-formatted");
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
            if selection_3 == 0 {
                println!("Exiting voting system. You may choose to vote later.");
                vote_choice = 0;
            }
            if selection_3 == 1 {
                vote_choice = 1;
            }
            if selection_3 == 2 {
                vote_choice = 0;
            }
            println!("Encrypting vote.");
            let votes: Vec<u64> = [vote_choice].to_vec();
            let pt = Plaintext::try_encode(&[votes[0]], Encoding::poly(), &params)?;
            let ct = pk_deserialized.try_encrypt(&pt, &mut thread_rng())?;
            println!("Vote encrypted.");
            println!("Calling voting contract with encrypted vote.");

            let request_contract = EncryptedVote { round_id: input_crisp_id, enc_vote_bytes: ct.to_bytes()};
            let out = serde_json::to_string(&request_contract).unwrap();
            let mut url = config.enclave_address.clone();
            url.push_str("/broadcast_enc_vote");
            let req = Request::builder()
                .method(Method::POST)
                .uri(url)
                .body(out)?;

            let resp = client.request(req).await?;

            println!("Response status: {}", resp.status());

            let body_bytes = resp.collect().await?.to_bytes();
            let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
            let contract_res: JsonResponseTxHash = serde_json::from_str(&body_str).expect("JSON was not well-formatted");
            println!("Contract call: {:?}", contract_res.response);
            println!("TxHash is {:?}", contract_res.tx_hash);
	    }

    }
    if selection_1 == 1 {
    	println!("Check back soon!");
    	std::process::exit(1);
    }

    Ok(())
}
