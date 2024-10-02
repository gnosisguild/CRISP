mod config;
mod voting;

use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use reqwest::Client;

use config::CONFIG;
use env_logger::{Builder, Target};
use log::{info, LevelFilter, Record};
use voting::{
    activate_e3_round, decrypt_and_publish_result, initialize_crisp_round,
    participate_in_existing_round,
};

use once_cell::sync::Lazy;

use sled::Db;
use std::sync::Arc;
use std::path::Path;
use std::io::Write;
use tokio::sync::RwLock;

pub static GLOBAL_DB: Lazy<Arc<RwLock<Db>>> = Lazy::new(|| {
    let pathdb = std::env::current_dir().unwrap().join("database/cli");
    Arc::new(RwLock::new(sled::open(pathdb).unwrap()))
});

fn init_logger() {
    let mut builder = Builder::new();
    builder
        .target(Target::Stdout)
        .filter(None, LevelFilter::Info)
        .format(|buf, record: &Record| {
            let file = record.file().unwrap_or("unknown");
            let filename = Path::new(file).file_name().unwrap_or_else(|| file.as_ref());

            writeln!(
                buf,
                "[{}:{}] - {}",
                filename.to_string_lossy(),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();
}


#[tokio::main]
pub async fn run_cli() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_logger();

    let client = Client::new();

    clear_screen();

    let environment = select_environment()?;
    if environment != 0 {
        info!("Check back soon!");
        return Ok(());
    }

    clear_screen();

    let action = select_action()?;

    match action {
        0 => {
            initialize_crisp_round().await?;
        }
        1 => {
            activate_e3_round().await?;
        }
        2 => {
            participate_in_existing_round(&client).await?;
        }
        3 => {
            decrypt_and_publish_result(&client).await?;
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
    let selections = &[
        "Initialize new E3 round.",
        "Activate an E3 round.",
        "Participate in an E3 round.",
        "Decrypt Ciphertext & Publish Results",
    ];
    Ok(FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Create a new CRISP round or participate in an existing round.")
        .default(0)
        .items(&selections[..])
        .interact()?)
}
