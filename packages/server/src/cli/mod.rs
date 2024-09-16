mod auth;
mod voting;

use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use hyper_tls::HttpsConnector;
use hyper_util::{client::legacy::{Client as HyperClient, connect::HttpConnector}, rt::TokioExecutor};
use bytes::Bytes;
use std::env;
use std::fs::File;
use std::io::Read;
use http_body_util::Empty;

use auth::{authenticate_user, AuthenticationResponse};
use voting::{initialize_crisp_round, participate_in_existing_round, activate_e3_round};
use serde::{Deserialize, Serialize};
use env_logger::{Builder, Target};
use log::LevelFilter;
use log::info;
use std::io::Write; // Use `std::io::Write` for writing to the buffer

fn init_logger() {
    let mut builder = Builder::new();
    builder
        .target(Target::Stdout) // Set target to stdout
        .filter(None, LevelFilter::Info) // Set log level to Info
        .format(|buf, record| {
            writeln!(
                buf, // Use `writeln!` correctly with the `buf`
                "[{}:{}] - {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();
}

type HyperClientGet = HyperClient<HttpsConnector<HttpConnector>, Empty<Bytes>>;
type HyperClientPost = HyperClient<HttpsConnector<HttpConnector>, String>;

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
#[tokio::main]
pub async fn run_cli() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_logger();

    let https = HttpsConnector::new();
    let client_get = HyperClient::builder(TokioExecutor::new()).build::<_, Empty<Bytes>>(https.clone());
    let client = HyperClient::builder(TokioExecutor::new()).build::<_, String>(https);

    clear_screen();

    let environment = select_environment()?;
    if environment != 0 {
        info!("Check back soon!");
        return Ok(());
    }

    clear_screen();

    let config = read_config()?;
    let action = select_action()?;

    match action {
        0 => {
            initialize_crisp_round(&config).await?;
        }
        1 => {
            let auth_res = authenticate_user(&config, &client).await?;
            activate_e3_round(&config).await?;
        }
        2 => {
            let auth_res = authenticate_user(&config, &client).await?;
            participate_in_existing_round(&config, &client, &auth_res).await?;
        }
        _ => unreachable!(),
    }

    Ok(())
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn select_environment() -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
    let selections = &["CRISP: Voting Protocol (ETH)", "More Coming Soon!"];
    Ok(FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Enclave (EEEE): Please choose the private execution environment you would like to run!")
        .default(0)
        .items(&selections[..])
        .interact()?)
}

fn select_action() -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
    let selections = &["Initialize new E3 round.", "Activate an E3 round.", "Continue Existing E3 round."];
    Ok(FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Create a new CRISP round or participate in an existing round.")
        .default(0)
        .items(&selections[..])
        .interact()?)
}

fn read_config() -> Result<CrispConfig, Box<dyn std::error::Error + Send + Sync>> {
    let config_path = env::current_dir()?.join("example_config.json");
    let mut file = File::open(config_path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok(serde_json::from_str(&data)?)
}