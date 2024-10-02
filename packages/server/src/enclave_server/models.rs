use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sled::Db;
use tokio::sync::RwLock;

pub struct AppState {
    pub db: Arc<RwLock<Db>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonResponse {
    pub response: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterNodeResponse {
    pub response: String,
    pub node_index: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonResponseTxHash {
    pub response: String,
    pub tx_hash: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonRequest {
    pub response: String,
    pub pk_share: Vec<u8>,
    pub id: u32,
    pub round_id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CrispConfig {
    pub round_id: u32,
    pub poll_length: u32,
    pub chain_id: u32,
    pub voting_address: String,
    pub ciphernode_count: u32,
    pub enclave_address: String,
    pub authentication_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RoundCount {
    pub round_count: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PKShareCount {
    pub round_id: u32,
    pub share_id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PKRequest {
    pub round_id: u64,
    pub pk_bytes: Vec<u8>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct CTRequest {
    pub round_id: u64,
    pub ct_bytes: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CRPRequest {
    pub round_id: u32,
    pub crp_bytes: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimestampRequest {
    pub round_id: u32,
    pub timestamp: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PollLengthRequest {
    pub round_id: u32,
    pub poll_length: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VoteCountRequest {
    pub round_id: u32,
    pub vote_count: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SKSShareRequest {
    pub response: String,
    pub sks_share: Vec<u8>,
    pub index: u32,
    pub round_id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct EncryptedVote {
    pub round_id: u32,
    pub enc_vote_bytes: Vec<u8>,
    pub postId: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetRoundRequest {
    pub round_id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetEmojisRequest {
    pub round_id: u32,
    pub emojis: [String; 2],
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SKSSharePoll {
    pub response: String,
    pub round_id: u32,
    pub ciphernode_count: u32, //TODO: dont need this
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SKSShareResponse {
    pub response: String,
    pub round_id: u32,
    pub sks_shares: Vec<Vec<u8>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReportTallyRequest {
    pub round_id: u32,
    pub option_1: u32,
    pub option_2: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WebResultRequest {
    pub round_id: u64,
    pub option_1_tally: u64,
    pub option_2_tally: u64,
    pub total_votes: u64,
    pub option_1_emoji: String,
    pub option_2_emoji: String,
    pub end_time: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AllWebStates {
    pub states: Vec<WebResultRequest>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct E3 {
    // Identifiers
    pub id: u64,
    pub chain_id: u64,
    pub enclave_address: String,
    
    // Status-related
    pub status: String,
    pub has_voted: Vec<String>,
    pub vote_count: u64,
    pub votes_option_1: u64,
    pub votes_option_2: u64,

    // Timing-related
    pub start_time: u64,
    pub block_start: u64,
    pub duration: u64,
    pub expiration: u64,

    // Parameters
    pub e3_params: Vec<u8>,
    pub committee_public_key: Vec<u8>,

    // Outputs
    pub ciphertext_output: Vec<u8>,
    pub plaintext_output: Vec<u8>,

    // Ciphertext Inputs
    pub ciphertext_inputs: Vec<(Vec<u8>, u64)>,

    // Emojis
    pub emojis: [String; 2],
}

#[derive(Debug, Deserialize, Serialize)]
pub struct E3StateLite {
    pub id: u64,
    pub chain_id: u64,
    pub enclave_address: String,
  
    pub status: String,
    pub vote_count: u64,
  
    pub start_time: u64,
    pub duration: u64,
    pub expiration: u64,
  
    pub committee_public_key: Vec<u8>,
    pub emojis: [String; 2],
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Ciphernode {
    pub id: u32,
    pub pk_share: Vec<u8>,
    pub sks_share: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetCiphernode {
    pub round_id: u32,
    pub ciphernode_id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetEligibilityRequest {
    pub round_id: u32,
    pub node_id: u32,
    pub is_eligible: bool,
    pub reason: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthenticationDB {
    pub jwt_tokens: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct AuthenticationLogin {
    pub postId: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthenticationResponse {
    pub response: String,
    pub jwt_token: String,
}

#[derive(Debug, Deserialize)]
pub struct CronRequestE3 {
    pub cron_api_key: String,
}