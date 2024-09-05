mod routes;
mod models;
mod database;

use iron::prelude::*;
use iron::Chain;
use router::Router;
use iron_cors::CorsMiddleware;

#[tokio::main]
pub async fn start_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut router = Router::new();
    routes::setup_routes(&mut router);

    let cors_middleware = CorsMiddleware::with_allow_any();
    println!("Allowed origin hosts: *");

    let mut chain = Chain::new(router);
    chain.link_around(cors_middleware);
    Iron::new(chain).http("0.0.0.0:4000").unwrap();

    Ok(())
}