mod routes;
mod models;
mod database;

use iron::prelude::*;
use iron::Chain;
use router::Router;
use iron_cors::CorsMiddleware;

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


#[tokio::main]
pub async fn start_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_logger();

    info!("Starting server...");
    let mut router = Router::new();
    routes::setup_routes(&mut router);

    let cors_middleware = CorsMiddleware::with_allow_any();
    info!("Allowed origin hosts: *");

    let mut chain = Chain::new(router);
    chain.link_around(cors_middleware);
    Iron::new(chain).http("0.0.0.0:4000").unwrap();

    Ok(())
}