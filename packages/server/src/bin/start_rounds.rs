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

use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;


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
    authentication_id: String,
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
    postId: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct AuthenticationLogin {
    postId: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct AuthenticationResponse {
    response: String,
    jwt_token: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let https = HttpsConnector::new();
    //let client = HyperClient::builder(TokioExecutor::new()).build::<_, Empty<Bytes>>(https);
    let client_get = HyperClient::builder(TokioExecutor::new()).build::<_, Empty<Bytes>>(https.clone());
    let client = HyperClient::builder(TokioExecutor::new()).build::<_, String>(https);
    let mut auth_res = AuthenticationResponse {
        response: "".to_string(),
        jwt_token: "".to_string(),
    };

    loop {
    	print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    	println!("Starting new CRISP round!");

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
            .method(Method::GET)
            .uri(url_id)
            .body(Empty::<Bytes>::new())?;

        let resp = client_get.request(req).await?;

        println!("Response status: {}", resp.status());

        let body_bytes = resp.collect().await?.to_bytes();
        let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
        let count: RoundCount = serde_json::from_str(&body_str).expect("JSON was not well-formatted");
        println!("Server Round Count: {:?}", count.round_count);

        // TODO: get secret from env var
        // let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret")?;
        // let mut claims = BTreeMap::new();
        // claims.insert("postId", config.authentication);
        // let mut bearer_str = "Bearer ".to_string();
        // let token_str = claims.sign_with_key(&key)?;
        // bearer_str.push_str(&token_str);
        // println!("{:?}", bearer_str);

        let round_id = count.round_count + 1;
        let response = CrispConfig { 
            round_id: round_id,
            poll_length: config.poll_length,
            chain_id: config.chain_id,
            voting_address: config.voting_address,
            ciphernode_count: config.ciphernode_count,
            enclave_address: config.enclave_address.clone(),
            authentication_id: config.authentication_id.clone(),
        };
        let out = serde_json::to_string(&response).unwrap();
        let mut url = config.enclave_address.clone();
        url.push_str("/init_crisp_round");
        let req = Request::builder()
            //.header("authorization", "Bearer fpKL54jvWmEGVoRdCNjG")
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

        let next_round_start = config.poll_length + 60;
        let seconds = time::Duration::from_secs(next_round_start as u64);
        thread::sleep(seconds);
    }

    Ok(())
}
