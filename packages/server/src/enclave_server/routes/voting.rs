
use std::{env, sync::Arc, str};
use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use router::Router;
use std::io::Read;
use ethers::{
    prelude::abigen,
    providers::{Http, Provider, Middleware},
    middleware::{SignerMiddleware, MiddlewareBuilder},
    signers::{LocalWallet, Signer},
    types::{Address, Bytes, TxHash, BlockNumber},
};


use crate::enclave_server::models::{EncryptedVote, JsonResponseTxHash, GetEmojisRequest, VoteCountRequest};
use crate::enclave_server::database::{GLOBAL_DB, get_state};


pub fn setup_routes(router: &mut Router) {
    router.post("/broadcast_enc_vote", broadcast_enc_vote, "broadcast_enc_vote");
    router.post("/get_vote_count_by_round", get_vote_count_by_round, "get_vote_count_by_round");
    router.post("/get_emojis_by_round", get_emojis_by_round, "get_emojis_by_round");
}

#[tokio::main]
async fn broadcast_enc_vote(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    // read the POST body
    req.body.read_to_string(&mut payload).unwrap();
    let incoming: EncryptedVote = serde_json::from_str(&payload).unwrap();
    let mut response_str = "";
    let mut converter = "".to_string();
    let (mut state, key) = get_state(incoming.round_id);

    for i in 0..state.has_voted.len() {
        if state.has_voted[i] == incoming.postId {
            response_str = "User Has Already Voted";
        } else {
            response_str = "Vote Successful";
        }
    };

    if response_str == "Vote Successful" {
        let sol_vote = Bytes::from(incoming.enc_vote_bytes);
        let tx_hash = call_contract(sol_vote, state.voting_address.clone()).await.unwrap();
        converter = "0x".to_string();
        for i in 0..32 {
            if tx_hash[i] <= 16 {
                converter.push_str("0");
                converter.push_str(&format!("{:x}", tx_hash[i]));
            } else {
                converter.push_str(&format!("{:x}", tx_hash[i]));
            }
        }

        state.vote_count = state.vote_count + 1;
        state.has_voted.push(incoming.postId);
        let state_str = serde_json::to_string(&state).unwrap();
        let state_bytes = state_str.into_bytes();
        GLOBAL_DB.insert(key, state_bytes).unwrap();
    };

    let response = JsonResponseTxHash { response: response_str.to_string(), tx_hash: converter };
    let out = serde_json::to_string(&response).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    println!("Request for round {:?} send vote tx", incoming.round_id);
    Ok(Response::with((content_type, status::Ok, out)))
}


fn get_emojis_by_round(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    // read the POST body
    req.body.read_to_string(&mut payload).unwrap();
    let mut incoming: GetEmojisRequest = serde_json::from_str(&payload).unwrap();
    println!("Request emojis for round {:?}", incoming.round_id);

    let (state, _key) = get_state(incoming.round_id);
    incoming.emojis = state.emojis;
    let out = serde_json::to_string(&incoming).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}

fn get_vote_count_by_round(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    // read the POST body
    req.body.read_to_string(&mut payload).unwrap();
    let mut incoming: VoteCountRequest = serde_json::from_str(&payload).unwrap();
    println!("Request vote count for round {:?}", incoming.round_id);

    let (state, _key) = get_state(incoming.round_id);
    incoming.vote_count = state.vote_count;
    let out = serde_json::to_string(&incoming).unwrap();

    let content_type = "application/json".parse::<Mime>().unwrap();
    Ok(Response::with((content_type, status::Ok, out)))
}

async fn call_contract(enc_vote: Bytes, address: String) -> Result<TxHash, Box<dyn std::error::Error + Send + Sync>> {
    println!("calling voting contract");

    let infura_val = env!("INFURAKEY");
    let mut rpc_url = "https://sepolia.infura.io/v3/".to_string();
    rpc_url.push_str(&infura_val);

    let provider = Provider::<Http>::try_from(rpc_url.clone())?;
    // let block_number: U64 = provider.get_block_number().await?;
    // println!("{block_number}");
    abigen!(
        IVOTE,
        r#"[
            function voteEncrypted(bytes memory _encVote) public
            function getVote(address id) public returns(bytes memory)
            event Transfer(address indexed from, address indexed to, uint256 value)
        ]"#,
    );

    //const RPC_URL: &str = "https://eth.llamarpc.com";
    let vote_address: &str = &address;

    let eth_val = env!("PRIVATEKEY");
    let wallet: LocalWallet = eth_val
        .parse::<LocalWallet>().unwrap()
        .with_chain_id(11155111 as u64);

    let nonce_manager = provider.clone().nonce_manager(wallet.address());
    let curr_nonce = nonce_manager
        .get_transaction_count(wallet.address(), Some(BlockNumber::Pending.into()))
        .await?
        .as_u64();

    let client = SignerMiddleware::new(provider.clone(), wallet.clone());
    let address: Address = vote_address.parse()?;
    let contract = IVOTE::new(address, Arc::new(client.clone()));

    let test = contract.vote_encrypted(enc_vote).nonce(curr_nonce).send().await?.clone();
    println!("{:?}", test);
    Ok(test)
}
