use config::{Config as ConfigManager, ConfigError};
use dotenvy::dotenv;
use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub private_key: String,
    pub http_rpc_url: String,
    pub ws_rpc_url: String,
    pub enclave_address: String,
    pub e3_program_address: String,
    pub ciphernode_registry_address: String,
    pub naive_registry_filter_address: String,
    pub chain_id: u64,
    pub cron_api_key: String,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv().ok();
        ConfigManager::builder()
            .add_source(config::Environment::default())
            .build()?
            .try_deserialize()
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    Config::from_env().expect("Failed to load configuration")
});