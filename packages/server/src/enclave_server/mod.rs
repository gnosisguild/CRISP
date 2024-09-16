mod database;
mod models;
mod routes;
pub mod blockchain;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};

use models::AppState;
use database::GLOBAL_DB;
use blockchain::listener::start_listener;

use env_logger::{Builder, Target};
use log::LevelFilter;
use std::io::Write;

fn init_logger() {
    let mut builder = Builder::new();
    builder
        .target(Target::Stdout) 
        .filter(None, LevelFilter::Info) 
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}:{}] - {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();
}

#[actix_web::main]
pub async fn start_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_logger();

    tokio::spawn(async {
        if let Err(e) = start_listener("0x5FbDB2315678afecb367f032d93F642f64180aa3").await {
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
            .app_data(web::Data::new(AppState {
                db: GLOBAL_DB.clone(),
            }))
            .configure(routes::setup_routes)
    })
    .bind("0.0.0.0:4000")?
    .run().await;

    Ok(())
}
