mod database;
mod models;
mod routes;

use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

use tokio::task;
use std::sync::Mutex;
use sled::Db;

use models::AppState;
use database::GLOBAL_DB;

use env_logger::{Builder, Target};
use log::info;
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

    let _ = HttpServer::new(|| {
        let cors =  Cors::default()
        .allow_any_origin()  // Allow all origins
        .allowed_methods(vec!["GET", "POST", "OPTIONS"])
        .allow_any_header()  // Allow any custom headers
        .supports_credentials()
        .max_age(3600);  // Cache preflight requests for an hour
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(AppState {
                db: GLOBAL_DB.clone(),
            }))
            .configure(routes::setup_routes)  // Modularized Actix routes
    })
    .bind("0.0.0.0:4000")?
    .run().await;

    Ok(())
}
