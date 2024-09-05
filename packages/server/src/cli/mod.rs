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
use voting::{initialize_crisp_round, participate_in_existing_round};
use serde::{Deserialize, Serialize};

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
    let https = HttpsConnector::new();
    let client_get: HyperClientGet = HyperClient::builder(TokioExecutor::new()).build(https.clone());
    let client: HyperClientPost = HyperClient::builder(TokioExecutor::new()).build(https);

    let mut auth_res = AuthenticationResponse {
        response: "".to_string(),
        jwt_token: "".to_string(),
    };

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let selections = &[
        "CRISP: Voting Protocol (ETH)",
        "More Coming Soon!"
    ];

    let selection_1 = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Enclave (EEEE): Please choose the private execution environment you would like to run!")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    if selection_1 == 0 {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        let selections_2 = &[
            "Initialize new CRISP round.",
            "Continue Existing CRISP round."
        ];

        let selection_2 = FuzzySelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Create a new CRISP round or participate in an existing round.")
            .default(0)
            .items(&selections_2[..])
            .interact()
            .unwrap();

        // Read configuration
        let path = env::current_dir().unwrap();
        let mut pathst = path.display().to_string();
        pathst.push_str("/example_config.json");
        let mut file = File::open(pathst).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let config: CrispConfig = serde_json::from_str(&data).expect("JSON was not well-formatted");

        if selection_2 == 0 {
            initialize_crisp_round(&config, &client_get, &client).await?;
        } else if selection_2 == 1 {
            auth_res = authenticate_user(&config, &client).await?;
            participate_in_existing_round(&config, &client, &auth_res).await?;
        }
    } else {
        println!("Check back soon!");
        std::process::exit(1);
    }

    Ok(())
}
