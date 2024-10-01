pub mod blockchain;
mod config;
mod database;
mod models;
mod routes;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};

use blockchain::listener::start_listener;
use blockchain::sync::sync_contracts_db;
use database::GLOBAL_DB;
use models::AppState;

use config::CONFIG;
use env_logger::{Builder, Target};
use log::{LevelFilter, Record};
use std::io::Write;
use std::path::Path;

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

#[actix_web::main]
pub async fn start_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_logger();
    if let Err(e) = sync_contracts_db().await {
        eprintln!("Failed to sync contracts: {:?}", e);
    }

    tokio::spawn(async {
        if let Err(e) = start_listener(&CONFIG.ws_rpc_url, &CONFIG.enclave_address, &CONFIG.ciphernode_registry_address).await {
            eprintln!("Listener failed: {:?}", e);
        }
    });

    let _ = HttpServer::new(|| {
        let cors =  Cors::default()
        .allow_any_origin()
        .allowed_methods(vec!["GET", "POST", "OPTIONS"])
        .allow_any_header()
        .supports_credentials()
        .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::new(r#"%a "%r" %s %b %T"#))
            .app_data(web::Data::new(AppState {
                db: GLOBAL_DB.clone(),
            }))
            .configure(routes::setup_routes)
    })
    .bind("0.0.0.0:4000")?
    .run().await;

    Ok(())
}
